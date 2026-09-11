# Binary Market Data Format Specification

Complete Data Structure Layout for FEED_DETAILS Stream Engine

**File Type:** Binary Capture (.bin) **Byte Order:** Little-Endian **Payload Struct:** FEED_DETAILS **Depth Level:** FEED_LEVEL_DEPTH = 5 **Struct Size:** 216 Bytes (Unpacked) **Version:** 2.0 (Clean Build)

## 1. Technical Summary

This specification defines the byte-level protocol for high-frequency Level-2 order book persistence. Market depth updates and execution events are serialized sequentially, pairing core instrument identification metadata with precise distribution latency timestamps.

### Structural Features

The FEED_DETAILS architecture minimizes runtime parsing overhead by combining symbol metadata, order depth levels (top 5 bids and asks), and end-to-end timing metrics (Source, Distribution Receive, and Distribution Send timestamps) into a single contiguous structure.

## 2. Stream Framing & Framing Envelope

The binary capture stream uses length-prefixed framing blocks. Each record frame starts with an explicit 8-byte signed integer detailing the length of the binary payload that directly follows.

**BINARY STREAM ENVELOPE STRUCT** 
**Header: nRead Payload Block (Length = nRead Bytes)** 8 Bytes (int64_t) [ Capture Timestamp (8B)] + [ FEED_DETAILS Data Structure (208B)]

|Field Name|Data Type|Size (Bytes)|Description|
|---|---|---|---|
|nRead|int64_t|8|Length identifier specifying total bytes in the succeeding payload|

frame. RcTime uint64_t 8 Capture timestamp recorded at stream intake. FEED_DETAILS struct 208 Full market depth update payload.

## 3. Binary Field Layout & Alignment Mapping

### Memory Padding Notice

The structure utilizes compiler default packing (unpacked). On 64-bit architectures, a 4-byte padding offset is automatically inserted following the bitfield members to enforce 8-byte natural alignment for subsequent 64-bit timestamp fields.

Technical Specification: FEED_DETAILS Binary Format Page 1 of 2

|0x0B2|LastTradedPrice|4 int32_t|Execution price of the most recent trade|
|---|---|---|---|
|0x0B6|Bitfield Area|Bitfields 2|LastTradedQty:12, FeedEventAggressor:2|
|0x0B8|Structure Padding|4 uint8_t[4]|Automatic alignment padding added by compiler|
|0x0BC|SrcTimestamp|8 uint64_t|Exchange packet emission timestamp|

|Offset|Field Name|Data Type|Bytes|Description|
|---|---|---|---|---|
|0x000|SymbolCode|char[50]|50|Null-terminated ticker symbol string identifier|
|0x032|Index|int32_t|4|Internal instrument index key|
|0x036|ExpiryDate|int32_t|4|Contract expiration date format (YYYYMMDD)|
|0x03A|BuyPrice[5]|int32_t[5]|20|Bid prices for top 5 market depth levels|
|0x04E|BuyQty[5]|uint32_t[5]|20|Bid quantities for top 5 market depth levels|
|0x062|SellPrice[5]|int32_t[5]|20|Ask prices for top 5 market depth levels|
|0x076|SellQty[5]|uint32_t[5]|20|Ask quantities for top 5 market depth levels|
|0x08A|NoOfBuyOrds[5]|int32_t[5]|20|Buy order count across top 5 depth levels|
|0x09E|NoOfSellOrds[5]|int32_t[5]|20|Sell order count across top 5 depth levels|

0x0C4 DS_rcv_timestamp uint64_t 8 Distribution server intake timestamp 0x0CC DS_snd_timestamp uint64_t 8 Distribution server dispatch timestamp

Technical Specification: FEED_DETAILS Binary Format Page 2 of 2
