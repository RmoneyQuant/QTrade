# Trading Engine

A single engine that runs the same compiled code for backtesting and live trading, market making on MCX commodity futures using CME and DGCX as pricing signals. This glossary fixes the vocabulary; the design is in [ARCHITECTURE.md](ARCHITECTURE.md) and the reasoning in [ARCHITECTURE-DECISIONS.md](ARCHITECTURE-DECISIONS.md).

## Language

### System and modes

**Backtest Mode**:
A run driven by recorded data against a Simulated Exchange.
_Avoid_: backtest (as a noun for the system), simulation mode

**Live Mode**:
A run driven by real feeds against a real Exchange Gateway.
_Avoid_: production mode, real mode

**qtrade**:
The exchange- and mode-agnostic engine: Scheduler, Data Engine, BookBuilder, Cache, strategy host, ExecutionEngine, RMS and Order Validation. Feed Adapters and the venue edge plug into it; they are not part of it.
_Avoid_: Core, Engine, engine core, kernel, backtester

**qtrade process**:
One running instance — qtrade plus whichever Feed Adapters and venue edge the configuration wires in.
_Avoid_: Engine, application, node, trading node

**CPU core**:
A physical processor core. Always written in full.
_Avoid_: core

**Run**:
One execution of qtrade over a defined data range with a fixed configuration and strategy set.
_Avoid_: test, session, simulation

### Data path

**Feed Adapter**:
One Transport, Decoder and Normalizer for a single provider and protocol. May serve several Venues.
_Avoid_: exchange adapter, connector

**Transport**:
The component producing framed messages from a source, whether a socket or a recorded file.
_Avoid_: data engine, reader, feed handler

**Decoder**:
The component translating one venue protocol's wire format into structured messages.
_Avoid_: parser

**Normalizer**:
The component converting decoded messages into internal event types, resolving epochs, price scales and instrument identity.
_Avoid_: translator, mapper

**Data Engine**:
The component owning subscriptions, session phase tracking and book state. Distinct from Transport.
_Avoid_: feed manager

**Sequencer**:
The component merging several ordered streams into one totally-ordered stream.
_Avoid_: merger, arbitrator

**Recorder**:
The component capturing raw messages and timestamps to durable storage during a Live run.
_Avoid_: capture, logger

**Journal**:
The record of the post-merge stream in qtrade consumption order, together with every command and report, used to verify that a Live run replays identically.
_Avoid_: log, audit trail

**Stream**:
One multicast channel carrying a subset of a venue's instruments. The venue's own term.
_Avoid_: channel (when meaning a stream), feed

**Feed**:
A venue's whole market data source.
_Avoid_: stream, datafeed

### Market data

**Venue**:
The exchange an instrument belongs to.
_Avoid_: exchange (when precision matters), market

**Instrument**:
A single tradable contract, identified internally by an interned `InstrumentId`.
_Avoid_: symbol, contract (when the internal identity is meant), security

**Order Book**:
The set of resting orders for one instrument, maintained by the BookBuilder.
_Avoid_: LOB, book depth, depth

**MBO**:
Order-by-order market data, where every resting order is individually visible.
_Avoid_: full depth, tick-by-tick

**MBP**:
Aggregated market data, where only totals per price level are visible.
_Avoid_: level 2, depth data

**Book State**:
Whether a book is trustworthy: `UNINIT`, `RECOVERING`, `OK` or `STALE`.
_Avoid_: book status, health

**Session State**:
Whether a venue is trading: open, closed, halted, in auction, or in maintenance break. Independent of Book State.
_Avoid_: market status, phase

**Warmup**:
The period after a run starts during which a strategy receives events and updates state but may not quote.
_Avoid_: burn-in, initialisation

### Trading

**Strategy**:
User-written trading logic, identical in both modes.
_Avoid_: algo, model

**Simulated Exchange**:
The Backtest Mode venue, holding its own order books and reachable only through order commands and execution reports.
_Avoid_: simulated fill engine, matching engine, simulator

**Exchange Gateway**:
The Live Mode venue adapter.
_Avoid_: order gateway, OMS

**Standard order**:
A persistent order that survives a session reconnect and supports GTC and GTD validity.
_Avoid_: normal order, persistent order

**Lean order**:
A non-persistent, session-scoped order, cancelled automatically at end of day or on session loss. The category used for quoting.
_Avoid_: light order, transient order

**Book-or-Cancel**:
An order instruction rejecting the order outright if it would execute immediately rather than resting.
_Avoid_: post-only, BOC (on first use)

**Market-to-Limit**:
An order executing against available liquidity and converting any unfilled remainder to a resting limit at the traded price.
_Avoid_: market order

**Freeze Quantity**:
The per-contract order size above which the venue will not accept an order.
_Avoid_: max order size, order limit

**Queue Position**:
The volume resting ahead of one's own order at its price level.
_Avoid_: priority, place in queue

**Skew**:
Deliberate displacement of both quotes away from fair value to encourage inventory-reducing fills.
_Avoid_: lean, tilt, bias

**Inventory**:
The net position accumulated through market making, as distinct from a deliberately sought position.
_Avoid_: position (when the unwanted sense is meant), holdings

**OTR**:
The ratio of orders submitted to trades executed, which venues cap and penalise.
_Avoid_: order-to-trade, message ratio

**STP**:
Prevention of one's own buy and sell orders executing against each other.
_Avoid_: self-match prevention, wash prevention

**MMP**:
A venue mechanism automatically cancelling a market maker's orders once fills exceed a configured threshold within a time window.
_Avoid_: market maker protection (after first use), kill switch

### Accounting and measurement

**Sub-account**:
One strategy's own position, inventory and profit attribution.
_Avoid_: strategy account, book (in the accounting sense)

**Firm Aggregate**:
The netted firm-level position, exposure and margin across all strategies.
_Avoid_: global position, total position, house account

**Order Validation**:
Stateless per-order checks against instrument reference data, answering whether the venue would accept the order at all.
_Avoid_: pre-trade risk, validation

**Risk Limit**:
A stateful, portfolio-scoped policy constraint on position, exposure or loss.
_Avoid_: limit, check

**Cost Model**:
The component computing transaction costs, queryable before quoting and applied to fills.
_Avoid_: fee model, commission

**Markout**:
The movement of the mid price at fixed horizons after a fill, used to measure adverse selection.
_Avoid_: slippage, post-trade drift

**Latency Model**:
The component determining how long orders and reports take to travel between qtrade and a venue.
_Avoid_: delay model, latency simulation

### Industry terms mapped

**OMS**:
Not used as a term here. What the industry calls an order management system is split across the **ExecutionEngine** (lifecycle logic) and the **Cache** (order state). Naming it as one component would imply a single thing that does not exist.
_Avoid_: OMS, order manager

**EMS**:
Not used. Order routing is a single venue per order with no smart routing, and parent/child order slicing is deliberately out of scope.
_Avoid_: EMS, execution management system

**Pre-trade risk**:
Split into two distinct gates that must not be conflated: **Order Validation** (stateless, will the venue accept it) and **RMS** (stateful policy, should we send it).
_Avoid_: pre-trade risk as a single concept
