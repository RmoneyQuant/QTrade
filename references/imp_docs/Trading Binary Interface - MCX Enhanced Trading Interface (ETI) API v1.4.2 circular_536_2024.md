# Trading Binary Interface - MCX Enhanced Trading Interface (ETI) API v1.4.2

# ______________________________________________________________________

In terms of provisions of the Rules, Bye-Laws and Business Rules of the Exchange and in 
continuation to Exchange, MCX/CTCL/416/2021 dated July 20, 2021, Circular no.: 
MCX/CTCL/271/2023 dated April 25, 2023,  Circular no.: MCX/CTCL/ 934/2023  dated
December 27, 2023, Circular no.:  MCX/CTCL/134/2024 dated  March 11, 2024, Circular 
no.: MCX/CTCL/265/2024  dated  April 29, 2024 and Circular no. MCX/CTCL/281/2024
dated April 30, 2024, Members of the Exchange are notified as under:

Trading Members and  Vendors are requested to find  Enhanced Trading Interface  - ETI
API 1.4.2, changes are provided in the document revision list.

Users are requested to make the changes in their application with respect to **IV Charset** 
**and Sequential Encryption/Decryption** which will be made available in Test 
environment shortly. Implementation in Live environment will be communicated 
separately.

For and on behalf of

Multi Commodity Exchange of India Ltd.

Abhay Angarkar

VP - Technology

Encl.: As below

Kindly contact Customer Service Team on 022  – 6649 4040 or send an email at 
customersupport@mcxindia.com for any clarification.

---

# Multi Commodity Exchange of India Limited

# Trading Binary Interface - MCX Enhanced Trading Interface (ETI)

Version 1.4.2

14 Aug 2024

---

## Copyright

All trademarks that appear in the document have been used for identification purposes only and belong to their 
respective companies.

**Document details**

| Name | Version no. | Description |
| --- | --- | --- |
| MCX_Trading_Interface_ETI_API | V 1.4.2 | API documentation for Trading Binary Interface |

## Document Revision List

| Revision No. | Revision Date | Revision Description |
| --- | --- | --- |
| 1 | 09-Jul-2021 | • Creation of Base Version |
| 2 | 02-May-2022(V1.1) | • Addition of new messages structure- 11028,10311,10313,4100,10102,10108,10111. • New enum added in tag 28703. • New enum added in tag 18. • New enum added in tag 625. • New fields 717 and 39040 added in message structure- 8030,8031,8033,8034,8036,8037. • New fields 378, 39040 added in message structure- 8040. • New field 25009 added in message structure-10991. • New field 1300 added in message structure- 8005. • New enums added in tag 28721. • Updated length of tag 554. • Updated field ComplianceText with field UserReferenceText. • Removed tag 25019. |
|  |  | • Updated tag 712 with tag 20655 in message structure 10100. • Updated tag 709,712,716 with tag 30737,30738,770 respectively -10117. • Removed tag 713 in message structure-10117. • Padding structure changed for message structure-8005. • Updation of tag 710 against field EncryptedDataMessageSize in message structure- 10997, 10996. • Updated description of section 3 and 4.15. • Updated Template id to 10990 from 10043. • Updated description of section 4.24.2. • Updated description of section 4.22.2.4 |

---

| Revision No. | Revision Date | Revision Description |
| --- | --- | --- |
| 3 | 22-Nov-2022(V1.2) | • Addition of MarginChangeRptGrp in message structure- 4025. • New enum added in tag 40 in message structure- 10991. • Updated description of message structure-10125. • Updated description of message structure under section 5.4. • Updated description of message structure-8005. • Added enum in tag 28587 in message structure-10500. • Updated description of section 4.23.10.1 • Updated flow diagram of message structure- 10991. • Updated description of tag 25001. • Updated header of message structure- 4125. • Updated description of tag 50. • Updated field mLReferenceNumber with tag 709. • Updated description of section 4.22.2.4 • Updated sample C++ code in section 6. • Updated description of tag 11. • Updated flow diagram of message structure-10025. • Updated Required as Mandatory for tag 1300. • Updated sequence of message structure- 8100,8101,10994. • Updated tag 18 in message structure- 10125,10126. |
|  |  | • Updated tag 1227 of message structure- 10102. • Removed enums from tag 786. • |
| 4 | 02-May-2023(V1.3) | • Addition of field tag 719 in message structure- 10102,10108. • Updated description of section 4.22.2.4 • Updated description of section 4.5 • Updated description of enum value 215 in tag 378. • Updated description of tag 707. |

---

| RevisionNo. | Revision Date | Revision Description |
| --- | --- | --- |
| 5 |  | • Updated description of section 4.14.1.• Addition of enums in tag 378 in message structure-10101,10103,10117,10993,10994,8040.• Updated description of section 4.10.• Updated description of tag 28782 in message structure-10121.• Update length of tag 28783 in message structure-10121.• Updated description of tag 718 and 1369.• Updated field name for tag 21002,7765.• Updated length and field name of tag 21003.• Updated description of section 4.22.2.4• Updated description of message structure-10025• Removed tag 39040 from message structure- 10121.• Removal of enum from tag 378 from messagestructure-10107,10101,10103,10117,1099310994,8040.• Removal of enum value from tag 39 in messagestructure-10101,10107,10110,10117,10104,10102,10108,10111• Removal of enum value from tag 59 from messagestructure- 10125• |
| 5 | 03-Nov-2023(V1.4) | • Updated description of section 4.14.1.• Addition of enums in tag 378 in message structure-10101,10103,10117,10993,10994,8040.• Updated description of section 4.10.• Updated description of tag 28782 in message structure-10121.• Update length of tag 28783 in message structure-10121.• Updated description of tag 718 and 1369.• Updated field name for tag 21002,7765.• Updated length and field name of tag 21003.• Updated description of section 4.22.2.4• Updated description of message structure-10025• Removed tag 39040 from message structure- 10121.• Removal of enum from tag 378 from messagestructure-10107,10101,10103,10117,1099310994,8040.• Removal of enum value from tag 39 in messagestructure-10101,10107,10110,10117,10104,10102,10108,10111• Removal of enum value from tag 59 from messagestructure- 10125• |
| 6 | 11-Mar-2024(V1.4) | • Addition of Section 4.24.3 Encryption/Decryption forInteractive API• Addition of New Tag 307574, 307575, 39020 inmessage template id 10021• Addition of Annexure for Encryption/Decryption withexample code snippets in C++ and C# |
| 7 | 29-April-2024(V1.4.1) | • In Session logon (template ID-10000) Tag 50 i.e.SenderSubID is mandatory for Encryption/Decryption. |
| 8 | • Trading Session Event 10307 - 105=Service Resumedenum added in TradSesEvent 1368 field.• Header file updated with enum 105 Service Resumed• 4.24.3 Encryption/Decryption for Interactive API –Note added in example for Charset.• Note added regarding OpenSSL version in section4.24.314-Aug-2024(V1.4.2)• 4.24.3.1 Sequential Encryption/Decryption sectionadded• Retransmit 10008, Gap Fill 10032 - 13 =  TradeEnhancement enum removed from RefApplID (1355)• Replace Order Response 10107 – 102 Modify OrderAccepted enum added in field 378ExecRestatementReason |  |

---

## MCX Contact Details

| The Exchange’s Member &amp; Vendor may contact Technology Division to seek clarification at: |  |
| --- | --- |
| Multi Commodity Exchange of India Limited Exchange Square, Suren Road, Chakala, Andheri (East), Mumbai 400 093. www.mcxindia.com http://www.mcxindia.com/ | Tel: +91 – 22 – 66494000 / 67318888 Fax: +91 – 22 – 66494151 Email – apisupport@mcxindia.com mailto:apisupport@mcxindia.com |

## Restriction on Use and Disclaimer of Information and Data

All The information contained in this document constitutes a trade secret and/or information that are 
commercial or financial and confidential or privileged. It is furnished in confidence with the understanding 
that it will not, without the prior written permission of MCX, be used or disclosed for other than allowed 
purposes.

The copyright in this work may be vested with MCX and / or its suppliers. No part of this document may 
be copied, reproduced, stored in a retrieval system, or transmitted, in any form or by any means whether, 
electronic, mechanical, or otherwise without the prior written permission of MCX.

The recipient acknowledges that MCX and its suppliers may have copyright in the work. The recipient 
further agrees that the work is confidential information and contains proprietary MCX information 
belonging to MCX and / or its suppliers. The recipient manifests, by its receipt of the work, its 
acknowledgment of MCX and / or its suppliers copyright in the work, its acceptance that the work is 
confidential information, and its compliance with the terms contained in this notice. Although MCX has 
made every effort to provide accurate information at the date of publication, it does not give any 
representations or warranties as to the accuracy, reliability or completeness of the information in this 
document. Accordingly, MCX, its subsidiaries and their employees,  officers and contractors and its 
suppliers shall not, to the extent permitted by law, be liable for any direct or indirect loss arising in any

way (including by way of negligence) from or in connection with anything provided in or omitted from this 
document or from any action taken, or inaction, in reliance on this document.

MCX reserves the right to amend details in this document at any time and without notice. Strictly for private 
circulation only.

This document must not be circulated to other users without prior permission of MCX.

---

## Contents

**1.List of Abbreviations..................................................................................................................................11**
**2. Introduction................................................................................................................................................12**
**3.Technical Overview....................................................................................................................................13**
**3.1 Standard...................................................................................................................................................13**
**3.2 Session Oriented.....................................................................................................................................13**
**3.3 Architecture Throttle...............................................................................................................................13**
**4. Service Description...................................................................................................................................14**
**4.1 FIX Semantics..........................................................................................................................................14**
**4.2 Party Identification..................................................................................................................................14**
**4.3 Security Identification.............................................................................................................................16**
**4.4 Order Types..............................................................................................................................................17**
**4.5 Order Quantity.........................................................................................................................................19**
**4.6 Cancellation.............................................................................................................................................19**
**4.7 Modification..............................................................................................................................................19**
**4.8 Disclosed Quantity (MaxShow ) quantity modification......................................................................20**
**4.9 Total Order Quantity Modification ........................................................................................................20**
**4.10 SMPF Self-Match Prevention Functionality.......................................................................................20**
**4.11 Order Mass Cancellation......................................................................................................................21**
**4.12 Text Fields..............................................................................................................................................21**
**4.13 Terminal Info..........................................................................................................................................21**
**4.14 Order Status and Execution Report ...................................................................................................22**
**4.14.1Square Off Suspended Orders..........................................................................................................23**
**4.15 Order Book Restatement......................................................................................................................23**
**4.16 Trade Notifications................................................................................................................................24**
**4.16.1 Trade Characteristics........................................................................................................................24**
**4.16.2 Trade Reconciliation..........................................................................................................................24**
**4.17 Trade Enhancement Notification ........................................................................................................25**
**4.18 Listener Broadcast................................................................................................................................25**
**4.19 News........................................................................................................................................................25**
**4.20 Timestamps............................................................................................................................................26**
**4.21 Strategy ID and STSN...........................................................................................................................26**
**4.22 Connectivity and Session Parameters...............................................................................................27**
**4.22.1.1 Session Concept.............................................................................................................................27**
**4.22.1.2 User Authentication........................................................................................................................27**
**4.22.1.3 Identification and Authentication .................................................................................................28**
**4.22.1.4 IP Addresses and Ports..................................................................................................................28**
**4.22.1.5 Session Authentication..................................................................................................................28**
**4.22.1.6 Password Management..................................................................................................................29**
**4.22.3 Throughput Limits..............................................................................................................................30**
**4.22.3.1 Transaction limit .............................................................................................................................30**
**4.22.3.2 Reject/Disconnect Limit.................................................................................................................30**

---

**4.23 Session Layer........................................................................................................................................31**
**4.23.1 Flat Binary Encoding.........................................................................................................................31**
**4.23.2 Logon...................................................................................................................................................31**
**4.23.3 Logout..................................................................................................................................................31**
**4.23.4 Heartbeat.............................................................................................................................................32**
**4.23.5 Reject...................................................................................................................................................32**
**4.23.6 Message Sequence Number.............................................................................................................33**
**4.23.7 Application Message Sequencing...................................................................................................33**
**4.23.7.1 Application Message Identifier......................................................................................................33**
**4.23.7.2 Application Message Sequence Number.....................................................................................34**
**4.23.8 Session Data.......................................................................................................................................34**
**4.23.9 Broadcast...........................................................................................................................................35**
**4.23.10.1 Retransmission.............................................................................................................................37**
**4.23.10.2 Best Practices for Order Handling..............................................................................................39**
**4.24 Message Formats..................................................................................................................................40**
**4.24.1 Message Fragmentation....................................................................................................................40**
**4.24.2 Data Types ..........................................................................................................................................41**
**4.24.3 Encryption/Decryption for Interactive API......................................................................................43**
**4.24.3.1 Sequential Encryption/Decryption................................................................................................44**
**5. Message Formats......................................................................................................................................47**
**5.1 Session Layer..........................................................................................................................................47**
**5.1.1 Connection Gateway Request-10020................................................................................................47**
**5.1.2 Connection Gateway Response-10021.............................................................................................48**
**5.1.3 Session Logon-10000..........................................................................................................................50**
**5.1.4 Session Logon Response-10001.......................................................................................................53**
**5.1.5 Session Logout-10002.........................................................................................................................55**
**5.1.6 Session Logout Response-10003......................................................................................................56**
**5.1.7 User Logon-10018................................................................................................................................57**
**5.1.8 User Logon Response-10019.............................................................................................................57**
**5.1.9 User Logout-10029...............................................................................................................................58**
**5.1.10 User Logout Response-10024..........................................................................................................59**
**5.1.11 Throttle Update Notification-10028.................................................................................................60**
**5.1.12 Heartbeat-10011 ................................................................................................................................60**
**5.1.13 Heartbeat Notification-10023...........................................................................................................61**
**5.1.14 Session password change-10997....................................................................................................61**
**5.1.15 Session password change Response-10995.................................................................................62**
**5.1.16 User password change-10996..........................................................................................................62**
**5.1.17 User Password Change Response - 10990 ....................................................................................63**
**5.1.18 Session Logout Notification-10012 .................................................................................................63**
**5.1.19 User Logout Notification-10043 .......................................................................................................64**
**5.2 Order Handling ........................................................................................................................................64**
**5.2.1 New Order Single-10100......................................................................................................................64**
**5.2.2 New Order Single (short layout)-10125.............................................................................................70**
**5.2.3 New Order Response (Standard Order)-10101.................................................................................75**

---

**5.2.4 Replace Order Single-10106...............................................................................................................78**
**5.2.5 Replace Order Single (short layout)-10126.......................................................................................83**
**5.2.6 Replace Order Response (Standard Order)-10107..........................................................................88**
**5.2.7 Reject-10010..........................................................................................................................................91**
**5.2.8 Cancel Order Single-10109.................................................................................................................93**
**5.2.9 Cancel Order Response (Standard Order)-10110............................................................................94**
**5.2.10 Immediate Execution Response-10103...........................................................................................96**
**5.2.11 Extended Order Information-10117 ...............................................................................................101**
**5.2.12 Book Order Execution-10104..........................................................................................................110**
**5.2.13 Order Mass Cancellation Request-10120......................................................................................116**
**5.2.14 Order Mass Cancellation Response-10121 ..................................................................................119**
**5.2.15 Order Mass Cancellation Response No Hits-10124....................................................................121**
**5.2.16 Delete All Order Broadcast-10122 .................................................................................................122**
**5.2.17 New Order Response (Lean Order)-10102....................................................................................126**
**5.2.18 Replace Order Response (Lean Order)-10108.............................................................................129**
**5.2.19 Cancel Order Response (Lean Order)-10111..............................................................................132**
**5.3 Multileg order Handling........................................................................................................................134**
**5.3.1 New Order Complex-10113...............................................................................................................134**
**5.3.2 Replace Order Complex-10114.........................................................................................................139**
**5.3.3 Cancel Order complex-10123...........................................................................................................145**
**5.3.4 New Order MultiLeg-10991................................................................................................................147**
**5.3.5 Immediate Execution Response-10993...........................................................................................152**
**5.3.6 Extended Order Information-10994 .................................................................................................155**
**5.3.7 Reject Multileg-10992.........................................................................................................................162**
**5.3.8 Cancel Order Notification-10112......................................................................................................164**
**5.4 Ex/Dex.....................................................................................................................................................167**
**5.4.1 Ex/Dex Entry Request........................................................................................................................167**
**5.4.2 Ex/Dex Entry Confirmation...............................................................................................................169**
**5.4.3 Ex/Dex Modification Request ...........................................................................................................171**
**5.4.4 Ex/Dex Modification Confirmation...................................................................................................173**
**5.4.5 Ex/Dex Cancellation Request...........................................................................................................175**
**5.4.6 Ex/Dex Cancellation Confirmation...................................................................................................177**
**5.4.7 Ex/Dex Notification.............................................................................................................................179**
**5.5.1 Trade Modification Request-8005....................................................................................................182**
**5.5.2 Trade Modification Response – 8010..............................................................................................185**
**5.5.3 Trade Enhancement Notification-10989..........................................................................................187**
**5.5.4 Trade Modification Notification........................................................................................................190**
**5.5.5 Resubmit for Approval Request ( 8500 ).........................................................................................192**
**5.5.6 Resubmit for Approval Confirmation (8510) ..................................................................................193**
**5.5.7 Trade Notification-10500...................................................................................................................195**
**5.6 Others .....................................................................................................................................................202**
**5.6.1 Subscribe-10025.................................................................................................................................202**
**5.6.2 Subscribe Response-10005..............................................................................................................204**
**5.6.3 Retransmit-10008...............................................................................................................................204**

---

**5.6.4 Retransmit Response-10009.............................................................................................................207**
**5.6.6 Retransmit Response (Order Event)-10027....................................................................................212**
**5.6.7 Gap Fill-10032.....................................................................................................................................214**
**5.6.8 Unsubscribe-10006............................................................................................................................216**
**5.6.9 Unsubscribe Response-10007..........................................................................................................217**
**5.6.10 Mass Cancellation Event-10308.....................................................................................................218**
**5.6.11 Trade Notification Status-10501.....................................................................................................220**
**5.6.12 Trading Session Event-10307.........................................................................................................222**
**5.6.13 News-10031.......................................................................................................................................224**
**5.6.14 Market Wide OI Violation-8100.......................................................................................................226**
**5.6.15 Market Wide OI Alert-8101 ..............................................................................................................229**
**5.6.16 Market Status Notification-4125....................................................................................................231**
**5.6.17 General Messages-Margin Change Notification – 4025..............................................................233**
**5.6.18 Service Availability-10030...............................................................................................................235**
**5.6.19 Auction Notification-11028 .............................................................................................................238**
**5.6.20 Inquire Pre-Trade Risk Limits Request-10311 .............................................................................240**
**5.6.21 Pre-Trade Risk Limit Response-10313..........................................................................................241**
**5.6.22 System Information Download-4100.............................................................................................243**
**6. Appendix...................................................................................................................................................245**
**7. Annexure for Encryption/Decryption....................................................................................................251**

---

## 1.List of Abbreviations

Please find a list of all the abbreviations used.

| UDF | User-defined field that is not part of the official FIXSpecification |
| --- | --- |
| RRM | Risk Reducing Mode |
| IOC | Immediate or Cancel Order |
| HOVC | High Order Value Check |
| SMPF | Self-Match Prevention |
| T7 | T7 trading system developed by Deutsche Börse Group |

---

## 2. Introduction

The MCX Exchange’s Trading Architecture is the trading platform of the Deutsche Börse Group, its 
affiliates, and partners.

The MCX Enhanced Trading Interface (MCX ETI) is the interface designed for participants who require 
the highest throughput and the lowest latency for their transactions.

All application messages between the client and the MCX ETI follow FIX V5.0 SP2 semantics including 
all officially approved extension packs.

A proprietary session layer with flat binary encoding is used in order to interface with the exchange

The ETI provides all trading functions of T7:

• Order Handling

• Execution Notifications

• Drop copy for complete order history of all standard orders of a session The following trading 
support information may be subscribed by each session:

General messages from MCX market supervision or OI and risk related messages Private risk control 
messages are automatically sent to each session of the corresponding trading and clearing business 
unit.

The MCX ETI does not provide any market data, reference data, or administrative functions.

---

## 3.Technical Overview

## 3.1 Standard

The MCX ETI is predominantly derived from the industry standard FIX protocol. The best practices of 
FIX have been adopted.

## 3.2 Session Oriented

The MCX ETI is a session-oriented interface.

Participant applications connect to the trading system via the Exchange Application Gateways that host 
the client sessions. A session is established by opening a TCP/IP session to the gateway.
The exchange provides a unique session identifier that is used when logging on. A Session ID 
can only establish one session at any time. Each participant application requires its own session.
MCX ETI based applications receive information on orders which were entered in their own session.

## 3.3 Architecture Throttle

The number of transaction requests transmitted to the exchange in pre-defined time interval by 
each participant session is limited. This is to:

• Prevent single participant sessions generating excessively high transaction rates, which might 
adversely affect the exchange’s trading as a whole

• Guaranteed fairness between participant sessions.

---

## 4. Service Description

## 4.1 FIX Semantics

All application messages between the participant and the ETI follow FIX V5.0 SP2 semantics 
including all officially approved extension packs.

Additionally, user defined fields (UDF) and messages have been added to cover functional gaps in 
the FIX standard or to increase performance.

• All rejections and errors on an application and session level are communicated via the FIX standard 
Reject

(3) message

• All parties are identified by individual UDFs instead of repeating groups. UDF tags and names were 
chosen in a way that supports automated translation to standard FIX repeating groups for parties

The FIX messages are denoted in the following way:

• Message name (Message Type)

The FIX fields are denoted in the following way:

• Field name (FIX Tag)

FIX repeating groups and components are denoted in the following way:

• <Group name>

## 4.2 Party Identification

The participant is the market entity accessing the Exchange. The MCX concept of a member remains in 
place and will be represented by a “participant”’.

A participant may have several business units (Member Code) as independent entities taking part in 
trading at the exchange. Business units are identified by a business unit ID. A business unit belongs to a 
participant. The MCX ETI deals only with the concept of a business unit of a participant.

A user is a person, such as a trader who actually interacts with the Exchange. Users are identified by a 
user ID. A user belongs to one business unit. A user is a trader or administrator that logs on to the system 
and can perform various actions on the Exchange.

All requests that are received by the MCX Enhanced Trading Interface (MCX ETI), with the exception 
of session related requests, must carry the ID of a user that enters the request.

The details of the user of the business unit needs to be filled in the SenderSubID (50) tag of the request 
header. All orders and quotes must carry the identification of a physical person that is legally responsible 
for the order

The following roles and attributes are supported in MCX ETI: (The relevant ETI tags are given for clarity)

---

| Party | Description | PartyAttributes | Relevant FIX Field |
| --- | --- | --- | --- |
| Participant | The participant is anentity accessing MCXExchange’s NewTrading Architecture. | ParticipantShort Name | RootPartyExecutingFirm (22401) |
| Business Unit | Indicates the companyor a part of a companythat is set up as anindependent entitytaking part in trading atthe exchange. | Business UnitID | PartyIDExecutingUnit (20059),RootPartyIDExecutingUnit (20459),PartyDetailIDExecutingUnit (20259) |
| User | A business unit canhave multiple users. Auser can be a tradinguser and/or anadministrator. | User ID | Username (553),SenderSubID (50) |
| Owning User | User who owns thetransaction. | Owning UserID | PartyIDExecutingTrader (20012),RootPartyIDExecutingTrader (20412),TargetPartyIDExecutingTrader (20612) |
| Owning User | User who owns thetransaction. | Owning UserShortName | RootPartyExecutingTrader (22412) |
| EnteringUser | User whoinitiates/submits theorder transaction; couldbe the head trader orsupervisor or a marketsupervision user. | Entering UserID | PartyIDEnteringTrader (20036) |
| EnteringEntity | Identifies the entity thatentered the transaction;might be marketsupervision, theparticipant, or theclearing member. | Entering EntityID | PartyIDEnteringFirm (20007) |
| EnteringEntity | Identifies the entity thatentered the transaction;might be marketsupervision, theparticipant, or theclearing member. | Entering EntityShort Name | RequestingPartyEnteringFirm (22807) |
| Session | Identification of thesession. A sessionbelongs to a businessunit. | Session ID | PartyIDSessionID (20055),TargetPartyIDSessionID (20655),RootPartyIDSessionID (20455) |
| System | Executing system | System ID | RequestingPartylDExecutingSystem(20816) |

---

UDF tags for MCX ETI parties consist of an offset and the standard “enum” value of PartyRole (452):

• PartyID (448) has an offset of 20000

• RootPartyID (1117) has an offset of 20400

• TargetPartyID (1462) has an offset of 20600

UDF names for MCX ETI parties consist of the standard FIX field name (PartyID (448)
RootPartyID (1117), TargetPartyID (1462)) concatenated with the symbolic name of the standard 
“enum” value of PartyRole (452) as defined in the FIX Repository.

For example, the MCX ETI tag and field name for the user is PartyIDExecutingTrader (2003)

## 4.3 Security Identification

Each MCX product is processed on exactly one partition; a partition is a grouping of products. To optimize 
the routing to the corresponding partition, the product identifier needs to be provided in each order by the 
participant. The exceptions to this rule are Short Order Message Layouts.

Both single and multi-leg instruments are uniquely identified by the unique instrument identifier.

Additionally, for multi leg instruments it is mandatory to specify the instrument type in all order requests.

| Identifier | Description | Relevant FIX Tags |
| --- | --- | --- |
| Product Identifier | The product identifier uniquely identifies a MCX product. | MarketSegmentID (1300) |
| Instrument Identifier | The instrument identifier uniquely SecurityID (48) identifies an instrument in the core SimpleSecurityID system. | (30048)1 |
| Instrument Type | Required for complex instruments, valid values are: 1 = Simple Instrument 5 = Futures Spread. Example of Simple Instruments (Futures and Options). FUTCOM GOLD 5AUG2021 OPTFUT GOLD 27JUL2021 | ProductComplex (1227) |

---

## 4.4 Order Types

The following MCX order types are supported via the MCX ETI interface:

| Order Type | Description | Relevant FIX Tags |
| --- | --- | --- |
| Market | An order for buying or selling at the bestprice prevailing in the market at the time ofsubmission of the order. Any unexecutedportion of the order remains as a pendingorder till it is matched, or its durationexpires. The MaxPricePercentage providesprotection to market orders from unfairexecution prices. The protection percentagewill be applied on the first trade price of theorder and the worst price is arrived at. Thesubsequent  execution of the  order  willhappen till the worst price if opposite sideavailable else will be converted to limitorder at the last executed price. | OrdType (40) = 5MaxPricePercentage(28740) |
| Limit | Limit orders include a specified price limitand may not be executed at a price worsethan that limit.In case Exchange has enabled, negative and‘0’  limit  price  will  be  allowed  for  theinstrument | OrdType (40) = 2Price (44) |
| Stop (Market) | Stop market orders create market orderswhen the specified trigger price is reached.Similar to market orders, stop orders are notvisible in the order book for any marketparticipant. The stop loss(market)order will be available at the exchange willnot be visible to the market till the stop losstrigger price is reached for that instrumentidentifier. Once it is triggered it will beconverted to a normal market order and willbe subject to the respective rules applicable.Trigger price could be negative, ‘0’ as well’. | OrdType (40) = 3StopPx (99) |
| Stop (Limit) | The stop loss (limit) order will be available atthe exchange but will not be will not bevisible to the market till the stop loss triggerprice is reached for that asset. Once it istriggered it will be converted to a normallimit  order  and  will  be  subject  to  therespective rules applicable.Both limit and trigger price can be specified | OrdType (40) = 4Price (44)StopPx (99) |

---

|  | as negative and ‘0’ |  |
| --- | --- | --- |
| Good-for-day (Day) | All orders are assumed to be day ordersunless otherwise specified. The validity of aday order ends at the close of that tradingperiod for the Business day. | TimeInForce (59) = 0 |
| End-Of-session(Session) | All orders are assumed to be session ordersunless otherwise specified. The validity of asession order ends at the close of thattrading session for the business day. | TimeInForce (59) = 7 |
| Immediate orCancel (IOC) | An IOC order is to be filled immediately,either completely or to the extent possible;the   portion   that   cannot   be   filledimmediately is cancelled. DQ is not allowedfor  TimeInForce IOC orders. | TimeInForce (59) = 3 |
| Persistent | A persistent order is an order that survivesa trading interruption. A persistent order isan order that is reinstated at the beginningof the Business Day (if valid for that BusinessDay) or after a Market Reset. | ExecInst (18) = H (encoded as bitmapwith first bit being set) |
| Standard | The complete order history of an orderhaving this flag set will be part of the listenerbroadcast  and  may  be  recovered  viaretransmission requests. Standard ordersare persistent. | ApplSeqIndicator (28703) = 1(Recovery_Required) |
| Non- Persistent orLean | Non persistent orders are automaticallycancelled at the end of the business day orafter a market reset.For Lean orders, the execution notificationsCan be recovered (via retransmissionrequests on the session data channel).Alldata on lean orders are visible only to thesession that submitted the order.Lean Ordersare always non-persistent.The valid combination is below one:ApplSeqIndicator=0 (No Recovery Required)ExecInst=2(Non Persistent Order, FIX valueQ) | ExecInst (18) = Q (encoded as bitmap with second bit being set.)ApplSeqIndicator(28703)=0(No_Recovery_Required) |

---

## 4.5 Order Quantity

Participants need to specify two quantity in the order request. The field OrderQty (38) will determine the 
total order quantity desired by the user. The field DisclosedQty (MaxShow) (210) will determine the 
quantity desired by the user to be shown in the market data.

The user can request to show total quantity in the market data or part of the total quantity subject to 
applicable criteria for the MaxShow quantity. The MaxShow quantity is also called as revealed quantity. 
Minimum DQ percentage will be applicable as set by exchange.

## 4.6 Cancellation

The owner of an order is the entering business unit, session and user. The Exchange also supports order 
cancellation on a different session for the same business unit. Head traders of the same trader group and 
the supervisor (MAT Terminal) of the business unit may cancel on behalf of the owning user.

Cancelling an order will remove the remainder of a live order from the Exchange’s order book. The 
participant should specify the OrderID (37) and instrument identifier to identify the corresponding order to 
cancel.

The Exchange will respond with an Execution Report (8) or Reject message for cancellation 
confirmation or cancellation rejection respectively.

## 4.7 Modification

• The participant should specify the combination of OrderID (37) and Instrument Identifier to 
identify the corresponding order to modify. The MCX ETI interface will respond with an 
Execution Report (8) or Reject message for modification confirmation or modification 
rejection respectively.

• Orders that have been completely filled cannot be modified.

• The following order attributes cannot be modified:

[Image: X20]Security identification: MarketSegmentID (1300), SimpleSecurityID (30048), 
SecurityID (48), ProductComplex (1227)

Side (54)

[Image: X20]TradingSessionSubID (625)

ExecInst (18)

[Image: X20]ApplSeqIndicator (28703)

[Image: X20]ExecInst (18): Persistent to non persistent and vice versa

• The following restrictions apply to the modification of the field *OrdType (40)*:

A Limit Order (OrdType (40) = 2) may only be modified to a Market Order (OrdType (40) 
= 5).

---

[Image: X22]A Stop (Market) Order (OrdType (40) = 3) may only be modified to a Stop (Limit) 
Order (OrdType (40) = 4) or Limit Order (OrdType(40)=2) or Market Order 
(OrdType(40)=1)

[Image: X22]A Stop (Limit) Order (OrdType (40) = 4) may only be modified to a Stop (Market) Order (OrdType
= 3) or Limit Order (OrdType(40)=2) or Market Order (OrdType(40)=5).

[Image: X22]An order that is modified will lose its time priority, i.e. it will get a new priority timestamp, if

*Price*(44) is modified in any way, or

*OrderQty (38)* is increased, or

*OrdType (40)* is changed, or

ExecType(150) “L = Triggered by system”.

## 4.8 Disclosed Quantity (MaxShow ) quantity modification

In modification request the user can modify the MaxShow(210) field to show either the full LeavesQty 
(151) or part of LeavesQty (151) in the market data.

The quantity to be sent in the MaxShow(210) field should be the desired visible quantity of the user.
All orders where the user desires to show the full LeavesQty (151) in the market data, the MaxShow 
(210) field can be set as 0 in the entry / modification request.

There are certain business rules on visible quantity, and it will be validated for each request.

## 4.9 Total Order Quantity Modification

Participants need to specify the new total order quantity when modifying the field OrderQty (38). This 
approach leads, from a participant’s point of view, to a clear and deterministic behaviour by specifying a 
total execution limit. The previously executed quantity of an order is maintained and is used to calculate 
the new open quantity. If this is zero or less, then the order will be cancelled.

During the lifetime of an order, the total quantity of the order is always equal to the sum of the open order 
quantity and the accumulated traded quantity:

OrderQty (38) = CumQty (14) + LeavesQty (151)

After an order has been cancelled (OrdStatus (39) is “4 = Cancelled”), the total quantity of the order is 
equal to the sum of the cancelled order quantity and the accumulated traded quantity, while the open order 
quantity is zero.

OrderQty (38) = CumQty (14) + CxlQty (84)

## 4.10 SMPF Self-Match Prevention Functionality

Self-Match refers to trades where both buyer and seller clients are same. Such Self-Match does not server 
any economic purpose of Members/Clients instead Members have to incur additional expenses like 
Transaction charges, CTT, etc

Buy and Sell orders having the same PAN (belonging to member and/or UCC) are considered as selfmatch orders and will not be matched with each other.

---

Self-match prevention refers to a mechanism by which Exchange restricts execution of self-match trades 
whereby when an active order gets potentially matched with a passive order resulting in Self-Match Trade.

User needs to refer the Extended Order Information (10117) message for SMPF cancellation.

System cancels the active or passive order as per SMPF order identifier received with order instead of 
executing the self-match trade.

## 4.11 Order Mass Cancellation

Using the Order Mass Cancellation Request, a user can cancel all orders within a specified instrument, 
user or session scope using single request to exchange.

The instrument scope can be extended to an entire product.

## 4.12 Text Fields

The new trading architecture supports three free-format text fields for trader-specific comments to an 
order or trade.

The mapping of the MCX text fields to the FIX tags is as follows:

| Text Field | Valid Characters (hex) | Relevant FIX Tags | Mapping to MCX Field |
| --- | --- | --- | --- |
| Free TextField 1 | \x20, \x22-\x7B,\x7D,\x7E | FreeText1 (58) | UCC Code/Client Code |
| Free TextField 2 | \x20, \x22-\x7B,\x7D,\x7E | FreeText2 (25008) | CP Code |
| Free TextField 3 | \x20, \x22-\x7B,\x7D,\x7E | FreeText3 (25009) |  |

## 4.13 Terminal Info

The Terminal Info of the end-user who is placing the orders should be registered with the Exchange. 
It consists of 15 digits and break-up of this is given below:

Terminal Info - for 15-digit unique no Initial 12 digits

• CTCL - 6 - pin code of Terminal Location,

• 3 - Running serial number of the Terminal within the branch

• IBT - "111111111111"

• DMA - "222222222222"

• Wireless Technology - "333333333333" 13th Digit Not generated through program trading software -
"0"

• Generated through program trading software - "1"

• Smart Order Routing Without Program trading software = "2"

• Smart Order Routing with Program trading software = "3" 14-15 digit - Valid vendor code / In-house 
CTCL code for member

---

## 4.14 Order Status and Execution Report

The ExecutionReport (8) message is used to communicate events that affect an order.

The field ExecType (150) specifies the type of event. The field OrdStatus (39) specifies the new status of 
the order.

The different scenarios and their usage of the OrdStatus (39) and ExecType (150) are as follows:

| Scenario | OrdStatus (39) | ExecType (150) |
| --- | --- | --- |
| Order book replay: Transmission of all active orders | = New 0 = Partially filled 1 | D = Restated |
| Successful submission of an order | 0 = New | 0 = New |
| Rejected submission of an order | 4 = Cancelled | 4 = Cancelled |
| Successful modification of an order | = New 0 = Partially filled 1 = Filled 2 | 5 = Replaced |
| Rejected modification of an order | 4 = Cancelled | n/a |
| Successful cancellation of an order | 4 = Cancelled | 4 = Cancelled |
| Rejected cancellation of an order | 4 = Cancelled | n/a |
| Partial fill | 1 = Partially filled | F = Trade (in FIX 4.4) |
| Complete fill | 2 = Filled | F = Trade (in FIX 4.4) |
| Triggered Stop Order | 0 = New | L = Triggered by system |
| Modification of an order to Immediate or Cancel (IOC) triggered by the system | 0 = New | 5 = Replaced |
| Unsolicited modification triggered by third party | 1= New = Partially filled 2 = Filled 3 | 5 = Replaced |
| Unsolicited cancellation triggered by third party | 4 = Cancelled | 4 = Cancelled |
| Unsolicited cancellation triggered by the trading system | 4 = Cancelled | 4 = Cancelled |

---

| Cancellation of not (fully)executedImmediate or Cancel (IOC)Order | 4 = Cancelled | 4 = Cancelled |
| --- | --- | --- |
| Cancellation due to Self-Match Prevention (SMP) | 4 = Cancelled | 4 = Cancelled |
| Cancellation of squareOffOrder | 4 = Cancelled | 4 = Cancelled |
| Cancellation of RRM Order | 4 = Cancelled | 4 = Cancelled |
| Approval of RRM Order | 7 = RRM Suspended | 0 = New |
| Approval of SquareOffOrder | 8 = SquareOffSuspended | 0 = New |

## 4.14.1Square Off Suspended Orders

Member whose status is Square Off Suspended can place the Order and would be sent to Bancs for 
further action. Below are the action steps which would perform and ExecRestatementReason should be 
considered along with Ord Status and Exec Type in Suspended and Square off Suspended scenario.

| Action | Ord Status | Exec Type | Exec RestatementReason |
| --- | --- | --- | --- |
| Order Placed by User with Status SquareOffSuspended and accepted by T7 | 8=SquareOffSuspended | 0=New | 101= Order Add |
| Order pending with CnS- Case 1- No Responsereceived at given time | 4=Cancelled | 4=Cancelled | 215=Risk ReductionTimer Expired |
| Order pending with CnS- Case 2- Order Accepted | 0=New | 0=New | 101= Order Add |
| Order pending with CnS- Case 2- Order Rejected | 4=Cancelled | 4=Cancelled | 197= Pending OrderDeletion |

## 4.15 Order Book Restatement

Order status inquiries are not supported by the Exchange Participants must maintain the state of orders 
based on the Execution Report (8) messages.

During the start-of-day phase and after a market reset event, all active orders of a session will be 
transmitted to the market participant via the respective session.

Initially a Trading Session Event message is sent informing the participant about start-of-day or a market 
reset event per partition respectively, optionally followed by Extended Order Information messages for 
each restated order of the corresponding session and finally a Trading Session Event message is sent 
indicating the end of the restatement per product.

In case a session is cancelled by the member (disconnected from the ETI- Gateway) all open nonpersistent orders and quotes which were entered via this session will be deleted and will not be restated.

---

Note: Order book restatement messages are  recoverable. The owning session can request a 
retransmission in the event it was not logged on at the time. Order book restatement messages are also 
sent on the listener broadcast.

## 4.16 Trade Notifications

## 4.16.1 Trade Characteristics

If subscribed to the Trade broadcast, a session will receive Trade Notification messages that confirm 
each trade for the entire business unit.

Notifications about trades are only provided on Exchange via Trade Capture Report (AE) messages. 
Information provided via Execution Reports (8, U8) is indicative only and needs to be confirmed via a 
Trade Capture Report (AE). For further details see chapter Best Practices for order handling.

The Exchange will send out the Trade Notification message for each order and quote execution to the 
trading and clearing business units involved. The receiving clearing business unit may belong to a 
different participant. For trades in complex instruments (Multi Leg), the Trade Notification message will 
be generated for each instrument leg executions.

## 4.16.2 Trade Reconciliation

There are several identifiers that can be used to associate an Execution Report (8) with Trade Capture 
Reports (AE) and public trades on the market data interface. Please find an example message flow in the 
appendix, Trade Reconciliation and Identifiers Trade reconciliation and identifiers.

Every match event with one or more executions (match steps) in a simple or complex instrument result in 
only one Execution Report (8) for each order respectively. A Trade Notification will then be sent to confirm 
each trade at each price level. For complex instruments, there is a Trade Notification for each leg execution 
of the instrument.

Every match step occurring at the exchange has an identifier that is provided in the field FillMatchID 
(28708) in the Execution Report (8) and TrdMatchID (880) in the Trade Notification. This identifier allows 
participants to link trade capture reports and the corresponding execution report.

The TradeID (1003) field in the Trade Notification in MCX ETI uniquely identifies all order leg allocations 
referring to the same matching event, simple instrument, and price.

The field SideTradeID (1506), which is unique for a product and business day, in the Trade Notification 
provides the private identifier of an order or quote match step event, which can be reconciled with the 
corresponding Execution Report (8) for orders and Execution Report (U8) for quotes the following way:

For order match events in simple instruments, the Execution Report (8) message provides the order 
execution ID on each price level, FillExecID (1363).

For order match events in complex instruments the Execution Report (8) message provides the order 
execution ID on each price level and additionally the order leg execution ID, LegExecID (28725)

---

| Match Reporting | Execution Report (8) | Trade Notification |
| --- | --- | --- |
| Trade event on instrumentlevel: public trade volumereporting | FillMatchID (28708), | TrdMatchID (880) |
| Identifier for all leg allocationsreferring to the same simpleinstrument |  | TradeID (1003),OrigTradeID (1126) |
| Order in simple instrument | FillExecID (1363) | SideTradeID (1506) |
| Order in complex instruments | LegExecID (28725) | SideTradeID (1506) |

• For order match events in complex instruments the Execution Report (8) message provides 
the order execution ID on each price level and additionally the order leg execution ID, LegExecID 
(28725).

## 4.17 Trade Enhancement Notification

If subscribed to the Trade Enhancement notification broadcast, a session will receive Trade 
Enhancement Notification messages that confirms the acceptance or rejection of trade by the 
respective custodian.

The Exchange will send out the Trade Enhancement Notification message for each accepted/ rejected 
trade by the custodian. This message will be sent to the owing session and the admin session of the 
business unit.

## 4.18 Listener Broadcast

The new trading architecture offers ’drop copies’ for standard orders to sessions within the same 
business unit. This service is provided on session level; for example, each listener session may 
subscribe to listener broadcast data of one specified session.

The listener broadcast provides information of the complete order history of standard orders of a session 
and can be retransmitted. Complete order history means here: all changes to an order happening on the 
current business day. Drop copies for standard orders are sent once it is entered into the persistent 
store.

## 4.19 News

The News message provides public information from the MCX market supervision.

This public stream provides a unique sequence number to support retransmission, i.e. news messages 
are recoverable

---

## 4.20 Timestamps

All ETI timestamps will provide date and time, in UTC, represented as nanoseconds past the UNIX epoch 
(00:00:00 UTC on 1 January 1970). ETI provides the following timestamp information:

| Timestamp | FIX field | Description |
| --- | --- | --- |
| Transaction timestamp | ExecID (17),MassActionReportID (1369),SecurityResponseID (322), | Taken when a transactionis functionally processedand used as  a uniquemessage  identifier  perproduct  for  messagessent by the MCX ETI. |
| Entry timestamp | TrdRegTSEntryTime (21009) | Time of the creation ofthe order. It will be setwhen an order is receivedand  processed  by  theMatching  Engine.  Thistimestamp    is    onlyprovided  for  StandardOrders. |
| Request in | RequestTime (5979) | Time  when   request  isconsidered for processing |

## 4.21 Strategy ID and STSN

The Strategy that a member wants to implement through Algorithm Trading shall be approved by the 
exchange. Post Approval Exchange will allocate particular Strategy ID to specific User ID (ATS User) for 
the approved strategy.

All Order Entry/ Modification / Cancellation requests coming through third party application should contain 
the Strategy ID. If Order Entry/ Modification / Cancellation requests contains the Strategy ID other than 
assigned to the user, the respective Order Entry/ Modification / Cancellation requests shall be rejected by 
the system.

The users can be assigned product specific strategies with different parameters and definition. For 
Strategy approved by exchange, Strategy ID may be assigned as ‘NNNNN’. All parameters of the user 
defined strategies should be stored by the third-party applications.

Apart from the strategy Id, the third-party application is expected to send ‘Strategy Trigger Sequence 
Number (STSN)’ with each of the entry / modification / cancellation request.

The format of STSN should be dddNNNNNnnnnnnnnn; where ddd denotes the Day of Year. In case if day of year is 1 then, ‘001’ needs to be entered

NNNNN - Number stored by 3rd party applications for product specific strategy, Max range is 99999

Small N’s- sequence running number, should be incremented with every request. Thus, the first part 
dddNNNNN - represent user defined strategies for a particular day of the year.

Each time there is need of sending entry or modification or cancellation request to exchange, third party 
application is expected to generate new STSN.

STSN should also be stored with relevant values of trigger / Opportunity.

Third-party applications should have provision for complete audit trail of values defined by user against a 
strategy. As part of exchange inspection / system audit, this data will be verified with actual orders and 
values sent.

## 4.22 Connectivity and Session Parameters

## 4.22.1.1 Session Concept

The Exchange implementation is a session-oriented interface whereby the session is the basic scope of 
the interaction with the new trading  architecture. Every session may only be instantiated once. Each 
TCP/IP connection may only support one session instance.

The receiver of the direct response to a request sent to the gateway is always the submitting session. 
Additionally, the session is informed about system events and all unsolicited messages referring to status 
changes of orders belonging to that session.

## 4.22.1.2 User Authentication

The User Logon message identifies and authenticates a qualified user establishing access to the new 
trading architecture.

The participant must provide the binary User ID in the Username (553) field, and the  corresponding 
password in the Password (554) field.

A successful user logon will grant the user access to the trading system.

All further transactions that require a user scope from the session are validated in the gateway against an 
authenticated User ID, i.e. SenderSubID (50) in the message header.

Users may logon to the new trading architecture via all sessions of their business unit. Multiple User Logon 
messages for a user via the same session are rejected.

---

## 4.22.1.3 Identification and Authentication

The MCX ETI has a three-step logon procedure, with a Connection Gateway Request message to retrieve 
the assigned application gateway from the connection gateway, followed by a Session Logon at the 
assigned gateway and followed by a User Logon message.

## 4.22.1.4 IP Addresses and Ports

At first, the participant must establish a TCP/IP session to the Connection Gateway — the IP/port numbers 
are provided by the exchange. Participants will be provided with a "Primary" and "Secondary"

Connection Gateway address. If the connection to the primary Connection Gateway fails, participants 
should connect to the secondary Connection Gateway.

Once that connection is established, the participant sends a Connection Gateway Request message. The 
Connection Gateway will validate PartyIDSessionID (20055) (CTCL User ID) and Password (554), which 
are parameters provided by the exchange. The Connection Gateway cannot be used for any other 
purpose.

The Connection Gateway Response contains the IP/ports of the primary and secondary application 
gateway where the member application can establish an active MCX ETI session. Once the Connection 
Gateway Response message is received, the Connection Gateway connection is closed.

The participant must now try to connect to one of the provided application gateways within 120 seconds. 
If the connection is not made within this time, the slots are released, and the participant must start again 
from scratch.

## 4.22.1.5 Session Authentication

The participant must first open a TCP/IP connection to the specified gateway. The Session Logon /Session 
Password change message must be the first message sent by the participant authenticating the MCX ETI 
session. Any other messages other than Session Logon will be rejected and the TCP connection will be 
disconnected by the Exchange.

The gateway will validate PartyIDSessionID (20055) and Password (554), which are parameters provided 
by the exchange. A successful logon will initiate a MCX ETI session.
Note: The Session Logon message is not used to log on and authenticate a user on the new MCX trading 
architecture.

Participants needs to specify existing user id and password in PartyIDSessionID and Password field 
respectively.

The following messages may be sent on a session without any authenticated trader:

## Session logon/logout

Heartbeat messages

Subscription and un-subscription of broadcasts

Retransmission of recoverable data

Trader authentication

All other requests must be submitted with an authenticated trader name in the SenderSubID (50) of the 
message header.

---

## 4.22.1.6 Password Management

Participants needs to use same password during session login and user login request.

## Password Policy

• It should be Minimum of 8 characters in length and maximum 12 characters.

• It must be composed of following three-character sets:

**o** Upper / Lower 26 letters of English Alphabet A-Za-z o Ten digits 0-9

o Special characters (32 characters) - ('~!@#$%%^&*()_+-={}[]\:\;<>?,./)

**o** Password should be case sensitive I.e. ABC is not as abc.

**o** If wrong password is attempted 5 times then session will get locked. In Login page error     
will come “Session locked out”

**o** Space is not a valid password character.

**o** Reset password- last 5 passwords cannot be used.

• The password cannot contain the user account name or parts of the user full name that 
exceed three consecutive characters.

• Password has to be encrypted as per RSA token.

• When password expires , user can send any one password change message for renewal.

• Connnection Gateway Request will succeed with existing password even if password is  
expired.

• Code Snippets are provided in section 6 – Appendix.

• In case of password expiry

**o** Connection and session logon would be successful

**o** In case of user login, user would receive error message “password has been expired”.

**o** After this using user password change , new password is required to be put and accordingly in 
response “password change successful” message would be received.

**o** After this,  user need to re-login again with new password.

---

## 4.22.3 Throughput Limits

A participant application may send multiple messages without waiting for a  response. However, the 
number of messages allowed within a given timeframe is limited by the use of throttles.

The limits are configured by the new trading architecture for each session type and are provided to the 
participant application in the Session Logon Response message. The limit parameters are upper limits 
and do not guarantee throughput rates. As loads fluctuate in the exchange system, actual throughput 
rates can vary.

A participant application may send multiple messages without waiting for a response. However, the 
number of messages allowed within a given timeframe is limited by the use of throttles.

The mechanism uses two components:

Transaction limit.

Reject/disconnect limit.

## 4.22.3.1 Transaction limit

The transaction limit is the  maximum number of messages that a participant application may send 
within a configured time interval without getting rejected (sliding window approach).

If a participant application exceeds the threshold “number of transactions per time  interval”, the 
exceeding request will be rejected and not queued. The unit of the time interval is milliseconds.

Required heartbeats do not count against the transaction limit.

For example,

A transaction limit of 200 messages per second could berepresented in the Logon (A) response message
as:

ThrottleNoMsgs (1613) = 200 ThrottleTimeInterval (1614) = 1000

## 4.22.3.2 Reject/Disconnect Limit

The purpose of the Reject/Disconnect Limit is to protect the gateway from large amounts of invalid data. 
It defines the maximum number of sequential message rejects due to the violation of the transaction limit
allowed by the MCX ETI.

Once an acknowledgement has been sent, the reject/disconnect limit counter is reset to zero. If the
participant application continues to send messages which are rejected for exceeding the transaction 
limit and the reject/disconnect limit is exceeded, the MCX ETI will disconnect the session.

For example, a disconnect limit of 500 rejects is represented in the Logon (A) response message as:

ThrottleRejectNoMsg (25002) = 500

---

## 4.23 Session Layer

The MCX ETI follows standard FIX 5.0 semantics, however, the header and trailers have been 
suitable modified.

Each message in the MCX ETI has a unique numeric TemplateID (28500) assigned to it in addition to 
the standard FIX MsgType (35) information provided in the header

The MCX ETI will echo the participant’s MsgSeqNum (34) of the request header in the 
corresponding response header.

Retransmission is not supported on the session layer; it is available on the application layer for a subset 
of messages. It is not possible to retransmit all messages received on a session. It is possible to 
retransmit some message types using application message sequencing.

## 4.23.1 Flat Binary Encoding

ETI messages have a defined order of fixed-length fields and arrays of fixed-length elements. MCX ETI 
avoids string fields wherever possible.

The arrays (repeating groups) consist of a counter (FIX NoXXX fields, indicating the number of array 
elements) and their fixed-length elements. In general, repeating groups are at the end of the ETI 
messages.

Binary values are presented in little endian byte order.

The length of ETI messages (BodyLen (9)) sent by the ETI gateway is always set to a multiple of 8. If there 
is a variable size string at the end of a message, it is "filled up" with binary zeroes.

Padding bytes required for proper alignment do not need to be initialized.

## 4.23.2 Logon

The participant application needs to open a TCP/IP connection to the new trading architecture during 
start-up.

The first message to be sent on the connection must be the Session Logon message. If the Session 
Logon message is not sent within a certain time interval, the connection will be closed by the new 
trading architecture

If the session logon fails, no further logon attempts will be accepted on that TCP connection. The 
application must drop the TCP session and restart the logon to the Connection Gateway

## 4.23.3 Logout

The participant may log out the session using the Session Logout message

The MCX ETI will automatically drop a session if:The TCP/IP session is disconnected., If three 
consecutive heartbeats are missed,If reject/disconnect limit is exceeded.

After a successful session logout, the participant should shut down the connection and close the socket.

Participant applications must disconnect from the MCX ETI each day after trading and should close 
the TCP/IP socket after logging off the session.

The system will perform a forced logout overnight after which time the participant may log back in.

---

## 4.23.4 Heartbeat

The HeartBtInt (108) must be specified by the participant in the Session Logon message. This parameter 
specifies the period in which the MCX ETI sends heartbeats to the participant and the interval the MCX 
ETI checks for request messages from the application.

The Heartbeat message should be sent by the participant if no other message has been processed during 
the defined HeartBtInt (108) interval. It is used by the MCX ETI gateway to monitor the status of the 
communication link to the ETI client during periods of inactivity.

A heartbeat interval of zero indicates that MCX ETI will not take any action for missed heartbeats. This 
setting can only be used on test systems. If the heartbeat value is sent as zero in the request, then the 
MCX ETI will use a default value.

The minimum value is 100 milliseconds in the production system. The upper limit is 60 seconds.
If the field is not supplied or set as zero, then the MCX ETI will use a default value. The applied heartbeat 
interval is provided in the Session Logon Response.

Note: Heartbeats do not count against, nor do they reduce, any of the throttle counters. The Heartbeat 
Notification is sent by MCX ETI based on the heartbeat interval, regardless if the participant application 
sends Heartbeat messages or not. It may be used by the ETI client to monitor the status of the 
communication link to the MCX ETI gateway during periods of inactivity.

## 4.23.5 Reject

All rejections and errors on the application and session level are communicated via the FIX standard 
Reject (3) message, i.e. none of the fields in the request message other than MsgSeqNum (34) will be 
echoed.

---

## 4.23.6 Message Sequence Number

The MsgSeqNum (34) in the request header must increment with each message sent by the participant 
to the gateway, starting with the Session Logon message as sequence number 1.

The MCX ETI will echo the participant’s MsgSeqNum (34) of the request header in the corresponding 
response header.

In case of any unexpected sequence numbers, sequence number gaps, or duplicate sequence numbers, 
the request message will be rejected with a sequence number error, and the session will be disconnected.

Note: There is no recovery mechanism for message sequence numbers in the MCX ETI. All participant 
connections (including a reconnection after a disconnection) are considered “new” and all Session Logon 
requests are expected to contain the message sequence number 1.

## 4.23.7 Application Message Sequencing

## 4.23.7.1 Application Message Identifier

All recoverable session data sent by MCX ETI will provide an application message identifier, ApplMsgID 
(28704), to uniquely identify order data sent by the gateway. With the help of the application message 
identifier, the participant is able to ask for a retransmission of recoverable order data.

The ApplMsgID (28704) has the following characteristics:

• It is unique per partition and business day.

• It is ascending during a business day until end-of-stream.

• Gap detection is not possible

• It does not start at any particular number

• Consists of 16 bytes, ordered with the highest significant byte first (as in big endian).

Memory comparison functions such as memcmp() can be used to compare two Application Message 
Identifiers.

Two Application Message Identifiers ApplMsgID1 and ApplMsgID2 are equal, if the character array of size

16 of ApplMsgID1 and ApplMsgID2 are equal at all positions

The ApplMsgID1 is greater than ApplMsgID2, if at the first differing position i, the corresponding character 
ApplMsgID1[i] is greater than ApplMsgID2[i].

The ApplMsgID1 is less than ApplMsgID2, if at the first differing position i, the corresponding character 
ApplMsgID1[i] is less than ApplMsgID2[i].

---

## 4.23.7.2 Application Message Sequence Number

MCX ETI will assign an application message sequence number, the ApplSeqNum (1181), to messages 
related to Trade Notification, News and Risk Control (Risk Notification).

The ApplSeqNum (1181) has the following characteristics:

• The first message will be the message sequence number 1.

• It is ascending during a business day until end-of-stream (Trade Notification).

• The message sequence will be gapless and allows gap detection.

• Trade notification: unique per business day, partition, and business unit.

• News: unique per market.

• Risk Control: unique per business unit.

## 4.23.8 Session Data

Each session receives information on orders and quotes which were entered in their own session 
automatically without any subscription.

The Session Data include Trading Session Event messages: start of service, market reset, and end of 
service.

For standard orders the complete order history may be recovered. Session Data are recoverable, if and 
only if they have an ApplMsgId (28704).

Session Data are recoverable, if and only if they have an *ApplMsgId (28704)*.

Should be populated with Session ID.

Note: The retransmission message template used for order events may differ from the session data 
response template. For more details, please refer Retransmit Response (Order Event).

---

| Broadcast | FIX Message Type | ApplicationIdentifier | BroadcastMessageIdentifier |
| --- | --- | --- | --- |
| Listener: completeorder   historyofstandard orders of asession | Execution Report (8),Trading Session Status(h),Order Mass ActionReport(BZ) | 5 | ApplMsgID (28704) |
| Trades: Trade data onsession level | Trade Capture Report(AE) | 1 | ApplSeqNum (1181) |
| Generalmessages from  MCXmarketsupervision | General Messages (B) | 2 | ApplSeqNum (1181) |
| Service  Availability:Provides informationon the availability of apartition . | User Notification (CB) | 3 | N/A |
| Trade Enhancement:Provides informationon   acceptance orrejection of trades bycustodian | Trade EnhancementNotification (U31) | 1 | ApplSeqNum (1181) |
| Ex/DEx | Ex/DEx notification | 12 | ApplSeqNum (1181) |

## 4.23.9 Broadcast

A broadcast is an application message that is available to multiple sessions, such as Trades or News 
messages.

Sessions may receive the following broadcast types:

After a session is established, it is not subscribed to any broadcast, but the risk control broadcast; 
nevertheless, unsolicited session data is received.

For broadcast subscription, the Subscribe message is used. Per request only one broadcast type, via 
RefApplID (1355), may be subscribed. The response provides a unique subscription identifier in 
ApplSubID (28727).

For broadcast un-subscription, the Unsubscribe message is used. Per request, only one subscription, via 
RefApplSubID (28728), may be un-subscribed.

---

For EX-DEX , RefApplID(1355) is used.

The following tables shows the results of different subscriptions:

| RefApplID (1355) | SubscriptionScope (25001) | Result |
| --- | --- | --- |
| Trade (1) | 0xFFFFFFFF (no value) | All trade data of the own session. |
| General Message | 0xFFFFFFFF (no value) | All public general messages from the marketsupervision of the specified market. |
| Service Availability (3) | Partition ID | Availability  of  the  services  provided  by  thespecified partition. |
| Service Availability (3) | 0xFFFFFFFF (no value) | Availability of the services provided by allknownpartitions. |
| Ex/Dex | RefApplID(1335) | Complete history of EX/Dex Requests (of thecurrent business day) submitted via thespecified session |

| RefApplID (1355) | SubscriptionScope (25001) | Result |
| --- | --- | --- |
| Listener Data (5) | Session ID | Complete history of standard orders (of thecurrent business day) submitted via thespecified session. |

The receipt of the risk control broadcast is required by the regulator for all sessions; therefore, no 
subscription is needed. The FIX Application Sequencing concept2 is used for broadcasts on MCX ETI:

Each broadcast type is assigned a unique ApplID (1180).

Application-level  messages  are  uniquely  identified  using  a  combination  of  ApplID  (1180)  and 
ApplSeqNum (1181) or ApplID (1180) and ApplMsgID (28704) respectively.

---

| RefApplID | Drop Copy at memberlevel | Drop Copy at usergroup | Drop copy at self |
| --- | --- | --- | --- |
| 5(Listener Data) | Subscription Scope=0Member Level Details | Not Provided | SubscriptionScope=Session IDUser Level |
| 1(Trade) | Subscription Scope=0Member Level Details | Subscription Scope=0Wild card/List of Users | SubscriptionScope=Session IDUser Level |
| 2(News/Gen Msg) | Market | Market | Market |
| 12(Ex/Dex) | Subscription Scope=0Member Level Details | Subscription Scope=0Wild card/List of users | SubscriptionScope=Session IDUser Level |

## 4.23.10.1 Retransmission

Re-transmission is supported for recoverable session data and the broadcast types,  trades, news, and 
risk control.

Since application message identifiers and sequence numbers are unique per partition, the PartitionID 
(5948) is a mandatory parameter for all retransmission requests.

| RefApplID | Drop Copy at memberlevel(Trading group notassigned) | Drop Copy at usergroup(Trading Groupassigned) | Drop copy at self |
| --- | --- | --- | --- |
| 5(Listener Data) | Not Provided | Not Provided | SubscriptionScope=Session IDUser Level |
| 1(Trade) | Subscription Scope=0Member Level Details | SubscriptionScope=Session IDUser Level | SubscriptionScope=Session IDUser Level |
| 2(News/Gen Msg) | Market | Market | Market |
| 12(Ex/Dex) | Subscription Scope=0Member Level Details | SubscriptionScope=Session IDUser Level | SubscriptionScope=Session IDUser Level |

For retransmission, the Retransmit and Retransmit (Order) message respectively is used. With a retransmission request, only data in the scope of one broadcast type and partition can be requested via 
RefApplID (1355) and PartitionID (5948).

---

The FIX application level recovery concept can be used by the participant for selective recovery and late 
start restatements:

Optionally, the application message identifiers and respectively the application message sequence 
numbers provide the retransmission sequencing range. If no start value is specified, it is assumed to be 
“1”. If ending range is absent, it is assumed to be infinity (“all available messages”).

The re-transmission response, Retransmit Response and Retransmit Response (Order) message 
respectively, will provide the range of recovered order data in the fields ApplBegMsgID (28718) and 
ApplEndMsgID (28719) and for all other broadcasts respectively in  the fields ApplBegSeqNum (1182) 
and ApplEndSeqNum (1183).

| Recoverable Data | Scope | ApplicationMessageSequencing | ApplicationIdentifierRefApplID(1355) | RetransmissionSequencing |
| --- | --- | --- | --- | --- |
| Recoverable sessiondata | session | ApplMsgID (28704) | 4 | ApplBegMsgID (28718),ApplEndMsgID (28719) |
| Listener broadcast(standard order dropcopy) | session² | ApplMsgID (28704) | 5 | ApplBegMsgID (28718),ApplEndMsgID (28719) |
| Trades | business unit | ApplSeqNum (1181) | 1 | ApplBegSeqNum (1182),ApplEndSeqNum (1183) |
| General Messages | market | ApplSeqNum (1181) | 2 | ApplBegSeqNum (1182),ApplEndSeqNum (1183) |
| TradeEnhancement | Session | ApplSeqNum (1181) | 1 | ApplBegSeqNum (1182),ApplEndSeqNum(1183) |
| Ex/DEx | Session | ApplSeqNum (1181) | 12 | ApplBegSeqNum (1182),ApplEndSeqNum(1183) |

The FIX application level recovery concept can be used by the participant for selective recovery and late 
start restatements:

This range may differ from the requested range, i.e. further retransmission requests may need to be 
submitted.

---

## 4.23.10.2 Best Practices for Order Handling

All order response information in the MCX ETI is sent out  immediately after the order has been 
processed by the core matching process.

All order response information in the MCX ETI is preliminary; this includes Execution Reports (8) sent 
out for non-persistent orders. This is also true for the standard order drop copy information published via 
the Listener broadcast

A participant application always needs to confirm the preliminary execution information with the 
corresponding legally binding Trade Notification message (Trade Capture Report (AE)).

In case of an exchange system failure, the participant is informed of a market reset event via the Trading 
Session Event message including the last persisted application message identifier. This message is 
followed by an order book restatement of all active orders.

In  this event it is highly recommended to reconcile all Execution Reports (8) with higher application 
message identifiers with the corresponding Trade Capture Reports (AE). If there is no Trade Capture 
Report (AE) for a given Execution Report (8) then this Execution Report (8) has to be considered invalid 
and should be discarded. Please find detailed information regarding trade reconciliation in corresponding 
chapter.

---

## 4.24 Message Formats

This chapter provides details on the administrative and application messages used by the MCX ETI. 
Information on data types and the most important error codes are provided.

Messages sent by participants not listed in this section are rejected by the server via a Reject (3) 
message.

Each ETI message format has a unique binary message type identifier (Template ID (28500)) and is 
based on a standard FIX message.

The Interface version used by the participant needs to be provided during Session Logon in 
DefaultCstmApplVerID (1408); the Session Logon Response will return the Interface Version, which the 
MCX ETI Gateway currently uses, in the same field.

In production, the Interface Version will allow the participant to recognize that the MCX ETI has 
changed. The Build Number shows to which ETI XML file, ETI XSD file, canned data, exchange 
software the ETI Manual belongs to.

## 4.24.1 Message Fragmentation

In case the complete data of a transaction does not fit into a single message, MCX ETI automatically 
sends a sequence of messages to the participant.

The field LastFragment (893) in the corresponding header structure indicates whether the current 
message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction 
or if the application should wait for further messages in order to retrieve the full data set.

Another mechanism for message fragmentation that is used for inquiring reference data (session) basis 
on the LastEntityProcess (25035) field that is part of the requests and the responses.

In the first inquiry (request), LastEntityProcess (25035) must not be set. The application should 
continue sending the inquiries using the LastEntityProcess (25035) from the recent response. A 
response with LastEntityProcess (25035) not set indicates that no more data is available.

---

## 4.24.2 Data Types

MCX ETI supports following data types:

| signed int | • little endian byte order -supported are 1, 2, 4 and 8-byte,signedintegers- two’s complementrepresentation | 1 byte signed int: 0x802 byte signed int: 0x80004 byte signed int: 0x800000008 byte signed int:0x8000000000000000 |
| --- | --- | --- |
| unsigned int | • little endian byte order -supported are 1, 2, 4 and 8-byte,unsignedintegers | 1 byte unsigned int: 0xFF 2byte unsigned int: 0xFFFF 4byte unsigned int:0xFFFFFFFF8-byte unsigned int:0xFFFFFFFFFFFFFFFF |
| Float | • encoded as 8-byte signedinteger 8 implicit decimals | 0x8000000000000000 |
| Fixed String | • length information specifies thefixed size - encoded ascharacter array- completely filled with validcharacters (space padding ifrequired) | 0x00 at the first position |
| Fixed String (0-terminable) | • -length information specifies thefixed size - encoded ascharacter arrayoptionally 0-terminable(minimum string size of1)- XML file: attribute “isTerminable”is set to true- padding after0-terminator required up to fixedsize | 0x00 at the first position |
| Variable String | • length information specifies themaximumpossible size- separate counter field specifiesthe transmittedstring size- XML file: attribute“variableSize” set to true - themessage is filled up to a multipleof 8 bytes and truncated | 0x00 at the first position (if thefield is not the last one of acertain message) |

---

| Counter | This data type represents an unsignedinteger as a counter for arrays ofvariable size. | see 1-byte unsigned int and 2byte unsigned int respectively |
| --- | --- | --- |
| LocalMktDate | Date of Local Market (as opposed toUTC) inYYYYMMDD 4-byte unsigned binaryformat. | see 4-byte unsigned int |
| PriceType | Price in integer format including 8decimals. For certain asset classes,prices may have negative values. Priceneeds to be multiply by 100000000.Ex-For5 in price then user need to send5*100000000. | see 8 bytes signed int |
| Amount | Amount in integer format including 2decimals. | see 8 bytes signed int |
| Qty | Quantity in integer format including 4decimals. Qty needs to be multiply by10000.Ex-For 1 lot-send Qty as 10000. | see 8 bytes signed int |
| SeqNum | Message sequence number in 8-byteunsigned binary format. | see 8-byte unsigned int |
| UTCTimestamp | Date and time, in UTC, represented asnanoseconds past the UNIX epoch(00:00:00 UTC on 1 January 1970). | see 8-byte unsigned int |
| char | String of length 1. | 0x00 |
| Data | Byte array where each byte has a valuefrom 0 to 255:• -the length information specifiesthe fixed size - encoded as bytearray - byte array is always filledcompletely | Each byte filled with 0x00. |
| chartext | • length information specifies thefixed size - encoded ascharacter arrayoptionally 0-terminable(minimum string size of1)- XML file: attribute“isTerminable” is set to true- padding after0-terminator required up to fixedsize | 0x00 at the first position |

---

## 4.24.3 Encryption/Decryption for Interactive API

## Background

Based on regulatory requirement, the exchange is introducing encryption of messages for ETI API, for 
encryption the exchange is introducing, GCM encryption mode with 256 bytes will be utilized for all 
transaction messages. The Key and IV (Initialization Vector) are provided in the Connection Gateway 
Response Message. Session logon connections use the 256 bytes GCM encryption mode with a 
standard 32-byte key and a 12-byte IV, without the use of an Authenticator.

## Overview

Member applications will now establish connections to the designated Exchange Gateway server via 
TCP, and each message will undergo encryption/decryption using the same session key (symmetric 
cryptography AES 256 bits GCM mode) at both the member and Exchange ends. The symmetric session 
key remains valid for the duration of a session. Members need to ensure encryption of messages within 
member environment towards their clients.  While encryption of messages within member environment 
towards their client may need to be done by respective members.

## Proposed Methodology

Exchange proposes AES-256 GCM symmetric encryption approach. Following is an overview.

## Step 1: Connection Gateway Request

In the new login process with encryption, users will submit a connection gateway request (10020) with 
an encrypted password using PKCS#1. In this first message, request is non encrypted with only 
encrypted password.

## Step 2: Connection Gateway Response

At the application-level encryption, the encrypted password from the connection gateway request will be 
utilized, and Gateway will create a Key & IV specific to each session login. These two new fields, key & 
IV, will be transmitted by the trading platform in the connection gateway response. The connection 
gateway response will be encrypted using the password (sent from connection gateway request) as the 
key & IV, with the remaining bytes being padded with '0' on the right for both key and IV.

## Example: -

Password: - abc@12345 (Received in Connection Gateway Request)

Correct approach: -
IV: - abc@12345000 
Key: - abc@1234500000000000000000000000

Incorrect approach: -

IV: - 000abc@12345
Key: - 00000000000000000000000abc@12345

**Note:** The Key and IV generated in the connection gateway response are character array of 32 and 12 
bytes respectively, where each byte is in the range of 0-255.

## Step 3: Session Logon Request / User Login Request / Other Order requests

For subsequent requests after receiving connection gateway response, user must encrypt the request 
(Body fields) using the AES 256 GCM encryption mode using Key and IV shared in connection gateway 
response.

## Step 4: Session Logon Response / User Login Response / Other Order response

Gateway will decrypt the request and send an encrypted response. Users are required to utilize the 
defined fields (Key & IV received in connection gateway response) to decrypt the responses.

---

| Message | FieldsEncrypted | Mode | KeySize | IVSize | Key and IV |
| --- | --- | --- | --- | --- | --- |
| ConnectionGatewayRequest(10020) | Password | PKCS#1 |  |  | Encrypted usingPKCS#1 256Bytes GatewayPublic Key |
| ConnectionGatewayResponse (10021) | Entire Msg bodyEncrypted exceptfor headers(which is of 8bytes) | AES_GCM_256  bytesencryptionwithoutauthenticator | 32 | 12 | Use Passwordas Key and IVAppended with'0's for rest ofthe bytes |
| All Request messages afterConnection Gateway | Entire Msgbody Encrypted except for headers(which is of 24bytes) | AES_GCM_256  bytesencryptionwithoutauthenticator | 32 | 12 | Use key and IVwhich is sharedin ConnectionGatewayResponse(10021) |
| All Response messages afterConnectionGateway | Entire Msgbody Encrypted except for headers(which is of 8bytes) | AES_GCM_256  bytesencryptionwithoutauthenticator | 32 | 12 | Use key and IVwhich is sharedinConnection GatewayResponse(10021) |

**Note:** OpenSSL version to be used 1.1.1 and above for encryption/decryption.

## 4.24.3.1 Sequential Encryption/Decryption

To avoid encryption object reset before sending every request, exchange have implemented 
sequential encryption/decryption methodology.

In Sequential encryption/decryption user should always follow same sequence for decryption in which 
sequence they are receiving messages at TCP level.

## For Example:

If messages encrypted in following sequence message1 message2 message3
Members needs to decrypt in same sequence i.e. message1 message2 message3
If member decrypt in wrong sequence i.e. (message2 message3 message1) then user will receive 
wrong data.

---

**Note:** user should not encrypt/decrypt heartbeat messages otherwise sequence will mismatch and 
there will be inconsistent behaviour.

For the proper illustration, member should follow below flow chart for existing version of non-sequential 
flow and upcoming release version of sequential flow.

**Non sequential encryption/decryption flow (existing production version)**

key Encryption Object
IV

object reset Message1 object reset Message2 object reset Message3

GCM ENCRYPTION

Encrypted msg1 Encrypted msg2 Encrypted msg3

**Sequential encryption/decryption flow (no need to reset encryption object for each request)**

IV key Encryption Object Message1 Message2 Message3 GCM ENCRYPTION Encrypted Message1 Encrypted Message2 Encrypted Message3

---

TCP receiver sequence should be maintained (correct handling)
Message1 Message1 → Message2 → Message3 Decryption Handling
Message2 if sequence not maintained then encryption decryption output will be wrong (wrong handling)
Message3 Message2 → Message3 → Message1 Decryption Handling

---

## 5. Message Formats

**5.1 Session Layer**

## 5.1.1 Connection Gateway Request-10020

Connection Gateway Response

This message is used to retrieve the assigned gateway from the connection gateway.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10020 (Logon,MsgType = A) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | U | 4 | 20 | unsigned int | User ID /Session ID |
|  |  |  |  |  |  |  |
| 20055 | PartyIDSessionID | Y | 4 | 24 | unsigned int | Session ID. / Logon ID |
| 1408 | DefaultCstmApplVer-ID | Y | 30 | 28 | Fixed String(0-terminable) | Indicates  the  MCX  ETI  interfaceversion  the  ETI  gateway  softwareuses.Must be set as 2.3 Valid charac-ters: \x20, \x22-\x7B, \x7D, \x7EValid characters: \x01-\x7E |
| 39020 | Pad2 | N | 2 | 58 | Fixed String | Not used |
| 701 | EncryptedData-MessageSize | Y | 4 | 60 | unsigned int | Specifies the length of the encryptedmessage |
| 30744 F | iller7 | N | 32 | 64 | Fixed String(0-terminable) |  |
| 554 | Password | Y | 344 | 96 | Fixed String(0-terminable) | Password. Refer  Password  policy,Password should be encrypted withRSA and provided. |

---

## 5.1.2 Connection Gateway Response-10021

This message confirms the Connection Gateway request.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10021 (Logon,MsgType = B) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 28644 | GatewayID | Y | 4 | 32 | unsigned int | IP address of the Primary ApplicationGateway where the member applica-tion can establish an active MCX ETIsession.Note: Binary values are presented inlittle endian byte order. |
| 28645 | GatewaySubID | Y | 4 | 36 | unsigned int | Port number to be used for the as-signed Primary Application Gateway.Note: Binary values are presented inlittle endian byte order. |
| 28725 | SecondaryGatewayID | Y | 4 | 40 | unsigned int | IP address of the Secondary Applica-tion Gateway where the member ap-plication can establish an active MCXETI session.Note:  Binary values arepresented in little endian byte order. |
| 28726 | SecondaryGateway-SubID | Y | 4 | 44 | unsigned int | Port number to be used for the as-signed Secondary Application Gate-way.Note: Binary values are presented inlittle endian byte order. |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | 28730 SessionMode | Y | 1 | 48 | unsigned int | Session type. |
|  | 28730 SessionMode | Y | 1 | 48 |  | Description |
|  | 28730 SessionMode | Y | 1 | 48 |  | 2 LF |
|  | 339 TradSesMode | Y | 1 | 49 | unsigned int | Trading session mode. |
|  | 339 TradSesMode | Y | 1 | 49 | unsigned int |  |
|  | 339 TradSesMode | Y | 1 | 49 | unsigned int | Description |
|  | 339 TradSesMode | Y | 1 | 49 | unsigned int | 2 Simulation/Test |
|  | 339 TradSesMode | Y | 1 | 49 | unsigned int | 3 Production |
| 39060 Pad6 |  | N | 6 | 50 | Fixed String |  |
| 307574 | Key | Y | 33 | 56 | Fixed String (0-terminable) | Cryptographic key for both the encryption and decryption of all messages between member application and allocated Gateway |
| 307575 | IVVector | Y | 13 | 89 | Fixed String (0-terminable) | Cryptographic IV (Initialization Vector) for both the encryption and decryption of all messages between member application and allocated Gateway Server. |
| 39020 Pad2 |  | N | 2 | 102 | Fixed String | Not Used. |

---

## 5.1.3 Session Logon-10000

This message must be the first message sent by the participant to the assigned gateway 
authenticating the MCX ETI session.

Session Logon-10000

Session Logon Response-10001

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETImes-sage layout.  Value:  10000 (Logon,MsgType = A) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Must be set to 1. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 108 | HeartBtInt | N | 4 | 24 | unsigned int | heartbeat interval 0 = gateway will |
|  |  |  |  |  |  | not  take  any  action  for  skippedheartbeats  (only  allowed  for  non-production  environments)  not  set:gateway will define interval |
| 20055 | PartyIDSessionID | Y | 4 | 28 | unsigned int | Session ID |
| 701 | EncryptedData-MessageSize | N | 4 | 32 | unsigned int | Size of Encrypted Message |
| 1408 | DefaultCstmApplVer-ID | Y | 30 | 36 | Fixed String(0-terminable) | Indicates the ETI interface version theparticipant application uses.The inter-face version can be found in XML andC-Header files.Valid characters: \x01-\x7E |
| 30744 F | iller7 | N | 32 | 66 | Fixed String(0-terminable) |  |
| 25012 | ApplUsageOrders | Y | 1 | 98 | char | Participant application: type of orderprocessing. |
| 25012 | ApplUsageOrders | Y | 1 | 98 | char | Description |
| 25012 | ApplUsageOrders | Y | 1 | 98 | char | Automated |
| 25012 | ApplUsageOrders | Y | 1 | 98 | char | Manual |
| 25012 | ApplUsageOrders | Y | 1 | 98 | char | Both (Automated and Man-ual) |
| 25012 | ApplUsageOrders | Y | 1 | 98 | char | None |
| 25013 | ApplUsageQuotes | Y | 1 | 99 | char | Participant application: type of quoteprocessing. |
| 25013 | ApplUsageQuotes | Y | 1 | 99 | char | Description |
| 25013 | ApplUsageQuotes | Y | 1 | 99 | char | Automated |
| 25013 | ApplUsageQuotes | Y | 1 | 99 | char | Manual |
| 25013 | ApplUsageQuotes | Y | 1 | 99 | char | Both (Automated and Man-ual) |
| 25013 | ApplUsageQuotes | Y | 1 | 99 | char | None |
| 25014 | OrderRoutingIndicator | Y | 1 | 100 | char | Indicates if the participant applicationis an order routing system. |
| 25014 | OrderRoutingIndicator | Y | 1 | 100 | char | Description |
| 25014 | OrderRoutingIndicator | Y | 1 | 100 | char | Yes |
| 25014 | OrderRoutingIndicator | Y | 1 | 100 | char | No |
| 1600 | FIXEngineName | N | 30 | 101 | Fixed String(0-terminable) | Provides  the  name  of  the  infras-tructure component being used forsession level communication.Normally |
|  |  |  |  |  |  | this would be the FIX Engine or FIXGateway product name.Valid characters: \x20, \x22-\x7B,\x7D, \x7E |
| 1601 | FIXEngineVersion | N | 30 | 131 | Fixed String(0-terminable) | Provides the version of the FIX infras-tructure component.Valid characters: \x20, \x22-\x7B,\x7D, \x7E |
| 1602 | FIXEngineVendor | N | 30 | 161 | Fixed String(0-terminable) | Provides the name of the vendor pro-viding the FIX infrastructure compo-nent.Valid characters: \x20, \x22-\x7B,\x7D, \x7E |
| 1603 | ApplicationSystem-Name | Y | 30 | 191 | Fixed String(0-terminable) | Provides the name of the applica-tion system being used to generateMCX ETI application messages.Thisnormally be a trading system, OMS,or EMS.Valid characters: \x20,  \x22-\x7B,\x7D, \x7E |
| 1604 | ApplicationSystem-Version | Y | 30 | 221 | Fixed String(0-terminable) | Provides the version of the applicationsystem being used to initiate  ETIapplication messages.Valid characters: \x20,  \x22-\x7B,\x7D, \x7E |
| 1605 | ApplicationSystem-Vendor | Y | 30 | 251 | Fixed String(0-terminable) | Provides  the  vendor  of  the  appli-cation  system.Valid characters: \x20,  \x22-\x7B,\x7D, \x7E |
| 554 | Password | Y | 344 | 281 | Fixed String(0-terminable) | Password.Should be encrypted with RSA andprovided. |
| 39070 | Pad7 | N | 7 | 625 | Fixed String | Not used. |

---

## 5.1.4 Session Logon Response-10001

This message confirms the Session Logon request.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10001 (Logon,MsgType = A) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | U | 4 | 28 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 1614 | ThrottleTimeInterval | Y | 8 | 32 | signed int | Throttle time interval in number ofmilliseconds;  applicable for transac-tion limit. |
| 820 | LastLoginTime | N | 8 | 40 | UTCTimestamp | The time when last successful loginwas performed. |
| 703 | LastLoginIP | N | 4 | 48 | unsigned int | The IP address of the machine fromwhere last successful login was made. |
| 1613 | ThrottleNoMsgs | Y | 4 | 52 | unsigned int | Transaction limit per ThrottleTime-Interval (1614). If set to 0, throttlingwill be switched off. |
| 25002 T | hrottleDisconnect-Limit | Y | 4 | 56 | unsigned int | Disconnect limit:  maximum numberof sequential message rejects due tothrottle violation allowed by the MCXETI. |
| 108 | HeartBtInt | Y | 4 | 60 | unsigned int | Actual  heartbeat  interval,  gatewayoverrides out of range value or sets todefault if logon omits the field |
| 25004 | SessionInstanceID | Y | 4 | 64 | unsigned int | Unique ID for the session instance as-signed by the MCX system during ses-sion logon; to be communicated tohelpdesk for troubleshooting. |
| 339 | TradSesMode | Y | 1 | 68 | unsigned int | Environment Type |
| 339 | TradSesMode | Y | 1 | 68 | unsigned int | Description |
| 339 | TradSesMode | Y | 1 | 68 | unsigned int | Simulation/Test |
| 339 | TradSesMode | Y | 1 | 68 | unsigned int | Production |
| 339 | TradSesMode | Y | 1 | 68 | unsigned int |  |
| 704 | NoOfPartition | N | 1 | 69 | unsigned int | Total number of Partitions in the seg-ment. |
| 705 D | aysLeftForPassword-Expiry | N | 1 | 70 | unsigned int | Number of days remaining for the ex-piry of password.  The field will bepopulated 3 days prior to the expiryelse it will contain "no value"data. |
| 706 | GraceLoginsLeft | N | 1 | 71 | unsigned int | Number of successful logins allowedwith expired password. The field willhave value set , once the password isexpired else it would contain no valuedata.  The value 0 indicates no newlogin with expired password will be al-lowed. |
| 1408 D | efaultCstmApplVer-ID | Y | 30 | 72 | Fixed String(0-terminable) | Indicates  the  MCX  ETI  interfaceversion  the  ETI  gateway  softwareuses.Must be set as 2.3 Valid charac-ters: \x20, \x22-\x7B, \x7D, \x7EValid characters: \x01-\x7E |
| 39020 | Pad2 | U | 2 | 102 | Fixed String | not used |

---

## 5.1.5 Session Logout-10002

Session Logout request to terminate a MCX ETI session.

Session Logout-10002

Session Logout Response-10003

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10002 (Logout,MsgType = 5) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
| <Re | questHeader> |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | U | 4 | 20 | unsigned int | Should be populated with sessionID. |

---

## 5.1.6 Session Logout Response-10003

This message confirms the Session Logout request.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10003 (Logout,MsgType = 5) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <Re | sponseHeader> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |

---

## 5.1.7 User Logon-10018

Each user needs to logon to MCX via the User Logon message.

User Logon-10018

User Logon Response-10019

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a  ETI mes-sage layout.   Value:  10018 (User-Request, MsgType = BE) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | U | 4 | 20 | unsigned int | not used |
|  |  |  |  |  |  |  |
| 553 | Username | Y | 4 | 24 | unsigned int | The trader ID |
| 701 | EncryptedData-MessageSize | N | 4 | 28 | unsigned int | Size of Encrypted Message |
| 30744 F | iller7 | N | 32 | 32 | Fixed String(0-terminable) | Valid characters: \x01-\x7E |
| 554 | Password | Y | 344 | 64 | Fixed String(0-terminable) | Password.Password should be encrypted withRSA and provided. |

## 5.1.8 User Logon Response-10019

The User Logon Response message is used to confirm a user logon.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes- |

---

|  |  |  |  |  |  | sage layout.   Value:  10019 (User-Response, MsgType = BF) |
| --- | --- | --- | --- | --- | --- | --- |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | U | 4 | 28 | Fixed String | not used |
| 820 | LastLoginTime |  |  |  |  | The time when last successful loginwas performed. |
| 820 | LastLoginTime | N | 8 | 32 | UTCTimestamp | The time when last successful loginwas performed. |
| 705 D | aysLeftForPassword-Expiry | N | 1 | 40 | unsigned int | Number of days remaining for the ex-piry of password.  The field will bepopulated 3 days prior to the expiryelse it will contain "no value"data. |
| 706 | GraceLoginsLeft | N | 1 | 41 | unsigned int | Number of successful logins allowedwith expired password. The field willhave value set , once the password isexpired else it would contain no valuedata.  The value 0 indicates no newlogin with expired password will be al-lowed. |
| 39060 | Pad6 | N | 6 | 42 | Fixed String | Not Used. |

## 5.1.9 User Logout-10029

Each user may logout from MCX via the User Logout message.

User Logout-10029
User Logout Response-10024

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  10029 (User-Request, MsgType = BE) |

---

| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| --- | --- | --- | --- | --- | --- | --- |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | U | 4 | 20 | unsigned int | not used |
|  |  |  |  |  |  |  |
| 553 | Username | Y | 4 | 24 | unsigned int | The trader ID |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |

## 5.1.10 User Logout Response-10024

The User Logout Response message is used to confirm a user logout.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  10024 (User-Response, MsgType = BF) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <Respon | seHeader> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |

---

## 5.1.11 Throttle Update Notification-10028

This message informs about throttle parameters that have been updated intraday.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  10028 (User-Notification, MsgType = CB) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |
|  |  |  |  |  |  |  |
| 1614 | ThrottleTimeInterval | Y | 8 | 16 | signed int | Throttle time interval in number ofmilliseconds;  applicable for transac-tion limit. |
| 1613 | ThrottleNoMsgs | Y | 4 | 24 | unsigned int | Transaction limit per ThrottleTime-Interval (1614). If set to 0, throttlingwill be switched off. |
| 25002 T | hrottleDisconnect-Limit | Y | 4 | 28 | unsigned int | Disconnect limit:  maximum numberof sequential message rejects due tothrottle violation allowed by the MCXETI. |

## 5.1.12 Heartbeat-10011

The Heartbeat message is used by the MCX ETI gateway to monitor the status of the 
communication link to the ETI client during periods of inactivity.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10011 (Heart-beat, MsgType = 0) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |

---

## 5.1.13 Heartbeat Notification-10023

The Heartbeat notification may be used by the ETI client to monitor the status of the 
communication link to the MCX ETI gateway during periods of inactivity.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10023 (Heart-beat, MsgType = 0) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |

## 5.1.14 Session password change-10997

This message is for Session Password Change

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a ETI mes-sage layout.  Value:  10997(SessionPasswordChange,MsgType = U84) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 20055 | PartyIDSessionID | Y | 4 | 24 | unsigned int | Session ID. |
| 710 | Filler 2 | N | 4 | 28 | unsigned int | Not used. |
| 554 | Password | Y | 344 | 32 | Fixed String(0-terminable) | Password.Should be encrypted with RSA andprovided. |
| 735 | NewPassword | Y | 344 | 376 | Fixed String(0-terminable) | Should be encrypted with RSA andprovided. |

---

## 5.1.15 Session password change Response-10995

This Message confirms session password change.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10995(SessionPasswordChangeResponse,MsgType = U82) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | esponseHeader> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |

## 5.1.16 User password change-10996

This message is used to change the password of the user.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a ETI mes-sage layout.  Value:  10996(UserPasswordChangeRequest,MsgType = U83) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 553 | Username | Y | 4 | 24 | unsigned int | User ID. |

---

| 701 | Filler2 | N | 4 | 28 | unsigned int | Not used. |
| --- | --- | --- | --- | --- | --- | --- |
| 554 | Password | Y | 344 | 32 | Fixed String(0-terminable) | Password.Should be encrypted with RSA andprovided. |
| 735 | NewPassword | Y | 344 | 376 | Fixed String(0-terminable) | Password.Should be encrypted with RSA andprovided. |

## 5.1.17 User Password Change Response - 10990

This message confirms the User Password Change request.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10990(UserPasswordChangeResponse,MsgType = U85) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <Respon | seHeader> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |

## 5.1.18 Session Logout Notification-10012

The Session Logout Notification message is used to inform about an unsolicited session logout 
triggered by the operator of the MCX system or by the MCX system itself.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10012 (Logout,MsgType = 5) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |
|  |  |  |  |  |  |  |
| 30354 | VarTextLen | Y | 2 | 16 | Counter | Number of used bytes for field VarText |

---

|  |  |  |  |  |  | (30355). |
| --- | --- | --- | --- | --- | --- | --- |
| 39060 | Pad6 | N | 6 | 18 | Fixed String | Not Used. |
| 30355 | VarText | Y | 2000 | 24 | Variable String | News text. Actual violation/Alert textValid characters: \x09, \x0A, \x0D,\x20-\x7B, \x7D, \x7E |

## 5.1.19 User Logout Notification-10043

The User Logout Notification message is used to confirm the forced logout of a user from the    
session that receives the message

| Tag Field Name |  | Req’d Len |  | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a  ETI mes-sage layout. Value: 10043 (Logout,MsgType = 5) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Gateway request in timestamp. |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not used. |

## 5.2 Order Handling

## 5.2.1 New Order Single-10100

The New Order Single message is used by the participant to submit an order for single leg 
securities. This message is sent to the service “Order and Quote Management”.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  10100 (New-OrderSingle, MsgType = D) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |

---

| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 44 | Price | N | 8 | 24 | PriceType | Limit price. Required if OrdType (40)is Limit (2) or Stop Limit (4). |
| 99 | StopPx | N | 8 | 32 | PriceType | Stop price. Required if OrdType (40)is Stop (3) or used as the trigger pricefor an One-cancels-the-other order. |
| 707 M | arketProtection-Percentage | N | 8 | 40 | PriceType | Price per unit of quantity (e.g.  pershare) In Case of Limit Price, NoValue is required to be send. |

| Tag | Field Name | Req’d | Len | Ofs | Data Type |  |  |  | Description |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 708 | TerminalInfo | Y | 8 | 48 | unsigned int |  |  |  | Total 15 CharactersFor    1st-   12    charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |  |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | Total 15 CharactersFor    1st-   12    charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |  |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | Total 15 CharactersFor    1st-   12    charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |  |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | Total 15 CharactersFor    1st-   12    charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |  |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | Total 15 CharactersFor    1st-   12    charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |  |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | Total 15 CharactersFor    1st-   12    charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |  |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | Total 15 CharactersFor    1st-   12    charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |  |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  |  |  | Value | Description |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | 11111111111 | IBT |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | 22222222222 | DMA |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  |  |  | 33333333333 | Wireless Technology |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | 0 | Order not generatedthrough   Programtrading software |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  | 1 | Order    generatedthrough   Programtrading software |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  |  | Smart  Order  Rout-2 ing without Programtrading software |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  |  | Smart Order Routing3 with program tradingsoftware |  |  |  |  |
| 708 | TerminalInfo |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 11 | ClOrdID | N | 8 | 56 | unsigned int |  |  |  | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining. |  |  |  |  |  |
| 727 | StrategyID | Y | 8 | 64 | unsigned int |  |  |  | Strategy Approved by the exchangeshould be used.Range being from 0 to99999 |  |  |  |  |  |

---

| 734 | StrategyTriggerSeqNo | Y | 8 | 72 | unsigned int | Strategy Sequence numbers |
| --- | --- | --- | --- | --- | --- | --- |
| 709 F | iller1 | N | 8 | 80 | unsigned int | Not Used. |
| 20655 T | argetPartyIDSession-ID | Y | 4 | 88 | unsigned int | Session ID |
| 711 | Echo | Y | 4 | 92 | signed int | Vendors can use this as a reference inresponse from exchange |
| 38 | OrderQty | Y | 8 | 96 | Qty | Total Order Quantity. |
| 732 | DisclosedQty | N | 8 | 104 | Qty | The quantity to be made visible in themarket data.DisclosedQty is set to 0in case full qty needs to be disclosed. |

|  | Tag Field Name 432 ExpireDate | Req’d N | Len 4 | Ofs 112 | Data Type LocalMktDate | Description | of order expiry.   Required if TimeInForce (59) = 6 then Expire- Date should be YYYYMMDD format, if  TimeInForce(59)=0  then  Expire- Date should be No value |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 432 ExpireDate | Req’d N | Len 4 | Ofs 112 | Data Type LocalMktDate |  | of order expiry.   Required if TimeInForce (59) = 6 then Expire- Date should be YYYYMMDD format, if  TimeInForce(59)=0  then  Expire- Date should be No value |
|  | Tag Field Name 432 ExpireDate | Req’d N | Len 4 | Ofs 112 | Data Type LocalMktDate |  | of order expiry.   Required if TimeInForce (59) = 6 then Expire- Date should be YYYYMMDD format, if  TimeInForce(59)=0  then  Expire- Date should be No value |
|  | 1300 MarketSegmentID | Y | 4 | 116 | signed int | Product identifier. |  |
|  | 30048 SimpleSecurityID | Y | 4 | 120 | unsigned int | Instrument identifier for simple instru- ments |  |
|  | 712 RegulatoryID | N | 4 | 124 | unsigned int | Not Used |  |
|  | 713 Filler4 | N | 2 | 128 | unsigned int | Not Used. |  |
|  | 20096 PartyIDTakeUp- TradingFirm | N | 5 | 130 | Fixed String | Not Used. |  |
|  | 20013 PartyIDOrder- OriginationFirm | N | 7 | 135 142 151 | Fixed String | Not used. |  |
|  | 20032 PartyIDBeneficiary | N Y | 9 | 135 142 151 | Fixed String | Not Used. |  |
|  | 714 AccountType | N Y | 1 1 | 135 142 151 | unsigned int |  |  |
|  | 714 AccountType | N Y | 1 1 |  | unsigned int | Value Description 1 Own |  |
|  | 714 AccountType |  | 1 1 |  | unsigned int | Value Description 1 Own |  |
|  | 714 AccountType |  | 1 1 |  | unsigned int | Value Description 1 Own |  |
|  | 714 AccountType |  | 1 1 |  | unsigned int | 3 | Client |
|  | 714 AccountType |  | 1 1 |  | unsigned int | 5 | Institution |
|  | 714 AccountType |  | 1 1 |  | unsigned int |  |  |
|  | 714 AccountType |  | 1 1 |  | unsigned int |  |  |
|  | 28703 ApplSeqIndicator | Y | 1 1 | 152 | unsigned int |  |  |
|  | 28703 ApplSeqIndicator |  | 1 1 |  | unsigned int |  |  |
|  | 28703 ApplSeqIndicator |  |  |  | unsigned int |  |  |
|  | 28703 ApplSeqIndicator |  |  |  | unsigned int | Value | Description |
|  | 28703 ApplSeqIndicator |  |  |  | unsigned int |  |  |
|  | 28703 ApplSeqIndicator |  |  |  | unsigned int | 0 | No recovery required |
|  | 28703 ApplSeqIndicator |  |  |  | unsigned int |  |  |
|  | 28703 ApplSeqIndicator |  |  |  | unsigned int | 1 | Standard Order |
|  | 54 Side | Y | 1 | 153 | unsigned int | Side of the order. |  |
|  |  | Y | 1 | 153 | unsigned int | Value Description |  |
|  |  | Y | 1 | 153 | unsigned int | 1 | Buy |
|  |  | Y | 1 | 153 | unsigned int | 2 Sell |  |
|  | 40 OrdType | Y | 1 | unsigned int |  | Order type. |  |
|  | 40 OrdType |  | 1 | unsigned int |  | Value Description |  |

---

|  |  |  |  |  |  | 2 | Limit |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  | 3 | Stop Market |
|  |  |  |  |  |  | 4 | Stop Limit |
|  |  |  |  |  |  | 5 | Market To Limit |
|  |  |  |  |  |  | 6 | Auction Buy IN |
|  |  |  |  |  |  | 7 | Auction_Sell_Out |

| Tag | Field Name | Req&#x27;d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 28710 | PriceValidityCheck-Type | Y | 1 | 139 | unsigned int | Not Used. |  |
| 28710 | PriceValidityCheck-Type | Y | 1 | 139 | unsigned int | Value | Description |
| 28710 | PriceValidityCheck-Type | Y | 1 | 139 | unsigned int | 0 | None |
| 59 | TimeInForce | Y | 1 | 140 | unsigned int | Execution and trading restriction pa- rameters supported by MCX. |  |
| 59 | TimeInForce | Y | 1 | 140 | unsigned int | Value | Description |
| 59 | TimeInForce | Y | 1 | 140 | unsigned int | 0 | Day (DAY) |
| 59 | TimeInForce | Y | 1 | 140 | unsigned int | 1 | Good Till Cancelled(GTC)-Standard Orders only |
| 59 | TimeInForce | Y | 1 | 140 | unsigned int | 3 | Immediate or Cancel (IOC) |
| 59 | TimeInForce | Y | 1 | 140 | unsigned int | 6 | Good Till Date(GTD)-Standard Orders only |
| 59 | TimeInForce | Y | 1 | 140 | unsigned int | 7 | Session (EOS) |
| 18 | ExecInst | Y | 1 | 141 | unsigned int | Instructions for order handling on ex- change trading floor. If more than one instruction is applicable to an order, this field can contain multiple instruc- tions separated by space. |  |
| 18 | ExecInst | Y | 1 | 141 | unsigned int | Value | Description |
| 18 | ExecInst | Y | 1 | 141 | unsigned int | 1 | Persistent Order(FIX value-H) |
| 18 | ExecInst | Y | 1 | 141 | unsigned int | 2 | Non-Persistent Order(FIX value-Q) |
| 715 | SMPFOrderIdentifier | Y | 1 | 142 | unsigned int |  |  |
| 715 | SMPFOrderIdentifier | Y | 1 | 142 | unsigned int | Value | Description |
| 715 | SMPFOrderIdentifier | Y | 1 | 142 | unsigned int | 0 | Passive |
| 715 | SMPFOrderIdentifier | Y | 1 | 142 | unsigned int | 1 | Active |
| 716 | Filler5 | N | 1 | 143 | unsigned int | Not Used. |  |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 625 | TradingSessionSubID | N | 1 | 160 | unsigned int | Used.Valid values are defined after thetable. |  |
| 1815 | TradingCapacity | Y | 1 | 161 | unsigned int | Not used.Set it to 1 by default. |  |
| 1 | Account | Y | 2 | 162 | Fixed String(0-terminable) | Not used.Must be sent as A1Valid characters: 1-9,  \x41,  \x47,\x49, \x4D, \x50, \x52 |  |
| 77 | PositionEffect | Y | 1 | 164 | char | Must be set to C, Valid characters:\x01-\x7E |  |
| 20075 | PartyIDLocationID | N | 2 | 165 | Fixed String(0-terminable) | Not Used. |  |
| 1031 | CustOrderHandling-Inst | N | 1 | 167 | Fixed String | Not usedValid characters: \x20,  \x22-\x7B,\x7D, \x7EValid characters: \x20,  \x22-\x7B,\x7D, \x7E |  |
| 718 | UserReferenceText | N | 20 | 168 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide additionalregulatory information(according  torespective rules andregs,  circularsand/or bilateral coordination betweenparticipantand  Trading SurveillanceOffice). Valid characters: \x20, \x22-\x7B, \x7D, \x7E. |  |

Valid Values for TradingSessionSubID are given below:

| Value | Description |
| --- | --- |
| 1 | Start of day |
| 2 | Pre-Trading |
| 3 | Trading |
| 4 | Closing Auction |
| 5 | Post trading |
| 6 | End of day |
| 7 | Post end of day |
| 8 | Halt |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 25007 F | reeText1 | Y | 12 | 188 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 200 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 25009 F | reeText3 | N | 12 | 212 | Fixed String | Free-format text field fortraderspecific  or  customer-relatedcomments.Valid  characters:\x00,  \x21,  \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |

---

## 5.2.2 New Order Single (short layout)-10125

This short order message layout is used by the participant to submit an order in a simple instrument. 
Selected order attributes are implicitly set. Using short layout only Limit orders, other than GTD orders 
can be placed. This message is sent to the service “Order and Quote Management”.
Order placed through short layout messages are implicitly non persistent.

Order

Executions?

Without Execution

New Order Response (Standard Order)
(Session Data)

Immediate Execution Response
(Session Data)

Extended Order Information
(Listener Data)

Extended Order Information
(Listener Data)

Trade Notification
(Trade)

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  10125 (New-OrderSingle, MsgType = D) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 44 | Price | Y | 8 | 24 | PriceType | Limit price. |

---

|  | Tag Field Name 708 TerminalInfo | Req’d Y | Len | Ofs | Data Type |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | 32 | unsigned int |  | Total 15 Characters For 1st- 12 characters value(11111111111-33333333333), For 13th character value(0-3) 14th - 15th digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  | Value Description |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  | 11111111111 IBT |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  | 22222222222 DMA |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  |  | 33333333333 Wireless Technology |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  |  | Order not generated 0 through Program |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  |  | trading software Order generated 1 through Program |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  | trading software Smart Order  Rout- |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  | 2 ing without Program trading software |  |  |
|  | Tag Field Name 708 TerminalInfo |  |  |  |  |  |  | Smart Order Routing 3 with program trading software |  |
|  | 11 ClOrdID | N | 8 | 40 | unsigned int |  | for client order id chaining. | Client Order ID: Unique participant defined order request identifier; used |  |
|  | 38 OrderQty | Y | 8 | 48 | Qty |  | Total Order Quantity. |  |  |
|  | 732 DisclosedQty | N | 8 | 56 | Qty |  |  | The quantity to be made visible in the market data.DisclosedQty is set to 0 in case full qty needs to be disclosed. |  |
|  | 727 StrategyID | Y | 8 | 64 | unsigned int |  | 99999 | Strategy Approved by the exchange should be used.Range being from 0 to |  |
|  | 728 StrategySequence- Number | Y | 8 | 72 | unsigned int |  |  | Strategy Sequence numbers |  |
|  | 30048 SimpleSecurityID | Y | 4 | 80 | unsigned int |  | ments. | Instrument identifier for simple instru- |  |
|  | 710 Filler2 | N | 4 | 84 | unsigned int |  | Not Used. |  |  |
| 713 Filler4 |  | N | 2 | 88 | unsigned int |  | Not Used. |  |  |

---

|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | Ofs | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 90 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 90 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 90 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 90 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 90 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | 54 Side | Y | 1 | 91 | unsigned int | Side of the order. |
|  | 54 Side |  | 1 | 91 | unsigned int | Value Description |
|  | 54 Side |  | 1 | 91 | unsigned int | 1 Buy |
|  | 54 Side |  | 1 | 91 | unsigned int | 2 Sell |
|  | 54 Side |  |  | 92 | unsigned int | Not Used. |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 92 | unsigned int | Not Used. |
|  |  | Y | 1 | 92 | unsigned int | Value Description |
|  |  | Y | 1 | 92 | unsigned int | 0 None |
|  | 59 TimeInForce | Y | 1 | 93 | unsigned int | Execution and trading restriction pa- rameters supported by MCX. |
|  | 59 TimeInForce | Y | 1 | 93 | unsigned int | Value Description |
|  | 59 TimeInForce | Y | 1 | 93 | unsigned int | 0 Day (DAY) |
|  | 59 TimeInForce | Y | 1 | 93 | unsigned int | 3 Immediate or Cancel (IOC) |
|  | 715 SMPFOrderIdentifier | Y | 1 | 94 | unsigned int | Indicates the preference of order to be cancelled if self-trade is encoun- tered.Preference not applicable in a modification requests. |
|  | 715 SMPFOrderIdentifier | Y | 1 | 94 | unsigned int | Value Description |
|  | 715 SMPFOrderIdentifier | Y | 1 | 94 | unsigned int | 0 Passive |
|  | 715 SMPFOrderIdentifier | Y | 1 | 94 | unsigned int | 1 Active |

---

| 18 | ExecInst | Y | 1 | 95 | unsigned int | Instructions for order handling on ex-change trading floor.If more than oneinstruction is applicable to an order,this field can contain multiple instruc-tionsseparated by space. |
| --- | --- | --- | --- | --- | --- | --- |
| 18 | ExecInst | Y | 1 |  | unsigned int | Description |
| 18 | ExecInst |  |  |  | unsigned int | Non-Persistent  Order(FIXvalue-Q) |
| 18 | ExecInst |  |  |  |  |  |
| 25007 | FreeText1 | Y | 12 | 96 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 108 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |

---

## 5.2.3 New Order Response (Standard Order)-10101

This message confirms a New Order request for a Standard Order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:   10101(ExecutionReport, MsgType = 8) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp | not used |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | Not Used. |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Not Used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not Used. |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 5948 | PartitionID | Y | 2 | 60 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Value Description |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | 4Session Data |
| 28704 | ApplMsgID | N | 16 | 63 | data | Application  message  identifier  as-signed to an order or quote event. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Last Message |
| 893 | LastFragment | Y | 1 | 79 | unsigned int |  |
| <M | essage Body> |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 80 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | 11 ClOrdID | N | 8 | 88 | unsigned int | Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |
|  | 719 PriceMkToLimitPx | N | 8 | 96 | PriceType | The price at which the market order got converted into regular limit or- der.Applicable for OrdType (40) = 5. |
|  | 720 Reserved1 | N | 8 | 104 | unsigned int | Reserved Field |
|  | 721 Reserved2 | N | 8 | 112 | unsigned int | Reserved Field |
|  | 17 ExecID | Y | 8 | 120 | UTCTimestamp | Transaction timestamp. |
|  | 21009 TrdRegTSEntryTime | Y | 8 | 128 | UTCTimestamp | The entry timestamp is the time of the creation of the order. |
|  | 722 Reserve14 | Y | 8 | 136 | UTCTimestamp | Not Used. |
|  | 723 LstUpdtTime | Y | 8 | 144 | UTCTimestamp | last updated time. |
|  | 709 Filler1 | N | 8 | 152 | unsigned int | the creation of the order. |
|  | 30048 SimpleSecurityID | Y | 4 | 160 | unsigned int | Instrument Identifier. |
|  | 710 Filler2 | N | 4 | 164 | unsigned int | Not Used. |
|  | 39 OrdStatus | N | 2 | 168 | unsigned int char char | Not Used. |
|  | 39 OrdStatus | Y | 1 | 170 | unsigned int char char | Conveys the current status of an or- der. |
|  | 39 OrdStatus |  |  |  | unsigned int char char | Value Description |
|  | 39 OrdStatus |  |  |  | unsigned int char char | 0 New |
|  | 39 OrdStatus |  |  |  | unsigned int char char | 1 Partially filled |
|  | 39 OrdStatus |  |  |  | unsigned int char char | 2 Filled |
|  | 39 OrdStatus |  |  |  | unsigned int char char | 4 Cancelled |
|  | 39 OrdStatus |  |  |  | unsigned int char char | 7 RRM Suspended |
|  | 39 OrdStatus |  |  |  | unsigned int char char | 8 SquareOff Suspended |
|  | 150 ExecType | Y | 1 | 172 |  | The reason why this message was gen- erated. |
|  |  |  | 1 | 172 |  | Value Description |
|  |  |  | 1 | 172 |  | 0 New |
|  |  |  | 1 | 172 |  | 4 Cancelled |
|  |  |  | 1 | 172 |  | L Triggered |
|  |  |  | 1 | 172 |  | 5 Replaced |
|  | 378 ExecRestatement- Reason | Y | 2 |  | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. |
|  |  |  |  |  |  | Valid values are listed after this table. |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | 1227 ProductComplex | Y | 1 | 174 | unsigned int | This field qualifies an instrument type on MCX. |
|  | 1227 ProductComplex | Y | 1 | 174 |  | Value Description |
|  | 1227 ProductComplex | Y | 1 | 174 |  | 1 Simple instrument |
|  | 1227 ProductComplex | Y | 1 | 174 |  | 5 Futures Spread |
|  | 716 Filler5 | N | 1 | 175 | unsigned int | Not Used. |

Valid Values of ExecRestatementReason (datatype ExecRestatementReason)

| Valid |  |  |  |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Value |  |  |  |  |  |  |  |
| 1 | Order book restatement |  |  |  |  |  |  |
| 101 | Order add accepted |  |  |  |  |  |  |
| 102 | Order modify accepted |  |  |  |  |  |  |
| 103 | Order delete accepted |  |  |  |  |  |  |
| 105 | IOC Order Cancelled |  |  |  |  |  |  |
| 108 | Book Order executed |  |  |  |  |  |  |
| 114 | Order has been changed to IOC |  |  |  |  |  |  |
| 135 | Market Order triggered and executed |  |  |  |  |  |  |
| 145 | Start Of Day Processing |  |  |  |  |  |  |
| 146 | End Of Day Processing |  |  |  |  |  |  |
| 155 | Order Refreshed |  |  |  |  |  |  |
| 172 | Stop Order has been triggered |  |  |  |  |  |  |
| 215 | Risk Reduction Timer Expired |  |  |  |  |  |  |
| 248 | Active order deletion due to SMPF |  |  |  |  |  |  |
| 250 | Active order modification due to SMPF |  |  |  |  |  |  |
| 252 | Passive order deletion due to SMPF |  |  |  |  |  |  |
| 254 | Passive oder modification due to SMPF |  |  |  |  |  |  |
| 261 | Panic Cancel |  |  |  |  |  |  |
| 302 | RRMIN |  |  |  |  |  |  |
| 303 | SQUAREOFFIN |  |  |  |  |  |  |
| 357 | Base Price Update |  |  |  |  |  |  |
| 358 | Order Deleted As PriceMoved Out Of DPL Range |  |  |  |  |  |  |

---

## 5.2.4 Replace Order Single-10106

This message is used to replace an existing order in a simple instrument. This message is sent to the 
service “Order and Quote Management”

| TagField Name      Req’dLen Ofs  Data Type  Description |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier for a  ETI mes-sage layout. Value: 10106  (Order-CancelReplaceRequest, MsgType =G) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
| <Reques | tHeader> |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
| <Messag | e Body> |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 24 | unsigned int | Exchange Order ID generated by theT7 System; it remains constant overthe lifetime of an order. |
| 11 | ClOrdID | N | 8 | 32 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining.ClOrdID should be mention inNewOrderSingle and same valuecan be passed in ClOrdID as well asOrigClOrdID. |
| 41 | OrigClOrdID | N | 8 | 40 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring tothe specific order. |
| 44 | Price | N | 8 | 48 | PriceType | Limit price. Required if OrdType (40)is Limit (2) or Stop Limit (4). |
| 99 | StopPx | N | 8 | 56 | PriceType | Stop price. Required if OrdType (40)is Stop (3) or used as the trigger pricefor an One-cancels-the-other order. |
| 731 | MaxPricePercentage | N | 8 | 64 | PriceType | Price per unit of quantity (e.g.pershare) In Case of Limit Price, NoValue is required to be send. |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 708 | TerminalInfo | Y | 8 | 72 | unsigned int | Total 15 CharactersFor   1st-  12   charactersvalue(11111111111-33333333333),For 13th character value(0-3)14th - 15th digit should be validvendor code / in House CTCL codefor member |
| 708 | TerminalInfo | Y | 8 | 72 | unsigned int | Description |
| 708 | TerminalInfo | Y | 8 | 72 | unsigned int | IBT |
| 708 | TerminalInfo | Y | 8 | 72 | unsigned int | DMA |
| 708 | TerminalInfo | Y | 8 | 72 | unsigned int | Wireless Technology |
| 708 | TerminalInfo |  | 8 | 72 | unsigned int | Order not generated0 through  Programtrading software |
| 708 | TerminalInfo |  | 8 | 72 | unsigned int | Order   generated1 through  Programtrading software |
| 708 | TerminalInfo |  | 8 | 72 | unsigned int | Smart Order  Rout-2 ing without Programtrading software |
| 708 | TerminalInfo |  | 8 | 72 | unsigned int | Smart Order Routing3 with program tradingsoftware |
| 723 | LstUpdtTime | Y | 8 | 80 | UTCTimestamp | LstUpdtTime received in the order no-tification |
| 709 F | iller1 | N | 8 | 88 | unsigned int | Not Used. |
| 710 F | iller2 | N | 4 | 96 | unsigned int | Not Used. |
| 711 | Echo | Y | 4 | 100 | signed int | Vendors can use this as a reference inresponse from exchange |
| 38 | OrderQty | Y | 8 | 104 | Qty | Total Order Quantity. |
| 732 | DisclosedQty | N | 8 | 112 | Qty | The quantity to be made visible in themarket data.DisclosedQty is set to 0in case full qty needs to be disclosed. |
| 727 | StrategyID | Y | 8 | 120 | unsigned int | Strategy Approved by the exchangeshould be used.Range being from 0 to99999 |
| 734 | StrategyTriggerSeqNo | Y | 8 | 128 | unsigned int | Strategy Sequence numbers |
| 432 | ExpireDate | N | 4 | 136 | LocalMktDate | Not used. |
| 1300 | MarketSegmentID | Y | 4 | 140 | signed int | Product identifier. |
| 30048 | SimpleSecurityID | Y | 4 | 144 | unsigned int | Instrument identifier for simple instru-ments. |

---

|  | Tag Field Name 20655 TargetPartyIDSession- ID | Req’d N | Len | Ofs | Data Type |  | Description Session ID. |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 20655 TargetPartyIDSession- ID | Req’d N | 4 | 148 | unsigned int |  | Description Session ID. |
|  | 712 RegulatoryID | N | 4 | 152 | unsigned int |  | For participants to uniquely identify trading startegies |
|  | 713 Filler4 | N N N | 2 | 156 | unsigned int |  | Not Used. |
|  | 20096 PartyIDTakeUp- TradingFirm | N N N | 5 158 7 163 |  | Fixed String |  | Not Used. |
|  | 20013 PartyIDOrder- OriginationFirm |  | 5 158 7 163 |  | Fixed String |  | Not used. |
|  | 20032 PartyIDBeneficiary | N | 9 | 170 | Fixed String |  | Not Used. |
|  | 714 AccountType | Y | 1 | 179 |  |  |  |
|  | 714 AccountType |  | 1 | 179 |  | Value Description 1 Own |  |
|  | 714 AccountType |  | 1 | 179 |  | Value Description 1 Own |  |
|  | 714 AccountType |  | 1 | 179 |  |  | 3 Client |
|  | 714 AccountType |  | 1 | 179 |  |  | 5 Institution |
|  | 28703 ApplSeqIndicator | Y | 1 | 180 | unsigned int |  | Value Description 0 No recovery Required |
|  |  | Y |  | 180 |  |  | Value Description 0 No recovery Required |
|  |  | Y |  | 180 |  |  | Value Description 0 No recovery Required |
|  |  | Y |  | 180 |  |  | 1 Standard Order |
|  |  |  |  | 181 | unsigned int |  | Side of the order. Value Description 1 Buy |
|  | 54 Side | Y | 1 | 181 | unsigned int |  | Side of the order. Value Description 1 Buy |
|  |  | Y |  | 181 | unsigned int |  | Side of the order. Value Description 1 Buy |
|  |  | Y |  | 181 | unsigned int |  | Side of the order. Value Description 1 Buy |
|  |  | Y | 1 | 181 | unsigned int |  | 2 Sell |
|  | 40 OrdType | Y | 1 | 182 | unsigned int |  | Order type. Value Description |
|  | 40 OrdType | Y | 1 | 182 | unsigned int |  | Order type. Value Description |
|  | 40 OrdType | Y | 1 | 182 | unsigned int |  | 2 Limit |
|  | 40 OrdType | Y | 1 | 182 | unsigned int |  | 3 Stop Market |
|  | 40 OrdType | Y | 1 | 182 | unsigned int |  | 4 Stop Limit |
|  | 40 OrdType | Y | 1 | 182 | unsigned int |  | 5 Market To Limit |
|  | 40 OrdType | Y | 1 | 182 | unsigned int |  | 6 Auction Buy IN |
|  | 40 OrdType |  | 1 | 182 |  | 7 Auction_Sell_Out |  |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 183 | unsigned int Not Used. Value |  |  |
|  | 28710 PriceValidityCheck- Type |  | 1 | 183 | unsigned int Not Used. Value |  | Description |
|  | 28710 PriceValidityCheck- Type |  | 1 | 183 | unsigned int Not Used. Value |  | 0 None |

---

| Tag | Field Name | Req&#x27;d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 59 | TimeInForce | Y | 1 | 184 | unsigned int | Execution and trading restriction pa- rameters supported by MCX. |  |
| 59 | TimeInForce | Y | 1 | 184 | unsigned int | Value | Description |
| 59 | TimeInForce | Y | 1 | 184 | unsigned int | 0 | Day (DAY) |
| 59 | TimeInForce | Y | 1 | 184 | unsigned int | 1 | Good Till Cancelled(GTC)-Standard Orders only |
| 59 | TimeInForce | Y | 1 | 184 | unsigned int | 3 | Immediate or Cancel (IOC) |
| 59 | TimeInForce | Y | 1 | 184 | unsigned int | 6 | Good Till Date(GTD)-Standard Orders only |
| 59 | TimeInForce | Y | 1 | 184 | unsigned int | 7 | Session (EOS) |
| 18 | ExecInst | Y | 1 | 185 | unsigned int | Instructions for order handling on ex- change trading floor. If more than one instruction is applicable to an order, this field can contain multiple instruc- tions separated by space. |  |
| 18 | ExecInst | Y | 1 | 185 | unsigned int | Value | Description |
| 18 | ExecInst | Y | 1 | 185 | unsigned int | 1 | Persistent Order(FIX value-H) |
| 18 | ExecInst | Y | 1 | 185 | unsigned int | 2 | Non-Persistent Order(FIX value-Q) |
| 716 | Filler5 | N | 1 | 186 | unsigned int | Not Used. |  |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | Used. |  |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | Value | Description |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 1 | Start of Day |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 2 | Pre-Trading |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 3 | Trading |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 4 | Closing or closing auction |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 5 | Post-Trading |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 6 | End of Day |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 7 | Post End of Day |
| 625 | TradingSessionSubID | N | 1 | 187 | unsigned int | 8 | Halt |
| 1815 | TradingCapacity | Y | 1 | 188 | unsigned int | Not Used. |  |
| 750 | Filler6 | N | 1 | 189 | char |  |  |

---

| Tag | Field Name | Req’d | Len | Ofs  Data Type |  | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Account | Y | 2 | 190 | Fixed String(0-terminable) | Not used.Must be sent as A1Valid characters:1-9, \x41, \x47,\x49, \x4D, \x50, \x52 |
| 77 | PositionEffect | Y | 1 | 192 | char | must be set to C |
| 20075 | PartyIDLocationID | N | 2 | 193 | Fixed String(0-terminable) | Not Used. |
| 1031 | CustOrderHandling-Inst | N | 1 | 195 | Fixed String | not used.Valid characters:\x20, \x22-\x7B,\x7D, \x7E |
| 718 | UserReferenceText | N | 20 | 196 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide additional regulatoryinformation(according to respectiverules andregs, circulars and/orbilateral  coordination  betweenparticipantand Trading SurveillanceOffice).Valid characters: \x20, \x22-\x7B, \x7D, \x7E.Valid characters:\x20, \x22-\x7B,\x7D, \x7E |
| 25007 | FreeText1 | Y | 12 | 216 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 228 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 25009 | FreeText3 | N | 12 | 240 | Fixed String | Free-format text field fortraderspecific or customer-relatedcomments.Valid characters:\x00,\x21,\x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 39040 | Pad4 | N | 4 | 252 | Fixed String | Not Used. |

---

## 5.2.5 Replace Order Single (short layout)-10126

This short order message layout is used by the participant to replace an order in a simple 
instrument. Selected order attributes are implicitly set. This message is sent to the service “Order 
and Quote Management”.

Order placed through short layout messages are implicitly non persistent.

Replace Order Single (Short Layout)

Executions?

Without Execution or with deletion

Full or partial fill

Replace Order Response (Standard Order) (Session Data)

Immediate Execution Response (Session Data)

Extended Order Information (Listener Data)

Extended Order Information (Listener Data)

Trade Notification (Trade)

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10126 (Order-CancelReplaceRequest,  MsgType  =G) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
| 37 |  |  |  |  |  |  |
| 37 | OrderID | N | 8 | 24 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 32 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining.ClOrdID should be mention inNewOrderSingle and same value canbe passed in ClOrdID as well asOrigClOrdID. |
| 41 | OrigClOrdID | N | 8 | 40 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring tothe specific order. |
| 44 | Price | Y | 8 | 48 | PriceType | Limit price. Required if OrdType (40)is Limit (2) or Stop Limit (4). |

---

|  | Tag Field Name 708 TerminalInfo | Req’d Y | Len | Ofs 56 | Data Type unsigned int |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  | Total 15 Characters For 1st- 12 characters value(11111111111-33333333333), For 13th character value(0-3) 14th - 15th digit should be valid vendor code / in House CTCL code for member Value Description |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  | Total 15 Characters For 1st- 12 characters value(11111111111-33333333333), For 13th character value(0-3) 14th - 15th digit should be valid vendor code / in House CTCL code for member Value Description |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  | Total 15 Characters For 1st- 12 characters value(11111111111-33333333333), For 13th character value(0-3) 14th - 15th digit should be valid vendor code / in House CTCL code for member Value Description |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  | 11111111111 |  | IBT |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  |  | 22222222222 DMA |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  |  |  | 33333333333 Wireless Technology |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  |  |  | Order not generated 0 through Program trading software |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  |  |  | Order generated 1 through Program trading software |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  |  |  | Smart Order  Rout- |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  |  |  | 2 ing without Program trading software Smart Order Routing |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 56 | Data Type unsigned int |  |  |  | 3 with program trading software |
|  | 723 LstUpdtTime | Y | 8 | 64 | UTCTimestamp |  | Order)-10102 |  | Send the value of ExecID (Tag 17) from New Order Response (Lean |
|  | 38 OrderQty | Y | 8 | 72 | Qty |  |  | Total Order Quantity. |  |
|  | 732 DisclosedQty | N | 8 | 80 | Qty |  |  |  | The quantity to be made visible in the market data.DisclosedQty is set to 0 in case full qty needs to be disclosed. |
|  | 727 StrategyID | Y | 8 | 88 | unsigned int |  | 99999 |  | Strategy Approved by the exchange should be used.Range being from 0 to |
|  | 728 StrategySequence- Number | Y | 8 | 96 | unsigned int |  |  | Strategy Sequence numbers |  |
| 30048 SimpleSecurityID |  | Y | 4 | 104 | unsigned int |  | ments. |  | Instrument identifier for simple instru- |
| 30048 SimpleSecurityID |  | N | 4 | 108 | unsigned int |  | Not Used. |  |  |
| 713 Filler4 N |  |  | 2 | 112 | unsigned int |  | Not Used. |  |  |

---

|  | Tag Field Name 714 AccountType | Req’d Y | Len | Ofs | Data Type unsigned int | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 714 AccountType | Req’d Y | 1 | 114 | Data Type unsigned int |  |  |
|  | Tag Field Name 714 AccountType | Req’d Y | 1 | 114 | Data Type unsigned int | Value | Description |
|  | Tag Field Name 714 AccountType | Req’d Y | 1 | 114 | Data Type unsigned int |  | 1 Own |
|  | Tag Field Name 714 AccountType | Req’d Y | 1 | 114 | Data Type unsigned int |  | 3 Client |
|  | Tag Field Name 714 AccountType | Req’d Y | 1 | 114 |  |  | 5 Institution |
|  | 54 Side | Y | 1 | 115 | unsigned int | Side of the order. |  |
|  | 54 Side | Y |  | 115 |  | Value | Description |
|  | 54 Side | Y |  | 115 |  |  | 1 Buy |
|  | 54 Side | Y |  | 115 |  |  | 2 Sell |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 116 | unsigned int | Not Used. |  |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 116 | unsigned int | Value | Description |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 116 | unsigned int |  |  |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 116 | unsigned int |  | 0 None |
|  | 59 TimeInForce | Y | 1 | 117 | unsigned int |  | Execution and trading restriction pa- rameters supported by MCX. |
|  | 59 TimeInForce | Y | 1 | 117 | unsigned int | Value | Description |
|  | 59 TimeInForce | Y | 1 | 117 | unsigned int |  | 0 Day (DAY) |
|  | 59 TimeInForce | Y | 1 | 117 | unsigned int |  | 1 Good Till Cancelled (GTC) - Standard Orders only |
|  | 59 TimeInForce | Y | 1 | 117 | unsigned int | 3 | Immediate or Cancel (IOC) |
|  | 59 TimeInForce | Y | 1 | 117 | unsigned int |  | 7 End of Session (EOS) |

---

| Tag | Field Name | Req’d Len |  | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 18 | ExecInst | Y | 1 | 118 | unsigned int | Instructions for order handling on ex-change trading floor. If more than oneinstruction is applicable to an order,this field can contain multiple instruc-tions separated by space |
| 18 | ExecInst | Y | 1 | 118 | unsigned int | Description |
| 18 | ExecInst | Y | 1 | 118 | unsigned int | Non-Persistent  Order(FIXvalue-Q) |
| 25007 | FreeText1 | Y | 12 | 119 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 131 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 39000 | Pad1 | N | 1 | 143 | Fixed String | Not Used. |

---

## 5.2.6 Replace Order Response (Standard Order)-10107

This message confirms a Replace Order request for a Standard Order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:   10107(ExecutionReport, MsgType = 8) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp | not used |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | Not Used. |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Not Used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not Used. |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 5948 | PartitionID | Y | 2 | 60 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Value Description |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | 4Session Data |
| 28704 | ApplMsgID | N | 16 | 63 | data | Not set if the submitting session is notthe owner of the replaced order. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Last Message |
| 893 | LastFragment | Y | 1 | 79 | unsigned int |  |
| <M | essage Body> |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 80 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |

---

|  | Tag Field Name 11 ClOrdID | Req’d N | Len | Ofs Data Type 88 unsigned int |  | Description Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 11 ClOrdID | Req’d N | 8 | Ofs Data Type 88 unsigned int |  | Description Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |  |
|  | 41 OrigClOrdID | N | 8 | 96 | unsigned int | ClOrdID (11) of the last successfully processed task (request) referring to the specific order. |  |
|  | 48 SecurityID | Y | 8 | 104 | signed int | Instrument identifier. |  |
|  | 17 ExecID | Y | 8 | 112 | UTCTimestamp | Transaction timestamp. |  |
|  | 719 PriceMkToLimitPx | N | 8 | 120 | PriceType | The price at which the market order got converted into regular limit or- der.Applicable for OrdType (40) = 5. |  |
|  | 724 Filler10 | N | 8 | 128 | signed int |  |  |
|  | 725 Filler11 | N | 8 | 136 | signed int |  |  |
|  | 722 Reserve14 | Y | 8 | 144 | UTCTimestamp | Not Used. |  |
|  | 723 LstUpdtTime | Y | 8 | 152 | UTCTimestamp | LstUpdtTime for that order |  |
|  | 709 Filler1 | N | 8 | 160 | unsigned int | Not Used. |  |
|  | 151 LeavesQty | Y | 8 | 168 | Qty |  | Remaining quantity of an order. |
|  | 14 CumQty | Y | 8 | 176 | Qty | Cumulated executed quantity of an or- der. |  |
|  | 84 CxlQty | Y | 8 | 184 | Qty | Total quantity cancelled for this order. |  |
|  | 710 Filler2 | N | 4 | 192 | unsigned int | Not Used. |  |
|  | 713 Filler4 | N | 2 1 1 | 196 | unsigned int char | Not Used. |  |
|  | 39 OrdStatus | Y | 2 1 1 | 198 | unsigned int char |  | Conveys the current status of an or- |
|  |  |  | 2 1 1 |  | unsigned int char | der. |  |
|  |  |  | 2 1 1 |  | unsigned int char | Value | Description |
|  |  |  | 2 1 1 |  | unsigned int char |  | 0 New |
|  |  |  | 2 1 1 |  | unsigned int char | 1 Partially filled 2 Filled |  |
|  |  |  | 2 1 1 |  | unsigned int char | 1 Partially filled 2 Filled |  |
|  |  |  | 2 1 1 |  | unsigned int char |  | 4 Cancelled |
|  |  |  | 2 1 1 |  | unsigned int char | 7 RRM Suspended |  |
|  |  |  | 2 1 1 |  | unsigned int char |  | 8 SquareOff Suspended |
|  | 150 ExecType | Y |  | 199 | char | erated. | The reason why this message was gen- |
|  | 150 ExecType |  |  |  | char | Value Description |  |
|  | 150 ExecType |  |  |  | char | 0 New |  |
|  | 150 ExecType |  |  |  | char | 4 Cancelled |  |
|  | 150 ExecType |  |  |  | char |  | L Triggered |
|  | 150 ExecType |  |  |  | char | 5 Replaced |  |

---

|  | Tag Field Name 378 ExecRestatement- Reason | Req’d | Len | Ofs | Data Type |  |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. |  |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  | Value | Description |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  |  | 1 Order book restatement |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  |  | 102 Order modify accepted |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  |  | 101 Order add accepted |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  |  | 105 IOC Order Cancelled |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  |  | 135 Market Order triggered and executed |
|  | Tag Field Name 378 ExecRestatement- Reason | Y | 2 | 200 | unsigned int |  |  | 246 Self-Trade Order Deleted |
|  | 1227 ProductComplex | Y | 1 | 202 | unsigned int |  | This field qualifies an instrument type on MCX. |  |
|  | 1227 ProductComplex |  |  | 202 | unsigned int |  |  |  |
|  | 1227 ProductComplex |  |  | 202 | unsigned int |  | Value | Description |
|  | 1227 ProductComplex |  |  | 202 | unsigned int |  |  | 1 Simple instrument |
|  | 1227 ProductComplex |  |  | 202 | unsigned int |  |  | 5 Futures Spread |
|  | 716 Filler5 | N | 1 | 203 | unsigned int |  | Not Used. |  |
| 39040 Pad4 |  | N | 4 | 204 | Fixed String |  | Not Used. |  |

---

## 5.2.7 Reject-10010

All rejections and errors on an application and session level are communicated via the FIX 
standard Reject (3) message.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <Me | ssageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10010 (Reject,MsgType = 3) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <NRR | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | In timestamp;  filled always by thegateway |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp | not used |
| 21002 | Reserve1 | U | 8 | 24 | UTCTimestamp | not used |
| 21003 | Reserve2 | U | 8 | 32 | UTCTimestamp | not used |
| 7765 | Reserve3 | U | 8 | 40 | UTCTimestamp | not used |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Outgoing delta timestamp; filled al-ways by the gateway |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 1 Last Message |
| 893 | LastFragment | Y | 1 | 60 | unsigned int |  |
| 39030 | Pad3 | U | 3 | 61 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 373 | SessionRejectReason | Y | 4 | 64 | unsigned int | Error code.Valid values are listed after this table. |
| 30354 | VarTextLen | Y | 2 | 68 | Counter | Values will use user-defined values for appli-cation level errors as well |
|  | 1409 SessionStatus | Y | 1 | 70 | unsigned int | Session status. |
|  |  |  |  |  |  | Value Description |
|  |  |  |  |  |  | 0 Session active |
|  |  |  |  |  |  |  |
| 39000 Pad1 |  | N | 1 | 71 | Fixed String | Not Used. |
|  | 30355 VarText | Y | 2000 | 72 | Variable String | News text. Actual violation/Alert text Valid characters: \x09, \x0A, \x0D, \x20-\ x7B, \x7D, \x7E |

Valid Values of SessionRejectReason (datatype SessionRejectReason)

| Valid |  |  |  |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Value |  |  |  |  |  |  |  |
| 1 | Required Tag Missing |  |  |  |  |  |  |
| 5 | Value is incorrect (out of range) for this tag |  |  |  |  |  |  |
| 7 | Decryption problem |  |  |  |  |  |  |
| 11 | Invalid TemplateID |  |  |  |  |  |  |
| 16 | Incorrect NumInGroup count for repeating group |  |  |  |  |  |  |
| 99 | Other |  |  |  |  |  |  |
| 100 | Throttle limit exceeded |  |  |  |  |  |  |
| 101 | Stale request was not forwarded toMCX |  |  |  |  |  |  |
| 102 | Service temporarily not available |  |  |  |  |  |  |
| 103 | Service not available |  |  |  |  |  |  |
| 104 | Result Of Transaction Unknown |  |  |  |  |  |  |
| 105 | Error converting response or broadcast |  |  |  |  |  |  |
| 152 | Heartbeat violation error |  |  |  |  |  |  |
| 200 | Internal technical error |  |  |  |  |  |  |
| 210 | Validation Error |  |  |  |  |  |  |
| 211 | User already logged in |  |  |  |  |  |  |
| 10000 | Order not found |  |  |  |  |  |  |
| 10001 | Price not reasonable |  |  |  |  |  |  |
| 10004 | BU Book Order Limit Ex-ceeded |  |  |  |  |  |  |
| 10005 | Session Book Order Limit Exceeded |  |  |  |  |  |  |
| 10006 | LstUpdate Timestamp Not Matched |  |  |  |  |  |  |

---

## 5.2.8 Cancel Order Single-10109

This message is used to cancel a single order. This message is sent to the service “Order and Quote 
Management”.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10109 (Order-CancelRequest, MsgType = F) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 37 | OrderID | Y | 8 | 24 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 32 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining.ClOrdID should be mention inNewOrderSingle and same value can bepassed in ClOrdID as well asOrigClOrdID. |
| 41 | OrigClOrdID | N | 8 | 40 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring tothe specific order. |
| 723 | LstUpdtTime | N | 8 | 48 | UTCTimestamp | LastUpdate timestamp for that order |
| 727 | StrategyID | N | 8 | 56 | unsigned int | Strategy Approved by the exchangeshould be used.Range being from 0 to99999 |
| 734 | StrategyTriggerSeqNo | N | 8 | 64 | unsigned int | Strategy Sequence numbers |
| 711 | Echo | N | 4 | 72 | signed int | Vendors can use this as a reference inresponse from exchange |
| 1300 | MarketSegmentID | Y | 4 | 76 | signed int | Product identifier. |
| 30048 | SimpleSecurityID | Y | 4 | 80 | unsigned int | Instrument identifier for simple instru-ments. |
| 20655 T | argetPartyIDSession-ID | N | 4 | 84 | unsigned int | Session ID that entered the order. |
| 712 | RegulatoryID | N | 4 | 88 | unsigned int | For participants to uniquely identifytrading startegies |
| 39040 | Pad4 | N | 4 | 92 | Fixed String | Not Used. |

---

## 5.2.9 Cancel Order Response (Standard Order)-10110

This message confirms the cancellation of a Standard Order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <Me | ssageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:   10110(ExecutionReport, MsgType = 8) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp | not used |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | Not used. |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Not used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not used. |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 5948 | PartitionID | Y | 2 | 60 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability andRetransmit requests. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Value Description |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | 4Session Data |
| 28704 | ApplMsgID | N | 16 | 63 | data | Not set if the submitting session is notthe owner of the cancelled order. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Last Message |
| <M | essage Body> |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 80 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 88 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining. |
| 41 | OrigClOrdID | N | 8 | 96 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring tothe specific order. |
| 48 | SecurityID | Y | 8 | 104 | signed int | Instrument identifier. |
| 17 | ExecID | Y | 8 | 112 | UTCTimestamp | Transaction timestamp. |
| 14 | CumQty | Y | 8 | 120 | Qty | Cumulated executed quantity of an or-der. |
| 84 | CxlQty | Y | 8 | 128 | Qty | Total quantity cancelled for this order. |
| 39 | OrdStatus | Y | 1 | 136 | char | Conveys the current status of an or-der. |
| 39 | OrdStatus | Y | 1 | 136 | char | Description |
| 39 | OrdStatus | Y | 1 | 136 | char | New |
| 39 | OrdStatus | Y | 1 | 136 | char | 1 Partially filled |
| 39 | OrdStatus | Y | 1 | 136 | char | Filled |
| 39 | OrdStatus | Y | 1 | 136 | char | 4 Cancelled |
| 39 | OrdStatus | Y | 1 | 136 | char | 7 RRM Suspended |
| 39 | OrdStatus | Y | 1 | 136 | char | 8 SquareOff Suspended |
| 39 | OrdStatus | Y | 1 | 136 | char |  |
| 150 | ExecType | Y | 1 | 137 | char | The reason why this message was gen-erated. |
| 150 | ExecType | Y | 1 | 137 | char | Description |
| 150 | ExecType | Y | 1 | 137 | char | 4 Cancelled |
| 378 | ExecRestatement-Reason | Y | 2 | 138 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message. |
| 378 | ExecRestatement-Reason | Y | 2 | 138 | unsigned int | Description |
| 378 | ExecRestatement-Reason | Y | 2 | 138 | unsigned int | Order delete accepted |
| 378 | ExecRestatement-Reason | Y | 2 | 138 | unsigned int | Pending order deletion |
| 1227 | ProductComplex | Y | 1 | 140 | unsigned int | This field qualifies an instrument typeon MCX. |
| 1227 | ProductComplex | Y | 1 | 140 | unsigned int | Description |
| 1227 | ProductComplex | Y | 1 | 140 | unsigned int | 1 Simple instrument |
| 1227 | ProductComplex | Y | 1 | 140 | unsigned int | 5 Futures Spread |
| 39030 | Pad3 | N | 3 | 141 | Fixed String | Not Used. |

---

## 5.2.10 Immediate Execution Response-10103

This message informs about the immediate execution of an incoming order or the execution of a book 
order due to a replace request.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <Me | ssageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:   10103(ExecutionReport, MsgType = 8) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <Re | sponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp | not used |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | Not Used |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Not Used |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not Used |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 5948 | PartitionID | Y | 2 | 60 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability andRetransmit requests. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Value Description |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | 4Session Data |
| 28704 | ApplMsgID | N | 16 | 63 | data | Not set if the submitting session is notthe owner of the executed order. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Not Last Message |
| 893 | LastFragment | Y | 1 | 79 | unsigned int |  |
| <Me | ssage Body> |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 80 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 88 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining. |
| 41 | OrigClOrdID | N | 8 | 96 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring tothe specific order. |
| 48 | SecurityID | Y | 8 | 104 | signed int | Instrument identifier. |
| 17 | ExecID | Y | 8 | 112 | UTCTimestamp | Transaction timestamp. |
| 21009 | TrdRegTSEntryTime | N | 8 | 120 | UTCTimestamp | The entry timestamp is the time ofthe creation of the order. |
| 742 | Reserve4 | N | 8 | 128 | UTCTimestamp |  |
| 723 | LstUpdtTime | Y | 8 | 136 | UTCTimestamp | Last Updated timestamp for that or-der |
| 732 | DisclosedQty | N | 8 | 144 | Qty | The quantity to be made visible in themarket data.DisclosedQty is set to 0in case full qty needs to be disclosed. |
| 736 | Reserve1 | N | 8 | 152 | unsigned int | Not Used |
| 710 F | iller2 | N | 4 | 160 | unsigned int | Not Used. |
| 1300 | MarketSegmentID | Y | 4 | 164 | signed int | Product identifier. |
| 151 | LeavesQty | Y | 8 | 168 | Qty | Remaining quantity of an order.If theorder has been executed partially thisfield contains the non-executed quan-tity.  A remaining size of 0 indicatesthat the order is fully matched or nolonger active. |
| 14 | CumQty | Y | 8 | 176 | Qty | Cumulated executed quantity of an or-der. |
| 84 | CxlQty | Y | 8 | 184 | Qty | Total quantity cancelled for this order. |
| 727 | StrategyID | Y | 8 | 192 | unsigned int | Strategy Approved by the exchangeshould be used.Range being from 0 to99999 |
| 728 | StrategySequence-Number | Y | 8 | 200 | unsigned int | Strategy Sequence numbers |
| 713 F | iller4 | N | 2 | 208 | unsigned int | Not Used. |
| 30555 | NoLegExecs | Y | 2 | 210 | Counter | Number of InstrmntLegExec repeat-ing group instances. |
| 378 | ExecRestatement-Reason | Y | 2 | 212 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.Valid values are listed after this table. |
|  | 1227 ProductComplex | Y | 1 | 214 | unsigned int | This field qualifies an instrument type on MCX. Value Description 1 Simple instrument 5 Futures Spread |
|  | 1227 ProductComplex | Y | 1 | 214 | unsigned int | This field qualifies an instrument type on MCX. Value Description 1 Simple instrument 5 Futures Spread |
|  | 1227 ProductComplex | Y | 1 | 214 | unsigned int | This field qualifies an instrument type on MCX. Value Description 1 Simple instrument 5 Futures Spread |
|  | 1227 ProductComplex | Y | 1 | 214 | unsigned int | This field qualifies an instrument type on MCX. Value Description 1 Simple instrument 5 Futures Spread |
|  | 1227 ProductComplex | Y | 1 | 214 | unsigned int | This field qualifies an instrument type on MCX. Value Description 1 Simple instrument 5 Futures Spread |
|  | 1227 ProductComplex | Y | 1 | 215 | unsigned int | Side of the order. |
|  | 54 Side | Y |  |  | unsigned int |  |
|  |  |  |  |  |  | Value Description |
|  |  |  |  |  |  | 2 Sell |
|  | 39 OrdStatus | Y | 1 | 216 | char | 2 Sell |
|  |  | Y |  |  | char | der. |
|  |  | Y |  |  | char | Value Description |
|  |  | Y |  |  | char | 1 Partially filled |
|  |  | Y |  |  | char | 4 Cancelled |
|  |  | Y | 1 |  | char |  |
|  |  | Y | 1 |  |  |  |
|  | 150 ExecType | Y | 1 | 217 | char |  |
|  | 150 ExecType | Y | 1 | 217 |  | The reason why this message was gen- erated. Value Description F Trade |
|  | 150 ExecType | Y | 1 | 217 |  | The reason why this message was gen- erated. Value Description F Trade |
|  | 150 ExecType | Y | 1 | 217 |  | The reason why this message was gen- erated. Value Description F Trade |
|  | 150 ExecType | Y | 1 | 217 |  | The reason why this message was gen- erated. Value Description F Trade |
|  | 150 ExecType | Y | 1 | 217 |  | The reason why this message was gen- erated. Value Description F Trade |
|  | 150 ExecType | Y | 1 | 217 |  | The reason why this message was gen- erated. Value Description F Trade |
|  | 1823 Triggered | Y | 1 | 218 | unsigned int | Indicates if an order has been previ- |
|  |  | Y | 1 |  | unsigned int | ously triggered. Value Description 0 Not triggered 1 Triggered Stop |
|  |  | Y | 1 |  | unsigned int | ously triggered. Value Description 0 Not triggered 1 Triggered Stop |
|  |  | Y | 1 |  | unsigned int | ously triggered. Value Description 0 Not triggered 1 Triggered Stop |
|  |  | Y | 1 |  | unsigned int | ously triggered. Value Description 0 Not triggered 1 Triggered Stop |
|  |  | Y | 1 |  | unsigned int | ously triggered. Value Description 0 Not triggered 1 Triggered Stop |
|  | 716 Filler5 | N | 1 | 219 | unsigned int | Not Used. |
|  | 1362 NoFills | Y | 1 | 220 | Counter | Specifies the number of partial fills in- cluded in this Execution Report. |
| 39030 Pad3 |  | N | 3 | 221 | Fixed String | Not Used. |
| &lt;FillsGrp&gt; |  |  |  |  | Fixed String | Cardinality:  0-100, Record counter: NoFills |
|  | 1364 &gt;FillPx | Y | 8 | 224 | PriceType | Price of Fill. |
|  | 1365 &gt;FillQty | Y | 8 | 232 240 | Qty | Quantity of Fill. |
| 28708 &gt;FillMatchID |  | Y | 4 | 232 240 | unsigned int | Unique identifier for each price level (match step) of a match event; it is used for public trade reporting. |

---

| Tag | Field Name | Req&#x27;d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 1363 | &gt;FillExecID | Y | 4 | 244 | signed int | Private identifier of an order match step event, which can be reconciled with the field SideTradeID (1506) in the Trade Notification. |
| 1443 | &gt;FillLiquidityInd | N | 1 | 248 | unsigned int | Not Used |
| 39070 | &gt;Pad7 | N | 7 | 249 | Fixed String | Not Used. |
| &lt;InstrmntLegExecGrp&gt; |  |  |  |  |  | Cardinality: 0-600, Record counter: NoLegExecs |
| 602 | &gt;LegSecurityID | Y | 8 | 0 | signed int | Instrument identifier of the leg secu- rity. |
| 637 | &gt;LegLastPx | Y | 8 | 8 | PriceType | Price of this leg fill. |
| 1418 | &gt;LegLastQty | Y | 8 | 16 | Qty | Quantity executed in this leg fill. |
| 1893 | &gt;LegExecID | Y | 4 | 24 | signed int | Private identifier of a leg match step,which can be reconciled with the field SideTradeID(1506)在TradeNotification. |
| 624 | &gt;LegSide | Y | 1 | 28 | unsigned int | The side of the individual leg of a strategy as defined in its signature. |
| 624 | &gt;LegSide | Y | 1 | 28 | unsigned int | Value Description |
| 624 | &gt;LegSide | Y | 1 | 28 | unsigned int | 1 Buy |
| 624 | &gt;LegSide | Y | 1 | 28 | unsigned int | 2 Sell |
| 2421 | &gt;FillRefID | Y | 1 | 29 | unsigned int | Reference to the corresponding Fills- Grp repeating group instance. |
| 39020 | &gt;Pad2 | N | 2 | 30 | Fixed String | Not Used. |

Valid Values of ExecRestatementReason (datatype ExecRestatementReason)

| Valid |  |  |  |  |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Value |  |  |  |  |  |  |  |  |
| 1 | Order book restatement |  |  |  |  |  |  |  |
| 101 | Order add accepted |  |  |  |  |  |  |  |
| 102 | Order modify accepted |  |  |  |  |  |  |  |
| 103 | Order delete accepted |  |  |  |  |  |  |  |
| 105 | IOC Order Cancelled |  |  |  |  |  |  |  |
| 108 | Book Order executed |  |  |  |  |  |  |  |
| 114 | Order has been changed to IOC |  |  |  |  |  |  |  |
| 135 | Market Order triggered and executed |  |  |  |  |  |  |  |
| 145 | Start Of Day Processing |  |  |  |  |  |  |  |
| 146 | End Of Day Processing |  |  |  |  |  |  |  |
| 155 | Order Refreshed |  |  |  |  |  |  |  |

---

| 172 | Stop Order has been triggered |
| --- | --- |
| 215 | Risk Reduction Timer Expired |
| 217 | Tick Size Change |
| 248 | Active order deletion due to SMPF |
| 250 | Active order modification due to SMPF |
| 252 | Passive order deletion due to SMPF |
| 254 | Passive oder modification due to SMPF |
| 261 | Panic Cancel |
| 302 | RRMIN |
| 303 | SQUAREOFFIN |
| 357 | Base Price Update |
| 358 | Order Deleted As PriceMoved Out Of DPL Range |

---

## 5.2.11 Extended Order Information-10117

This message format is used for order book restatement, retransmission of order events, the Listener 
Broadcast and for unsolicited order events within the session data.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:  10117(ExecutionReport, MsgType = 8) |  |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |  |
|  |  |  |  |  |  |  |  |  |
| 21003 | Reserve2 | N | 8 | 8 | UTCTimestamp | Not Used. |  |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |  |  |
| 28727 | ApplSubID | N | 4 | 24 | unsigned int | Only set for Listener Data. |  |  |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |  |  |
| 28704 | ApplMsgID | N | 16 | 30 | data | Application  message  identifier  as-signed to an order or quote event. |  |  |
|  |  |  |  |  |  | Value | Description |  |
|  |  |  |  |  |  | 4 | Session Data |  |
|  |  |  |  |  |  | 5 | Listener Data |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | Indicates a retransmission message. |  |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | Value | Description |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | 0 | False |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | 1 | True |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |  |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | Value | Description |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | 0 | Not Last Message |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | 1 | Last Message |  |
| 39070 | Pad7 | U | 7 | 49 | Fixed String | not used |  |  |
|  |  |  |  |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 56 | unsigned int | Exchange Order ID generated by theMCXSystem;  itremains constantover the lifetime of an order. |  |  |

---

|  | Tag Field Name 11 ClOrdID | Req’d | Len | Ofs | Data Type | Description | Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 11 ClOrdID | N | 8 | 64 | unsigned int |  | Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |  |
|  | 41 OrigClOrdID | N | 8 | 72 | unsigned int | ClOrdID (11) of the last successfully processed task (request) referring to the specific order. |  |  |
|  | 48 SecurityID | Y | 8 | 80 | signed int |  | Instrument identifier. |  |
|  | 731 MaxPricePercentage | N | 8 | 88 | PriceType | Price per unit of quantity (e.g.  per share) |  |  |
|  | 708 TerminalInfo | Y | 8 | 104 UTCTimestamp |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description |  |  |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description |  |  |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  |  | 11111111111 IBT |  |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  |  | 22222222222 DMA |  |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  |  |  | 33333333333 Wireless Technology Order not generated 0 through Program |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  |  |  | trading software Order generated |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  |  |  | 1 through Program trading software |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  | 2 ing without Program trading software Smart Order Routing 3 with program trading |  |  |
|  | 708 TerminalInfo |  |  | 104 UTCTimestamp |  |  |  | software |
|  | 17 ExecID | Y | 8 |  |  |  | Transaction timestamp. |  |
|  | 21009 TrdRegTSEntryTime | Y | 8 | 112 | UTCTimestamp | The entry timestamp is the time of the creation of the order. |  |  |
|  | 722 Reserve14 | Y | 8 | 120 | UTCTimestamp | Not used. |  |  |
|  | 44 Price | N | 8 | 128 | PriceType |  | is Limit (2) or Stop Limit (4). | Limit price. Required if OrdType (40) |
|  | 99 StopPx | N | 8 | 136 | PriceType | for an One- |  | Stop price. Required if OrdType (40) is Stop (3) or used as the trigger price cancels-the-other order. |
|  | 721 Reserved2 | N | 8 | 144 | unsigned int | Not used. |  |  |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 720 | Reserved1 | N | 8 | 152 | unsigned int | Not used. |
| 740 | LstUpdTime | Y | 8 | 160 | UTCTimestamp | Last Update timestamp for that order |
| 709 F | iller1 | N | 8 | 168 | unsigned int | Not used. |
| 151 | LeavesQty | Y | 8 | 176 | Qty | Remaining quantity of an order. |
| 732 | DisclosedQty | N | 8 | 184 | Qty | The quantity to be made visible in themarket data.DisclosedQty is set to 0in case full qty needs to be disclosed. |
| 14 | CumQty | Y | 8 | 192 | Qty | Cumulated executed quantity of an or-der. |
| 84 | CxlQty | Y | 8 | 200 | Qty | Total quantity cancelled for this order. |
| 38 | OrderQty | Y | 8 | 208 | Qty | Total Order Quantity. |
| 727 | StrategyID | Y | 8 | 216 | unsigned int | Strategy Approved by the exchangeshould be used.Range being from 0 to99999 |
| 728 | StrategySequence-Number | Y | 8 | 224 | unsigned int | Strategy Sequence numbers |
| 30738 | SpreadPrice | N | 8 | 232 | PriceType | Price per unit of quantity (e.g.  pershare) |
| 1300 | MarketSegmentID | Y | 4 | 240 | signed int | Product identifier. |
| 711 | Echo | N | 4 | 244 | signed int | Vendors can use this as a reference inresponse from exchange |
| 432 | ExpireDate | N | 4 | 248 | LocalMktDate | Date of order expiry.   Required ifTimeInForce (59) = 6. |
| 20059 | PartyIDExecutingUnit | Y | 4 | 252 | unsigned int | Business Unit ID. |
| 20055 | PartyIDSessionID | Y | 4 | 256 | unsigned int | Session ID. |
| 20012 P | artyIDExecuting-Trader | Y | 4 | 260 | unsigned int | Owning User ID. |
| 20036 P | artyIDEntering-Trader | N | 4 | 264 | unsigned int | Entering User ID. |
| 30555 | NoLegExecs | Y | 2 | 268 | Counter | Number of InstrmntLegExec repeat-ing group instances. |
| 378 | ExecRestatement-Reason | Y | 2 | 270 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.Valid values are listed after this table. |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type unsigned int | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | 714 AccountType | Y | 1 272 |  | Data Type unsigned int | Description |
|  | 714 AccountType | Y | 1 272 |  | Data Type unsigned int | Value Description |
|  | 714 AccountType | Y | 1 272 |  |  | 1 Own |
|  | 714 AccountType | Y | 1 272 |  |  | 3 Client |
|  | 714 AccountType | Y | 1 272 |  |  | 5 Institution |
|  |  | Y | 1 272 |  | unsigned int | Entering Entity ID. |
|  | 20007 PartyIDEnteringFirm | Y | 1 | 273 | unsigned int | Entering Entity ID. |
|  |  | Y |  | 273 | unsigned int | Value Description |
|  |  | Y |  | 273 | unsigned int | 1 Participant |
|  |  | Y |  |  | unsigned int | 2 Market Supervision |
|  | 1227 ProductComplex |  | 1 274 |  | unsigned int | This field qualifies an instrument type on MCX. Value Description |
|  | 1227 ProductComplex |  | 1 274 |  | unsigned int | This field qualifies an instrument type on MCX. Value Description |
|  | 1227 ProductComplex |  | 1 274 |  | unsigned int | 1 Simple instrument |
|  | 1227 ProductComplex |  | 1 274 |  | unsigned int | 5 Futures Spread |
|  | 39 OrdStatus | Y | 1 | 275 | char | Conveys the current status of an or- der. |
|  | 39 OrdStatus |  | 1 276 |  |  | Conveys the current status of an or- der. |
|  | 39 OrdStatus |  | 1 276 |  |  | Value Description |
|  | 39 OrdStatus |  | 1 276 |  |  | 0 New |
|  | 39 OrdStatus |  | 1 276 |  |  | 1 Partially filled |
|  | 39 OrdStatus |  | 1 276 |  |  | 2 Filled 4 Cancelled 7 RRM Suspended |
|  | 39 OrdStatus |  | 1 276 |  |  | 2 Filled 4 Cancelled 7 RRM Suspended |
|  | 39 OrdStatus |  | 1 276 |  |  | 2 Filled 4 Cancelled 7 RRM Suspended |
|  | 39 OrdStatus |  | 1 276 |  |  | 8 SquareOff Suspended |
|  | 150 ExecType | Y | 1 276 |  | char | The reason why this message was gen- |
|  | 150 ExecType | Y | 1 |  |  | erated. |
|  | 150 ExecType | Y | 1 |  |  | Value Description |
|  | 150 ExecType | Y | 1 |  |  | 0 New |
|  | 150 ExecType | Y | 1 |  |  | 4 Cancelled |
|  | 150 ExecType | Y | 1 |  |  | 5 Replaced |
|  | 150 ExecType | Y | 1 |  |  | D Restated |
|  | 150 ExecType | Y | 1 |  |  | L Triggered |
|  | 150 ExecType |  | 1 |  |  | F Trade |
|  | 54 Side | Y | 1 | 277 | unsigned int | Side of the order. |
|  | 54 Side |  | 1 | 277 | unsigned int | Value Description |
|  | 54 Side |  |  | 277 | unsigned int | 1 Buy |
|  | 54 Side |  |  | 277 | unsigned int | 2 Sell |

---

|  | Tag Field Name 40 OrdType | Req’d | Len | Ofs | Data Type unsigned int | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 40 OrdType | 1 |  | 278 | Data Type unsigned int | Order type. Value Description 2 Limit 3 Stop Market 4 Stop Limit 5 Market To Limit 6 Auction Buy IN |  |
|  | Tag Field Name 40 OrdType | 1 |  |  | Data Type unsigned int | Order type. Value Description 2 Limit 3 Stop Market 4 Stop Limit 5 Market To Limit 6 Auction Buy IN |  |
|  | Tag Field Name 40 OrdType | 1 |  |  | Data Type unsigned int | Order type. Value Description 2 Limit 3 Stop Market 4 Stop Limit 5 Market To Limit 6 Auction Buy IN |  |
|  | Tag Field Name 40 OrdType | 1 |  |  | Data Type unsigned int | Order type. Value Description 2 Limit 3 Stop Market 4 Stop Limit 5 Market To Limit 6 Auction Buy IN |  |
|  | Tag Field Name 40 OrdType | 1 |  | 279 280 | Data Type unsigned int | Order type. Value Description 2 Limit 3 Stop Market 4 Stop Limit 5 Market To Limit 6 Auction Buy IN |  |
|  | Tag Field Name 40 OrdType | 1 |  | 279 280 | Data Type unsigned int | Order type. Value Description 2 Limit 3 Stop Market 4 Stop Limit 5 Market To Limit 6 Auction Buy IN |  |
|  | Tag Field Name 40 OrdType | 1 |  | 279 280 | Data Type unsigned int | Order type. Value Description 2 Limit 3 Stop Market 4 Stop Limit 5 Market To Limit 6 Auction Buy IN |  |
|  | Tag Field Name 40 OrdType | 1 |  | 279 280 | Data Type unsigned int |  | 7 Auction_Sell_Out |
|  | 1815 TradingCapacity | Y | 1 | 279 280 | unsigned int | Not used. |  |
|  | 59 TimeInForce | N | 1 | 279 280 | unsigned int | Execution and trading restriction pa- rameters supported by MCX. Value Description |  |
|  |  | N |  | 279 280 |  | Execution and trading restriction pa- rameters supported by MCX. Value Description |  |
|  |  | N |  | 279 280 |  |  | 0 Day (DAY) |
|  |  | N |  | 279 280 |  | 1 Good Till Cancelled (GTC) - Standard Orders only 3 Immediate or Cancel (IOC) 6 Good  Till  Date  (GTD)  - Standard Orders only |  |
|  |  | N |  |  |  | 1 Good Till Cancelled (GTC) - Standard Orders only 3 Immediate or Cancel (IOC) 6 Good  Till  Date  (GTD)  - Standard Orders only |  |
|  |  | N |  |  |  | 1 Good Till Cancelled (GTC) - Standard Orders only 3 Immediate or Cancel (IOC) 6 Good  Till  Date  (GTD)  - Standard Orders only |  |
|  |  | N |  |  |  | 1 Good Till Cancelled (GTC) - Standard Orders only 3 Immediate or Cancel (IOC) 6 Good  Till  Date  (GTD)  - Standard Orders only |  |
|  | 18 ExecInst | N |  |  |  | 1 Good Till Cancelled (GTC) - Standard Orders only 3 Immediate or Cancel (IOC) 6 Good  Till  Date  (GTD)  - Standard Orders only |  |
|  | 18 ExecInst |  |  |  |  |  | 7 End Of Session (EOS) |
|  | 18 ExecInst | N | 1 281 |  | unsigned int |  | Instructions for order handling on ex- |
|  | 18 ExecInst | N | 1 281 |  | unsigned int |  | change trading floor. If more than one |
|  | 18 ExecInst | N | 1 281 |  | unsigned int |  | instruction is applicable to an order, |
|  | 18 ExecInst | N | 1 281 |  | unsigned int |  | this field can contain multiple |
|  | 18 ExecInst | N | 1 281 |  | unsigned int | 105nstruct- tions separated by space. Persistent Order(FIX value- 1 H) |  |
|  | 18 ExecInst | N | 1 281 |  | unsigned int | 105nstruct- tions separated by space. Persistent Order(FIX value- 1 H) |  |
|  | 18 ExecInst | N | 1 281 |  | unsigned int | 105nstruct- tions separated by space. Persistent Order(FIX value- 1 H) |  |
|  | 18 ExecInst | N | 1 281 |  | unsigned int | 105nstruct- tions separated by space. Persistent Order(FIX value- 1 H) |  |
|  | 18 ExecInst | N | 1 281 |  | unsigned int |  | 2 value-Q) |
|  | 18 ExecInst | N | 1 281 |  | unsigned int |  |  |
|  | 18 ExecInst | N | 1 281 |  | unsigned int | Used. |  |
|  | 18 ExecInst |  |  |  |  | Value | Description |
|  | 18 ExecInst |  |  |  |  | 1 | Start of Day |
|  |  |  |  |  |  | 2 | Pre-Trading |
|  |  |  |  |  |  | 3 | Trading |
|  |  |  |  |  |  | 4 | Closing or closing auction |
|  |  |  |  |  |  | 5 | Post-Trading |
|  |  |  |  |  |  | 6 | End of Day |
|  |  |  |  |  |  | 7 | Post End of Day |
| 625 | TradingSessionSubID | N | 282 |  | unsigned int | 8 | Halt |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 28703 | ApplSeqIndicator | N | 1 | 283 | unsigned int |  |  |
| 28703 | ApplSeqIndicator | N | 1 | 283 | unsigned int | Value | Description |
| 28703 | ApplSeqIndicator | N | 1 | 283 | unsigned int | 0 | No recoveryrequired |
| 28703 | ApplSeqIndicator | N | 1 | 283 | unsigned int | 1 | Standard Order |
| 741 | STPCFlag | Y | 1 | 284 | unsigned int |  |  |
| 741 | STPCFlag | Y | 1 | 284 | unsigned int | Value | Description |
| 741 | STPCFlag | Y | 1 | 284 | unsigned int | 0 | Passive |
| 741 | STPCFlag | Y | 1 | 284 | unsigned int | 1 | Active |
| 770 | MultiLegType | N | 1 | 285 | unsigned int |  |  |
| 1 | Account | N | 2 | 286 | Fixed String(0-terminable) | Not used.Must be sent as A1Valid characters: 1-9, \x41,  \x47,\x49, \x4D, \x50, \x52 |  |
| 77 | PositionEffect | N | 1 | 288 | char | must be set to C |  |
| 20096 | PartyIDTakeUp-TradingFirm | N | 5 | 289 | Fixed String | Not Used. |  |
| 20013 | PartyIDOrder-OriginationFirm | N | 7 | 294 | Fixed String | Not used. |  |
| 20032 | PartyIDBeneficiary | N | 9 | 301 | Fixed String | Not Used. |  |
| 20075 | PartyIDLocationID | N | 2 | 310 | Fixed String(0-terminable) | Not Used. |  |
| 1031 | CustOrderHandling-Inst | N | 1 | 312 | Fixed String | Not used.Valid characters: \x20, \x22-\x7B,\x7D, \x7E |  |
| 718 | UserReferenceText | N | 20 | 313 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide addi-tional regulatory information (accord-ing to respective rules &amp; regs, circu-lars and/or bilateral coordination be-tween participant and Trading Surveil-lance Office).Valid characters: \x20, \x22-\x7B,\x7D, \x7E |  |
| 25007 | FreeText1 | N | 12 | 333 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60,\x7B, \x7D |  |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 717 | CPCode | N | 12 | 345 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60,\x7B, \x7D. |
| 25009 | FreeText3 | N | 12 | 357 | Fixed String | Free-format text field fortraderspecific  or  customer-relatedcomments.Validcharacters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 1362 | NoFills | Y | 1 | 369 | Counter | Specifies the number of partial fills in-cluded in this Execution Report. |
| 555 | NoLegs | Y | 1 | 370 | Counter | Number of LegOrd repeating groupinstances. |
| 1823 | Triggered | Y | 1 | 371 | unsigned int | Indicates if an order has been previ-ously triggered. |
| 1823 | Triggered | Y | 1 | 371 | unsigned int | Description |
| 1823 | Triggered | Y | 1 | 371 | unsigned int | Not triggered |
| 1823 | Triggered | Y | 1 | 371 | unsigned int | Triggered Stop |
| 1823 | Triggered | Y | 1 | 371 | unsigned int | 2Triggered OCO |
| 39020 | Pad2 | N | 2 | 372 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality: 0-20, Record counter:NoLegs |
| 2680 | >LegAccount | Y | 2 | 374 | Fixed String(0-terminable) | Leg-specific account to book tradesand keep positions on.Valid characters: 1-9, \x41, \x47,\x4D, \x50 |
| 564 > | LegPositionEffect | Y | 1 | 376 | char | not used., Must be set as C.Valid characters: \x01-\x7E |
| 39050 | >Pad5 | N | 5 | 377 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality: 0-100, Record counter:NoFills |
| 1364 > | FillPx | Y | 8 | 0 | PriceType | Price of Fill. |
| 1365 > | FillQty | Y | 8 | 8 | Qty | Quantity of Fill. |
| 28708 > | FillMatchID | Y | 4 | 16 | unsigned int | Unique identifier for each price level(match step) of a match event; it isused for public tradereporting. |
| 1363 > | FillExecID | Y | 4 | 20 | signed int | Private identifier of an order matchstep event, which can be reconciledwith the field SideTradeID (1506) inthe Trade Notification. |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | Not used. |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | Description |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | Added Liquidity |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | 2 Removed Liquidity |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | Auction |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | 5 Triggered Stop Order |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | 6 Triggered OCO Order |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int | 7 Triggered Market Order |
| 1443 > | FillLiquidityInd | N | 1 | 24 | unsigned int |  |
| 39070 | >Pad7 | N | 7 | 25 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality:  0-600, Record counter:NoLegExecs |
| 602 | >LegSecurityID | Y | 8 | 0 | signed int | Instrument identifier of the leg secu-rity. |
| 637 | >LegLastPx | Y | 8 | 8 | PriceType | Price of this leg fill. |
| 1418 | >LegLastQty | Y | 8 | 16 | Qty | Quantity executed in this leg fill. |
| 1893 | >LegExecID | Y | 4 | 24 | signed int | Private identifier of a leg match step,which can be reconciled with the fieldSideTradeID (1506) in the Trade No-tification. |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | The side of the individual leg of astrategy as defined in its signature. |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Description |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Buy |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Sell |
| 624 | >LegSide | Y | 1 | 28 | unsigned int |  |
| 2421 > | FillRefID | Y | 1 | 29 | unsigned int | Reference to the corresponding Fills-Grp repeating group instance. |
| 39020 | >Pad2 | N | 2 | 30 | Fixed String | Not Used. |

## Valid Values of ExecRestatementReason (datatype ExecRestatementReason)

| Valid |  |  |  |  |  | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Value |  |  |  |  |  | Description |  |
| 1 | Order book restatement |  |  |  |  |  |  |
| 101 | Order add accepted |  |  |  |  |  |  |
| 102 | Order modify accepted |  |  |  |  |  |  |
| 103 | Order delete accepted |  |  |  |  |  |  |
| 105 | IOC Order Cancelled |  |  |  |  |  |  |
| 108 | Book Order executed |  |  |  |  |  |  |

---

| 114 | Order has been changed to IOC |  |
| --- | --- | --- |
| 135 | Market Order triggered and executed |  |
| 145 | Start Of Day Processing |  |
| 146 | End Of Day Processing |  |
| 155 | Order Refreshed |  |
| 172 | Stop Order has been triggered |  |
| 215 | Risk Reduction Timer Expired |  |
| 217 | Tick Size Change |  |
| 248 | Active order deletion due to SMPF |  |
| 250 | Active order modification due to SMPF |  |
| 252 | Passive order deletion due to SMPF |  |
| 254 | Passive oder modification due to SMPF |  |
| 261 | Panic Cancel |  |
| 302 | RRMIN |  |
| 303 | SQUAREOFFIN |  |
| 357 | Base Price Update |  |
| 358 | Order Deleted As PriceMoved Out Of DPL Range |  |

---

## 5.2.12 Book Order Execution-10104

This message informs about the execution of a resting book order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value: 10104(ExecutionReport, MsgType = 8) |  |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |  |
| <R | BCHeaderME> |  |  |  |  |  |  |  |
| 21003 | Reserve2 | N | 8 | 8 | UTCTimestamp | Not Used. |  |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |  |  |
| 28727 | ApplSubID | U | 4 | 24 | unsigned int | not used |  |  |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |  |  |
| 28704 | ApplMsgID | Y | 16 | 30 | data | Application  message  identifier  as-signed to an order or quote event. |  |  |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Identifier for an ETI data stream. |  |  |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Value | Description |  |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | 4 | Session Data |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | Indicates a retransmission message. |  |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | Value | Description |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | 0 | False |  |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | 1 | True |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |  |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | Value | Description |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | 0 | Not Last Message |  |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | 1 | Last Message |  |
| 39070 | Pad7 | U | 7 | 49 | Fixed String | not used |  |  |
|  |  |  |  |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 56 | unsigned int | Exchange Order ID generated by theMCXSystem;  itremains constantover the lifetime of an order. |  |  |

---

|  | Tag Field Name 708 TerminalInfo | Req’d Y | Len | Ofs 64 | Data Type unsigned int |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  | Total 15 Characters st For 1 - th For 13 th th 14 – 15 for member Value 11111111111 22222222222 DMA | 12 value(11111111111-33333333333), character value(0-3) vendor code / in House CTCL code IBT 33333333333 Wireless Technology Order not 0 through Order 1 through Smart software Client Order ID: Unique participant defined order request identifier; used | characters generated Program trading software generated Program trading software Order  Rout- 2 ing without Program trading software Smart Order Routing 3 with program trading |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  | digit should be valid |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  | digit should be valid |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  | Description |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 | Data Type unsigned int |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 |  |  |  |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 64 |  |  |  |  |  |
|  | 11 ClOrdID | N | 8 | 72 | unsigned int |  |  | for client order id chaining. |  |
|  | 41 OrigClOrdID | N | 8 | 80 | unsigned int |  | the specific order. | ClOrdID (11) of the last successfully processed task (request) referring to |  |
|  | 48 SecurityID | Y | 8 | 88 | signed int |  | Instrument identifier. |  |  |
|  | 17 ExecID | Y | 8 | 96 | UTCTimestamp |  | Transaction timestamp. |  |  |
|  | 723 LstUpdtTime | Y | 8 | 104 | UTCTimestamp |  |  | Last Update timestamp for that order |  |
|  | 727 StrategyID | Y | 8 | 112 | unsigned int |  | 99999 | Strategy Approved by the exchange should be used.Range being from 0 to |  |
|  | 728 StrategySequence- Number | Y | 8 | 120 | unsigned int |  |  | Strategy Sequence numbers |  |
|  | 736 Reserve1 | N | 8 | 128 | unsigned int |  | Not Used. |  |  |
|  | 737 Reserve2 | N | 4 | 136 | unsigned int |  | Not Used. |  |  |
|  | 711 Echo | Y | 4 | 140 | signed int |  | response from exchange | Vendors can use this as a reference in |  |

---

|  | Tag Field Name 151 LeavesQty | Req’d Y | Len 8 | Ofs 144 152 | Data Type Qty | Description Remaining quantity of an order. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 151 LeavesQty | Req’d Y | Len 8 | Ofs 144 152 | Data Type Qty | Description Remaining quantity of an order. |  |
|  | 14 CumQty | Y | 8 | Ofs 144 152 | Qty | der. | Cumulated executed quantity of an or- |
|  | 84 CxlQty | Y | 8 | 160 | Qty |  | Total quantity cancelled for this order. |
|  | 1300 MarketSegmentID | Y | 4 | 168 | signed int |  | Product identifier. |
|  | 30555 NoLegExecs | Y | 2 | 174 176 | Counter | Number of InstrmntLegExec repeat- ing group instances. |  |
|  | 713 Filler4 | N | 2 | 174 176 | unsigned int | Not Used. |  |
|  | Reason | Y | 2 | 174 176 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. |  |
|  | Reason |  |  | 174 176 |  | Value | Description |
|  | Reason |  |  | 174 176 |  |  | 001 Order Book Restatement |
|  | Reason |  |  | 174 176 |  |  | 101 Order Add Accepted |
|  | Reason |  |  | 174 176 |  |  | 102 Order Add Accepted |
|  | Reason |  |  | 174 176 |  |  | 103 Order Delete Accepted |
|  | Reason |  |  | 174 176 |  |  | 105 IOC Order Cancelled |
|  | Reason |  |  | 174 176 |  |  | 108 Book Order Executed |
|  | Reason |  |  | 174 176 |  |  | 122 Instrument State Change |
|  | Reason |  |  | 174 176 |  |  | 135 Market Order Triggered |
|  | Reason |  |  | 174 176 |  |  | 172 Stop Order Triggered |
|  | Reason |  |  | 174 176 |  |  | 199 Pending Order Cancellation |
|  | Reason |  |  | 174 176 |  |  | Executed (Not used) |
|  | Reason |  |  | 174 176 |  |  | 200 Order Add sent for Risk Val- |
|  | Reason |  |  | 174 176 |  |  | idation |
|  | Reason |  |  | 174 176 |  |  | 201 Self Trade Order Deleted |
|  | Reason |  |  | 174 176 |  |  | 202 OrderCancelled: Session Ex- |
|  | Reason |  |  | 174 176 |  |  | piry |
| 714 AccountType Y 1 178 unsigned int |  |  |  |  |  |  |  |
|  |  |  |  |  |  | Value | Description |
|  |  |  |  |  |  |  | 1 Own |
|  |  |  |  |  |  |  | 3 Client |
|  |  |  |  |  |  | 5 Institution |  |
|  | 1227 ProductComplex | Y | 1 | 179 | unsigned int | on MCX. | This field qualifies an instrument type |
|  | 1227 ProductComplex |  | 1 | 179 | unsigned int | Value | Description |
|  | 1227 ProductComplex |  | 1 | 179 | unsigned int |  | 1 Simple instrument |
|  | 1227 ProductComplex |  | 1 | 179 | unsigned int |  | 5 Futures Spread |

---

|  | Tag Field Name | Req’d | Len | Ofs |  | Data Type | Description Conveys the current status of an or- der. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | 39 OrdStatus | Y | 1 | 180 |  | char | Description Conveys the current status of an or- der. |  |
|  |  |  |  |  |  |  | Value | Description |
|  |  |  |  |  |  |  | 0 | New |
|  |  |  |  |  |  |  | 1 | Partially filled |
|  |  |  |  |  |  |  | 2 | Filled |
|  |  |  |  |  |  |  | 4 | Cancelled |
| 7 RRM Suspended 8 SquareOff Suspended |  |  |  |  |  |  |  |  |
| 7 RRM Suspended 8 SquareOff Suspended |  |  |  |  |  |  |  |  |
| 7 RRM Suspended 8 SquareOff Suspended |  |  |  |  |  |  |  |  |
|  | 150 ExecType | Y | 1 |  |  |  | erated. Value Description |  |
|  | 150 ExecType | Y | 1 |  |  |  | erated. Value Description |  |
|  | 150 ExecType | Y | 1 |  |  |  | F | Trade |
|  | 1823 Triggered | Y | 1 | 182 | unsigned int |  | Indicates if an order has been previ- ously triggered. |  |
|  |  | Y | 1 |  | unsigned int |  | Indicates if an order has been previ- ously triggered. |  |
|  |  | Y | 1 |  | unsigned int |  | Value | Description |
|  |  | Y | 1 |  | unsigned int |  | 0 | Not triggered |
|  |  | Y | 1 |  | unsigned int |  | 1 | Triggered Stop |
|  |  | Y | 1 |  |  |  | 2 | Triggered OCO |
|  | 1362 NoFills | Y | 1 | 183 |  | Counter | Specifies the number of partial fills in- cluded in this Execution Report. |  |
|  |  |  | 1 |  |  | unsigned int | Specifies the number of partial fills in- cluded in this Execution Report. |  |
|  | 54 Side | Y | 1 | 184 |  | unsigned int | Side of the order. Value Description 1 Buy 2 Sell |  |
|  | 54 Side | Y | 1 | 184 |  | unsigned int | Side of the order. Value Description 1 Buy 2 Sell |  |
|  | 54 Side | Y | 1 | 184 |  | unsigned int | Side of the order. Value Description 1 Buy 2 Sell |  |
|  | 54 Side | Y | 1 | 184 |  | unsigned int | Side of the order. Value Description 1 Buy 2 Sell |  |
|  | 716 Filler5 | N Y | 1 | 185 |  | unsigned int | Not Used. |  |
|  | 1 Account | N Y | 2 | 186 | Fixed String (0-terminable) Fixed String |  | Not used. | Must be sent as A1 Valid characters:  1-9,  \x41,  \x47, \x49, \x4D, \x50, \x52 |
|  | 25007 FreeText1 | Y | 12 | 188 |  | (0-terminable) person \x5C, |  | The Unique Client Code (UCC) of the for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5D, \x5F, \x60, \x7B, \x7D |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 717 | CPCode | N | 12 | 200 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |  |
| 25009 | FreeText3 | N | 12 | 212 | Fixed String | Free-format text field fortraderspecific  or  customer-relatedcomments.Valid characters:\x00,  \x21,  \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7DUpto 15 Char allowed.This field is |  |
| 718 | UserReferenceText | N | 20 | 224 | Fixed String | used to provide additionalregulatoryinformation(according to respectiverules andregs, circulars and/orbilateral  coordination  betweenparticipantandTradingSurveillanceOffice).Valid characters:\x20, \x22-\x7B, \x7D, \x7E. |  |
| 39040 | Pad4 | U | 4 | 244 | Fixed String | Not used. |  |
|  |  |  |  |  |  | Cardinality: 0-100, Record counter:NoFills |  |
| 1364 > | FillPx | Y | 8 | 248 | PriceType | Price of Fill. |  |
| 1365 > | FillQty | Y | 8 | 256 | Qty | Quantity of Fill. |  |
| 28708 > | FillMatchID | Y | 4 | 264 | unsigned int | Unique identifier for each price level(match step) of a match event; it isused for public trade reporting. |  |
| 1363 > | FillExecID | Y | 4 | 268 | signed int | Private identifier of an order matchstep event, which can be reconciledwith the field SideTradeID (1506) inthe Trade Notification. |  |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | Not Used. |  |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | Value | Description |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | 1 | Added Liquidity |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | 2 | Removed Liquidity |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | 4 | Auction |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | 5 | Triggered Stop Order |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | 6 | Triggered OCO Order |
| 1443 > | FillLiquidityInd | N | 1 | 272 | unsigned int | 7 | Triggered Market Order |
| 39070 | >Pad7 | U | 7 | 273 | Fixed String | not | used |

---

| Tag Field Name |  | Req’d Len |  | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  | Cardinality: 0-600, Record counter:NoLegExecs |
| 602 | >LegSecurityID | Y | 8 | 0 | signed int | Instrument identifier of the leg secu-rity. |
| 637 | >LegLastPx | Y | 8 | 8 | PriceType | Price of this leg fill. |
| 1418 | >LegLastQty | Y | 8 | 16 | Qty | Quantity executed in this leg fill. |
| 1893 | >LegExecID | Y | 4 | 24 | signed int | Private identifier of a leg match step,which can be reconciled with the fieldSideTradeID (1506) in the Trade No-tification. |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | The side of the individual leg of astrategy as defined in its signature. |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Description |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Buy |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Sell |
| 624 | >LegSide | Y | 1 | 28 | unsigned int |  |
| 2421 | >FillRefID | Y | 1 | 29 | unsigned int | Reference to the corresponding Fills-Grp repeating group instance. |
| 39020 | >Pad2 | N | 2 | 30 | Fixed String | Not used. |

---

## 5.2.13 Order Mass Cancellation Request-10120

This message is used for mass cancellation of orders. This message is sent to the service “Order 
and Quote Management”.

Order Mass Cancellation Request

Order In Book?

Yes, if Standard Orders are involved

Order Mass Cancellation Response (Session Data)

Order Mass Cancellation Notification (Listener Data)

NoNotAffectedOrders?

!=0

Cancel order Notification (Session Data)

NoNotAffectedOrders?

!=0

Cancel order Notification (Listener Data)

Order mass Cancellation response No Hits (Session Data)

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderIn> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10120 (Order-MassActionRequest, MsgType = CA) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 48 | SecurityID | N | 8 | 24 | signed int | Instrument identifier. |
| 1300 | MarketSegmentID | Y | 4 | 32 | signed int | Product identifier. |
| 712 | RegulatoryID | N | 4 | 36 | unsigned int | For participants to uniquely identifytrading startegies |
| 20655 T | argetPartyIDSession-ID | N | 4 | 40 | unsigned int | Session ID. |
| 20612 T | argetParty-IDExecutingTrader | N | 4 | 44 | unsigned int | Owning User ID. |
| 727 | StrategyID | Y | 8 | 48 | unsigned int | Strategy Approved by the exchangeshould be used.Range being from 0 to99999 |
| 728 | StrategySequence-Number | Y | 8 | 56 | unsigned int | Strategy Sequence numbers |
| 37 | OrderID | N | 8 | 64 | unsigned int | Exchange Order ID generated by theMCXSystem;  it remains constantover the lifetime of an order. |
| 711 | Echo | Y | 4 | 72 | signed int | User defined free field.  The field isechoed in the trade notifications, lis-tener data, session data. |
| 54 | Side | N | 1 | 76 | unsigned int | Side of the order. |
| 54 | Side | N | 1 | 76 | unsigned int | ValueDescription |
| 54 | Side | N | 1 | 76 | unsigned int | 1Buy |
| 54 | Side | N | 1 | 76 | unsigned int | 2Sell |

---

|  | Tag Field Name 1 Account | Req’d N | Len 2 | Ofs 77 | Data Type Fixed String (0-terminable) | Description Not used. \x49, \ | Must be sent as A1 Valid characters:  1-9,  \x41,  \x47, x4D, \x50, \x52 |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 1 Account | Req’d N | Len 2 | Ofs 77 | Data Type Fixed String (0-terminable) | Description Not used. \x49, \ | Must be sent as A1 Valid characters:  1-9,  \x41,  \x47, x4D, \x50, \x52 |
|  | Tag Field Name 1 Account | Req’d N | Len 2 | Ofs 77 | Data Type Fixed String (0-terminable) | Description Not used. \x49, \ | Must be sent as A1 Valid characters:  1-9,  \x41,  \x47, x4D, \x50, \x52 |
|  | 25007 FreeText1 | N | 12 | 79 | Fixed String (0-terminable) | UCC code of the orders to be can- celled. In case space is set, then filter will not be applied. |  |
|  | 717 CPCode | N | 12 | 91 | Fixed String (0-terminable) | The Participant code in case INST ac- count orders to be cancelled. Execution and trading restriction pa- |  |
|  | 59 TimeInForce | N N | 1 103 |  | unsigned int | rameters supported by MCX. |  |
|  | 59 TimeInForce | N N | 1 103 |  | unsigned int | Value | Description |
|  | 59 TimeInForce | N N | 1 103 |  | unsigned int | 0 | Day (DAY) |
|  | 59 TimeInForce | N N | 1 103 |  | unsigned int | 1 | Good Till Cancelled (GTC) - Standard Orders only |
|  | 59 TimeInForce | N N | 1 103 |  | unsigned int | 3 | Immediate or Cancel (IOC) |
|  | 59 TimeInForce | N N | 1 103 |  | unsigned int | 6 | Good  Till  Date  (GTD)  - Standard Orders only |
|  | 59 TimeInForce | N N | 1 103 |  | unsigned int |  |  |
|  | 59 TimeInForce | N N |  |  | unsigned int |  |  |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | Order type. |  |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | Value | Description |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | 2 | Limit |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | 3 | Stop Market |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | 4 | Stop Limit |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | 5 | Market To Limit |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | W | Auction Buy IN |
|  | 40 OrdType |  | 1 7 | 104 105 | unsigned int Fixed String | X | Auction_Sell_Out |
| 39070 Pad7 | 40 OrdType | N | 1 7 |  | unsigned int Fixed String | Not Used. |  |

---

## 5.2.14 Order Mass Cancellation Response-10121

This message confirms the Mass Cancellation request for orders.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10121 (Order-MassActionReport, MsgType = BZ) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp | Not used. |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | Not used. |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Not used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not used. |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 5948 | PartitionID | Y | 2 | 60 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | ValueDescription |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | 4 Session Data |
| 28704 | ApplMsgID | Y | 16 | 63 | data | Application  message  identifier  as-signed to an order or quote event. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | 0 Not Last Message |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | 1 Last Message |
|  |  |  |  |  |  |  |
| 1369 | MassActionReportID | Y | 8 | 80 | UTCTimestamp | Last Update timestamp. |
| 1370 | NoNotAffectedOrders | Y | 2 | 88 | Counter | Number of NotAffectedOrders repeat-ing group instances. |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 28782 | NoAffectedOrder-Request | Y | 2 | 90 | Counter | Number of affected order transactions.It will be 0 incase of non-persistentorder even ifall non-persistentorder gotdeleted. |  |
| 39040 | Pad4 | N | 4 | 92 | Fixed String | Not Used. |  |
|  |  |  |  |  |  | Cardinality: 0-500, Record counter:NoNotAffectedOrders |  |
| 1371 > | NotAffectedOrderID | Y | 8 | 96 | unsigned int | Exchange Order ID of an order whosecancellation is pending. |  |
| 1372 > | NotAffOrigClOrdID | N | 8 | 104 | unsigned int | Original Client Order ID of an orderwhose cancellation is pending. |  |
|  |  |  |  |  |  | Cardinality: 0-500, Record counter:NoAffectedOrderRequests |  |
| 28783 > | AffectedOrder-RequestID | Y | 8 | 112 | unsigned int | ETI Exchange Ordernumber affected by thetransaction |  |

---

## 5.2.15 Order Mass Cancellation Response No Hits-10124

This message confirms the Mass Cancellation request for orders if the order book of the session 
was not affected.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <Mes | sageHeaderOut> |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10124 (Order-MassActionReport, MsgType = BZ) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
| <NRR | esponseHeaderME> |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |  |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp | not used |  |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | Not used. |  |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Not used. |  |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not used. |  |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |  |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |  |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |  |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Value | Description |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 1 | Last Message |
| 893 | LastFragment | Y | 1 | 60 | unsigned int |  |  |
| 39030 | Pad3 | U | 3 | 61 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 1369 | MassActionReportID | Y | 8 | 64 | UTCTimestamp | Last Up | date timestamp. |

---

## 5.2.16 Delete All Order Broadcast-10122

This message informs about an unsolicited mass cancellation event of orders.

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| &lt;MessageHeaderOut&gt; |  |  |  |  |  |  |
|  | 9 BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in- cluding this field. |
|  | 28500 TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes- sage layout.  Value:  10122 (Order- MassActionReport, MsgType = BZ) |
| 39020 Pad2 U 2 6 Fixed String not used |  |  |  |  |  |  |
| &lt;RBCHeaderME&gt; |  |  |  |  |  |  |
|  | 21003 Reserve2 | N | 8 | 8 | UTCTimestamp | Not used. |
|  | 52 SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
|  | 28727 ApplSubID | N | 4 | 24 | unsigned int | Only set for Listener Data. |
|  | 5948 PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products. Belongs to the scope of Service Avail- ability and Retransmit requests. |
|  | 28704 ApplMsgID | Y | 16 | 30 | data | Application  message  identifier  as- signed to an order or quote event. |
|  | 1180 ApplID | Y | 1 | 46 | unsigned int | Identifier for an ETI data stream. Value Description 4 Session Data 5 Listener Data |
|  |  | Y | 1 |  |  | Identifier for an ETI data stream. Value Description 4 Session Data 5 Listener Data |
|  |  | Y | 1 |  |  | Identifier for an ETI data stream. Value Description 4 Session Data 5 Listener Data |
|  |  | Y | 1 |  |  | Identifier for an ETI data stream. Value Description 4 Session Data 5 Listener Data |
|  |  | Y | 1 |  |  | Identifier for an ETI data stream. Value Description 4 Session Data 5 Listener Data |
|  |  | Y | 1 |  |  | Identifier for an ETI data stream. Value Description 4 Session Data 5 Listener Data |
|  |  |  |  |  |  |  |
|  | 1352 ApplResendFlag | Y | 1 | 47 | unsigned int | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  |  |  | 1 | 48 |  | Indicates a retransmission message. Value Description 0 False |
|  | 893 LastFragment | Y | 1 |  | unsigned int | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated |
|  |  | U | 1 |  | Fixed String | transaction. Value Description 0 Not Last Message 1 Last Message not used |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
| 39070 Pad7 7 49 |  |  |  |  |  |  |
| &lt;MessageBody&gt; |  |  |  |  |  |  |
| 1369 | MassActionReportID | Y | 8 | 56 | UTCTimestamp | Last Update timestamp. |
|  | 48 SecurityID | N | 8 | 64 | signed int | Only set for mass cancellations on in- strument (SecurityID) level. |

---

|  | Tag Field Name 727 StrategyID | Req’d N | Len 8 | Ofs 72 | Data Type unsigned int | Description Strategy Approved by the exchange should be used.Range being from 0 to 99999 |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 727 StrategyID | Req’d N | Len 8 | Ofs 72 | Data Type unsigned int | Description Strategy Approved by the exchange should be used.Range being from 0 to 99999 |  |
|  | 728 StrategySequence- Number | N | 8 | 80 | unsigned int | Strategy Sequence numbers |  |
|  | 37 OrderID | N | 8 | 88 | unsigned int | Exchange Order ID generated by the MCX System; it remains constant over the lifetime of an order. |  |
|  | 1300 MarketSegmentID | Y | 4 | 96 | signed int | Product identifier. |  |
|  | 20655 TargetPartyIDSession- ID | Y | 4 | 100 | unsigned int | Session ID. |  |
|  | 20612 TargetParty- IDExecutingTrader | N | 4 | 104 | unsigned int | Owning User ID. |  |
|  | 20036 PartyIDEntering- Trader | N | 4 | 108 | unsigned int | Entering User ID. |  |
|  | 712 RegulatoryID | N | 4 | 112 | unsigned int | For participants to uniquely identify trading startegies |  |
|  | 711 Echo | N | 4 | 116 | signed int |  | Vendors can use this as a reference in response from exchange |
|  | 1370 NoNotAffectedOrders | Y | 2 | 120 122 | Counter | Number of NotAffectedOrders repeat- |  |
|  | 20007 PartyIDEnteringFirm | N | 1 | 120 122 | unsigned int |  | Entering Entity ID. |
|  | 20007 PartyIDEnteringFirm | N | 1 | 120 122 |  | Value Description |  |
|  | 20007 PartyIDEnteringFirm | N | 1 | 120 122 |  | Value Description |  |
|  | 20007 PartyIDEnteringFirm | N | 1 |  | unsigned int | 2 Market Supervision |  |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int |  | Reason for mass cancellation. |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | Value | Description |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 0 | No special reason |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 1 Trading was stopped |  |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 2 Emergency Stop button |  |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 4 | Activated |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 5 | Business unit 5suspended |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 6 | Session loss or logout |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 8 Clearing Risk Control |  |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 100 | Internal connection loss |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 107 | Instrument Suspended |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 108 | Price Band Shrink |
|  | 28721 MassActionReason | Y | 1 | 123 | unsigned int | 119 | RRM: Market Wide Position Limit |

---

| Tag | Tag Field Name | Req'd | Len | Ofs | Ofs Data Type | Description |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  | Value | Description |  |  |  |  |  |  |
|  |  |  |  |  |  | 3 | Market Maker protection |  |  |  |  |  |  |
|  |  |  |  |  |  | 7 | Duplicate Session Login |  |  |  |  |  |  |
|  |  |  |  |  |  | 105 | Product State Halt |  |  |  |  |  |  |
|  |  |  |  |  |  | 106 | Product State Holiday |  |  |  |  |  |  |
|  |  |  |  |  |  | 109 | Complex instrument deletion |  |  |  |  |  |  |
|  |  |  |  |  |  | 110 | Volatility Interruption |  |  |  |  |  |  |
|  |  |  |  |  |  | 111 | Product temporarily not tradeable |  |  |  |  |  |  |
|  |  |  |  |  |  | 117 | Member disabled |  |  |  |  |  |  |
|  |  |  |  |  |  | 120 | Product state EOD |  |  |  |  |  |  |
|  |  |  |  |  |  | 179 | Member Status change |  |  |  |  |  |  |
|  |  |  |  |  |  | 202 | RRM cancellation |  |  |  |  |  |  |
|  |  |  |  |  |  | 203 | SquareOff |  |  |  |  |  |  |
|  |  |  |  |  |  | 205 | Debarred |  |  |  |  |  |  |
|  |  |  |  |  |  | 206 | Disallowed |  |  |  |  |  |  |
|  |  |  |  |  |  | 207 | User Disabled |  |  |  |  |  |  |
|  |  |  |  |  |  | 208 | Entity SquareOff |  |  |  |  |  |  |
|  |  |  |  |  |  | 209 | Suspended SquareOff |  |  |  |  |  |  |
|  |  |  |  |  |  | 217 | Suspended |  |  |  |  |  |  |
|  |  |  |  |  |  | 18 | ExecInst | Y | 1 | 124 | unsigned int | Instructions for order handling on exchange trading floor. If more than one instruction is applicable to an order, this field can contain multiple 124instructions separated by space. |  |
|  |  |  |  |  |  | 18 | ExecInst | Y | 1 | 124 | unsigned int | Value | Description |
|  |  |  |  |  |  | 18 | ExecInst | Y | 1 | 124 | unsigned int | 1 | Persistent Order(FIX value-H) |
|  |  |  |  |  |  | 18 | ExecInst | Y | 1 | 124 | unsigned int | 2 | Non-Persistent Order(FIX value-Q) |
| 3 | Persistent and Non-Persistent Orders Affected |  |  |  |  | 18 | ExecInst | Y | 1 | 124 | unsigned int |  |  |
| 54 | Side | N | 1 | 125 | unsigned int | 18 | ExecInst | Y | 1 | 124 | unsigned int | Side of the order. |  |
| 54 | Side | N | 1 | 125 | unsigned int | Value | Description |  |  |  |  |  |  |
| 54 | Side | N | 1 | 125 | unsigned int | 1 | Buy |  |  |  |  |  |  |
| 54 | Side | N | 1 | 125 | unsigned int | 2 | Sell |  |  |  |  |  |  |
| 1 | Account | N | 2 | 126 | Fixed String (0-terminable) | Not used. Must be sent as A1 Valid characters: 1-9, \x41, \x47, \x49, \x4D, \x50, \x52 |  |  |  |  |  |  |  |

---

| Tag 59 | Field Name TimeInForce | Req’d N | Len | Ofs 128 | Data Type unsigned int |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tag 59 | Field Name TimeInForce | Req’d N | 1 | Ofs 128 | Data Type unsigned int |  | Execution and trading restriction pa- rameters supported by MCX. Description 0 Day (DAY) Good Till Cancelled (GTC) – 1 Standard Orders only 3 Immediate or Cancel (IOC) Good Till Date (GTD) – 6 Standard Orders only 7 Session (EOS) |
| Tag 59 | Field Name TimeInForce | Req’d N | 1 | Ofs 128 | Data Type unsigned int | Value | Execution and trading restriction pa- rameters supported by MCX. Description 0 Day (DAY) Good Till Cancelled (GTC) – 1 Standard Orders only 3 Immediate or Cancel (IOC) Good Till Date (GTD) – 6 Standard Orders only 7 Session (EOS) |
| Tag 59 | Field Name TimeInForce | Req’d N | 1 | Ofs 128 | Data Type unsigned int |  | Execution and trading restriction pa- rameters supported by MCX. Description 0 Day (DAY) Good Till Cancelled (GTC) – 1 Standard Orders only 3 Immediate or Cancel (IOC) Good Till Date (GTD) – 6 Standard Orders only 7 Session (EOS) |
| Tag 59 | Field Name TimeInForce | Req’d N | 1 | Ofs 128 | Data Type unsigned int |  | Execution and trading restriction pa- rameters supported by MCX. Description 0 Day (DAY) Good Till Cancelled (GTC) – 1 Standard Orders only 3 Immediate or Cancel (IOC) Good Till Date (GTD) – 6 Standard Orders only 7 Session (EOS) |
| Tag 59 | Field Name TimeInForce | Req’d N | 1 | Ofs 128 | Data Type unsigned int |  | Execution and trading restriction pa- rameters supported by MCX. Description 0 Day (DAY) Good Till Cancelled (GTC) – 1 Standard Orders only 3 Immediate or Cancel (IOC) Good Till Date (GTD) – 6 Standard Orders only 7 Session (EOS) |
| Tag 59 | Field Name TimeInForce | Req’d N | 1 | Ofs 128 | Data Type unsigned int |  | Execution and trading restriction pa- rameters supported by MCX. Description 0 Day (DAY) Good Till Cancelled (GTC) – 1 Standard Orders only 3 Immediate or Cancel (IOC) Good Till Date (GTD) – 6 Standard Orders only 7 Session (EOS) |
| 40 | OrdType | N | 1 | 129 | unsigned int | Order type. |  |
|  |  | N | 1 |  | unsigned int | Value Description |  |
|  |  | N | 1 |  | unsigned int | 2 | Limit |
|  |  | N | 1 |  | unsigned int | 3 Stop Market 4 Stop Limit |  |
|  |  | N | 1 |  | unsigned int | 3 Stop Market 4 Stop Limit |  |
|  |  | N | 1 |  | unsigned int | 5 | Market To Limit |
|  |  | N | 1 |  | unsigned int | 6 | Auction Buy IN |
|  |  | N | 1 |  | unsigned int | 7 | Auction_Sell_Out |
|  |  | N | 1 |  | unsigned int |  |  |
| 25007 |  | N N |  |  |  |  |  |
| 25007 | FreeText1 | N N | 12 | 130 | Fixed String (0-terminable) | person | The Unique Client Code (UCC) of the for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, |
| 25007 |  | N N |  | 142 154 |  |  | \x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 142 154 | Fixed String (0-terminable) |  | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 39030 | Pad3 | N | 3 |  | Fixed String | Not Used. |  |
|  | &lt;NotAffectedOrdersGrp&gt; |  |  |  |  | Cardinality: | 0-500, Record counter: NoNotAffectedOrders |
| 1371 | &gt;NotAffectedOrderID | Y | 8 | 157 | unsigned int |  | Exchange Order ID of an order whose cancellation is pending. |
| 1372 | &gt;NotAffOrigClOrdID | N | 8 | 165 | unsigned int |  | Original Client Order ID of an order whose cancellation is pending. |

---

## 5.2.17 New Order Response (Lean Order)-10102

This message confirms a New Order request for a Lean Order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- |
| &lt;MessageHeaderOut&gt; |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int |  | Number of bytes for the message, in- cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int |  | Unique identifier for a ETI message layout. Value: 10102 (ExecutionReport, MsgType = 8) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String |  | not used |
| &lt;NRResponseHeaderME&gt; |  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp |  | In timestamp; filled always by the gateway |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp |  | not used |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp |  | Matching engine in timestamp. |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp |  | Matching engine out timestamp. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp |  | Timestamp the gateway receives a message from the Matching Engine |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp |  | Outgoing delta timestamp; filled al- ways by the gateway |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int |  | Message sequence number used by the participant for requests sent to the gateway. |
|  |  |  |  |  |  |  | Message sequence number used by the participant for requests sent to the gateway. |
|  |  |  |  |  |  |  | Message sequence number used by the participant for requests sent to the gateway. |
|  |  |  |  |  |  |  | Message sequence number used by the participant for requests sent to the gateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int |  | Message sequence number used by the participant for requests sent to the gateway. |
|  |  | Y | 1 |  |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  | Y | 1 |  |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  | Y | 1 |  |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  | Y | 1 |  |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  | Y | 1 |  |  |  |  |
| U 39030 3 61 Fixed String not used Pad3 |  |  |  |  |  |  |  |
| &lt;Message Body&gt; |  |  |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 64 | unsigned int | Exchange Order ID generated by the System; it remains constant over the lifetime of an order. |  |
| 11 | ClOrdID | N | 8 | 72 | unsigned int |  | Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |
| 719 | PriceMkToLimitPx | N | 8 | 80 | PriceType |  | The price at which the market order got converted into regular limit or- der.Applicable for OrdType (40) = 5. |
| 48 | SecurityID | Y | 8 | 88 | signed int | Instrument identifier. |  |
| 17 | ExecID | Y | 8 | 96 | UTCTimestamp |  | Transaction timestamp. |

---

| Tag | Field Name OrdStatus | Req’d Y | Len | Ofs 104 | Data Type char | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 | Data Type char | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 | Data Type char | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 | Data Type char | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 | Data Type char | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 | Data Type char | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 | Data Type char | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 | Data Type char | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 | Field Name OrdStatus | Req’d Y | 1 | Ofs 104 |  | der. Value 0 1 2 4 7 8 | Description New Partially filled Filled Cancelled RRM Suspended SquareOff Suspended |
| 39 |  |  | 1 |  |  |  |  |
| 150 | ExecType | Y | 1 105 |  | char | The reason why this message was gen- erated. Value Description |  |
| 150 |  | Y | 1 105 |  | char | The reason why this message was gen- erated. Value Description |  |
| 150 |  | Y | 1 105 |  | char | 0 | New |
| 150 |  | Y | 1 105 |  | char | 4 | Cancelled |
| 150 |  | Y | 1 105 |  | char | L Triggered |  |
| 378 | ExecRestatement- Reason | Y | 2 | 106 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. |  |
|  | ExecRestatement- Reason | Y | 2 |  |  | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. |  |
|  | ExecRestatement- Reason | Y | 2 |  |  | Value Description |  |
|  | ExecRestatement- Reason | Y | 2 |  |  | 101 | Order add accepted |
|  | ExecRestatement- Reason | Y | 2 |  |  | 105 IOC Order Cancelled |  |
|  | ExecRestatement- Reason | Y | 2 |  |  | 212 | Book or Cancel Order ac- cepted |
|  | ExecRestatement- Reason | Y | 2 |  |  | 114 | Order has been changed to IOC |
|  | ExecRestatement- Reason | Y |  |  |  |  |  |
| 2523 | CrossedIndicator | Y |  | 108 |  | Indicates SMP involvement. |  |
| 2523 | CrossedIndicator | Y |  | 108 |  | Value Description |  |
| 2523 | CrossedIndicator | Y |  | 108 |  | 0 | No crossing (Order not sub- ject to crossing) |
| 2523 | CrossedIndicator | Y |  | 108 |  | 1 | Cross rejected (Order subject to crossing and match pre- vented) |

---

| TagField Name |  | Req’dLen |  | Ofs | Data Type |  | DescriptionThis field qualifies an instrument typeon MCX. |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1227 | ProductComplex | Y | 1 | 109 | unsigned int |  | DescriptionThis field qualifies an instrument typeon MCX. |  |  |  |
| 1227 | ProductComplex |  |  |  | unsigned int |  |  | Value | Description |  |
| 1227 | ProductComplex |  |  |  |  |  | 1 | Simple instrument |  |  |
| 1227 | ProductComplex |  |  |  |  |  | 5 | Futures Spread |  |  |
| 1227 | ProductComplex |  |  |  |  |  |  |  |  |  |
| 1823 | Triggered | Y | 1 | 110 | unsigned int |  | Indicates if an order has been previ-ously triggered. |  |  |  |
| 1823 | Triggered | Y | 1 | 110 | unsigned int |  | Value | Description |  |  |
| 1823 | Triggered | Y | 1 | 110 | unsigned int |  | 0 | Not triggered |  |  |
| 1823 | Triggered | Y | 1 | 110 | unsigned int |  | 1 | Triggered Stop |  |  |
| 1823 | Triggered | Y | 1 | 110 | unsigned int |  | 2 | Triggered OCO |  |  |
| 1823 | Triggered | Y | 1 | 110 | unsigned int |  |  |  |  |  |
| 25159 | TransactionDelay-Indicator | Y | 1 | 111 | unsigned int |  | Indicator for a delayed transaction |  |  |  |
| 25159 | TransactionDelay-Indicator | Y | 1 | 111 | unsigned int |  | Value | Description |  |  |
| 25159 | TransactionDelay-Indicator | Y | 1 | 111 | unsigned int |  | 0 | Transaction not delayed |  |  |
| 25159 | TransactionDelay-Indicator | Y | 1 | 111 | unsigned int |  | 1 | Transaction delayed |  |  |
| 25159 | TransactionDelay-Indicator | Y | 1 | 111 | unsigned int |  |  |  |  |  |

---

## 5.2.18 Replace Order Response (Lean Order)-10108

This message confirms a Replace Order request for a Lean Order

| Tag Field Name |  | Req’d | Len | Ofs |  |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- |
| &lt;MessageHeaderOut&gt; |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 |  |  | Number of bytes for the message, in- cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 |  | Unique | identifier for a ETI message layout. Value: 10108 (ExecutionReport, MsgType = 8) |
| 39020 | Pad2 | U | 2 | 6 |  | not used In timestamp; filled always by the gateway |  |
|  | &lt;NRResponseHeaderME&gt; |  |  |  |  | not used In timestamp; filled always by the gateway |  |
| 5979 | RequestTime | Y | 8 | 8 |  | not used In timestamp; filled always by the gateway |  |
| 745 | Reserve0 | U | 8 | 16 |  |  | not used |
| 21002 | Reserve1 | Y | 8 | 24 |  |  | Matching engine in timestamp. |
| 21003 | Reserve2 | Y | 8 | 32 |  |  | Matching engine out timestamp. |
| 7765 | Reserve3 | Y | 8 | 40 |  |  | Timestamp the gateway receives a message from the Matching Engine |
| 52 | SendingTime | Y | 8 | 48 |  |  | Outgoing delta timestamp; filled al- ways by the gateway |
| 34 | MsgSeqNum | Y | 4 | 56 |  |  | Message sequence number used by the participant for requests sent to the gateway. |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |
|  |  | Y |  |  |  |  |  |
| 893 | LastFragment |  | 1 | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  |  | 1 | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  |  | 1 | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  |  | 1 | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  |  | 1 | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  |  | 1 | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  |  | 1 | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |  |
|  |  |  | 1 | unsigned int |  |  |  |
| U 39030 3 61 Fixed String not used Pad3 |  |  |  |  |  |  |  |
| &lt;Message Body&gt; |  |  |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 64 |  |  | Exchange Order ID generated by the System; it remains constant over the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 72 |  |  | Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |
| 41 | OrigClOrdID | N | 8 | 80 |  |  | ClOrdID (11) of the last successfully processed task (request) referring to the specific order. |
| 719 | PriceMkToLimitPx | N | 8 | 88 |  |  | The price at which the market order got converted into regular limit or- |

---

|  |  |  |  |  |  | der.Applicable for OrdType (40) = 5. |
| --- | --- | --- | --- | --- | --- | --- |
| 48 | SecurityID | Y | 8 | 96 | signed int | Instrument identifier. |

| Tag | Field Name ExecID | Req’d Y | Len | Ofs | Data Type | Description Transaction timestamp. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 17 | Field Name ExecID | Req’d Y | 8 | 104 | UTCTimesta mp | Description Transaction timestamp. |  |
| 151 | LeavesQty | Y | 8 | 112 | Qty | Remaining quantity of an order. |  |
| 14 | CumQty | Y | 8 | 120 | Qty | Cumulated executed quantity of an or- der. |  |
| 84 | CxlQty | Y | 8 1 | 128 | Qty | Total quantity cancelled for this order. |  |
| 39 | OrdStatus | Y | 8 1 | 136 | char | Conveys the current status of an or- der. |  |
| 39 | OrdStatus | Y | 8 1 |  | char | Value Description |  |
| 39 | OrdStatus | Y | 8 1 |  | char | 0 | New |
| 39 | OrdStatus | Y | 8 1 |  | char | 1 | Partially filled |
| 39 | OrdStatus | Y | 8 1 |  | char | 2 | Filled |
| 39 | OrdStatus | Y | 8 1 |  | char | 4 | Cancelled |
| 39 | OrdStatus | Y | 8 1 |  | char | 7 | RRM Suspended |
|  | OrdStatus | Y |  |  | char | 8 | SquareOff Suspended |
| 150 | ExecType | Y | 1 | 137 | char | The reason why this message was gen- erated. |  |
| 150 | ExecType | Y | 1 | 137 |  | The reason why this message was gen- erated. |  |
| 150 | ExecType | Y | 1 | 137 |  | Value Description 4 Cancelled 5 Replaced |  |
| 150 | ExecType | Y | 1 | 137 |  | Value Description 4 Cancelled 5 Replaced |  |
| 150 | ExecType | Y | 1 | 137 |  | Value Description 4 Cancelled 5 Replaced |  |
| 150 | ExecType | Y | 1 | 137 |  | L Triggered |  |
| 378 | ExecRestatement- Reason | Y | 2 | 138 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. |  |
| 378 | ExecRestatement- Reason | Y | 2 | 138 | unsigned int | Value | Description |
| 378 | ExecRestatement- Reason | Y | 2 | 138 | unsigned int | 102 Order modify accepted |  |
| 378 | ExecRestatement- Reason | Y | 2 | 138 | unsigned int | 105 | IOC Order Cancelled |
| 378 | ExecRestatement- Reason | Y | 2 | 138 | unsigned int | 181 | Ownership Changed |
| 378 | ExecRestatement- Reason | Y | 2 | 138 | unsigned int | 212 | Book or Cancel Order ac- cepted |
| 378 | ExecRestatement- Reason | Y | 2 | 138 | unsigned int | 114 | Order has been changed to IOC |

---

| 2523 | CrossedIndicator | Y | 1 | 140 | unsigned int | Indicates SMP involvement. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 2523 | CrossedIndicator | Y | 1 | 140 | unsigned int | Value | Description |
| 2523 |  |  |  |  | unsigned int | 0 | No crossing (Order not sub- ject to crossing) |
| 2523 |  |  |  |  | unsigned int | 1 | Cross rejected (Order subject to crossing and match pre- vented) |

| Tag | Field Name | Req’d | Len | Ofs 141 | unsigned int | Description This field qualifies an instrument type on MCX. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1227 | ProductComplex | Y | 1 | Ofs 141 | unsigned int | Description This field qualifies an instrument type on MCX. |  |
| 1227 | ProductComplex | Y | 1 | Ofs 141 | unsigned int | Value | Description |
| 1227 | ProductComplex | Y | 1 | Ofs 141 | unsigned int | 1 | Simple instrument |
| 1227 | ProductComplex | Y | 1 | Ofs 141 | unsigned int | 2 | Standard Option Strategy |
| 1227 | ProductComplex | Y | 1 | Ofs 141 | unsigned int | 3 | Non-Standard Option Strat- egy |
| 1227 | ProductComplex | Y | 1 | Ofs 141 | unsigned int | 4 | Volatility Strategy |
| 1227 | ProductComplex | Y | 1 |  | unsigned int | 5 | Futures Spread |
| 1227 | ProductComplex | Y | 1 |  | unsigned int | 6 | Inter-Product Spread |
| 1227 | ProductComplex | Y | 1 |  | unsigned int | 7 | Standard Future Strategy |
| 1227 | ProductComplex | Y | 1 |  | unsigned int | 8 | Pack and Bundle |
| 1227 | ProductComplex | Y | 1 |  | unsigned int |  |  |
| 1227 | ProductComplex | Y | 1 |  | unsigned int | 9 | Strip |
| 1227 | ProductComplex | Y | 1 | 142 unsigned int |  |  |  |
| 1823 |  |  | 1 | 142 unsigned int |  | Indicates if an order has been previ- ously triggered. |  |
| 1823 |  |  | 1 | 142 unsigned int |  | Value | Description |
| 1823 |  |  | 1 | 142 unsigned int |  | 0 | Not triggered |
| 1823 |  |  | 1 | 142 unsigned int |  | 1 | Triggered Stop |
| 1823 |  |  | 1 | 142 unsigned int |  | 2 | Triggered OCO |
| 1823 |  |  | 1 | 142 unsigned int |  |  |  |
| 25159 | TransactionDelay- Indicator | Y | 1 | 143 | unsigned int | Indicator for a delayed transaction. |  |
| 25159 | TransactionDelay- Indicator | Y |  | 143 | unsigned int | Value | Description |
| 25159 | TransactionDelay- Indicator | Y |  | 143 | unsigned int | 0 | Transaction not delayed |
| 25159 | TransactionDelay- Indicator | Y |  | 143 | unsigned int | 1 | Transaction delayed |

---

**5.2.19 Cancel Order Response (Lean Order)-10111**
This message confirms the cancellation of a Lean Order.

| Tag | Req’d Len Ofs Field Name |  |  |  | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| &lt;MessageHeaderOut&gt; |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in- cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a ETI message layout. Value: 10111 (ExecutionReport, MsgType = 8) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| &lt;NRResponseHeaderME&gt; |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | In timestamp; filled always by the gateway |
| 745 | Reserve0 | U | 8 | 16 | UTCTimestamp | not used |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | Matching engine in timestamp. |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Matching engine out timestamp. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Timestamp the gateway receives a message from the Matching Engine |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Outgoing delta timestamp; filled al- ways by the gateway |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by the participant for requests sent to the gateway. |
|  |  |  |  |  |  | Message sequence number used by the participant for requests sent to the gateway. |
|  |  |  |  |  |  | Message sequence number used by the participant for requests sent to the gateway. |
|  |  |  |  |  |  | Message sequence number used by the participant for requests sent to the gateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |
| 893 |  | Y |  |  | unsigned int | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |
| 893 |  | Y |  |  | unsigned int | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |
| 893 |  | Y |  |  | unsigned int | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |
| 893 |  | Y |  |  | unsigned int | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |
| 893 |  | Y |  |  | unsigned int | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |
| 893 |  | Y |  |  | unsigned int | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description 1 Last Message |
| U 39030 3 61 Fixed String not used Pad3 |  |  |  |  |  |  |
| &lt;Message Body&gt; |  |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 64 | unsigned int | Exchange Order ID generated by the System; it remains constant over the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 72 | unsigned int | Client Order ID: Unique participant defined order request identifier; used for client order id chaining. |
| 41 | OrigClOrdID | N | 8 | 80 | unsigned int | ClOrdID (11) of the last successfully processed task (request) referring to the specific order. |
| 48 | SecurityID | Y | 8 | 88 | signed int | Instrument identifier. |

---

| Tag 17 | Field Name ExecID | Req’d Y Y | Len Ofs |  | Data Type UTCTimestam p Qty | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tag 17 | Field Name ExecID | Req’d Y Y | 8 | 104 112 120 | Data Type UTCTimestam p Qty | Transaction timestamp. Cumulated executed quantity of an or- der. |  |
| 14 | CumQty | Req’d Y Y | 8 | 104 112 120 | Data Type UTCTimestam p Qty | Transaction timestamp. Cumulated executed quantity of an or- der. |  |
| 84 | CxlQty OrdStatus | Y | 8 1 | 104 112 120 | Qty | Total quantity cancelled for this order. |  |
| 39 | CxlQty OrdStatus | Y | 8 1 | 104 112 120 | char | Conveys the current status of an or- der. |  |
| 39 | CxlQty OrdStatus | Y | 8 1 | 104 112 120 |  | Value | Description |
| 39 | CxlQty OrdStatus | Y | 8 1 |  |  | 4 Cancelled |  |
| 39 | CxlQty OrdStatus | Y | 8 1 |  |  |  |  |
| 150 | ExecType | Y | 1 | 121 | char | The reason why this message was gen- erated. |  |
| 150 | ExecType |  | 1 |  | char | The reason why this message was gen- erated. |  |
| 150 | ExecType | Y | 1 |  | char | 4 | Cancelled |
| 150 | ExecType | Y | 1 |  | char |  |  |
| 378 | ExecRestatement- Reason | Y | 2 | 122 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report |  |
| 378 | ExecRestatement- Reason | Y | 2 |  |  | Code to further qualify the field Exec- Type (150) of the Execution Report |  |
| 378 | ExecRestatement- Reason | Y | 2 |  | (8) message. |  |  |
| 378 | ExecRestatement- Reason | Y | 2 |  |  | Value | Description |
| 378 | ExecRestatement- Reason |  | 2 |  |  | 103 | Order delete accepted |
| 378 | ExecRestatement- Reason |  | 2 | 124 |  | 197 | Pending order deletion |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int |  |  |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | on MCX Value Description |  |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | on MCX Value Description |  |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 1 | Simple instrument |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 2 Standard Option Strategy |  |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 3 | Non-Standard Option Strat- egy |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 4 | Volatility Strategy |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 5 | Futures Spread |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 6 | Inter-Product Spread |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 7 | Standard Future Strategy |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 8 | Pack and Bundle |
| 1227 | ProductComplex | Y | 1 | 124 | unsigned int | 9 | Strip |

---

| 25159 | TransactionDelay- Indicator | Y | 1 | 125 | unsigned int | Indicator for a delayed transaction |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 25159 | TransactionDelay- Indicator | Y | 1 | 125 | unsigned int | Value 0 1 | Description Transaction not delayed Transaction delayed |
| 25159 | TransactionDelay- Indicator | Y | 1 | 125 | unsigned int | Value 0 1 | Description Transaction not delayed Transaction delayed |
| 25159 | TransactionDelay- Indicator | Y | 1 | 125 | unsigned int | Value 0 1 | Description Transaction not delayed Transaction delayed |
| 39020 | Pad2 | U | 2 | 126 | unsigned int | not used |  |

## 5.3 Multileg order Handling

## 5.3.1 New Order Complex-10113

The New Order Multi Leg message is provided to submit orders for securities that are made up of 
multiple securities, known as “legs”. This message is sent to the service “Order and Quote 
Management”.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  10113 (New-OrderMultileg, MsgType = AB) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  | ClOrdID |  |  |  |  |  |
| 11 | ClOrdID | N | 8 | 24 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining. |
| 48 | SecurityID | Y | 8 | 32 | signed int | Instrument identifier.  -The U/L As-set / producer Identifier is specifiedby Exchange for permitted U/L Asset/ products for trading |
| 731 | MaxPricePercentage | N | 8 | 40 | PriceType | Price per unit of quantity (e.g.  pershare) In Case of Limit Price, NoValue is required to be send. |

---

|  | Tag Field Name 708 TerminalInfo | Req’d Y | Len | Ofs 48 |  | Data Type unsigned int | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | 11111111111 IBT |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | 22222222222 DMA |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | 33333333333 Wireless Technology |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int |  | Order not 0 through | generated Program |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int |  | Order 1 through | trading software generated Program |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | trading software Smart Order  Rout- 2 ing without Program trading software Smart Order Routing |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int | trading software Smart Order  Rout- 2 ing without Program trading software Smart Order Routing |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 48 |  | Data Type unsigned int |  |  | 3 with program trading |
|  |  |  | 8 | Ofs 48 |  | Data Type unsigned int |  | software |  |
|  | 44 Price | N | 8 | 56 |  | PriceType |  | Limit price. Required if OrdType (40) is Limit (2) or Stop Limit (4). |  |
|  | 709 Filler1 | N | 8 | 64 |  | unsigned int | Not Used. |  |  |
|  | 710 Filler2 | N | 4 | 72 |  | unsigned int | Not Used. |  |  |
|  | 711 Echo | Y | 4 | 76 |  | signed int | tener data, session data. | User defined free field.  The field is echoed in the trade notifications, lis- |  |
|  | 38 OrderQty | Y | 8 | 80 |  | Qty | Total Order Quantity. |  |  |
|  | 732 DisclosedQty | N | 8 | 88 |  | Qty |  | The quantity to be made visible in the market data.DisclosedQty is set to 0 in case full qty needs to be disclosed. |  |
|  | 727 StrategyID | Y | 8 | 96 |  | unsigned int | 99999 | Strategy Approved by the exchange should be used.Range being from 0 to |  |
|  | 728 StrategySequence- Number | Y | 8 | 104 |  | unsigned int | Strategy Sequence numbers |  |  |
|  | 1300 MarketSegmentID | Y | 4 | 112 |  | signed int | Product identifier. |  |  |
| 432 ExpireDate |  | N | 4 | 116 |  | LocalMktDate | Not Used |  |  |

---

|  | Tag Field Name 712 RegulatoryID | Req’d N | Len | Ofs | Data Type | Description Not Used |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 712 RegulatoryID | Req’d N | 4 2 | 120 | unsigned int | Description Not Used |
|  | 713 Filler4 | N | 4 2 | 124 | unsigned int Fixed String | Not Used. |
|  | 20096 PartyIDTakeUp- TradingFirm | N | 5 | 126 | unsigned int Fixed String | Not Used |
|  | 20013 PartyIDOrder- OriginationFirm | N | 7 | 131 | Fixed String | Not Used |
|  | 20032 PartyIDBeneficiary | N | 9 | 138 | Fixed String | Not Used |
|  | 714 AccountType | Y | 1 | 147 | unsigned int | Value Description 1 Own |
|  |  | Y | 1 |  | unsigned int | Value Description 1 Own |
|  |  | Y | 1 |  | unsigned int | Value Description 1 Own |
|  |  | Y | 1 |  | unsigned int | 3 Client |
|  |  | Y | 1 |  | unsigned int | 5 Institution |
| 28703 ApplSeqIndicator Y 1 148 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
| 28703 ApplSeqIndicator Y 1 148 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
| 28703 ApplSeqIndicator Y 1 148 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
| 28703 ApplSeqIndicator Y 1 148 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
|  | 1227 ProductComplex | Y | 1 | 149 | unsigned int | This field qualifies an instrument type |
|  | 1227 ProductComplex |  | 1 | 149 | unsigned int | on MCX. Value Description 5 Futures Spread 1 Simple Instrument |
|  | 1227 ProductComplex |  | 1 | 149 | unsigned int | on MCX. Value Description 5 Futures Spread 1 Simple Instrument |
|  | 1227 ProductComplex |  | 1 | 149 | unsigned int | on MCX. Value Description 5 Futures Spread 1 Simple Instrument |
|  | 1227 ProductComplex |  | 1 | 149 | unsigned int |  |
|  | 54 Side | Y | 1 | 150 | unsigned int | Side of the order. |
|  | 54 Side | Y | 1 | 150 |  | Value Description |
|  | 54 Side | Y | 1 | 150 |  | 1 Buy |
|  | 54 Side | Y | 1 | 150 |  | 2 Sell |
|  | 40 OrdType | Y | 1 | 151 | unsigned int | Order type. |
|  |  | Y |  | 151 | unsigned int | Value Description |
|  |  | Y |  | 151 | unsigned int | 2 Limit |
|  |  | Y |  | 151 | unsigned int | 5 Market To Limit |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 152 | unsigned int | Not Used. |
|  | 28710 PriceValidityCheck- Type | Y |  | 152 | unsigned int | Value Description |
|  | 28710 PriceValidityCheck- Type | Y |  | 152 | unsigned int | 0 None |

---

| Tag 18 | Field Name | Req’d | Len | Ofs | Data Type |  | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tag 18 | ExecInst | Y | 1 | 153 | unsigned int |  | Instructions for order handling on ex- change trading floor. If more than one instruction is applicable to an order, this field can contain multiple 137nstruct- tions separated by space. |  |
| Tag 18 | ExecInst | Y | 1 | 153 | unsigned int |  | Value Description |  |
| Tag 18 | ExecInst | Y | 1 | 153 | unsigned int |  | 1 | Persistent Order(FIX value- H) |
| Tag 18 | ExecInst | Y | 1 | 153 | unsigned int |  | 2 value-Q) |  |
| Tag 18 | ExecInst | Y | 1 | 153 | unsigned int |  |  |  |
| 59 | TimeInForce | Y | 1 | 154 |  |  | Execution and trading restriction pa- rameters supported by MCX. |  |
| 59 | TimeInForce | Y | 1 | 154 |  |  | 0 Day (DAY) 3 Immediate or Cancel (IOC) |  |
| 59 | TimeInForce | Y | 1 | 154 |  |  | 0 Day (DAY) 3 Immediate or Cancel (IOC) |  |
| 59 | TimeInForce | Y | 1 | 154 |  |  | 0 Day (DAY) 3 Immediate or Cancel (IOC) |  |
| 59 | SMPFOrderIdentifier | Y | 1 | 155 | unsigned int |  | 7 | Session (EOS) |
| 715 |  |  | 1 |  |  |  |  |  |
| 715 |  |  | 1 |  |  |  | Value 0 | Description |
| 715 |  |  | 1 |  |  |  | 1 | Passive |
| 716 |  | N | 1 | 156 | unsigned int |  | Active Not Used. |  |
| 1815 | Filler5 TradingCapacity | Y | 1 | 157 | unsigned int |  | This field designates if the trader is acting in the capacity of agent, trad- ing for its own account or acting as a market maker. Value Description |  |
| 1815 |  |  |  | 157 | unsigned int |  | This field designates if the trader is acting in the capacity of agent, trad- ing for its own account or acting as a market maker. Value Description |  |
| 1815 |  |  |  | 157 | unsigned int |  | 1 | Customer (Agency) |
| 1815 |  |  |  |  |  |  |  |  |
| 20075 | PartyIDLocationID | N | 2 | 158 | Fixed String (0-terminable) |  | Not Used. |  |

---

| Tag | Field Name | Req’d | Len | Ofs  Data Type |  | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 718 | UserReferenceText | N | 20 | 160 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide additional regulatoryinformation(according to respectiverules andregs, circulars and/orbilateral  coordination  betweenparticipantand Trading SurveillanceOffice).Valid characters: \x20, \x22-\x7B, \x7D, \x7E.Valid characters:\x20, \x22-\x7B,\x7D, \x7E |
| 1031 | CustOrderHandling-Inst | N | 1 | 180 | Fixed String | not used.Valid characters:\x20, \x22-\x7B,\x7D, \x7E |
| 25007 | FreeText1 | Y | 12 | 181 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 193 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 25009 | FreeText3 | N | 12 | 205 | Fixed String | Free-format text field fortraderspecific or customer-relatedcomments.Valid characters:\x00,\x21,\x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 555 | NoLegs | Y | 1 | 217 | Counter | Number of LegOrd repeating groupinstances. |
| 39060 | Pad6 | N | 6 | 218 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality: 0-20, Record counter:NoLegs |
| 2680 | >LegAccount | N | 2 | 224 | Fixed String(0-terminable) | Leg-specific account to book tradesand keep positions on.Valid characters:1-9, \x41, \x47,\x4D, \x50 |

---

| 564 | >LegPositionEffect | Y | 1 | 226 | char |  | Leg-specific field used for MCX posi-tion management purposes and indi-cates whether the leg is submitted toopen or close a position. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 564 | >LegPositionEffect | Y | 1 | 226 | char |  |  | Description |
| 564 | >LegPositionEffect | Y | 1 | 226 | char |  |  | Close |
| 564 | >LegPositionEffect | Y | 1 | 226 | char |  | Open |  |
| 564 | >LegPositionEffect | Y |  |  |  |  |  |  |
| 39050 | >Pad5 | N | 5 | 227 | Fixed String |  | Not Used. |  |

## 5.3.2 Replace Order Complex-10114

This message is used to replace a multi leg order (previously submitted using the New Order Multi 
Leg(Complex) message).

Replace Order Complex

Executions?

Without Execution

Full or partial fill

Replace Order Response (Standard Order)
(Session Data)

Immediate Execution Response
(Session Data)

Extended Order Information
(Listener Data)

Extended Order Information
(Listener Data)

Trade Notification
(Trade)

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout. Value: 10114 (Multileg-OrderCancelReplace, MsgType = AC) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
| 37 |  |  |  |  |  | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 37 | OrderID | N | 8 | 24 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 32 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining.ClOrdID should be mention inNewOrderSingle and same value canbe passed in ClOrdID as well asOrigClOrdID. |
| 41 | OrigClOrdID | N | 8 | 40 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring tothe specific order. |
| 48 | SecurityID | Y | 8 | 48 | signed int | Instrument identifier. |
| 44 | Price | Y | 8 | 56 | PriceType | Limit price. Required if OrdType (40)is Limit (2) or Stop Limit (4). |
| 731 | MaxPricePercentage | N | 8 | 64 | PriceType | Price per unit of quantity (e.g.  pershare) In Case of Limit Price, NoValue is required to be send. |

---

|  | Tag Field Name 708 TerminalInfo | Req’d Y | Len | Ofs 72 | Data Type |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 | unsigned int |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 | UTCTimestamp |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  | 11111111111 IBT |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  | 22222222222 DMA |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  | 33333333333 Wireless Technology |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  |  | Order not | generated |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  |  | 0 through trading software | Program |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  |  | Order 1 through trading software | generated Program |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  | Smart Order  Rout- 2 ing without Program |  |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 | Ofs 72 |  |  |  | trading software Smart Order Routing |  |
|  | Tag Field Name 708 TerminalInfo | Req’d Y | 8 |  |  |  |  | 3 with program trading software |  |
|  |  |  | 8 |  |  |  |  |  |  |
|  | 723 LstUpdtTime | Y | 8 | 80 |  |  | Last Updated timestamp for that or- der |  |  |
|  | 709 Filler1 | N | 8 | 88 | unsigned int |  | Not Used. |  |  |
|  | 710 Filler2 | N | 4 | 96 | unsigned int |  | Not Used. |  |  |
|  | 1300 MarketSegmentID | Y | 4 | 100 | signed int |  | Product identifier. |  |  |
|  | 38 OrderQty | Y | 8 | 104 | Qty |  | Total Order Quantity. |  |  |
|  | 732 DisclosedQty | N | 8 | 112 | Qty |  | The quantity to be made visible in the market data.DisclosedQty is set to 0 in case full qty needs to be disclosed. |  |  |
|  | 727 StrategyID | Y | 8 | 120 | unsigned int |  | 99999 | Strategy Approved by the exchange should be used.Range being from 0 to |  |
|  | 728 StrategySequence- Number | Y | 8 | 128 | unsigned int |  |  | Strategy Sequence numbers |  |
|  | 711 Echo | N | 4 | 136 | signed int |  | response from exchange | Vendors can use this as a reference in |  |
| 432 ExpireDate |  | N | 4 | 140 | LocalMktDate |  | Not used. |  |  |

---

|  | Tag Field Name 20655 TargetPartyIDSession- ID | Req’d N | Len 4 | Ofs 144 148 | Data Type unsigned int | Description Session ID. |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 20655 TargetPartyIDSession- ID | Req’d N | Len 4 | Ofs 144 148 | Data Type unsigned int | Description Session ID. |
|  | 712 RegulatoryID | N | 4 | Ofs 144 148 | unsigned int | For participants to uniquely identify trading startegies |
|  | 713 Filler4 | N | 2 | 152 | unsigned int | Not Used. |
|  | 20096 PartyIDTakeUp- TradingFirm | N | 5 | 154 | Fixed String Fixed String Fixed String | Not Used. |
|  | 20013 PartyIDOrder- OriginationFirm | N | 7 | 159 | Fixed String Fixed String Fixed String | Not used. |
|  | 20032 PartyIDBeneficiary | N | 9 | 166 |  | Not Used. |
|  | 714 AccountType | Y | 1 | 175 | unsigned int |  |
|  | 714 AccountType | Y |  | 175 |  | Value Description 1 Own 3 Client |
|  | 714 AccountType | Y |  | 175 |  | Value Description 1 Own 3 Client |
|  | 714 AccountType | Y |  | 175 |  | Value Description 1 Own 3 Client |
|  | 714 AccountType | Y |  | 175 |  | 5 Institution |
| 28703 ApplSeqIndicator Y 1 176 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
| 28703 ApplSeqIndicator Y 1 176 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
| 28703 ApplSeqIndicator Y 1 176 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
| 28703 ApplSeqIndicator Y 1 176 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
| 28703 ApplSeqIndicator Y 1 176 unsigned int Value Description 0 No Recovery Required 1 Standard Order |  |  |  |  |  |  |
|  | 1227 ProductComplex | Y | 1 177 |  | unsigned int | This field qualifies an instrument type |
|  |  | Y | 1 177 |  |  | on MCX. |
|  |  | Y | 1 177 |  |  | Value Description |
|  |  | Y | 1 177 |  |  | 5 Futures Spread |
|  |  | Y | 1 177 |  |  | 1 Simple Instrument. |
|  | 54 Side | Y |  |  |  |  |
|  | 54 Side | Y | 1 178 |  | unsigned int | Side of the order. |
|  | 54 Side | Y | 1 178 |  | unsigned int | Value Description |
|  | 54 Side | Y | 1 178 |  | unsigned int | 1 Buy |
|  | 54 Side | Y | 1 178 |  | unsigned int | 2 Sell |
|  | 40 OrdType | Y | 1 | 179 | unsigned int | Order type. |
|  | 40 OrdType |  | 1 | 179 | unsigned int | Value Description |
|  | 40 OrdType |  | 1 | 179 | unsigned int | 2 Limit |
|  | 28710 PriceValidityCheck- Type | Y | 1 | 180 | unsigned int | Not Used. |
|  | 28710 PriceValidityCheck- Type | Y |  | 180 |  | Value Description |
|  | 28710 PriceValidityCheck- Type | Y |  | 180 |  | 0 None |

---

| Tag 18 | Field Name | Req’d Y | Len | Ofs | Data Type unsigned int | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tag 18 | ExecInst | Req’d Y | 1 1 | 181 | Data Type unsigned int | this 143nstruct- space. Value 1 2 | Instructions for order handling on ex- change trading floor. If more than one instruction is applicable to an order, field can contain multiple tions separated by Description Persistent Order(FIX value- H) Non-Persistent Order(FIX value-Q) |
| Tag 18 | ExecInst | Req’d Y | 1 1 | 181 | Data Type unsigned int |  |  |
| Tag 18 | ExecInst | Req’d Y | 1 1 | 181 | Data Type unsigned int |  |  |
| Tag 18 | ExecInst | Req’d Y | 1 1 | 181 | Data Type unsigned int |  |  |
| Tag 18 | ExecInst | Req’d Y | 1 1 | 181 | Data Type unsigned int |  |  |
| Tag 18 | ExecInst | Req’d Y | 1 1 | 181 | Data Type unsigned int |  |  |
| 59 | TimeInForce | Y |  | 182 | unsigned int Execution and trading restriction pa- rameters supported by MCX. |  |  |
| 59 | TimeInForce | Y |  | 182 |  | Value | Description |
| 59 | TimeInForce | Y |  | 182 |  | 0 | Day (DAY) |
| 59 | TimeInForce | Y |  | 182 |  | 3 | Immediate or Cancel (IOC) |
| 59 | TimeInForce | Y |  | 182 |  |  |  |
| 59 | TimeInForce | Y |  | 182 |  | 7 | Session (EOS) |
| 59 | TimeInForce | Y |  |  |  |  |  |
| 716 | Filler5 | N | 1 | 183 unsigned int Not Used. |  |  |  |
| 1815 | TradingCapacity | Y | 1 | 184 | unsigned int | Not used, Must be sent as 1 |  |
| 750 | Filler6 | N | 1 | 185 | char | not used |  |
| 20075 | PartyIDLocationID | N | 2 | 186 | Fixed String (0-terminable) | Not Used. |  |
| 1031 | CustOrderHandling- Inst | N | 1 | 188 | Fixed String | not used. \x7D, \x7E | Valid characters: \x20, \x22-\x7B, |
| 718 | UserReferenceText | N | 20 | 189 | Fixed String (0-terminable) | rules bilateral \x7D, \x7E | Upto 15 Char allowed.This field is used to provide additional regulatory information(according to respective andregs, circulars and/or coordination between participantand Trading Surveillance Office).Valid characters: \x20, \x22- \x7B, \x7D, \x7E. Valid characters: \x20, \x22-\x7B, |

---

| TagField Name |  | Req’dLen |  | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 25007 | FreeText1 | Y | 12 | 209 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 221 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 25009 | FreeText3 | N | 12 | 233 | Fixed String | Free-format text field fortraderspecific or customer-relatedcomments.Valid characters:\x00,\x21,\x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 555 | NoLegs | Y | 1 | 245 | Counter | Number of LegOrd repeating groupinstances.Must be greater than 1. |
| 39020 | Pad2 | N | 2 | 246 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality: 0-20, Record counter:NoLegs |
| 2680 | >LegAccount | N | 2 | 248 | Fixed String(0-terminable) | Leg-specific account to book tradesand keep positions on.Valid characters: 1-9, \x41, \x47,\x4D, \x50 |
| 564 | >LegPositionEffect | Y | 1 | 250 | char | not used, Must be set as CValid characters: \x01-\x7E |
| 39050 | >Pad5 | N | 5 | 251 | Fixed String | Not Used. |

---

## 5.3.3 Cancel Order complex-10123

This message is used to cancel a multi leg order. This message is sent to the service “Order and 
Quote Management”.

Cancel Order Complex

Cancel Order Response (Standard Order)
(Session Data)

Cancel Order Notification
(Listener Data)

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderIn> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10123 (Order-CancelRequest, MsgType = F) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 37 | OrderID | N | 8 | 24 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 11 | ClOrdID | N | 8 | 32 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining.ClOrdID should be mention inNewOrderSingle and same value canbe passed in ClOrdID as well asOrigClOrdID. |
| 41 | OrigClOrdID | N | 8 | 40 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring tothe specific order. |
| 48 | SecurityID | Y | 8 | 48 | signed int | Instrument identifier. – The U/L As-set / producer Identifier is specified by |

---

|  |  |  |  |  |  | Exchange for permitted U/L Asset /products for trading |
| --- | --- | --- | --- | --- | --- | --- |
| 723 | LstUpdtTime | N | 8 | 56 | UTCTimestamp | Last Updated timestamp for that or-der |
| 711 | Echo | Y | 4 | 64 | signed int | Vendors can use this as a reference inresponse from exchange |
| 1300 | MarketSegmentID | Y | 4 | 68 | signed int | Product identifier. |
| 20655 T | argetPartyIDSession-ID | N | 4 | 72 | unsigned int | Session ID. |
| 712 | RegulatoryID | N | 4 | 76 | unsigned int | For participants to uniquely identifytrading startegies |

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | 727 StrategyID | Y | 8 | 80 | unsigned int | Strategy Approved by the exchange should be used.Range being from 0 to 99999 |
|  | 728 StrategySequence- Number | Y | 8 | 88 | unsigned int | Strategy Sequence numbers |

---

## 5.3.4 New Order MultiLeg-10991

The New Order Multileg message is used by the participant to submit an order for multiple securities. In 
essence, multileg orders are IOC (Immediate or cancel) orders. This means that all the legs must execute 
for the order to be successfully executed. If anyone of the legs in the order fails to execute, then all the 
remaining orders automatically get canceled. Each leg’s side is independent of one another i.e all leg can 
be Buy or all leg can be sell or in any combination. Each leg may belong to different U/L Asset , but 
instrument identifiers in individual legs of the same Multi leg order should be within the same capacity 
group as provided in Instrument Master (MCXScrips.bcp) file; else the order would be rejected by the 
exchange. Any of the leg should not be Auction or Spread Instrument. Each leg should have individual 
price and qty and order type. If MultiLegType is selected as Spread, then both the legs shoud belong to 
same underlying for calender spread position.

New Order Multileg
Execution 
as per ratio Full or Partial
Reject (Multileg)                  Immediate Execution Response(Multileg)
Session Data                                            (Session Data)  
Individual leg
Extended Order Information(Listener Data)
Individual leg                  
Trade Notification (Trade)

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a  ETI mes-sage layout.  Value:  10991 (Order-MassActionReport, MsgType = BZ) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  | characters |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  |  |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  |  |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  |  |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  |  |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  | generated Program |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  |  |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  | generated Program |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  | Order  Rout- |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  |  |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  |  |
|  | 708 TerminalInfo Y 8 24 unsigned int Total 15 Characters st For 1 - 12 value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member Value Description 11111111111 IBT 22222222222 DMA 33333333333 Wireless Technology Order not 0 through trading software Order 1 through trading software Smart 2 ing without Program trading software Smart Order Routing 3 with program trading software 11 ClOrdID N 8 32 unsigned int Client OrderID: Unique defined order request identifier; used |  |  |  |  |  |  | participant |
|  |  |  |  |  |  | for client order id chaining. Strategy Approved by the exchange should be used.Range being from 0 to |  |  |
|  | 727 StrategyID | Y | 8 | 40 | unsigned int | for client order id chaining. Strategy Approved by the exchange should be used.Range being from 0 to |  |  |
|  | 728 StrategySequence- Number | Y | 8 | 48 | unsigned int | Strategy Sequence numbers |  |  |
|  | 714 AccountType | Y | 1 | 56 | unsigned int |  |  |  |
|  |  | Y |  |  | unsigned int | Value Description 1 Own 3 Client 5 Institution |  |  |
|  |  | Y |  |  | unsigned int | Value Description 1 Own 3 Client 5 Institution |  |  |
|  |  | Y |  |  | unsigned int | Value Description 1 Own 3 Client 5 Institution |  |  |
|  |  | Y |  |  | unsigned int | Value Description 1 Own 3 Client 5 Institution |  |  |
|  | 769 NoOfMultiLeg | Y | 1 | 57 | Counter | Number of MultiLegOrdGrp repeating group instances. |  |  |

---

| 715 | SMPFOrderIdentifier | Y | 1 | 58 | unsigned int | 0-Passive1-Active |
| --- | --- | --- | --- | --- | --- | --- |
| 770 | MultiLegType | Y | 1 | 59 | unsigned int | Spread – 0, Multi Leg Order – 1 |
| 768 | AllOrNoneFlag | N | 1 | 60 | char | N- Full or parial execution only |
| 28703 | ApplSeqIndictaor | Y | 1 | 61 | unsigned int | 0- No Recovery Required1- Standard Order |
| 18 | ExecInst | Y | 1 | 62 | unsigned int | 1-Persistent Order(H)2-Non-Persistent Order(Q) |
| 25007 | FreeText1 | Y | 12 | 63 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 717 | CPCode | N | 12 | 75 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |

---

| Tag | Field Name | Req’d | Len | Ofs  Data Type |  | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 718 | UserReferenceText | N | 20 | 87 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide additionalregulatoryinformation(according to respectiverules andregs, circulars and/orbilateral  coordination  betweenparticipantand Trading SurveillanceOffice).Valid characters:\x20, \x22-\x7B, \x7D, \x7E. |
| 25009 | FreeText3 | N | 12 | 107 | Fixed String | Free-format text field fortraderspecific or customer-relatedcomments.Validcharacters:\x00,\x21,\x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 39030 | Pad1 | N | 1 | 119 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality: 0-254,Record counter:NoOfMultiLeg |
| 743 | >InstrumentIdentifier | Y | 8 | 120 | unsigned int | The U/L Asset / producer Identifieris specified by Exchange for permittedU/L Asset / products for trading |
| 44 | >Price | N | 8 | 128 | PriceType | Limit price. Required if OrdType (40)is Limit (2) or Stop Limit (4). |
| 731 | >MaxPricePercentage | N | 8 | 136 | PriceType | Price per unit of quantity (e.g.pershare) In Case of Limit Price, NoValue is required to be send. |
| 711 | >Echo | Y | 4 | 144 | signed int | Vendors can use this as a reference inresponse from exchange |
| 1300 | >MarketSegmentID | N | 4 | 148 | signed int | Product identifier. |
| 38 | >OrderQty | Y | 8 | 152 | Qty | Total Order Quantity. |
| 1227 | >ProductComplex | N | 1 | 160 | unsigned int | This field qualifies an instrument typeon MCX. |
| 54 | >Side | Y | 1 | 161 | unsigned int | Side of the order.1-Buy2-sell |

---

| Tag | Field Name | Req’d Len |  | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 40 | >OrdType | Y | 1 | 162 | unsigned int | Order type.2-limit.5-Market to limit. |
| 39050 | >Pad5 | N | 5 | 163 | Fixed String | Not Used. |

## 5.3.5 Immediate Execution Response-10993

This message informs about the immediate execution of an incoming multileg order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type |  | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int |  | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int |  | Unique identifier for a MCX ETI mes-sage layout.  Value:  10993 (Order-MassActionReport, MsgType = BZ) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String |  | not used |  |
| <R | esponseHeaderME> |  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp |  | Time when request was considered forprocessing |  |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp |  | Not used. |  |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp |  | Not used. |  |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp |  | Not used |  |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp |  | Not used. |  |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp |  | Gateway response out timestamp. |  |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int |  | Message sequence number used by theparticipant for requests sent to thegateway. |  |
| 5948 | PartitionID | Y | 2 | 60 | unsigned int |  | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |  |
| 1180 | ApplID | Y | 1 | 62 | unsigned int |  | Identifier for an ETI data stream. |  |
| 1180 | ApplID | Y | 1 | 62 | unsigned int |  | Description |  |
| 1180 | ApplID | Y | 1 | 62 | unsigned int |  | Session Data |  |
| 1180 | ApplID | Y | 1 | 62 | unsigned int |  |  |  |
| 28704 | ApplMsgID | Y | 16 | 63 | data |  | Application  message  identifier  as-signed to an order or quote event. |  |
| 893 | LastFragment | Y | 1 | 79 | unsigned int |  | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |  |
| 893 | LastFragment | Y | 1 | 79 | unsigned int |  | Description |  |
| 893 | LastFragment | Y | 1 | 79 | unsigned int |  |  | 0 Not Last Message |

---

|  |  |  |  |  |  | 1 | Last Message |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |
| 11 | ClOrdID | N | 8 | 80 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining. |  |

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 17 | ExecID | Y | 8 | 88 | UTCTimestamp | Transaction timestamp. |
| 727 | StrategyID | Y | 8 | 96 | unsigned int | Strategy Approved by the exchangeshould be used.Range being from 0 to99999 |
| 728 | StrategySequence-Number | Y | 8 | 104 | unsigned int | Strategy Sequence numbers |
| 30555 | NoLegExecs | Y | 2 | 112 | Counter | Number of InstrmntLegExec repeat-ing group instances.Applicable only incase of spread instruments. |
| 769 | NoOfMultiLeg | Y | 1 | 114 | Counter | Number of MultiLegExecGrp repeat-ing group instances |
| 772 | NoOfMultiLegExecs | Y | 1 | 115 | Counter | Number of MultiLegFillGrp repeatinggroup instances. |
| 770 | MultiLegType | Y | 1 | 116 | unsigned int | Spread – 0, Multi Leg Order – 1 |
| 39030 | Pad3 | N | 3 | 117 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality:  0-254, Record counter:NoOfMultiLeg |
| 37 | >OrderID | Y | 8 | 120 | unsigned int | Exchange Order ID generated by theMCX System;  it remains constantover the lifetime of an order. |
| 743 > | InstrumentIdentifier | Y | 8 | 128 | unsigned int | The U/L Asset / producer Identifieris specified by Exchange for permittedU/L Asset / products for trading |
| 14 | >CumQty | Y | 8 | 136 | Qty | Cumulated executed quantity of an or-der. |
| 84 | >CxlQty | Y | 8 | 144 | Qty | Total quantity cancelled for this order. |
| 378 | >ExecRestatement-Reason | Y | 2 | 152 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.Valid values are listed after this table. |

---

|  | Tag Field Name 39 &gt;OrdStatus | Req’d Y | Len | Ofs 128 | Data Type char | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 39 &gt;OrdStatus | Req’d Y | 1 | Ofs 128 | Data Type char | Conveys the current status of an or- der. Value Description 0 New 1 Partially filled 2 Filled 4 Cancelled |
|  | Tag Field Name 39 &gt;OrdStatus | Req’d Y | 1 | Ofs 128 | Data Type char | Conveys the current status of an or- der. Value Description 0 New 1 Partially filled 2 Filled 4 Cancelled |
|  | Tag Field Name 39 &gt;OrdStatus | Req’d Y | 1 | Ofs 128 | Data Type char | Conveys the current status of an or- der. Value Description 0 New 1 Partially filled 2 Filled 4 Cancelled |
|  | Tag Field Name 39 &gt;OrdStatus | Req’d Y | 1 | Ofs 128 | Data Type char | Conveys the current status of an or- der. Value Description 0 New 1 Partially filled 2 Filled 4 Cancelled |
|  | Tag Field Name 39 &gt;OrdStatus | Req’d Y | 1 | Ofs 128 | Data Type char | Conveys the current status of an or- der. Value Description 0 New 1 Partially filled 2 Filled 4 Cancelled |
|  | Tag Field Name 39 &gt;OrdStatus | Req’d Y | 1 | Ofs 128 | Data Type char | Conveys the current status of an or- der. Value Description 0 New 1 Partially filled 2 Filled 4 Cancelled |
|  | 150 &gt;ExecType | Y | 1 | 155 | char | The reason why this message was gen- erated. |
|  | 150 &gt;ExecType |  | 1 |  |  | Value Description |
|  | 150 &gt;ExecType |  | 1 |  |  | 4 Cancelled |
|  | 150 &gt;ExecType |  | 1 |  |  | F Trade |
|  | 39040 &gt;Pad4 | N | 4 | 156 | Fixed String | Not Used. |
|  | &lt;InstrmntLegExecGrp&gt; |  |  |  |  | Cardinality: 0-600, Record counter: NoLegExecs |
|  | &lt;InstrmntLegExecGrp&gt; | Y | 8 | 0 | signed int | Instrument identifier of the leg secu- |
|  |  |  |  |  |  | rity. |
|  | 637 &gt;LegLastPx | Y | 8 | 8 | PriceType | Price of this leg fill. |
|  | 1418 &gt;LegLastQty | Y | 8 | 16 | Qty | Quantity executed in this leg fill. |
|  | 1893 &gt;LegExecID | Y | 4 | 24 | signed int | Private identifier of a leg match step, which can be reconciled with the field SideTradeID (1506) in the Trade No- tification. |
|  | 624 &gt;LegSide | Y | 1 | 28 | unsigned int | The side of the individual leg of a strategy as defined in its signature. |
|  | 624 &gt;LegSide | Y |  | 28 |  | Value Description |
|  | 624 &gt;LegSide | Y |  | 28 |  | 1 Buy |
|  | 624 &gt;LegSide | Y |  | 28 |  | 2 Sell |
|  | 2421 &gt;FillRefID | Y | 1 | 29 | unsigned int | Reference to the corresponding Fills- Grp repeating group instance. |
|  | 39020 &gt;Pad2 | N | 2 | 30 | Fixed String | Not Used. |

Valid Values of ExecRestatementReason (datatype ExecRestatementReason)

| Valid |  |  |  |  |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Value |  |  |  |  |  |  |  |  |
| 1 | Order book restatement |  |  |  |  |  |  |  |
| 101 | Order add accepted |  |  |  |  |  |  |  |
| 102 | Order modify accepted |  |  |  |  |  |  |  |
| 103 | Order delete accepted |  |  |  |  |  |  |  |
| 105 | IOC Order Cancelled |  |  |  |  |  |  |  |

---

| 108 | Book Order executed |  |  |
| --- | --- | --- | --- |
| 114 | Order has been changed to IOC |  |  |
| 135 | Market Order triggered and executed |  |  |
| 145 | Start Of Day Processing |  |  |
| 146 | End Of Day Processing |  |  |
| 155 | Order Refreshed |  |  |
| 172 | Stop Order has been triggered |  |  |
| 215 | Risk Reduction Timer Expired |  |  |
| 217 | Tick Size Change |  |  |
| 248 | Active order deletion due to SMPF |  |  |
| 250 | Active order modification due to SMPF |  |  |
| 252 | Passive order deletion due to SMPF |  |  |
| 254 | Passive oder modification due to SMPF |  |  |
| 261 | Panic Cancel |  |  |
| 302 | RRMIN |  |  |
| 303 | SQUAREOFFIN |  |  |
| 357 | Base Price Update |  |  |
| 358 | Order Deleted As PriceMoved Out Of DPL Range |  |  |

## 5.3.6 Extended Order Information-10994

This message format is used for re-transmission of multi-leg execution response, within the session 
data.

| Tag Field Name         Req’d Len Ofs |  |  |  |  | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout. Value: 10994 (Order-MassActionReport, MsgType = BZ) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <RB | CHeaderME> |  |  |  |  |  |
| 21003 | Reserve2 | N | 8 | 8 | UTCTimestamp | Not Used. |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 28704 | ApplMsgID | Y | 16 | 30 | data | Application message identifier as-signed to an order or quote event. |

---

| 1180 | ApplID | Y | 1 | 46 | unsigned int | Identifier for an ETI data stream. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Value 4 5 | Description Session Data Listener Data |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Value 4 5 | Description Session Data Listener Data |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Value 4 5 | Description Session Data Listener Data |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Value 4 5 | Description Session Data Listener Data |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | Indicates a retransmission message. |  |
| 1352 |  | Y | 1 | 47 | unsigned int | Value | Description |
| 1352 |  | Y | 1 | 47 | unsigned int | 0 | False |
| 1352 |  | Y | 1 | 47 | unsigned int | 1 | True |
| 893 | LastFragment | Y | 1 | 48 | unsigned int |  | Indicates whether this message is the |
|  | LastFragment | Y | 7 | 48 | unsigned int Fixed String | transaction. | last fragment (part) of a sequence of messages belonging to one dedicated |
|  | LastFragment | Y | 7 | 48 | unsigned int Fixed String | 0 | Not Last Message |
|  | LastFragment | Y | 7 | 48 | unsigned int Fixed String | 1 | Last Message |
|  | LastFragment | Y | 7 | 48 | unsigned int Fixed String |  |  |
| 39070 Pad7 |  | N | 7 | 49 | unsigned int Fixed String | Not Used. |  |

---

| Tag | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | 12 characters value(11111111111-33333333333), character value(0-3) digit should be valid vendor code / in House CTCL code |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | Description |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | IBT |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | DMA |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | Wireless Technology |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | Order not generated through Program trading software |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | Order generated through Program trading software |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | Smart Order Rout- ing without Program trading software |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | Smart Order Routing with program trading software |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  |  |
| 708 11 | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | Client Order ID: Unique participant |
|  | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  | defined order request identifier; used |
|  | Req’d Len Ofs Data Type Description Y 8 56 unsigned int Total 15 Characters st For 1 th For 13 th th 14 – 15 for member. Value 11111111111 22222222222 33333333333 N 8 64 unsigned int |  |  |  |  |  |  |  |
|  | ClOrdID |  |  |  |  |  | for client order id chaining. |  |
| 17 |  | Y | 8 | 72 | UTCTimestamp |  | Transaction timestamp. |  |
| 727 | StrategyID | Y | 8 | 80 | unsigned int |  | Strategy Approved by the exchange should be used.Range being from 0 to 99999 |  |
| 727 | StrategySequence- |  |  |  |  |  | Strategy Approved by the exchange should be used.Range being from 0 to 99999 |  |
| 727 |  |  |  |  |  |  | Strategy Approved by the exchange should be used.Range being from 0 to 99999 |  |
| 728 | Number | Y | 8 | 88 | unsigned int | Strategy Sequence numbers |  |  |
| 30555 | NoLegExecs | Y | 2 | 96 | Counter | Strategy Sequence numbers |  |  |
|  |  |  |  |  |  |  |  | Number of InstrmntLegExec repeat- |
|  |  |  |  |  |  |  | ing group instances. |  |
| 714 | AccountType | Y | 1 | 98 | unsigned int |  |  |  |
|  |  |  |  |  | Counter | Value | Description 1 Own 3 Client 5 Institution |  |
|  |  |  |  |  | Counter |  | Description 1 Own 3 Client 5 Institution |  |
|  |  |  |  |  | Counter |  | Description 1 Own 3 Client 5 Institution |  |
|  |  |  |  |  | Counter |  | Description 1 Own 3 Client 5 Institution |  |
|  |  |  |  |  | Counter |  | Description 1 Own 3 Client 5 Institution |  |
|  |  |  |  |  | Counter |  | Description 1 Own 3 Client 5 Institution |  |
| 769 | NoOfMultiLeg | Y | 1 | 99 |  |  | Number of MultiLegExecGrp repeat- ing group instances. |  |

---

| Tag | Field Name       Req’d |  | Len Ofs |  | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 772 | NoOfMultiLegExecs | Y | 1 | 100 | Counter | Number of MultiLegFillGrp repeatinggroup instances. |
| 25007 | FreeText1 | Y | 12 | 101 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 718 | UserReferenceText | N | 20 | 113 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide additional regulatoryinformation(according to respectiverules andregs, circulars and/orbilateral  coordination  betweenparticipantand Trading SurveillanceOffice).Valid characters:\x20, \x22-\x7B, \x7D, \x7E. |
| 717 | CPCode | N | 12 | 133 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 770 | MultiLegType | Y | 1 | 145 | unsigned int | Spread – 0, Multi Leg Order – 1 |
| 25009 | FreeText3 | N | 12 | 146 | Fixed String | Free-format text field fortraderspecific or customer-relatedcomments.Validcharacters:\x00,\x21,\x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 39020 | Pad2 | N | 2 | 158 | Fixed String | Not Used. |
|  |  |  |  |  |  | Cardinality:0-254, Record counter:NoOfMultiLeg |

---

|  | Tag Field Name &gt;InstrumentIdentifier | Req’d Y | Len 8 | Ofs 160 168 | Data Type unsigned int | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 743 | Tag Field Name &gt;InstrumentIdentifier | Req’d Y | Len 8 | Ofs 160 168 | Data Type unsigned int | Description |
|  | 37 &gt;OrderID | Y | 8 |  | unsigned int | Exchange Order ID generated by the T7 System; it remains constant over the lifetime of an order. |
|  | 44 &gt;Price | N | 8 | 176 | PriceType | Limit price. Required if OrdType (40) is Limit (2) or Stop Limit (4). |
|  | 731 &gt;MaxPricePercentage | N | 8 | 184 | PriceType | Price per unit of quantity (e.g.  per share) |
|  | 711 &gt;Echo | Y | 4 | 192 | signed int |  |
|  | 39040 &gt;Pad4 | U | 4 | 196 | Fixed String | not used |
|  | 38 &gt;OrderQty | Y | 8 | 200 | Qty | Total Order Quantity. Cumulated executed quantity of an or- |
|  | 14 &gt;CumQty | Y | 8 | 208 | Qty | Total Order Quantity. Cumulated executed quantity of an or- |
|  | 84 &gt;CxlQty | Y Y | 8 | 216 | Qty | Total quantity cancelled for this order. |
| 378 | &gt;ExecRestatement- Reason | Y Y | 2 | 224 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. Value Description 101 Order add accepted |
| 378 | &gt;ExecRestatement- Reason | Y Y | 2 | 224 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. Value Description 101 Order add accepted |
| 378 | &gt;ExecRestatement- Reason | Y Y | 2 | 224 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. Value Description 101 Order add accepted |
| 378 | &gt;ExecRestatement- Reason | Y Y | 2 | 224 | unsigned int | 105 IOC Order Cancelled |
| 378 | &gt;ExecRestatement- Reason | Y Y | 2 | 224 | unsigned int | 201 Self Trade Order Deleted |
|  | 39 &gt;OrdStatus | Y | 1 | 226 | char | Conveys the current status of an or- der. Value Description 4 Cancelled |
|  |  | Y | 1 | 226 | char | Conveys the current status of an or- der. Value Description 4 Cancelled |
|  |  | Y | 1 | 226 | char | Conveys the current status of an or- der. Value Description 4 Cancelled |
|  |  | Y | 1 | 226 | char | Conveys the current status of an or- der. Value Description 4 Cancelled |
|  |  | Y | 1 | 226 | char | 2 Filled |
|  | 150 &gt;ExecType | Y | 1 | 227 | char | The reason why this message was gen- erated. Value Description |
|  |  |  | 1 |  | char | The reason why this message was gen- erated. Value Description |
|  |  |  | 1 |  | char | The reason why this message was gen- erated. Value Description |
|  |  |  | 1 |  | char | 4 Cancelled |
|  |  |  | 1 |  | char | F Trade |
|  | 54 &gt;Side | Y | 1 | 228 | unsigned int | Side of the order. |
|  | 54 &gt;Side |  | 1 | 228 | unsigned int | Value Description |
|  | 54 &gt;Side |  | 1 | 228 | unsigned int | 1 Buy |
|  | 54 &gt;Side |  | 1 | 228 | unsigned int | 2 Sell |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 40 | >OrdType | Y | 1 | 229 | unsigned int | Order type. |
| 40 | >OrdType | Y | 1 | 229 | unsigned int | Description |
| 40 | >OrdType | Y | 1 | 229 | unsigned int | Limit |
| 40 | >OrdType | Y | 1 | 229 | unsigned int | Market To Limit |
| 39020 | >Pad2 | N | 2 | 230 | Fixed String | Not used. |
|  |  |  |  |  |  | Cardinality:0-254, Record counter:NoOfMultiLegExecs |
| 1364 > | FillPx | Y | 8 | 0 | PriceType | Price of Fill. |
| 48 > | SecurityID | Y | 8 | 8 | signed int | Instrument identifier. |
| 1365 > | FillQty | Y | 8 | 16 | Qty | Quantity of Fill. |
| 28708 > | FillMatchID | Y | 4 | 24 | unsigned int | Unique identifier for each price level(match step) of a match event; it isused for public trade reporting. |
| 1363 > | FillExecID | Y | 4 | 28 | signed int | Private identifier of an order matchstep event, which can be reconciledwith the field SideTradeID (1506) inthe Trade Notification. |
| <Instrm | ntLegExecGrp> |  |  |  |  | Cardinality:0-600, Record counter:NoLegExecs |
| 602 > | LegSecurityID | Y | 8 | 0 | signed int | Instrument identifier of the leg secu-rity. |
| 637 | >LegLastPx | Y | 8 | 8 | PriceType | Price of this leg fill. |
| 1418 | >LegLastQty | Y | 8 | 16 | Qty | Quantity executed in this leg fill. |
| 1893 | >LegExecID | Y | 4 | 24 | signed int | Private identifier of a leg match step,which can be reconciled with the fieldSideTradeID (1506) in the Trade No-tification. |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | The side of the individual leg of astrategy as defined in its signature. |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Description |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Buy |
| 624 | >LegSide | Y | 1 | 28 | unsigned int | Sell |
| 2421 > | FillRefID | Y | 1 | 29 | unsigned int | Reference to the corresponding Fills-Grp repeating group instance. |
| 39020 | >Pad2 | N | 2 | 30 | Fixed String | Data structure padding (2 bytes).Valid characters: \x01-\x7E |

---

## Valid Values of ExecRestatementReason (datatype ExecRestatementReason)

| Valid |  |  |  |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Value |  |  |  |  |  |  |  |
| 1 | Order book restatement |  |  |  |  |  |  |
| 101 | Order add accepted |  |  |  |  |  |  |
| 102 | Order modify accepted |  |  |  |  |  |  |
| 103 | Order delete accepted |  |  |  |  |  |  |
| 105 | IOC Order Cancelled |  |  |  |  |  |  |
| 108 | Book Order executed |  |  |  |  |  |  |
| 114 | Order has been changed to IOC |  |  |  |  |  |  |
| 135 | Market Order triggered and executed |  |  |  |  |  |  |
| 145 | Start Of Day Processing |  |  |  |  |  |  |
| 146 | End Of Day Processing |  |  |  |  |  |  |
| 155 | Order Refreshed |  |  |  |  |  |  |
| 172 | Stop Order has been triggered |  |  |  |  |  |  |
| 215 | Risk Reduction Timer Expired |  |  |  |  |  |  |
| 217 | Tick Size Change |  |  |  |  |  |  |
| 248 | Active order deletion due to SMPF |  |  |  |  |  |  |
| 250 | Active order modification due to SMPF |  |  |  |  |  |  |
| 252 | Passive order deletion due to SMPF |  |  |  |  |  |  |
| 254 | Passive order modification due toSMPF |  |  |  |  |  |  |
| 261 | Panic Cancel |  |  |  |  |  |  |
| 302 | RRMIN |  |  |  |  |  |  |
| 303 | SQUAREOFFIN |  |  |  |  |  |  |
| 357 | Base Price Update |  |  |  |  |  |  |
| 358 | Order Deleted As PriceMoved Out Of DPL Range |  |  |  |  |  |  |

---

**5.3.7 Reject Multileg-10992**

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <Me | ssageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a  ETI mes-sage layout. Value: 10992 (MultiLeg-OrderAcknowledgement, MsgType =U29) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <NRR | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | In timestamp;  filled always by thegateway |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp |  |
| 21002 | Reserve1 | N | 8 | 24 | UTCTimestamp | Matching engine in timestamp. |
| 21003 | Reserve2 | N | 8 | 32 | UTCTimestamp | Matching engine out timestamp. |
| 7765 | Reserve3 | N | 8 | 40 | UTCTimestamp | Timestamp the gateway receives amessage from the Matching Engine |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Outgoing delta timestamp; filled al-ways by the gateway |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | ValueDescription |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 1 Last Message |
| 39030 | Pad3 | N | 3 | 61 | Fixed String | Not used. |
| 48 | Body>SecurityID |  |  |  |  |  |
| 48 | Body>SecurityID | N | 8 | 64 | signed int | Instrument identifier. |
| 373 | SessionRejectReason | Y | 4 | 72 | unsigned int | Error code.Valid values are listed after this table. |
| 30354 | VarTextLen | Y | 2 | 76 | Counter | Values will use user-defined values for appli-cation level errors as well |

---

| Tag | Field Name 1409 SessionStatus | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| Tag | Field Name 1409 SessionStatus | Y | 1 | 78 | unsigned int | Session status. |
| Tag | Field Name 1409 SessionStatus | Y | 1 | 78 |  | Value Description |
| Tag | Field Name 1409 SessionStatus | Y | 1 | 78 |  | 0 Session active |
| Tag | Field Name 1409 SessionStatus | Y | 1 | 78 |  | 4 Session logout complete |
| 39000 Pad1 |  | N | 1 | 79 | Fixed String | Not used. |
|  | 30355 VarText | Y | 2000 | 80 | Variable String | Error text. Valid characters: \x09, \x0A, \x0D, \x20-\ x7B, \x7D, \x7E |

Valid Values of SessionRejectReason (datatype SessionRejectReason)

| Valid |  |  |  |  | Descrip | tion |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Value |  |  |  |  |  |  |  |  |
| 1 | Required Tag Missing |  |  |  |  |  |  |  |
| 5 | Value is incorrect (out of range) for this tag |  |  |  |  |  |  |  |
| 7 | Decryption problem |  |  |  |  |  |  |  |
| 11 | Invalid TemplateID |  |  |  |  |  |  |  |
| 16 | Incorrect NumInGroup count for repeating group |  |  |  |  |  |  |  |
| 99 | Other |  |  |  |  |  |  |  |
| 100 | Throttle limit exceeded |  |  |  |  |  |  |  |
| 101 | Stale request was not forwarded to T7 |  |  |  |  |  |  |  |
| 102 | Service temporarily not available |  |  |  |  |  |  |  |
| 103 | Service not available |  |  |  |  |  |  |  |
| 104 | Result Of Transaction Unknown |  |  |  |  |  |  |  |
| 105 | Error converting response or broadcast |  |  |  |  |  |  |  |
| 152 | Heartbeat violation error |  |  |  |  |  |  |  |
| 200 | Internal technical error |  |  |  |  |  |  |  |
| 210 | Validation Error |  |  |  |  |  |  |  |
| 211 | User already logged in |  |  |  |  |  |  |  |
| 10000 | Order not found |  |  |  |  |  |  |  |
| 10001 | Price not reasonable |  |  |  |  |  |  |  |
| 10004 | BU Book Order Limit Ex-ceeded |  |  |  |  |  |  |  |
| 10005 | Session Book Order Limit Exceeded |  |  |  |  |  |  |  |
| 10006 | LstUpdate Timestamp Not Matched |  |  |  |  |  |  |  |

---

## 5.3.8 Cancel Order Notification-10112

This message informs about an unsolicited cancellation of a single order.

| Tag | Req’d Len Ofs Field Name |  |  |  | Data Type |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- |
| &lt;MessageHeaderOut&gt; |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in- cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a message layout. Value: (ExecutionReport, MsgType = 8) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
| &lt;RBCHeaderME&gt; |  |  |  |  |  |  |  |
| 21003 | Reserve2 | N | 8 | 8 | UTCTimestamp | Matching engine out timestamp. |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp |  | Outgoing timestamp; filled always by the gateway |
| 28727 | ApplSubID | N | 4 | 24 | unsigned int | Unique ID assigned by the MCX system during broadcast subscription in order  to link broadcasts to the |  |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | scription. Grouping of MCX products. Belongs to the scope of Service Avail- |  |
| 28704 | ApplMsgID | N | 16 | 30 | data |  | Application message identifier as- |
|  | ApplID |  | 1 |  |  | signed to an order or quote event. Identifier for an ETI data stream. |  |
| 1180 | ApplID | N | 1 | 46 | unsigned int | signed to an order or quote event. Identifier for an ETI data stream. |  |
|  | ApplID |  | 1 | 46 | unsigned int | Value Description |  |
|  | ApplID | Y | 1 | 46 | unsigned int | 4 | Session Data |
| 1352 | ApplID | Y | 1 | 46 | unsigned int | 5 Listener Data Indicates a retransmission message. |  |
| 1352 | ApplResendFlag | Y |  |  |  |  |  |
| 1352 | ApplResendFlag | Y | 1 48 |  |  | Value | Description |
| 1352 | ApplResendFlag |  | 1 48 |  |  | 0 False 1 True |  |
| 893 | LastFragment | Y | 1 48 |  | unsigned int |  |  |
|  |  | Y |  |  |  | last fragment (part) of a sequence of messages belonging to one dedicated transaction. Value Description |  |
|  |  | Y |  |  |  | 1 Last Message |  |
|  |  |  |  |  |  |  |  |
| 39070 | Pad7 | N | 7 | 49 | Fixed String | Not used |  |

---

| TagField Name      Req’dLen Ofs Data Type DescriptionExchange Order ID generated by theMCX System; it remains constant overthe lifetime of an order. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TagField Name      Req’dLen Ofs Data Type DescriptionExchange Order ID generated by theMCX System; it remains constant overthe lifetime of an order. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 11 | ClOrdID | N | 8 | 64 | unsigned int | Client Order ID: Unique participantdefined order request identifier; usedfor client order id chaining. |  |  |  |  |  |  |  |  |
| 41 | OrigClOrdID | N | 8 | 72 | unsigned int | ClOrdID (11) of the last successfullyprocessed task (request) referring to thespecific order. |  |  |  |  |  |  |  |  |
| 48 | SecurityID | Y | 8 | 80 | signed int | Instrument identifier. |  |  |  |  |  |  |  |  |
| 17 | ExecID | Y | 8 | 88 | UTCTimestamp | Transaction timestamp. |  |  |  |  |  |  |  |  |
| 14 | CumQty | Y | 8 | 96 | Qty | Cumulated executed quantity of an or-der. |  |  |  |  |  |  |  |  |
| 84 | CxlQty | Y | 8 | 104 | Qty | Total quantity cancelled for this order. |  |  |  |  |  |  |  |  |
| 711 | Echo | Y | 4 | 112 | signed int |  |  |  |  |  |  |  |  |  |
| 1300 | MarketSegmentID | N | 4 | 116 | signed int | Product identifier. |  |  |  |  |  |  |  |  |
| 20036 | PartyIDEntering-Trader | N | 4 | 120 | unsigned int | Entering User ID. |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 378 | ExecRestatement-Reason | Y | 2 | 124 | unsigned int | Code to further qualify the field Exec-Type (150) of the Execution Report(8) message.103145146148197217 |  |  |  |  |  |  |  |  |
| 20007 | PartyIDEnteringFirm | N | 1 | 126 | unsigned int | Entering Entity ID. |  |  |  |  |  |  |  |  |
| 20007 | PartyIDEnteringFirm | N | 1 | 126 | unsigned int | Value | Description |  |  |  |  |  |  |  |
| 20007 | PartyIDEnteringFirm | N | 1 | 126 | unsigned int | 1 | Participant |  |  |  |  |  |  |  |
| 20007 | PartyIDEnteringFirm | N | 1 | 126 | unsigned int | 2 | Market Supervision |  |  |  |  |  |  |  |
| 39 | OrdStatus | Y | 1 | 127 | char | Conveys the current status of an or- der.Value |  |  |  |  |  |  |  |  |
| 39 | OrdStatus | Y | 1 | 127 | char | Conveys the current status of an or- der.Value |  |  |  |  |  |  |  |  |
| 39 | OrdStatus | Y | 1 | 127 | char | Conveys the current status of an or- der.Value |  |  |  |  |  |  |  |  |

---

| 150 | ExecType | Y | 1 |  | 128 | char | The reason why this message was gen- erated. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 150 | ExecType | Y | 1 |  | 128 | char | Value | Description |
| 150 | ExecType | Y | 1 |  | 128 |  | 4 | Cancelled |

| Tag | Field Name | Req’d | Len | Ofs | Data Type unsigned int | Description This field qualifies an instrument type on MCX. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1227 | ProductComplex | Y | 1 | 129 | Data Type unsigned int | Description This field qualifies an instrument type on MCX. |  |
|  |  |  |  |  | Data Type unsigned int | Value | Description |
|  |  |  |  |  | Data Type unsigned int | 1 | Simple instrument |
|  |  |  |  |  |  | 5 | Futures Spread |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |
| 54 | Side |  | 1 | 130 | unsigned int | Side of Value 1 2 | the order. Description Buy Sell |
| 39050 | Pad5 | N | 5 | 131 | Fixed String | Not used. |  |

---

## 5.4 Ex/Dex

## 5.4.1 Ex/Dex Entry Request

Exercise and don’t exercise instruction entry request.This is only applicable for options contracts.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8030(ExDexEntry Request, MsgType = U74) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 812 | Quantity | Y | 8 | 24 | unsigned int | Refer to Rules for Order Quantity |
| 30048 | SimpleSecurityID | Y | 4 | 32 | unsigned int | Instrument Identifier. |
| 711 | Echo | Y | 4 | 36 | signed int | Vendors can use this as a reference inresponse from exchange |
| 20655 T | argetPartyIDSession-ID | N | 4 | 40 | unsigned int | Session ID. |
| 813 | ExDExFlag | Y | 2 | 44 | unsigned int |  |
| 813 | ExDExFlag | Y | 2 | 44 | unsigned int | Description |
| 813 | ExDExFlag | Y | 2 | 44 | unsigned int | 1 Exercise Instruction |
| 813 | ExDExFlag | Y | 2 | 44 | unsigned int | 2 Dont_Exercise_Instruction |
| 714 | AccountType | Y | 1 | 46 | unsigned int |  |
| 714 | AccountType | Y | 1 | 46 | unsigned int | Description |
| 714 | AccountType | Y | 1 | 46 | unsigned int | Own |
| 714 | AccountType | Y | 1 | 46 | unsigned int | Client |
| 714 | AccountType | Y | 1 | 46 | unsigned int | 5 Institution |
| 54 | Side | Y | 1 | 47 | unsigned int | Side of the order. |
| 54 | Side | Y | 1 | 47 | unsigned int | Description |
| 54 | Side | Y | 1 | 47 | unsigned int | Buy |
| 54 | Side | Y | 1 | 47 | unsigned int | Sell |

---

| 25007 | FreeText1 | Y | 12 | 48 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| --- | --- | --- | --- | --- | --- | --- |
| 717 | CPCode | N | 12 | 60 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 718 | UserReferenceText | N | 20 | 72 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide additionalregulatoryinformation(according to respectiverules  andregs,  circulars  and/orbilateral   coordination   betweenparticipantand Trading SurveillanceOffice).Valid characters: \x20, \x22-\x7B, \x7D, \x7E. |
| 39040 | Pad4 | N | 4 | 92 | Fixed String | Not Used. |

---

## 5.4.2 Ex/Dex Entry Confirmation

This message confirms an Ex/Dex entry request. This message is used to replace an existing ex/dex 
order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8031(ExDexEntryConfirmation, MsgType =U75) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <NRR | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp | Not used. |
| 21002 | Reserve1 | N | 8 | 24 | UTCTimestamp | Not used. |
| 21003 | Reserve2 | N | 8 | 32 | UTCTimestamp | Not used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not used. |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Not Last Message |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Last Message |
| 39030 | Pad3 | N | 3 | 61 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 814 | ExDExOrderNumber | Y | 8 | 64 | unsigned int | Unique Number assigned by Exchange |
| 21009 | TrdRegTSEntryTime | N | 8 | 72 | UTCTimestamp | The entry timestamp is the time ofthe creation of the order. |
| 815 | LastUpdatedTime | Y | 8 | 80 | UTCTimestamp | Time at which request is confirmed bythe exchange in terms of seconds from01-01-1970 00:00:00 hours. |
| 812 | Quantity | Y | 8 | 88 | unsigned int | Refer to Rules for Order Quantity |
| 30048 | SimpleSecurityID | Y | 4 | 96 | unsigned int | Instrument Identifier. |
| 711 | Echo | Y | 4 | 100 | signed int | Vendors can use this as a reference inresponse from exchange |
| 20655 T | argetPartyIDSession-ID | N | 4 | 104 | unsigned int | Session ID. |

---

|  | Tag Field Name 813 ExDExFlag | Req’d Y | Len | Ofs | Data Type unsigned int | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 813 ExDExFlag | Req’d Y | 2 | 108 | Data Type unsigned int | Value Description 1 Exercise Instruction 2 Dont_Exercise_Instruction |
|  | 714 AccountType | Y | 1 | 110 | unsigned int | Value Description 1 Own 3 Client 5 Institution |
|  | 54 Side | Y | 1 | 111 | unsigned int | Side of the order. Value Description 1 Buy 2 Sell |
|  | 25007 FreeText1 | Y | 12 | 112 | Fixed String (0-terminable) | The Unique Client Code (UCC) of the person for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |
|  | 717 CPCode | N | 12 | 124 | Fixed String (0-terminable) | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
|  | 718 UserReferenceText | N | 20 | 136 | Fixed String (0-terminable) | Upto 15 Char allowed.This field is used to provide additional regulatory information(according  to respective rules  andregs,  circulars and/or bilateral coordination between participantand  Trading  Surveillance Office).Valid characters: \x20, \x22- \x7B, \x7D, \x7E. |
| 39040 Pad4 |  | N | 4 | 156 | Fixed String | Not Used. |

---

## 5.4.3 Ex/Dex Modification Request

This message is used to replace an existing Ex/Dex order.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8033(ExDexModificationRequest, MsgType = U77) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 812 | Quantity | Y | 8 | 24 | unsigned int | Refer to Rules for Order Quantity |
| 814 | ExDExOrderNumber | Y | 8 | 32 | unsigned int | Unique Number assigned by Exchange |
| 815 | LastUpdatedTime | Y | 8 | 40 | UTCTimestamp | Time at which request is confirmed bythe exchange in terms of seconds from01-01-1970 00:00:00 hours. |
| 30048 | SimpleSecurityID | Y | 4 | 48 | unsigned int | Instrument Identifier. |
| 711 | Echo | Y | 4 | 52 | signed int | Vendors can use this as a reference inresponse from exchange |
| 20655 T | argetPartyIDSession-ID | N | 4 | 56 | unsigned int | Session ID. |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int |  |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int | Description |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int | 1 Exercise Instruction |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int | 2 Dont_Exercise_Instruction |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int |  |
| 714 | AccountType | Y | 1 | 62 | unsigned int |  |
| 714 | AccountType | Y | 1 | 62 | unsigned int | Description |
| 714 | AccountType | Y | 1 | 62 | unsigned int | Own |
| 714 | AccountType | Y | 1 | 62 | unsigned int | Client |
| 714 | AccountType | Y | 1 | 62 | unsigned int | 5 Institution |

---

|  | Tag Field Name | Req’d Y | Len 1 | Ofs 63 | Data Type unsigned int | Description Side of the order. Value Description 1 Buy 2 Sell |
| --- | --- | --- | --- | --- | --- | --- |
|  | 54 Side | Req’d Y | Len 1 | Ofs 63 | Data Type unsigned int | Description Side of the order. Value Description 1 Buy 2 Sell |
|  | 25007 FreeText1 | Y | 12 | 64 | Fixed String (0-terminable) | The Unique Client Code (UCC) of the person for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |
|  | 717 CPCode | N | 12 | 88 | Fixed String (0-terminable) | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
|  | 718 UserReferenceText | N | 20 | 108 | Fixed String (0-terminable) | Upto 15 Char allowed.This field is used to provide additional regulatory information(according  to respective rules  andregs,  circulars and/or bilateral coordination between participantand  Trading  Surveillance Office).Valid characters: \x20, \x22- \x7B, \x7D, \x7E. |
| 39040 | Pad4 | N | 4 | 108 | Fixed String | Not used. |

---

## 5.4.4 Ex/Dex Modification Confirmation

This message confirms a Replace Ex/Dex modification request.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8034(ExDexModificationConfirmation, MsgType = U80) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <NRR | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp | Not Used. |
| 21002 | Reserve1 | N | 8 | 24 | UTCTimestamp | Not Used. |
| 21003 | Reserve2 | N | 8 | 32 | UTCTimestamp | Not Used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not Used. |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 0 Not Last Message |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 1 Last Message |
| 39030 | Pad3 | N | 3 | 61 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 812 | Quantity | Y | 8 | 64 | unsigned int | Refer to Rules for Order Quantity |
| 814 | ExDExOrderNumber | Y | 8 | 72 | unsigned int | Unique Number assigned by Exchange |
| 815 | LastUpdatedTime | Y | 8 | 80 | UTCTimestamp | Time at which request is confirmed bythe exchange in terms of seconds from01-01-1970 00:00:00 hours. |
| 711 | Echo | Y | 4 | 88 | signed int | Vendors can use this as a reference inresponse from exchange |
| 20655 T | argetPartyIDSession-ID | N | 4 | 92 | unsigned int | Session ID. |
| 30048 | SimpleSecurityID | Y | 4 | 96 | unsigned int | Instrument Identifier. |

---

|  | Tag Field Name 813 ExDExFlag | Req’d Y | Len 2 | Ofs 100 | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 813 ExDExFlag | Req’d Y | Len 2 | Ofs 100 | unsigned int | Value Description 1 Exercise Instruction 2 Dont_Exercise_Instruction |
|  | 714 AccountType | Y | 1 | 102 | unsigned int | Value Description 1 Own 3 Client 5 Institution |
|  | 54 Side | Y | 1 | 103 | unsigned int | Side of the order. Value Description 1 Buy 2 Sell |
|  | 25007 FreeText1 | Y | 12 | 104 | Fixed String (0-terminable) | The Unique Client Code (UCC) of the person for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |
|  | 717 CPCode | N | 12 | 116 | Fixed String (0-terminable) | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
|  | 718 UserReferenceText | N | 20 | 128 | Fixed String (0-terminable) | Upto 15 Char allowed.This field is used to provide additional regulatory information(according  to respective rules  andregs,  circulars and/or bilateral coordination between participantand  Trading  Surveillance Office).Valid characters: \x20, \x22- |
| 39040 | Pad4 | N | 4 | 148 | Fixed String | \x7B, \x7D, \x7E. Not used. |

---

## 5.4.5 Ex/Dex Cancellation Request

This message is used to Cancel an Ex/Dex Order

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8036(ExDexCancellationRequest, MsgType = U76) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 814 | ExDExOrderNumber | Y | 8 | 24 | unsigned int | Unique Number assigned by Exchange |
| 815 | LastUpdatedTime | Y | 8 | 32 | UTCTimestamp | Time at which request is confirmed bythe exchange in terms of seconds from01-01-1970 00:00:00 hours. |
| 812 | Quantity | Y | 8 | 40 | unsigned int | As sent with the Ex/Dex Entry Re-quest |
| 30048 | SimpleSecurityID | Y | 4 | 48 | unsigned int | Instrument Identifier. |
| 711 | Echo | Y | 4 | 52 | signed int | Vendors can use this as a reference inresponse from exchange |
| 20655 T | argetPartyIDSession-ID | N | 4 | 56 | unsigned int | Session ID. |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int |  |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int | Description |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int | 1 Exercise Instruction |
| 813 | ExDExFlag | Y | 2 | 60 | unsigned int | 2 Dont_Exercise_Instruction |
| 54 | Side | Y | 1 | 62 | unsigned int | Side of the order. |
| 54 | Side | Y | 1 | 62 | unsigned int | Description |
| 54 | Side | Y | 1 | 62 | unsigned int | Buy |
| 54 | Side | Y | 1 | 62 | unsigned int | Sell |

---

|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | Ofs 63 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | Ofs 63 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | Ofs 63 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | Ofs 63 | Data Type unsigned int | Description Value Description 1 Own 3 Client 5 Institution |
|  | 25007 FreeText1 | Y | 12 | 64 | Fixed String (0-terminable) | The Unique Client Code (UCC) of the person for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |
|  | 717 CPCode | N | 12 | 76 | Fixed String (0-terminable) | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
|  | 718 UserReferenceText | N | 20 | 88 | Fixed String (0-terminable) | Upto 15 Char allowed.This field is used to provide additional regulatory information(according  to respective rules  andregs,  circulars and/or bilateral coordination between participantand  Trading  Surveillance Office).Valid characters: \x20, \x22- \x7B, \x7D, \x7E. |
| 39040 Pad4 |  | N | 4 | 108 | Fixed String | Not Used. |

---

## 5.4.6 Ex/Dex Cancellation Confirmation

This message confirms the Ex/Dex cancelled order

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <Mes | sageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8037(ExDexCancellationConfirmation, MsgType = U81) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <NRRe | sponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | In timestamp;  filled always by thegateway |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp | Not used. |
| 21002 | Reserve1 | N | 8 | 24 | UTCTimestamp | Not used. |
| 21003 | Reserve2 | N | 8 | 32 | UTCTimestamp | Not used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not used. |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 0 Not Last Message |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 1 Last Message |
| 39030 | Pad3 | N | 3 | 61 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 812 | Quantity | Y | 8 | 64 | unsigned int | Refer to Rules for Order Quantity |
| 814 | ExDExOrderNumber | Y | 8 | 72 | unsigned int | Unique Number assigned by Exchange |
| 815 | LastUpdatedTime | Y | 8 | 80 | UTCTimestamp | Time at which request is confirmed bythe exchange in terms of seconds from01-01-1970 00:00:00 hours. |
| 30048 | SimpleSecurityID | Y | 4 | 88 | unsigned int | Instrument Identifier. |
| 711 | Echo | Y | 4 | 92 | signed int | Vendors can use this as a reference inresponse from exchange |
| 20655 T | argetPartyIDSession-ID | N | 4 | 96 | unsigned int | User ID for whom this request is place |

---

|  | Tag Field Name 813 ExDExFlag | Req’d Y | Len | Ofs 100 | Data Type unsigned int |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 813 ExDExFlag | Req’d Y | 2 | Ofs 100 | Data Type unsigned int |  | Value Description 1 Exercise Instruction 2 Dont_Exercise_Instruction |
|  | Tag Field Name 813 ExDExFlag | Req’d Y | 2 | Ofs 100 | Data Type unsigned int |  | Value Description 1 Exercise Instruction 2 Dont_Exercise_Instruction |
|  | Tag Field Name 813 ExDExFlag | Req’d Y | 2 | Ofs 100 | Data Type unsigned int |  | Value Description 1 Exercise Instruction 2 Dont_Exercise_Instruction |
|  | 714 AccountType | Y | 1 | 102 | unsigned int |  | Value Description 1 Own 3 Client 5 Institution |
|  | 54 Side | Y | 1 12 | 103 | unsigned int |  | Side of the order. Value Description 1 Buy 2 Sell |
|  |  |  | 1 12 |  |  |  |  |
|  | 25007 FreeText1 | Y |  | 104 | Fixed String (0-terminable) |  | The Unique Client Code (UCC) of the person for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |
|  | 717 CPCode | N | 12 | 116 | Fixed String (0-terminable) |  | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
|  | 718 UserReferenceText | N | 20 | 128 | Fixed String (0-terminable) |  | Upto 15 Char allowed.This field is used to provide additional regulatory information(according  to respective rules  andregs,  circulars and/or bilateral coordination between participantand  Trading  Surveillance Office).Valid characters: \x20, \x22- \x7B, \x7D, \x7E. |
| 39040 Pad4 |  | N | 4 | 148 | Fixed String |  | Not Used. |

---

## 5.4.7 Ex/Dex Notification

Ex/Dex Notification provide details for Ex/Dex orders

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <Me | ssageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8040(ExDexNotification, MsgType =U78) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | BCHeader> |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Application  sequence  number  as-signed to a non-order related MCXETI data stream. |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | 0 Trade Enhancement |
| 1180 | ApplID | Y | 1 | 31 | unsigned int |  |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Last Message |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 812 | Quantity | Y | 8 | 40 | unsigned int | Refer to Rules for Order Quantity |
| 814 | ExDExOrderNumber | Y | 8 | 48 | unsigned int | Unique Number assigned by Exchange |

---

|  | Tag Field Name 815 LastUpdatedTime | Req’d Y | Len 8 | Ofs 56 | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 815 LastUpdatedTime | Req’d Y | Len 8 | Ofs 56 | UTCTimestamp | Time at which request is confirmed by the exchange in terms of seconds from 01-01- 1970 00:00:00 hours. |
|  | 30048 SimpleSecurityID | Y | 4 | 64 | unsigned int | Instrument Identifier. |
|  | 711 Echo | Y | 4 | 68 | signed int | Vendors can use this as a reference in response from exchange |
|  | 20655 TargetPartyIDSession- ID | N | 4 | 72 | unsigned int | User ID for whom this request is place |
|  | 378 ExecRestatement- Reason | Y | 2 | 76 | unsigned int | Code to further qualify the field Exec- Type (150) of the Execution Report (8) message. Valid values are listed after this table. |
|  | 714 AccountType | Y | 1 | 78 | unsigned int | Value Description |
|  | 714 AccountType |  | 1 |  |  | 1 Own 3 Client 5 Institution |
|  | 54 Side | Y | 1 | 79 | unsigned int | Side of the order. Value Description 1 Buy 2 Sell |
|  | 816 ExDExstatus | Y | 2 | 80 | unsigned int | Value Description 1 Accepted 2 Cancelled |
|  | 813 ExDExFlag | Y | 2 | 82 | unsigned int | Value Description 1 Exercise Instruction 2 Dont_Exercise_Instruction |
|  |  |  |  |  |  |  |

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |

---

| 25007 | FreeText1 | Y | 12 | 84 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| --- | --- | --- | --- | --- | --- | --- |
| 717 | CPCode | N | 12 | 96 | Fixed String(0-terminable) | The Participant code.Valid characters:\x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D. |
| 718 | UserReferenceText | N | 20 | 108 | Fixed String(0-terminable) | Upto 15 Char allowed.This field isused to provide additionalregulatory information(according  torespective rules  andregs,  circularsand/or bilateral coordination betweenparticipantand  Trading  SurveillanceOffice).Valid characters: \x20, \x22-\x7B, \x7D,\x7E. |

Valid Values of ExecRestatementReason (datatype ExecRestatementReason)

| ValidValue |  |  |  |  | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Order book restatement |  |  |  |  |  |  |
| 101 | Order add accepted |  |  |  |  |  |  |
| 102 | Order modify accepted |  |  |  |  |  |  |
| 103 | Order delete accepted |  |  |  |  |  |  |
| 105 | IOC Order Cancelled |  |  |  |  |  |  |
| 108 | Book Order executed |  |  |  |  |  |  |
| 114 | Order has been changed to IOC |  |  |  |  |  |  |
| 135 | Market Order triggered and executed |  |  |  |  |  |  |
| 145 | Start Of Day Processing |  |  |  |  |  |  |
| 146 | End Of Day Processing |  |  |  |  |  |  |
| 155 | Order Refreshed |  |  |  |  |  |  |
| 172 | Stop Order has been triggered |  |  |  |  |  |  |
| 215 | Risk Reduction Timer Expired |  |  |  |  |  |  |
| 217 | Tick Size Change |  |  |  |  |  |  |
| 248 | Active order deletion due to SMPF |  |  |  |  |  |  |
| 250 | Active order modification due to SMPF |  |  |  |  |  |  |
| 252 | Passive order deletion due to SMPF |  |  |  |  |  |  |
| 254 | Passive oder modification due to SMPF |  |  |  |  |  |  |
| 261 | Panic Cancel |  |  |  |  |  |  |
| 302 | RRMIN |  |  |  |  |  |  |

---

| 303 | SQUAREOFFIN |
| --- | --- |
| 357 | Base Price Update |
| 358 | Order Deleted As PriceMoved Out Of DPL Range |

## 5.5.1 Trade Modification Request-8005

The Trade Modify Instruction Request message provides the ability to request the change in Account 
Information for an already executed trade.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8005(TradeModificationRequest, MsgType = U92) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 24 | unsigned int | Exchange Order ID generated by theMCXSystem;  it remains constantover the lifetime of an order. |
| 30048 | SimpleSecurityID | Y | 4 | 32 | unsigned int | Instrument Identifier |
| 20412 | RootParty-IDExecutingTrader | Y | 4 | 36 | unsigned int | Owning User ID. |
| 1506 | SideTradeID | Y | 4 | 40 | unsigned int | Private trade identifier of an order orquote match step. |
| 1003 | TradeID | Y | 4 | 44 | unsigned int | Uniquely identifies all order leg allo-cations referring to the same matchevent, simple instrument and price. |
| 1300 | MarketSegmentID | Y | 4 | 48 | signed int | Product identifier. |
| 711 | Echo | N | 4 | 52 | signed int | Vendors can use this as a reference inresponse from exchange |
| 751 | OldAccountType | Y | 1 | 56 | VariableString |  |
| 751 | OldAccountType | Y | 1 | 56 | VariableString | Description |
| 751 | OldAccountType | Y | 1 | 56 | VariableString | Own |
| 751 | OldAccountType | Y | 1 | 56 | VariableString | Client |

---

|  |  |  |  |  |  | 5 Institution |
| --- | --- | --- | --- | --- | --- | --- |
|  | 756 OldUCCCode | N | 10 | 57 | Fixed String (0-terminable) | Previous Account information |
|  | 753 OldCPCode | N | 12 | 67 | Fixed String (0-terminable) | Previous Participant Code |

---

|  | Tag Field Name 754 NewAccountType | Req’d N | Len 1 | Ofs 79 | Data Type Fixed String (0-terminable) | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 754 NewAccountType | Req’d N | Len 1 | Ofs 79 | Data Type Fixed String (0-terminable) |  |  |
|  | Tag Field Name 754 NewAccountType | Req’d N | Len 1 | Ofs 79 | Data Type Fixed String (0-terminable) | Value | Description |
|  | Tag Field Name 754 NewAccountType | Req’d N | Len 1 | Ofs 79 |  | 1 | Own |
|  | Tag Field Name 754 NewAccountType | Req’d N | Len 1 | Ofs 79 |  | 3 | Client |
|  | Tag Field Name 754 NewAccountType | Req’d N | Len 1 | Ofs 79 |  | 5 | Institution |
|  | 755 NewUCCCode | N | 10 | 80 | Fixed String (0-terminable) |  | New UCC Code |
|  | 759 NewCPCode | N | 12 | 90 | Fixed String (0-terminable) |  | New Participant code. Valid characters: \x01-\x7E |
|  | 58 Text | N | 12 | 102 | Fixed String (0-terminable) | der. | Latest User reference Text for the or- Comma(, ) is not allowed in string for request messages. |
| 39060 Pad6 |  | N | 6 | 114 | Fixed String | Not Used. |  |

---

**5.5.2 Trade Modification Response – 8010**

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8010(TradeModificationResponse, MsgType = U93) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp |  |
| 21002 | Reserve1 | Y | 8 | 24 | UTCTimestamp | time of matching engine entry |
| 21003 | Reserve2 | Y | 8 | 32 | UTCTimestamp | Not Used. |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Time of message transmission fromcomponent to gateway |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 5948 | PartitionID | Y | 2 | 60 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 62 | unsigned int | Session Data |
| 28704 | ApplMsgID | Y | 16 | 63 | data | Not set if the submitting session is notthe owner of the cancelled order. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 79 | unsigned int | Last Message |
| 893 | LastFragment | Y | 1 | 79 | unsigned int |  |
|  |  |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 80 | unsigned int | Exchange Order ID generated by theMCXSystem;  it remains constantover the lifetime of an order. |

---

|  | Tag Field Name 779 LastUpdateTime | Req’d Y | Len 8 | Ofs 88 | Data Type UTCTimestamp | Description Time at which exchange has con- firmed the trade modification |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 779 LastUpdateTime | Req’d Y | Len 8 | Ofs 88 | Data Type UTCTimestamp | Description Time at which exchange has con- firmed the trade modification |  |
|  | 30048 SimpleSecurityID | Y | 4 | 96 | unsigned int | Instrument Identifier |  |
|  | 711 Echo | N | 4 | 100 | signed int | Vendors can use this as a reference in response from exchange |  |
|  | 1506 SideTradeID | Y | 4 | 104 | unsigned int | Private trade identifier of an order or quote match step. |  |
|  | 1003 TradeID | Y | 4 | 108 | unsigned int | Uniquely identifies all order leg allo- cations referring to the same match event, simple instrument and price. |  |
|  | 20412 RootParty- IDExecutingTrader | Y | 4 | 112 | unsigned int | Owning User ID. |  |
|  | 751 OldAccountType | Y | 1 | 116 | Fixed String (0-terminable) | Owning User ID. |  |
|  | 751 OldAccountType | Y | 1 | 116 | Fixed String (0-terminable) | Value Description |  |
|  | 751 OldAccountType | Y | 1 | 116 | Fixed String (0-terminable) | 1 Own 3 Client 5 Institution |  |
|  | 751 OldAccountType | Y | 1 | 116 | Fixed String (0-terminable) | 1 Own 3 Client 5 Institution |  |
|  | 751 OldAccountType | Y | 1 | 116 | Fixed String (0-terminable) |  |  |
|  | 756 OldUCCCode | N | 10 | 117 | Fixed String |  | Previous Account information |
|  |  |  |  |  | (0-terminable) Fixed String (0-terminable) |  |  |
|  | 753 OldCPCode | N | 12 | 127 | (0-terminable) Fixed String (0-terminable) | Previous Participant Code |  |
|  |  |  | 1 |  | (0-terminable) Fixed String (0-terminable) | Previous Participant Code |  |
|  | 754 NewAccountType | N | 1 | 139 | Fixed String (0-terminable) | New Account Type Value Description |  |
|  | 754 NewAccountType | N | 1 | 139 | Fixed String (0-terminable) | New Account Type Value Description |  |
|  | 754 NewAccountType | N | 1 | 139 | Fixed String (0-terminable) | 1 Own |  |
|  | 754 NewAccountType | N | 1 | 139 | Fixed String (0-terminable) | 3 | Client |
|  | 754 NewAccountType | N | 1 | 139 | Fixed String (0-terminable) | 5 Institution |  |
|  | 755 NewUCCCode | N | 10 | 140 | Fixed String (0-terminable) | New UCC Code |  |
|  | 759 NewCPCode | N | 12 | 150 | Fixed String (0-terminable) | New Participant code. Valid characters: \x01-\x7E |  |
|  | 58 Text | N | 12 | 162 | Fixed String (0-terminable) | Latest User reference Text for the or- der. Comma(, ) is not allowed in string for request messages |  |
| 39020 Pad2 |  | N | 2 | 174 | Fixed String | Not Used. |  |

---

## 5.5.3 Trade Enhancement Notification-10989

This message informs about the acceptance/rejection of a trade by the custodian . The alert is sent to 
the originating session and the MAT/Sub MAT session of the business unit.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10989 (Order-MassActionReport, MsgType = BZ) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Application  sequence  number  as-signed to a non-order related MCXETI data stream. |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability andRetransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | 0 False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | 1 True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | 0 Trade Enhancement |
| 1180 | ApplID | Y | 1 | 31 | unsigned int |  |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | ValueDescription |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | 1 Last Message |
| 893 | LastFragment | Y | 1 | 32 | unsigned int |  |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 1596 ClearingTradePrice | N | 8 | 40 | PriceType | Price in clearing notation which can be preliminary or final depending on TradeReportType (856). |  |
|  |  |  |  |  |  | Price in clearing notation which can be preliminary or final depending on TradeReportType (856). |  |
|  | 757 TransactionTime | Y | 8 | 48 | UTCTimestamp | Transaction timestamp |  |
|  | 37 OrderID | Y | 8 | 56 | unsigned int | Exchange Order ID generated by the MCX System;  it remains constant over the lifetime of an order. |  |
|  | 28736 ClearingTradeQty | N | 8 | 64 | Qty | Quantity in clearing notation. |  |
|  | 30048 SimpleSecurityID | Y | 4 | 72 | unsigned int | Instrument Identifier. |  |
|  | 1003 TradeID | Y | 4 | 76 | unsigned int | Uniquely identifies all order leg allo- cations referring to the same match event, simple instrument and price. |  |
| 20455 | RootPartyIDSessionID | Y | 4 | 80 | unsigned int | Session ID. Private trade identifier of an order or |  |
|  | 1506 SideTradeID | Y | 4 | 84 | unsigned int | quote match step. |  |
|  | 1300 MarketSegmentID | N | 4 | 88 | signed int | Product identifier. |  |
|  | 28582 MatchDate | N N | 4 | 92 LocalMktDate 96 unsigned int |  | Business day of the match event. |  |
|  |  | N N |  | 92 LocalMktDate 96 unsigned int |  | Business day of the match event. |  |
|  | 714 AccountType | N N | 1 | 92 LocalMktDate 96 unsigned int |  | Value Description 1 Own 3 Client 5 Institution |  |
|  | 714 AccountType | N N | 1 | 92 LocalMktDate 96 unsigned int |  | Value Description 1 Own 3 Client 5 Institution |  |
|  | 714 AccountType | N N | 1 | 92 LocalMktDate 96 unsigned int |  | Value Description 1 Own 3 Client 5 Institution |  |
|  | 54 Side | N | 1 | 97 | unsigned int | Side of the order. Value Description 1 Buy |  |
|  | 54 Side | N | 1 | 97 | unsigned int | Side of the order. Value Description 1 Buy |  |
|  | 54 Side | N | 1 | 97 | unsigned int | Side of the order. Value Description 1 Buy |  |
|  | 758 GiveupStatus | N | 1 | 98 | unsigned int | Value Description |  |
|  |  | N | 1 |  | unsigned int | 1 2 3 | Accepted Rejected Pending |
|  |  |  | 1 |  | unsigned int |  |  |
|  | 717 CPCode | N | 12 | 99 | Fixed String (0-terminable) | \x5C, \ | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, x5D, \x5F, \x60, \x7B, \x7D. |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 25007 F | reeText1 | N | 12 | 111 | Fixed String(0-terminable) | The Unique Client Code (UCC) of thepersonfor whom the order is entered. Validcharacters: \x00, \x21, \x23,\x24, \x28, \x29, \x2A, \x2B, \x2D,\x2E, \x2F, \x30-\x39, \x3A, \x3B,\x3D, \x3E, \x3F, \x40, \x41-\x5A,\x5C, \x5D, \x5F, \x60, \x7B, \x7D |
| 39050 | Pad5 | N | 5 | 123 | Fixed String | Not Used. |

---

## 5.5.4 Trade Modification Notification

Trade Modification Notification message notify for the trade modification.

| Tag | Field Name | Req’d | Len | Ofs | Data Type |  | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| &lt;MessageHeaderOut&gt; |  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int |  | Number of bytes for the message, in- cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int |  | Unique identifier for a MCX ETI mes- sage layout. Value: 8020 (TradeModificationNotification , MsgType = U94) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String |  | not used |  |
| &lt;RBCHeader&gt; |  |  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp |  | Gateway response out timestamp. |  |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int |  | Application sequence number as- signed to a non-order related MCX ETI data stream. Unique ID assigned by the MCX sys- |  |
| 28727 | ApplSubID | Y | 4 24 |  | unsigned int |  | tem during broadcast subscription in order to link broadcasts to the related subscription. |  |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int |  | Grouping of MCX products. Belongs to the scope of Service Avail- ability and Retransmit requests. |  |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int |  |  | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 |  | unsigned int |  | Value Description |  |
| 1352 | ApplResendFlag | Y | 1 |  | unsigned int |  | 0 | False |
| 1352 | ApplResendFlag | Y | 1 |  | unsigned int |  |  |  |
| 1352 | ApplResendFlag | Y |  |  | unsigned int |  | 1 | True |
| 1180 | ApplID |  | 1 | 31 | unsigned int |  | Identifier for an ETI data stream. Value Description |  |
| 1180 | ApplID |  | 1 |  | unsigned int |  | 0 Trade Enhancement |  |
| 893 | LastFragment | Y | 1 32 |  | unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of |  |
|  |  |  |  |  |  |  | messages belonging to one dedicated transaction. |  |
|  |  |  |  |  |  |  | 1 Last Message |  |
|  |  |  |  |  |  |  |  |  |
| 39070 | Pad7 | N | 7 | 33 | Fixed String |  | Not Used. |  |

---

| 37 | OrderID | Y | 8 | 40 | unsigned int |  | Exchange Order ID generated by the |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  | MCX System; it remains constant over the lifetime of an order. |

| Tag 30048 | Field Name SimpleSecurityID | Req’d Y | Len | Ofs 48 | Data Type unsigned int | Description The U/L Asset / producer Identifier is specified by Exchange for permitted U/L Asset / products for trading |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tag 30048 | Field Name SimpleSecurityID | Req’d Y | 4 | Ofs 48 | Data Type unsigned int | Description The U/L Asset / producer Identifier is specified by Exchange for permitted U/L Asset / products for trading |  |
| 20412 | RootParty- IDExecutingTrader | Y | 4 | 52 | unsigned int | Owning User ID. |  |
| 1506 | SideTradeID | Y | 4 | 56 | unsigned int | Private trade identifier of an order or quote match step. |  |
| 1003 | TradeID | N | 4 | 60 | unsigned int | Uniquely identifies all order leg allo- cations referring to the same match event, simple instrument and price. |  |
| 766 | RootPartyIDSubmitter | Y | 4 | 64 | unsigned int |  |  |
| 711 | Echo | N | 4 | 68 | signed int | response from exchange |  |
| 761 | OldCustomerorFirm | Y | 1 | 72 | Fixed String (0-terminable) | Value Description 1 Own 3 Client 5 Institution |  |
| 761 | OldCustomerorFirm | N | 10 | 72 | Fixed String (0-terminable) | Value Description 1 Own 3 Client 5 Institution |  |
|  | OldAccount |  |  |  | (0-terminable) | Previous Account information Valid characters: \x01-\x7E |  |
| 764 | OldClearingAccount | N | 12 | 83 | Fixed String | Previous Account information Valid characters: \x01-\x7E |  |
|  | NewCustomerorFirm |  | 1 |  | (0-terminable) | Valid characters: \x01-\x7E Value Description |  |
| 767 | NewCustomerorFirm | N | 1 | 95 | Fixed String (0-terminable) | Valid characters: \x01-\x7E Value Description |  |
| 767 | NewCustomerorFirm | N | 1 |  | Fixed String (0-terminable) | Valid characters: \x01-\x7E Value Description |  |
| 767 | NewCustomerorFirm | N | 1 |  | Fixed String (0-terminable) | Valid characters: \x01-\x7E Value Description |  |
| 767 | NewCustomerorFirm | N | 1 |  | Fixed String (0-terminable) | 1 | Own |
| 767 | NewCustomerorFirm | N | 1 |  | Fixed String (0-terminable) | 3 | Client |
| 767 | NewCustomerorFirm | N | 1 |  | Fixed String (0-terminable) | 5 | Institution |
| 767 | NewCustomerorFirm | N | 1 |  | Fixed String (0-terminable) |  |  |
| 763 | NewAccount | N | 10 | 96 | Fixed String (0-terminable) | New UCC Code Valid characters: \x01-\x7E |  |
| 765 | NewClearingAccount | N | 12 | 106 | Fixed String (0-terminable) | New Participant code. Valid characters: \x01-\x7E |  |
| 58 | Text | N | 12 | 118 | Fixed String (0-terminable) |  | Latest User reference Text for the or- der.Comma(,) is not allowed in string for request messages |
| 39060 | Pad6 | N | 6 | 130 | Fixed String | Not Used. |  |

---

## 5.5.5 Resubmit for Approval Request ( 8500 )

This message is sent to the service “Order and Quote Management”.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderIn> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8500(ResubmitApprovalRequest, MsgType = U86) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
| <R | equestHeader> |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  | OrderID |  |  |  |  |  |
| 37 | OrderID | Y | 8 | 24 | unsigned int | Exchange Order ID generated by theMCXSystem;  it remains constantover thelifetime of an order. |
| 1506 | SideTradeID | Y | 4 | 32 | unsigned int | Private trade identifier of an order orquote match step. |
| 30048 | SimpleSecurityID | Y | 4 | 36 | unsigned int | Instrument identifier. |
| 54 | Side | N | 1 | 40 | unsigned int | Side of the order. |
| 54 | Side | N | 1 | 40 | unsigned int | Description |
| 54 | Side | N | 1 | 40 | unsigned int | Buy |
| 54 | Side | N | 1 | 40 | unsigned int | Sell |
| 54 | Side | N | 1 | 40 | unsigned int |  |
| 39070 | Pad7 | N | 7 | 41 | Fixed String | Not Used. |

---

## 5.5.6 Resubmit for Approval Confirmation (8510)

This message is sent to the service “Order and Quote Management”.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <Me | ssageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  8510(ResubmitApprovalConfirmation, MsgType = U87) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <NRR | esponseHeaderME> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |
| 745 | Reserve0 | N | 8 | 16 | UTCTimestamp | Not Used |
| 21002 | Reserve1 | N | 8 | 24 | UTCTimestamp | Not Used |
| 21003 | Reserve2 | N | 8 | 32 | UTCTimestamp | Not Used |
| 7765 | Reserve3 | Y | 8 | 40 | UTCTimestamp | Not Used |
| 52 | SendingTime | Y | 8 | 48 | UTCTimestamp | Gateway response out timestamp. |
| 34 | MsgSeqNum | Y | 4 | 56 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | ValueDescription |
| 893 | LastFragment | Y | 1 | 60 | unsigned int | 1Last Message |
| 39030 | Pad3 | N | 3 | 61 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 774 | LastUpdateDate | N | 8 | 64 | UTCTimestamp | Last updated date. |
| 37 | OrderID | Y | 8 | 72 | unsigned int | Exchange Order ID generated by theMCXSystem;  it remains constantover the lifetime of an order. |
| 1506 | SideTradeID | Y | 4 | 80 | unsigned int | Private trade identifier of an order orquote match step. |
| 30048 | SimpleSecurityID | Y | 4 | 84 | unsigned int | The U/L Asset / producer Identifieris specified by Exchange for permittedU/L Asset / products for tradin |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 54 | Side | Y | 1 | 88 | unsigned int | Side of the order. |  |
| 54 | Side | Y | 1 | 88 | unsigned int | Value | Description |
| 54 | Side | Y | 1 | 88 | unsigned int | 1 | Buy |
| 54 | Side | Y | 1 | 88 | unsigned int | 2 | Sell |
| 773 | GiveUpStatus | Y | 1 | 89 | unsigned int | Flag informing the status of the accep- tance/rejection of trade by the custo- dian |  |
| 773 | GiveUpStatus | Y | 1 | 89 | unsigned int | Value | Description |
| 773 | GiveUpStatus | Y | 1 | 89 | unsigned int | 1 | Accepted |
| 773 | GiveUpStatus | Y | 1 | 89 | unsigned int | 2 | Rejected |
| 773 | GiveUpStatus | Y | 1 | 89 | unsigned int | 3 | Pending |
| 39060 | Pad6 | N | 6 | 90 | Fixed String | Not Used. |  |

---

## 5.5.7 Trade Notification-10500

The Trade Notification message is the legally binding confirmation for a trade.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10500 (Trade-CaptureReport, MsgType = AE) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | BCHeader> |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Application  sequence  number  as-signed to a non-order related MCXETI data stream. |
| 28727 | ApplSubID | N | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | 1 Trade |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | ValueDescription |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | 1 Last Message |
| 39070 | Pad7 | U | 7 | 33 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 1650 | RelatedSecurityID | N | 8 | 40 | signed int | Not used. |

---

|  | Tag Field Name 44 Price | Req’d N | Len 8 | Ofs 48 | Data Type PriceType |  | Description Limit price. Required if OrdType (40) is Limit (2) or Stop Limit (4). |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 44 Price | Req’d N | Len 8 | Ofs 48 | Data Type PriceType |  | Description Limit price. Required if OrdType (40) is Limit (2) or Stop Limit (4). |  |
|  | 31 LastPx | Y | 8 | 56 | PriceType |  | Price of this fill. |  |
|  | 28585 SideLastPx | N | 8 | 64 | PriceType |  | Fill price for the original MCX strat- egy. |  |
|  | 1596 ClearingTradePrice | N | 8 | 72 | PriceType |  | Price in clearing notation which can be preliminary or final depending on TradeReportType (856). |  |
|  | 724 Filler10 | N | 8 | 80 | signed int |  | Not used. |  |
|  | 725 Filler11 | N | 8 8 8 8 8 | 88 | signed int |  | Not used. |  |
|  | 60 TransactTime | Y | 8 8 8 8 8 | 96 | UTCTimestamp |  | Transaction timestamp. |  |
|  | 37 OrderID | N | 8 8 8 8 8 | 104 | unsigned int |  | Exchange Order ID generated by the MCX system. |  |
|  | 708 TerminalInfo | Y | 8 8 8 8 8 | 112 unsigned int |  |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |
|  |  |  | 8 8 8 8 8 | 112 unsigned int |  |  | Total 15 Characters st For 1 - 12 characters value(11111111111-33333333333), th For 13 character value(0-3) th th 14 – 15 digit should be valid vendor code / in House CTCL code for member |  |
|  |  |  | 8 8 8 8 8 | 112 unsigned int |  |  | 11111111111 IBT |  |
|  |  |  | 8 8 8 8 8 | 112 unsigned int |  |  | 22222222222 DMA |  |
|  |  |  | 8 8 8 8 8 | 112 unsigned int |  |  | 33333333333 Wireless Technology Order not generated 0 through Program trading software |  |
|  |  |  | 8 8 8 8 8 | 112 unsigned int |  |  | 33333333333 Wireless Technology Order not generated 0 through Program trading software |  |
|  |  |  | 8 8 8 8 8 | 112 unsigned int |  |  | Order generated 1 through Program trading software Smart Order  Rout- |  |
|  |  |  | 8 8 8 8 8 | 112 unsigned int |  |  | 2 ing without Program trading software |  |
|  |  |  | 8 8 8 8 8 |  |  |  | Smart Order Routing 3 with program trading software |  |
|  | 11 ClOrdID | N |  | 120 | unsigned int |  | Client Order ID: Unique participant defined order request identifier.For quotes the QuoteMsgID (1166) is pro- vided in the trade notification. Not used. |  |
| 723 LstUpdtTime |  | Y | 8 | 128 | UTCTimestamp |  | Client Order ID: Unique participant defined order request identifier.For quotes the QuoteMsgID (1166) is pro- vided in the trade notification. Not used. |  |

---

|  | Tag Field Name 727 StrategyID | Req’d Y | Len 8 | Ofs 136 | Data Type unsigned int | Description Strategy Approved by the exchange should be used.Range being from 0 to 99999 |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 727 StrategyID | Req’d Y | Len 8 | Ofs 136 | Data Type unsigned int | Description Strategy Approved by the exchange should be used.Range being from 0 to 99999 |
|  | 728 StrategySequence- Number | Y | 8 | 144 | unsigned int | Strategy Sequence numbers |
|  | 32 LastQty | Y | 8 | 152 | Qty | Quantity executed in this fill. |
|  | 1009 SideLastQty | N | 8 | 160 | Qty | Fill quantity for the original MCX strategy. |
|  | 709 Filler1 | N | 8 | 168 | unsigned int | Not Used. |
|  | 14 CumQty | N | 8 | 176 | Qty | Cumulated executed quantity of an or- der. |
|  | 151 LeavesQty | N | 8 | 184 | Qty | Remaining quantity of the order at the time of the execution.If the order has been executed partially this field con- tains the non-executed quantity.A re- maining size of 0 indicates that the order is fully matched or no longer ac- tive. |
|  | 30048 SimpleSecurityID | Y | 4 | 192 | unsigned int | Not used. |
|  | 710 Filler2 | N | 4 | 196 | unsigned int | Not used. |
|  | 711 Echo | Y | 4 | 200 | signed int | Vendors can use this as a reference in response from exchange |
|  | 1003 TradeID | Y | 4 | 204 | unsigned int | Uniquely identifies all order leg allo- cations referring to the same match event, simple instrument and price. |
|  | 1126 OrigTradeID | N | 4 | 208 | unsigned int | Not used. |
|  | 20459 RootParty- IDExecutingUnit | Y | 4 | 212 | unsigned int | Business Unit ID. (Value 2 indicates entered by exchange.) |
| 20455 | RootPartyIDSessionID | N | 4 | 216 | unsigned int | Session ID. |
|  | 20412 RootParty- IDExecutingTrader | Y | 4 | 220 | unsigned int | Owning User ID. |
|  | 25026 RootPartyIDClearing- Unit | Y | 4 | 224 | unsigned int | Not used. |
|  | 1300 MarketSegmentID | Y | 4 | 228 | signed int | Product identifier. |
|  | 1506 SideTradeID | Y | 4 | 232 | unsigned int | Private trade identifier of an order or quote match step. |
|  | 28582 MatchDate | Y | 4 | 236 | LocalMktDate | Business day of the match event. |
|  | 880 TrdMatchID | Y | 4 | 240 | unsigned int | Unique identifier for each price level (match step) of a match event; it is used for public trade reporting. |
|  | 1851 StrategyLinkID | N | 4 | 244 | unsigned int | Identifier that links all trades resulting from a match step of a strategy order. |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 748 | TotNumTradeReports | N | 4 | 248 | signed int | Number of leg executions of the orig-inal strategy order. |  |  |
| 1 | Account | N | 2 | 252 | Fixed String(0-terminable) | Not used.Must be sent as A1Valid characters: 1-9,  \x41,  \x47,\x49, \x4D, \x50, \x52 |  |  |
| 713 F | iller4 | N | 2 | 254 | unsigned int | Not Used. |  |  |
| 442 M | ultiLegReporting-Type | N | 1 | 256 | unsigned int | Indicates if the trade resulted from asingle order or a multi leg instrument. |  |  |
| 442 M | ultiLegReporting-Type | N | 1 | 256 | unsigned int | Value | Description |  |
| 442 M | ultiLegReporting-Type | N | 1 | 256 | unsigned int | 1 | Single Order |  |
| 442 M | ultiLegReporting-Type | N | 1 | 256 | unsigned int | 2 | Complex Order |  |
| 442 M | ultiLegReporting-Type | N | 1 | 256 | unsigned int |  |  |  |
| 856 | TradeReportType | Y | 1 | 257 | unsigned int | Not used. |  |  |
| 830 | TransferReason | Y | 1 | 258 | unsigned int | Not used. |  |  |
| 716 F | iller5 | N | 1 | 259 | unsigned int | Not used. |  |  |
| 20432 R | ootParty-IDBeneficiary | N | 9 | 260 | Fixed String | Not used.Valid characters:  \x20, \x22-\x7B,\x7D, \x7E |  |  |
| 20496 | RootPartyIDTakeUp-TradingFirm | N | 5 | 269 | Fixed String | Not used.Valid characters: A-Z, 0-9, \x20 |  |  |
| 20413 R | ootPartyIDOrder-OriginationFirm | N | 7 | 274 | Fixed String | Not used.Valid characters: A-Z, 0-9, \x20 |  |  |

---

|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 281 | unsigned int |  |  |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 281 |  | Value | Description |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 281 |  |  | 1 Own |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 281 | unsigned int |  | 3 Client |
|  | Tag Field Name 714 AccountType | Req’d Y | Len 1 | 1 282 | unsigned int | 5 Institution The point in the matching process at which this trade was matched. The value 3(=Confirmed Trade Re- port) represents Trades entered by |  |
|  | 574 MatchType | N |  | 1 282 | unsigned int | 5 Institution The point in the matching process at which this trade was matched. The value 3(=Confirmed Trade Re- port) represents Trades entered by |  |
|  | 574 MatchType |  |  | 1 282 | unsigned int | Value | Market Supervision. Description |
|  | 574 MatchType |  |  | 1 282 | unsigned int |  | Confirmed Trade Report (re- 3 porting from recognized mar- |
|  | 574 MatchType |  |  | 1 282 | unsigned int |  | kets) |
|  | 574 MatchType |  |  | 1 282 | unsigned int | 4 | Auto-match incoming order |
|  | 574 MatchType |  |  |  | unsigned int |  | 11 Auto match resting order |
|  | 28610 MatchSubType | N Y | 1 | 283 | unsigned int | Indicates the auction type the trade |  |
|  | 28610 MatchSubType | N Y | 1 | 283 | unsigned int |  | crossing, i.e.when a complex instru- |
|  | 28610 MatchSubType | N Y | 1 | 283 | unsigned int | ment switches to the instrument state “Continuous”. Value Description |  |
|  | 28610 MatchSubType | N Y | 1 | 283 | unsigned int | ment switches to the instrument state “Continuous”. Value Description |  |
|  | 28610 MatchSubType | N Y | 1 | 283 | unsigned int |  | 1 Opening auction |
|  | 54 Side | N Y | 1 | 284 unsigned int 285 unsigned int |  | Side of | the trade.  Leg executions of sell orders for complex instruments |
|  | 54 Side |  | 1 | 284 unsigned int 285 unsigned int |  |  | will have an inverted value compared |
|  | 54 Side |  | 1 | 284 unsigned int 285 unsigned int |  | Value | to the signature. Description |
|  | 54 Side |  | 1 | 284 unsigned int 285 unsigned int |  |  |  |
|  | 54 Side |  | 1 | 284 unsigned int 285 unsigned int |  |  |  |
|  | 54 Side |  |  | 284 unsigned int 285 unsigned int |  | 2 Sell |  |
|  | 726 AggressorIndicator | N Y | 1 1 |  | unsigned int | Indicates whether the order added or removed liquidity. |  |
|  |  | N Y | 1 1 |  | unsigned int | Value | Description |
|  | 1815 TradingCapacity | N Y | 1 1 |  | unsigned int |  | 0 Passive |
|  | 1815 TradingCapacity | N Y | 1 1 |  | unsigned int |  | 1 Aggressive |
|  | 1815 TradingCapacity | N Y | 1 1 | 286 | unsigned int | Not used. |  |

---

|  | Tag Field Name 77 PositionEffect | Req’d N | Len 1 | Ofs 287 | Data Type char | Description Not used. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 77 PositionEffect | Req’d N | Len 1 | Ofs 287 | Data Type char | Description Not used. |  |
|  | 1031 CustOrderHandling- Inst | N | 1 | 288 | Fixed String | Not used. Valid characters:  \x20, \x22-\x7B, \x7D, \x7E |  |
|  | 25007 FreeText1 | Y | 12 | 289 | Fixed String (0-terminable) | The Unique Client Code (UCC) of the person for whom the order is entered. Valid characters: \x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |  |
|  | 717 CPCode | N | 12 | 301 | Fixed String (0-terminable) | The Participant code. Valid characters:\x00, \x21, \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \ x5D, \x5F, \x60, \x7B, \x7D. |  |
|  | 25009 FreeText3 | N | 12 | 313 | Fixed String | Free-format text field for traderspecific  or  customer-related comments. Valid characters:\x00, \x21,  \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |  |
|  |  | N |  | 313 |  | Free-format text field for traderspecific  or  customer-related comments. Valid characters:\x00, \x21,  \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |  |
|  | 1115 OrderCategory | N | 1 | 325 | char | Free-format text field for traderspecific  or  customer-related comments. Valid characters:\x00, \x21,  \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |  |
|  |  | N |  |  |  | Free-format text field for traderspecific  or  customer-related comments. Valid characters:\x00, \x21,  \x23, \x24, \x28, \x29, \x2A, \x2B, \x2D, \x2E, \x2F, \x30-\x39, \x3A, \x3B, \x3D, \x3E, \x3F, \x40, \x41-\x5A, \x5C, \x5D, \x5F, \x60, \x7B, \x7D |  |
|  |  | N |  |  |  |  | Indicates if the trade notification re- sults from an order or quote. |
|  |  |  |  |  |  | Value Description 1 Order |  |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  | 3 | Multileg order |
|  | 40 OrdType | N | 1 | 326 | unsigned int | Order type. |  |
|  | 40 OrdType |  | 1 | 326 | unsigned int | Value Description |  |
|  | 40 OrdType |  | 1 | 326 | unsigned int | 2 | Limit |
|  | 40 OrdType |  | 1 | 326 | unsigned int | 3 | Stop Market |
|  | 40 OrdType |  | 1 | 326 | unsigned int | 4 | Stop Limit |
|  | 40 OrdType |  | 1 | 326 | unsigned int | 5 | Market To Limit |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type |  | Description Instrument type of the orginal MCX strategy. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 28587 | RelatedProduct- Complex | N | 1 | 327 | unsigned int |  | Description Instrument type of the orginal MCX strategy. |  |
|  |  |  |  |  |  |  | Value | Description |
|  |  |  |  |  |  |  | 5 | Futures Spread |
|  |  |  |  |  |  |  | 1 | Simple Instrument. |
|  |  |  |  |  |  |  |  |  |
| 28586 | OrderSide | N | 1 | 328 | unsigned int |  | Side of the order in the original MCX strategy. |  |
| 28586 |  |  |  |  | unsigned int |  | Value | Description |
| 28586 |  |  |  |  | unsigned int |  | 1 | Buy |
| 28586 |  |  |  |  | unsigned int |  | 2 | Sell |
| 28586 | RootPartyClearing- |  |  |  |  |  |  |  |
| 22421 | Organization | Y | 4 | 329 | Fixed String |  | Not used. \x7D, \x7E | Valid characters: \x20, \x22-\x7B, |
|  |  |  |  |  |  |  |  |  |
| 22401 | Firm RootPartyExecuting- |  | 5 | 333 | Fixed String |  | \x7D, \x7E | Participant Short Name. Valid characters: \x20, \x22-\x7B, |
| 22412 | Trader | Y | 6 | 338 | Fixed String Not used. Valid characters: \x20, \x22-\x7B, \x7D, \x7E |  |  |  |
| 22404 | RootPartyClearing- Firm | Y | 5 | 344 | Fixed String |  | Not used. Valid characters: \x20, \x22-\x7B, \x7D, \x7E |  |
| 718 | UserReferenceText | N | 20 | 349 | Fixed String (0-terminable) |  | Upto 15 Char allowed.This field is used to provide additionalregulatory information(according to respective rules andregs, circulars and/or bilateral coordination between participantand Trading Surveillance Office).Valid characters: \x20, \x22- \x7B, \x7D, \x7E. |  |
| 39030 | Pad3 | N | 3 | 369 | Fixed String |  | Not Used. |  |
| 39040 | Pad4 | U | 4 | 372 | Fixed String |  | Not Used. |  |

---

## 5.6 Others

## 5.6.1 Subscribe-10025

This message is used to subscribe to a MCX ETI broadcast. Multiple subscriptions to the same 
pair of RefApplId (1355) and SubscriptionScope (25001) are not allowed

| RefApplID | MessageTemplateID | MessageTemplateName |
| --- | --- | --- |
| 1 Trade | 10500 | Trade Notification |
| 1 Trade | 10501 | Trade Notification Status |
| 1 Trade | 10989 | Trade Enhancement Notification |
| 1 Trade | 8020 | Trade Modification notification |
|  |  |  |
| 2 Genral Message | 10031 | News |
| 2 Genral Message | 8100 | Market Wide OI Violation |
| 2 Genral Message | 8101 | Market Wide OI Alert |
| 2 Genral Message | 4025 | Margin Change Notification |
|  |  |  |
| 3 Service Availability | 10030 | Service Availability |
|  |  |  |
| 5 Listener | 10117 | Extended Order Information |
| 5 Listener | 4125 | Market Status Notification |
| 5 Listener | 10122 | Delete All Order Broadcast |
| 5 Listener | 10112 | Cancel Order Notification |
| 5 Listener | 10308 | Mass Cancellation Event |
| 5 Listener | 10307 | Trading Session Event |
|  |  |  |
| 12 Ex/Dex | 8040 | Ex/Dex Notification |
| 12 Ex/Dex | 4100 | System Information Download |

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.    Value:  10025(ApplicationMessageRequest,   Msg-Type = BW) |  |  |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |  |  |

---

| 39020 Pad2 |  |  | 2 | 14 | Fixed String | not used |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| &lt;RequestHeader&gt; |  |  |  |  |  |  |  |
|  | 34 MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by the participant for requests sent to the gateway. |  |
|  | 50 SenderSubID | U | 4 | 20 | unsigned int User ID |  |  |
| &lt;MessageBody&gt; |  |  |  |  |  |  |  |
|  | 25001 SubscriptionScope | N | 4 | 24 | unsigned int | Value ID | For General Messages – Scope is No For all others scope is Session |
|  | 1355 RefApplID |  |  | 28 29 | unsigned int |  |  |
|  | 1355 RefApplID | Y | 1 3 | 28 29 | unsigned int | stream. | Application identifier of a  ETI data |
|  | 1355 RefApplID |  | 1 3 | 28 29 | unsigned int |  | Value Description |
|  | 1355 RefApplID |  | 1 3 | 28 29 | unsigned int | 1 Trade |  |
|  | 1355 RefApplID |  | 1 3 | 28 29 | unsigned int |  | 2 General Messages |
|  | 1355 RefApplID |  | 1 3 | 28 29 | unsigned int |  | 3 Service Availability |
|  | 1355 RefApplID |  | 1 3 | 28 29 | unsigned int | 5 | Listener Data |
|  | 1355 RefApplID |  | 1 3 | 28 29 | unsigned int |  | 12 Ex/Dex |
| 39030 Pad3 | 1355 RefApplID | N | 1 3 | 28 29 | Fixed String | Not used. |  |

---

## 5.6.2 Subscribe Response-10005

The Subscribe Broadcast Response message is used to confirm the broadcast subscription.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:   10005(ApplicationMessageRequestAck,MsgType = BX) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |  |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |  |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |  |
|  |  |  |  |  |  |  |  |
| 28727 | ApplSubID | Y | 4 | 32 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |  |
| 39040 | Pad4 | N | 4 | 36 | Fixed String | Not Used. |  |

## 5.6.3 Retransmit-10008

This message is used for re-transmission of trade, risk control and news data for recovery purposes.The 
specified application sequence number range will lead to the retransmission of data whose application 
sequence number is >= ApplBegSeqNum (1182) and <= ApplEndSeqNum (1183).Depending on 
RefApplID (1355), the request is sent to the respective service:

- RefApplID = 1 (Trade) => request is sent to the Trades service

-RefApplID = 2 (News) => request is sent to the News service

- RefApplID = 12 (Ex/Dex) => request is sent to the Ex/Dex service.

---

| TagField Name      Req’dLen Ofs  Data Type |  |  |  |  |  | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a ETI messagelayout.     Value:     10008(ApplicationMessageRequest, Msg-Type = BW) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
| <Reques | tHeader> |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | U | 4 | 20 | unsigned int | User ID |
| <Messag | e Body> |  |  |  |  |  |
| 1182 | ApplBegSeqNum | N | 8 | 24 | SeqNum | “novalue”meansfirstknownse-quence number: 1 |
| 1183 | ApplEndSeqNum | N | 8 | 32 | SeqNum | Ending range of application sequencenumbers. |
| 25001 | SubscriptionScope | N | 4 | 40 | unsigned int | - Trade Broadcast(Business Unit):Wildcard (0)(Applicable for dropcopy  session Only)- -Trade Broadcast(Own Session orother session in case of drop copysession):SessionID- NewsBroadcast:MarketID(1=MCX)- For rest-use SessionID |
| 5948 | PartitionID | N | 2 | 44 | unsigned int | The applicationsequencenumbersare only unique per RefApplID,SubcriptionScope  and  Partion-ID.Therefore, the PartitionID isrequired to define the scope of aRetransmission Request.Not to be set for News, Risk Control,and SRQS Maintenance. |

---

| 1355 | RefApplID | Y |  | 1 | 46 | unsigned int | Application identifier of a MCX ETI datastream. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1355 | RefApplID | Y |  | 1 | 46 | unsigned int | Value | Description |
| 1355 | RefApplID | Y |  | 1 | 46 |  | 1 | Trade |
| 1355 | RefApplID | Y |  | 1 | 46 |  | 2 | General Messages |
| 1355 | RefApplID | Y |  | 1 | 46 |  | 12 | Ex/Dex |

| Tag Field Name |  | Req’d Len |  | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 39000 | Pad1 | N | 1 | 47 | Fixed String | Not Used. |

---

## 5.6.4 Retransmit Response-10009

The Retransmission Response message confirms the Retransmit request and delivers only a fixed 
number of the requested data packages. So the requesting client would  have to send a new 
retransmission request (with updated sequence numbers) if the captured response does not contain all 
requested data. (See below: ApplEndSeqNum (1183)).

The field ApplTotalMessageCount (1349) indicates how many retransmitted broadcast messages 
will follow. For fragmented messages, each fragment counts as one message. They will not be 
interrupted by other messages. All these messages will consist of:

• <MessageHeaderOut>.

• < RBCHeader>, where ApplSubID (28727) is always set to ’no value’ and ApplResendFlag 
(1352) is always set to True (indicating a retransmission message).

• <MessageBody>,  which  is  specific  for  the  TemplateID  (28500)  in 
<MessageHeaderOut>.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:   10009(ApplicationMessageRequestAck,MsgType = BX) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |  |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |  |
| 39040 | Pad4 | U | 4 | 28 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 1183 | ApplEndSeqNum | N | 8 | 32 | SeqNum | Ending range of application sequencenumbers.If it is not set to the relatedrequest’s ApplEndSeqNum, the clientwill have to send another retransmis-sion request (with an updated Appl-BegSeqNum).”no value” means therequested data is not available. |  |
| 1357 | RefApplLastSeqNum | N | 8 | 40 | SeqNum | Last  application  sequence   num-ber  known  by  the  MCX   systemfor  a  certain  scope  of  RefApplID,SubscriptionScope    and   Partition-ID.”no value” means that there is nodata persisted yet for the requestedstream/subscription scope. |  |
| 1349 | ApplTotalMessage-Count | Y | 2 | 48 | unsigned int | Total  number of  messages   (frag-ments) included in transmission. |  |
| 39060 | Pad6 | N | 6 | 50 | Fixed String | Not Used. |  |

---

## 5.6.5 Retransmit (Order Event)-10026

This message is used for re-transmission of recoverable order/quote event data for 
recovery purposes.The specified application message identifier range will lead to the 
retransmission of data whose application message identifier is > ApplBegMsgID 
(28718) and <= ApplEndMsgID (28719).

This message is sent to the service “Retransmission of Order and Quote Events”.

Order Mass Cancellation Notification
(Listener Data)
Listener Data
Extended Order Information
(Listener Data)
Listener Data
Cancel Order Notification
(Listener Data)
Listener Data
Mass Cancellation Event
(Listener Data)
Listener Data
Trading Session Event
(Listener Data)
Listener Data
Session Data
Trading Session Event
(Session Data)
Retransmit (Order/Quote Event) → Retransmit Response (Order/Quote Event)
Session Data
Mass Cancellation Event
(Session Data)
Session Data
Extended Order Information
(Session Data)
Session Data
Extended Order Information(MultiLeg)
(Session Data)
Session Data
Quote Mass Cancellation Notification
(Session Data)
Session Data
Order Mass Cancellation Notification
(Session Data)
Quote Execution Notification

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | UniqueidentifierforaMCXETImessagelayout.  Value:  10026(ApplicationMessageRequest, Msg-Type = BW) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | U | 4 | 20 | unsigned int | User ID |
|  |  |  |  |  |  |  |
| 25001 | SubscriptionScope | N | 4 | 24 | unsigned int | Retransmission scope(for thesupported RefAppllIds):4- Session Data “no value”5-Listener Data:session ID |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Application  message  identifiersareonlyuniqueperRefApplID,SubcriptionScope and  Partition-ID.Therefore, the  PartitionID isrequiredtodefinethescopeofaRetransmit request. |

---

|  | Tag Field Name 1355 RefApplID | Req’d Y | Len | Ofs 30 | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 1355 RefApplID | Req’d Y | 1 | Ofs 30 | unsigned int | Application identifier of a MCX ETI data stream. |
|  | Tag Field Name 1355 RefApplID | Req’d Y | 1 | Ofs 30 |  | Value Description |
|  | Tag Field Name 1355 RefApplID | Req’d Y | 1 | Ofs 30 |  | 4 Session Data |
|  | Tag Field Name 1355 RefApplID | Req’d Y | 1 | Ofs 30 |  | 5 Listener Data |
|  | 28718 ApplBegMsgID | N | 16 | 31 | data | Beginning range of application mes- sage identifiers; ’no value’ indicates first known application message iden- tifier. |
|  | 28719 ApplEndMsgID | N | 16 | 47 | data | Ending range of application message identifiers;  “no  value”  means  last known application message identifier. |
| 39000 Pad1 |  | N | 1 | 63 | Fixed String | Not Used. |

---

## 5.6.6 Retransmit Response (Order Event)-10027

This message confirms the Retransmit (Order/Quote Event) request and delivers only a fixed number of 
the requested data packages. So the requesting client would have to send a new Retransmit (Order/Quote 
Event) request (with updated application message identifiers), if the captured response does not contain 
all requested data.(See below: ApplEndMsgID (28719)). The field ApplTotalMessageCount (1349) 
indicates how many retransmitted broadcast messages will follow.For fragmented messages, each 
fragment counts as one message. They will not be interrupted by other messages.
All these messages will consist of:

• <MessageHeaderOut>.

• < RBCHeaderME>, where ApplSubID (28727) is always set to  “no value”, TrdRegTSTimeOut 
(21003) is always set to “no value”, and ApplResendFlag (1352) is always set to True (indicating 
a retransmission message).

• <MessageBody>,  which  is  specific  for  the  TemplateID  (28500)  in 
<MessageHeaderOut>.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.   Value:  10027(ApplicationMessageRequestAck,MsgType = BX) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |  |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |  |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |  |
|  |  |  |  |  |  |  |  |
| 1349 | ApplTotalMessage-Count | Y | 2 | 32 | unsigned int | Total  number  of  messages  (frag-ments) included in transmission. |  |
| 28719 | ApplEndMsgID | N | 16 | 34 | data | Ending range of application messageidentifiers.If it is not set to the relatedrequest’s ApplEndMsgID, the  clientwill have to send another Retransmitrequest (with an updated ApplBeg-MsgID). |  |
| 28722 | RefApplLastMsgID | N | 16 | 50 | data | Last  application  message  identi-fier  known  by  the  MCX    systemfor  a  certain  scope  of  RefApplID,SubscriptionScope and PartitionID. |  |
| 39060 | Pad6 | N | 6 | 66 | Fixed String | Not Used. |  |

---

## 5.6.7 Gap Fill-10032

This message informs that the provided message must be skipped over, due to the fact that the MCX 
system is not able to provide the functional data to the participant.The exchange may be contacted for 
further error analysis.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.  Value:  10032(ApplicationMessageReport,  Msg-Type = BY) |  |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |  |
|  |  |  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |  |  |
|  |  |  |  |  |  |  |  |  |
| 28724 | ApplIDStatus | Y | 4 | 16 | unsigned int | Application sequencing related errorcode. |  |  |
| 28724 | ApplIDStatus | Y | 4 | 16 | unsigned int | Value | Description |  |
| 28724 | ApplIDStatus | Y | 4 | 16 | unsigned int | 105 E | rror converting response orbroadcast |  |
| 28724 | ApplIDStatus | Y | 4 | 16 | unsigned int | 106 | Other values are possible |  |
| 28724 | ApplIDStatus | Y | 4 | 16 | unsigned int |  |  |  |
| 28728 | RefApplSubID | N | 4 | 20 | unsigned int | Unique ID for the subscription in-stance assigned by the MCX systemduring broadcast subscription. |  |  |
| 30354 | VarTextLen | Y | 2 | 24 | Counter | Number of used bytes for field VarText(30355). |  |  |

---

|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int | Application identifier of a MCX ETI data stream. |  |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int | Value | Description |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int |  | 1 Trade |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int |  | 2 News |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int |  | 3 Service Availability |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int |  | 4 Session Data |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int |  | 5 Listener Data |
|  | Tag Field Name 1355 RefApplID | Req’d Y | Len 1 | Ofs 26 | Data Type unsigned int |  | 12 Ex/Dex |
|  |  |  | Len 1 | Ofs 26 | Data Type unsigned int |  |  |
|  | 1409 SessionStatus | Y | 1 | 27 | unsigned int | Value | Session status. Description 0 Session active 4 Session logout complete |
|  | 1409 SessionStatus | Y | 1 | 27 | unsigned int |  |  |
|  | 1409 SessionStatus | Y | 1 | 27 | unsigned int |  |  |
|  | 1409 SessionStatus | Y | 1 | 27 | unsigned int |  |  |
|  | 1409 SessionStatus | Y | 1 | 27 | unsigned int |  |  |
|  | 1409 SessionStatus | Y | 1 | 27 | unsigned int |  |  |
| 39040 Pad4 |  | N | 4 | 28 | Fixed String |  | Not Used. |
|  | 30355 VarText | Y | 2000 | 32 | Variable String | \x20-\ | News text. Actual violation/Alert text Valid characters: \x09, \x0A, \x0D, x7B, \x7D, \x7E |

---

## 5.6.8 Unsubscribe-10006

This message is used to revoke a broadcast subscription.

Unsubscribe-10006

Unsubscribe Response-10007

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.   Value:  10006(ApplicationMessageRequest,  Msg-Type = BW) |  |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |  |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |  |
| 50 | SenderSubID | U | 4 | 20 | unsigned int | User ID |  |
|  |  |  |  |  |  |  |  |
| 28728 | RefApplSubID | Y | 4 | 24 | unsigned int | Unique ID for the subscription in-stance assigned by the MCX systemduring broadcast subscription. |  |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |  |

---

## 5.6.9 Unsubscribe Response-10007

The Unsubscribe Broadcast Response message is used to confirm the revocation of a 
broadcast subscription.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique  identifier  for  a  MCX  ETImessage  layout.   Value:   10007(ApplicationMessageRequestAck,MsgType = BX) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Time when request was considered forprocessing |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |  |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |  |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not Used. |  |

---

## 5.6.10 Mass Cancellation Event-10308

This message informs about an event that implicitly led to the mass cancellation of orders.

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| &lt;MessageHeaderOut&gt; |  |  |  |  |  |  |
|  | 9 BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in- cluding this field. |
|  | 28500 TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes- sage layout.  Value:  10308 (Order- MassActionReport, MsgType = BZ) |
| 39020 Pad2 U 2 6 Fixed String not used |  |  |  |  |  |  |
| &lt;RBCHeaderME&gt; |  |  |  |  |  |  |
|  | 21003 Reserve2 | N | 8 | 8 | UTCTimestamp | Not Used. |
|  | 52 SendingTime | Y | 8 | 16 | UTCTimestamp | Outgoing timestamp; filled always by the gateway |
|  | 28727 ApplSubID | N | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys- tem during broadcast subscription in order to link broadcasts to the related subscription. |
|  |  |  |  |  |  |  |
|  | 5948 PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products. Belongs to the scope of Service Avail- ability and Retransmit requests. |
|  | 28704 ApplMsgID | N | 16 | 30 | data | Application  message  identifier  as- signed to an order or quote event. |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  | 1180 ApplID | Y | 1 | 46 | unsigned int | Identifier for an ETI data stream. Value Description 4 Session Data 5 Listener Data |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
|  |  |  | 1 |  |  |  |
|  |  |  |  |  | unsigned int |  |
|  | 1352 ApplResendFlag | Y | 1 | 47 | unsigned int | Indicates a retransmission message. Value Description 0 False 1 True |
|  |  |  | 1 |  | unsigned int |  |
|  |  |  | 1 |  | unsigned int |  |
|  | 893 LastFragment | Y | 1 7 | 48 | unsigned int | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. |
|  | 893 LastFragment |  | 1 7 | 48 |  |  |
|  | 893 LastFragment |  | 1 7 | 48 |  | Value Description |
|  | 893 LastFragment |  | 1 7 | 48 |  | 1 Last Message |
|  | 893 LastFragment |  | 1 7 | 48 |  |  |
|  | 893 LastFragment | N | 1 7 | 48 | Fixed String | Not Used. |
| 39070 Pad7 |  |  | 1 7 | 49 |  |  |
| &lt;MessageBody&gt; |  |  |  |  |  |  |
| &lt;MessageBody&gt; |  |  |  |  |  |  |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type |  | Description Last Update timestamp. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1369 | MassActionReportID | Y | 8 | 56 | UTCTimestam- p |  | Description Last Update timestamp. |  |
| 48 | SecurityID | N | 8 | 64 | signed int |  | Instrument identifier. |  |
| 1300 | MarketSegmentID | Y | 4 | 72 | signed int |  | Product identifier. |  |
| 28721 | MassActionReason | Y | 1 76 |  | unsigned int |  | Reason for mass cancellation. |  |
| 28721 | MassActionReason |  | 1 76 |  | unsigned int | Value Description |  |  |
| 28721 | MassActionReason |  | 1 76 |  | unsigned int |  | 105 Product State Halt |  |
| 28721 | MassActionReason |  | 1 76 |  | unsigned int | 106 Product State Holiday |  |  |
| 28721 | MassActionReason |  | 1 76 |  | unsigned int | 107 Instrument Suspended |  |  |
| 28721 | MassActionReason |  | 1 76 |  | unsigned int |  | 109 | Complex Instrument Deletion |
| 28721 | MassActionReason |  | 1 76 |  | unsigned int |  | 111 | Product temporarily not tradeable |
| 18 | ExecInst | N | 1 2 | 77 | unsigned int Fixed String | Cancellation scope for orders. Quotes are always cancelled by Mass Cancel- lation Events.’No value’ indicates no order cancellation. Value Description |  |  |
| 18 | ExecInst |  | 1 2 |  | unsigned int Fixed String | Cancellation scope for orders. Quotes are always cancelled by Mass Cancel- lation Events.’No value’ indicates no order cancellation. Value Description |  |  |
| 18 | ExecInst |  | 1 2 |  | unsigned int Fixed String | Persistent Order(FIX 1 value- H) |  |  |
| 18 | ExecInst |  | 1 2 |  | unsigned int Fixed String |  | 2 | Non-Persistent Order(FIX value-Q) |
| 18 | ExecInst |  | 1 2 |  | unsigned int Fixed String |  | 3 | Persistent and Non- Persistent Order(FIX value- H,Q) |
| 18 | ExecInst |  | 1 2 |  | unsigned int Fixed String |  |  |  |
| 39020 | Pad2 | N | 1 2 | 78 | unsigned int Fixed String |  | Not Used. |  |

---

## 5.6.11 Trade Notification Status-10501

This message informs about the end of the Trade Notification stream for the respective 
business day.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout. Value: 10501 (Trading-SessionStatus, MsgType = h) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Application  sequence  number  as-signed to a non-order related MCXETI data stream. |
| 28727 | ApplSubID | N | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | 0 False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | 1 True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | 1 Trade |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | 1 Last Message |
| 893 | LastFragment | Y | 1 | 32 | unsigned int |  |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | 1368 TradSesEvent | Y | 1 | 40 | unsigned int | Trading session event type. |
|  | 1368 TradSesEvent | Y | 1 | 40 | unsigned int | Value Description |
|  | 1368 TradSesEvent | Y | 1 | 40 | unsigned int | 104 End of Service |
| 39070 Pad7 |  | N | 7 | 41 | Fixed String | Not Used. |

---

## 5.6.12 Trading Session Event-10307

This message informs about session related events

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout. Value: 10307 (Trading-SessionStatus, MsgType = h) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 21003 | Reserve2 | N | 8 | 8 | UTCTimestamp | Not Used. |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |
| 28727 | ApplSubID | N | 4 | 24 | unsigned int | Only set for Listener Data. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 28704 | ApplMsgID | Y | 16 | 30 | data | Application  message  identifier  as-signed to an order or quote event. |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | 4 Session Data |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | 5 Listener Data |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | False |
| 1352 | ApplResendFlag | Y | 1 | 47 | unsigned int | True |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 48 | unsigned int | 1 Last Message |
| 893 | LastFragment | Y | 1 | 48 | unsigned int |  |
| 39070 | Pad7 | N | 7 | 49 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 1300 | MarketSegmentID | N | 4 | 56 | signed int | Product identifier. |
| 75 | TradeDate | N | 4 | 60 | LocalMktDate | Business date. |

---

|  | Tag Field Name 1368 TradSesEvent | Req’d | Len | Ofs | Data Type |  | Description |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 1368 TradSesEvent | Y | 1 | 64 | unsigned int | Trading session event type. |  |
|  | Tag Field Name 1368 TradSesEvent | Y | 1 | 64 | unsigned int | Value | Description |
|  | Tag Field Name 1368 TradSesEvent | Y | 1 | 64 | unsigned int |  | 101 Start of Service |
|  | Tag Field Name 1368 TradSesEvent | Y | 1 | 64 | unsigned int |  | 102 Market Reset |
|  | Tag Field Name 1368 TradSesEvent | Y | 1 | 64 | unsigned int |  | 103 End of Restatement |
|  | Tag Field Name 1368 TradSesEvent | Y | 1 | 64 | unsigned int |  | 104 End of Service |
|  | Tag Field Name 1368 TradSesEvent | Y | 1 | 64 | unsigned int |  | 105 Service Resumed |
|  | 28722 RefApplLastMsgID | N | 16 | 65 | data | event. | Last  persisted  application  message identifier in case of a Market Reset |
| 39070 Pad7 |  | N | 7 | 81 | Fixed String | Not Used. |  |

---

## 5.6.13 News-10031

The News message provides public information from the MCX market supervision. News messages 
distributed without an ApplSeqNum are not part of the retransmission responses.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  10031 (News,MsgType = B) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | BCHeader> |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |
| 1181 | ApplSeqNum | N | 8 | 16 | unsigned int | Application  sequence  number  as-signed to a non-order related MCXETI data stream. |
| 28727 | ApplSubID | N | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | News |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Last Message |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 42 | OrigTime | Y | 8 | 40 | UTCTimestamp | Creation timestamp of the News mes-sage. |
| 30354 | VarTextLen | Y | 2 | 48 | Counter | Number of used bytes for field VarText(30355). |
| 148 | Headline | Y | 256 | 50 | Fixed String | The headline of a News message.Valid characters:  \x20, \x22-\x7B,\x7D, \x7E |
| 39060 | Pad6 | N | 6 | 306 | Fixed String | Not Used. |
| 30355 | VarText | N | 2000 | 312 | Variable String | News text.Valid characters: \x09, \x0A, \x0D,\x20-\x7B, \x7D, \x7E |

---

**5.6.14 Market Wide OI Violation-8100**

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  8100(MarketWideOIViolation,MsgType = U88) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | BCHeader> |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Application  sequence  number  as-signed to a non-order related MCXETI data stream. |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability andRetransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | News |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Last Message |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 42 | OrigTime | Y | 8 | 40 | UTCTimestamp | Creation timestamp of the News mes-sage. |
| 774 | LastUpdateDate | Y | 8 | 48 | UTCTimestamp | This contains the latest time for whichmessage is received. |

---

|  | Tag Field Name 900 OIPercentageLimit | Req’d Y | Len 8 | Ofs | Data Type | Description | This defines OI% Limit in terms of % Market wide Open interest for partic- ular Asset Type and Instrument Iden- |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 900 OIPercentageLimit | Req’d Y | Len 8 | 56 | floatDecimal6 | tifier | This defines OI% Limit in terms of % Market wide Open interest for partic- ular Asset Type and Instrument Iden- |
|  | 541 MaturityDate | N | 4 | 64 | LocalMktDate | Maturity Date of instrument in terms of seconds from 01-01-1970 00:00:00 hours.When Asset type is 2 – Underly- ing and Security Type is Options (3), this will suggest violation at Option Expiry Level. For other cases, it can be zero. |  |
|  | 30048 SimpleSecurityID | N | 4 | 68 | unsigned int | Instrument Identifier. |  |
|  | 798 OIQuantityLimit | Y | 4 | 72 | unsigned int | This defines the fixed quantity limit of % market wide open interest for particular Asset Type and Instrument Identifier |  |
|  | 800 EffectiveOILimit | Y | 4 | 76 | unsigned int | This defines OI% Limit in terms of % Market wide Open interest for partic- ular Asset Type and Instrument Iden- tifier |  |
|  | 801 ActualOIQuantity | Y | 4 | 80 | unsigned int | This defines actual OI Quantity for Particular Asset Type and Instrument Identifier for which Violation has been |  |
|  | 808 SecurityType | N | 2 | 86 | unsigned int | (30355). |  |
|  |  |  |  |  | Value Description |  |  |
|  |  |  |  |  |  | 0 | Cummulative  at  AssetType Level |
|  |  |  |  |  |  | 1 | Underlying Futures |
|  |  |  |  |  |  | 3 | Options |
|  |  |  |  |  |  | 4 | Futures |
|  |  |  |  |  |  | 5 | Auctions |

---

|  | Tag Field Name 796 AssetType | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 796 AssetType | Y | 1 | 88 | unsigned int |  |  |
|  | Tag Field Name 796 AssetType | Y | 1 | 88 |  | Value | Description |
|  | Tag Field Name 796 AssetType | Y | 1 | 88 |  |  | 0 Asset Class |
|  | Tag Field Name 796 AssetType | Y | 1 | 88 |  |  | 1 Asset |
|  | Tag Field Name 796 AssetType | Y | 1 | 88 |  |  | 2 Underlying |
|  | Tag Field Name 796 AssetType | Y | 1 | 88 |  |  | 3 Product |
|  | 797 ViolationStatus | Y |  | 89 | unsigned int |  |  |
|  | 797 ViolationStatus | Y |  | 89 | unsigned int | Value | Description |
|  | 797 ViolationStatus | Y |  | 89 | unsigned int |  | 0 active Indicates violation sta- |
|  | 797 ViolationStatus | Y |  | 89 | unsigned int |  | tus is Active |
|  | 797 ViolationStatus | Y |  | 89 | unsigned int |  | This means received message 1 is for change of status from active status to violated |
|  | 797 ViolationStatus | Y |  | 89 | unsigned int |  |  |
| 39060 Pad6 |  | N | 6 | 90 | Fixed String | Not used. |  |
|  | 148 Headline | Y | 256 | 96 | Fixed String | The headline of a News message. Valid characters:  \x20, \x22-\x7B, \x7D, \x7E |  |
|  | 30355 VarText | Y | 2000 | 352 | Variable String |  | News text. Actual violation/Alert text Valid characters: \x09, \x0A, \x0D \x20-\x7B, \x7D, \x7E |

---

## 5.6.15 Market Wide OI Alert-8101

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| <M | essageHeaderOut> |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.  Value:  8101(MarketWideOIAlert,MsgType = U89) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <R | BCHeader> |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Application  sequence  number  as-signed to a non-order related MCXETI datastream. |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | News |
| 1180 | ApplID | Y | 1 | 31 | unsigned int |  |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Last Message |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 42 | OrigTime | Y | 8 | 40 | UTCTimestamp | Creation timestamp of the News mes-sage. |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type |  | Description This contains the latest time for which message is received |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | 807 LastModifyDate | Y | 8 | 48 | UTCTimestamp |  | Description This contains the latest time for which message is received |  |
|  | 48 SecurityID | Y | 8 | 56 | signed int |  | Instrument identifier. |  |
|  | 805 AlertPercentage | Y | 8 | 64 | floatDecimal6 |  | Indicates % of each alert level |  |
| 806 ge | UtilizedAlertPercenta- | Y | 8 | 72 | floatDecimal6 |  | Utilized % for particular Asset Type and Instrument Identifier |  |
|  | 541 MaturityDate | N | 4 | 80 | LocalMktDate |  | Maturity Date of instrument in terms of seconds from 01-01-1970 00:00:00 hours.When Asset type is 2 – Underly- ing and Security Type is Options (3), this will suggest violation at Option Expiry Level. For other cases it can be zero. Number of used bytes for field VarText |  |
|  | 30354 VarTextLen | Y | 2 | 84 | Counter | (30355) |  |  |
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  | Value 0 | Description Cummulative  at AssetType level |
|  |  |  |  |  |  |  | 1 3 4 | Underlying Futures Options Futures |
|  | 808 SecurityType | N | 2 | 86 | unsigned int |  | 5 | Auctions |
|  | 796 AssetType | Y | 1 | 88 | unsigned int |  | Value 0 1 2 3 | Description Asset Class Asset Underlying Product |
|  |  |  | 1 6 | 89 |  |  | Value | Description |
|  |  |  | 1 6 | 89 |  |  | 1 | 1st (Lowest) of 3 Alerts set for OI |
|  |  |  | 1 6 | 89 |  |  | 2 | 2nd of 3 Alert set for OI |
|  |  |  | 1 6 | 89 |  |  | 3 | 3rd (Highest) of 3 alerts set for OI |
|  | 804 AlertLevel | Y | 1 6 | 89 | unsigned int |  | 4 of OI |  |
|  |  | U | 1 6 | 90 | Fixed String |  | not used |  |
|  | 148 Headline | Y | 256 | 96 | Fixed String |  | The headline of a News message. Valid characters:  \x20, \x22-\x7B, \x7D, \ x7E |  |
| 30355 | VarText | Y | 2000 | 352 | VariableString |  | News text. Actual violation/Alert text Valid characters: \x09, \x0A, \x0D, Valid characters: \x09, \x0A, \x0D, |  |

---

**5.6.16 Market Status Notification-4125**

| Tag Field Name |  | Req’d | Len | Ofs | Data Type | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout. Value:4125(MarketStatusNotification, MsgType = U90) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
| <RBC | HeaderME> |  |  |  |  |  |  |
| 21003 | Reserve2 | N | 8 | 8 | UTCTimestamp | Matching engine out timestamp. |  |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Gateway response out timestamp. |  |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |  |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |  |
| 28704 | ApplMsgID | N | 16 | 30 | data | Application message identifierassigned to an order or quoteevent. |  |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | value | Description |
| 1180 | ApplID | Y | 1 | 46 | unsigned int | 2 | News |
| 1352 | ApplResendFlag | Y | 1 | 47 | Unsigned int | value | Description |
| 1352 | ApplResendFlag | Y | 1 | 47 | Unsigned int | 0 | False |
| 1352 | ApplResendFlag | Y | 1 | 47 | Unsigned int | 1 | True |
| 893 | Last Fragment | Y | 1 | 48 | Unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedTransaction |  |
| 893 | Last Fragment | Y | 1 | 48 | Unsigned int | value | Description |
| 893 | Last Fragment | Y | 1 | 48 | Unsigned int | 0 | Not Last Message |
| 893 | Last Fragment | Y | 1 | 48 | Unsigned int | 1 | Last Message |
| 39070 | Pad7 | U | 7 | 49 | Fixed String | Not used. |  |
| <Mes | sageBody> |  |  |  |  |  |  |
| 1300 | MarketSegmentID | N | 4 | 56 | Signed int | Product Identifier |  |

---

|  | Tag Field Name 817 GroupID | Req’d N | Len | Ofs 60 | Data Type unsigned int | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 817 GroupID | Req’d N | 2 | Ofs 60 | Data Type unsigned int | group | Nonzero indicates for specific product Description 0 Indicates  applicable  for  all groups |
|  | Tag Field Name 817 GroupID | Req’d N |  | Ofs 60 | Data Type unsigned int | Value | Nonzero indicates for specific product Description 0 Indicates  applicable  for  all groups |
|  | Tag Field Name 817 GroupID | Req’d N |  | Ofs 60 | Data Type unsigned int | Value | Nonzero indicates for specific product Description 0 Indicates  applicable  for  all groups |
|  | Y 2 62 unsigned int |  |  |  |  |  |  |
|  |  |  | 2 64 |  |  | Value Description |  |
|  |  |  | 2 64 |  |  | 1 Open |  |
|  |  |  | 2 64 |  |  | 2 Closed |  |
|  |  |  | 2 64 |  |  | 3 Halt |  |
|  | 819 TradingSessionID | Y |  |  | unsigned int | Not used. |  |
|  |  |  |  | 66 67 | unsigned int | Used. |  |
|  |  |  | 1 5 | 66 67 | unsigned int | Value | Description |
|  |  |  | 1 5 | 66 67 | unsigned int | 1 | Start of Day |
|  |  |  | 1 5 | 66 67 | unsigned int | 2 | Pre-Trading |
|  |  |  | 1 5 | 66 67 | unsigned int | 3 | Trading |
|  |  |  | 1 5 | 66 67 | unsigned int | 4 | Closing or closing auction |
|  |  |  | 1 5 | 66 67 | unsigned int | 5 |  |
|  |  |  | 1 5 | 66 67 | unsigned int |  | Post-Trading |
|  |  |  | 1 5 | 66 67 | unsigned int | 6 | End of Day |
|  |  |  | 1 5 | 66 67 | unsigned int | 7 | Post End of Day |
|  | 625 TradingSessionSubID | Y | 1 5 | 66 67 |  | 8 | Halt |
| 39050 Pad5 |  | N | 1 5 | 66 67 | Fixed String | . Not used |  |

---

**5.6.17 General Messages-Margin Change Notification – 4025**

| TagField Name |  | Req’dLen |  | Ofs | Data Type  Description |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a  ETI mes-sage layout.Value:4025(MarginChangeNotification, MsgType = U91) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |  |
|  |  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |  |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Applicationsequencenumberas- signed to a non-order relatedMCX ETI data stream. |  |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCXsystem during broadcast subscriptioninorder  to link broadcasts to therelated subscription. |  |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |  |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |  |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Value | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | 0 | False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | 1 | True |
| 1180 | ApplID | Y | 1 | 31 | unsigned int |  |  |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Value | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | 1 | Trade |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | 4 | Session Data |
| 1180 | ApplID | Y | 1 | 31 | unsigned int |  |  |

---

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | ValueDescription |
| 893 | LastFragment | Y | 1 | 32 | unsigned int | 1Last Message |
| 893 | LastFragment | Y | 1 | 32 | unsigned int |  |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not used. |
| <Messa | geBody> |  |  |  |  |  |
| 775 | NoMarginChange | Y | 1 | 40 | Counter |  |
| 39070 | Pad7 | N | 7 | 42 | Fixed String | not used |
| <Marg | inChangeRpt | Grp> |  |  |  | Cardinality:0-65534,Record counter:NoMarginChange |
| 743 | InstrumentIdentifier | Y | 8 | 48 | unsigned int | The U/L Asset / producer Identifieris specified by Exchange for permittedU/L Asset / products for trading |
| 777 | OldInitialBuyMargin-Rate | N | 8 | 56 | floatDecimal6 | Initial Margin on Buy position withimplicit 2 decimal |
| 778 | OldInitialSellMargin-Rate | N | 8 | 64 | floatDecimal6 | Initiall Margin on Sell position withimplicit 2 decimal |
| 781 | OldSpecialBuyMargin-Rate | N | 8 | 72 | floatDecimal6 | Special Margin on Buy position withimplicit 2 decimal |
| 782 | OldSpecialSellMargin-Rate | N | 8 | 80 | floatDecimal6 | Special Margin on Sell positionwithimplicit 2 decimal |
| 784 | OldExtremeLossBuy-MarginRate | N | 8 | 88 | floatDecimal6 | Extreme Loss Margin on Buypositionwith implicit 2 decimal |
| 785 | OldExtremeLossSell-MarginRate | N | 8 | 96 | floatDecimal6 | Extreme Loss Margin on Sellpositionwith implicit 2 decimal |
| 787 | NewInitialBuyMargin-Rate | N | 8 | 104 | floatDecimal6 | Initial Margin on Buy position withimplicit 2 decimal |
| 788 | NewInitialSellMargin-Rate | N | 8 | 112 | floatDecimal6 | Initiall Margin on Sell position withimplicit 2 decimal |

---

| Tag | Field Name      Req’d |  | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
| 791 | NewSpecialBuy-MarginRate | N | 8 | 120 | floatDecimal6 | Special Margin on Buy position withimplicit2 decimal |
| 792 | NewSpecialSell-MarginRate | N | 8 | 128 | floatDecimal6 | Special Margin on Sell position withimplicit 2 decimal |
| 794 | NewExtremeLossBuy-MarginRate | N | 8 | 136 | floatDecimal6 | Extreme Loss Margin on Buy positionwith implicit 2 decimal |
| 795 | NewExtremeLossSell-MarginRate | N | 8 | 144 | floatDecimal6 | Extreme Loss Margin on Sell positionwith implicit 2 decimal |
| 776 | OldInitMarginFlatPerc | N | 2 | 152 | unsigned int | Old Initial Margin Percentage |
| 901 | OldInitialMargin-SpreadBenefitonorOff-Flag | N | 2 | 154 | unsigned int |  |
| 901 | OldInitialMargin-SpreadBenefitonorOff-Flag | N | 2 | 154 | unsigned int | Description |
| 901 | OldInitialMargin-SpreadBenefitonorOff-Flag | N | 2 | 154 | unsigned int | Off |
| 901 | OldInitialMargin-SpreadBenefitonorOff-Flag | N | 2 | 154 | unsigned int | On |
| 901 | OldInitialMargin-SpreadBenefitonorOff-Flag | N | 2 | 154 | unsigned int |  |
| 780 | OldSpecialMarginFlat-Perc | N | 2 | 156 | unsigned int | Old Special Margin Percentage |
| 783 | OldConfigurationfor-ExtremeLossMargin | N | 2 | 158 | unsigned int | 1 – Extreme loss margin in percentagewith 2 decimals |
| 786 | NewInitMarginFlat-Perc | N | 2 | 160 | unsigned int | New Initial Margin Percentage |
| 789 | NewInitialMargin-SpreadBenefitOnor-OffFlag | N | 2 | 162 | unsigned int | 0 – Off, 1 – ON |
| 790 | NewSpecialMargin-FlatPerc | N | 2 | 164 | unsigned int | New Special Margin Percentage |
| 793 | NewConfigurationfor-ExtremeLossMargin | N | 2 | 166 | unsigned int | 1 – Extreme loss margin in percentagewith 2 decimals |

## 5.6.18 Service Availability-10030

---

The Service Availability message provides information on the availability of a partition.

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  10030 (User-Notification, MsgType = CB) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Gateway response out timestamp. |
| 28727 | ApplSubID | Y | 4 | 16 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 1180 | ApplID | Y | 1 | 20 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 20 | unsigned int | ValueDescription |
| 1180 | ApplID | Y | 1 | 20 | unsigned int | 3 Service Availability |
| 1180 | ApplID | Y | 1 | 20 | unsigned int |  |
| 893 | LastFragment | Y | 1 | 21 | unsigned int | Indicates whether this message is thelast fragment (part) of a sequence ofmessages belonging to one dedicatedtransaction. |
| 893 | LastFragment | Y | 1 | 21 | unsigned int | Description |
| 893 | LastFragment | Y | 1 | 21 | unsigned int | 0 Not Last Message |
| 893 | LastFragment | Y | 1 | 21 | unsigned int | 1 Last Message |
| 39020 | Pad2 | N | 2 | 22 | Fixed String | Not Used. |
|  |  |  |  |  |  |  |
| 25030 | MatchingEngine-TradeDate | N | 4 | 24 | LocalMktDate | Current business day for Order/QuoteManagement service. |
| 25031 | TradeManagerTrade-Date | N | 4 | 28 | LocalMktDate | Current business day of Trades ser-vice. |
| 25032 | ApplSeqTradeDate | N | 4 | 32 | LocalMktDate | Current business day of Retransmis-sion of Order/Quote Events service. |
| 5948 | PartitionID | Y | 2 | 36 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability and Retransmit requests. |

---

|  | Tag Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | 25005 MatchingEngine- Status | Y | 1 | 38 | unsigned int | Order/Quote  Management service: Informs if trading is active for group- ing of MCX products. Value Description 0 Unavailable 1 Available |
|  | 25005 MatchingEngine- Status | Y | 1 | 38 | unsigned int | Order/Quote  Management service: Informs if trading is active for group- ing of MCX products. Value Description 0 Unavailable 1 Available |
|  | 25005 MatchingEngine- Status | Y | 1 | 38 | unsigned int | Order/Quote  Management service: Informs if trading is active for group- ing of MCX products. Value Description 0 Unavailable 1 Available |
|  | 25005 MatchingEngine- Status | Y | 1 | 38 | unsigned int | Order/Quote  Management service: Informs if trading is active for group- ing of MCX products. Value Description 0 Unavailable 1 Available |
|  | 25005 MatchingEngine- Status | Y | 1 | 38 | unsigned int | Order/Quote  Management service: Informs if trading is active for group- ing of MCX products. Value Description 0 Unavailable 1 Available |
|  | 25006 TradeManagerStatus | Y | 1 | 39 | unsigned int | Trades service: Informs about the availability of the retransmission ser- vice of trades for a grouping of MCX products. |
|  | 25006 TradeManagerStatus | Y | 1 | 39 | unsigned int | Value Description |
|  | 25006 TradeManagerStatus | Y | 1 | 39 | unsigned int | 0 Unavailable |
|  | 25006 TradeManagerStatus | Y | 1 | 39 | unsigned int | 1 Available |
|  | 28732 ApplSeqStatus | Y N | 1 | 40 41 | unsigned int Fixed String | Informs about the availability of the  retransmission services for or- der events (session data and listener data). |
|  |  | Y N |  | 40 41 | unsigned int Fixed String | Value Description |
|  |  | Y N |  | 40 41 | unsigned int Fixed String | 0 Unavailable |
|  |  | Y N |  | 40 41 | unsigned int Fixed String | 1 Available |
| 39070 Pad7 |  | Y N | 7 | 40 41 | unsigned int Fixed String | Not Used. |

---

## 5.6.19 Auction Notification-11028

| Tag Field Name &lt;MessageHeaderOut&gt; |  |  | Len 4 | Ofs 0 | Data Type unsigned int |  | Description Number of bytes for the message, in- cluding this field. |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tag Field Name &lt;MessageHeaderOut&gt; |  |  | Len 4 | Ofs 0 | Data Type unsigned int |  | Description Number of bytes for the message, in- cluding this field. |  |
| 9 | BodyLen | Y | Len 4 | Ofs 0 | Data Type unsigned int |  | Description Number of bytes for the message, in- cluding this field. |  |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int |  | Unique identifier for a  ETI mes- sage layout. Value: 11028 (NotificationAuctionRequest , MsgType = U44) |  |
| 39020 | Pad2 | U | 2 | 6 | Fixed String |  | not used |  |
| &lt;RBCHeader&gt; |  |  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp |  | the gateway | Outgoing timestamp; filled always by |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int |  | Application sequence number as- signed to a non-order related MCX ETI data stream. |  |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int |  | Unique in | ID assigned by the MCX system during broadcast subscription order  to link broadcasts to the related subscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int |  | Grouping of MCX products. Belongs to the scope of Service Avail- ability and Retransmit requests. |  |
| 1352 | ApplResendFlag | Y |  | 30 | unsigned int |  | Indicates a retransmission message. |  |
| 1352 | ApplResendFlag | Y |  | 30 | unsigned int |  | Value Description |  |
| 1352 | ApplResendFlag | Y |  | 30 | unsigned int |  | 0 | False |
| 1352 | ApplResendFlag | Y |  | 30 | unsigned int |  | 1 True |  |

---

| Tag 893 | Field Name LastFragment | Req’d Y | Len 1 | Ofs 32 | Data Type unsigned int | Description |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tag 893 | Field Name LastFragment | Req’d Y | Len 1 | Ofs 32 | Data Type unsigned int | transaction. | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated Description 1 Last Message |
| Tag 893 | Field Name LastFragment | Req’d Y | Len 1 | Ofs 32 | Data Type unsigned int | Value | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated Description 1 Last Message |
| Tag 893 | Field Name LastFragment | Req’d Y | Len 1 | Ofs 32 | Data Type unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated Description 1 Last Message |
| Tag 893 | Field Name LastFragment | Req’d Y | Len 1 | Ofs 32 | Data Type unsigned int |  | Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated Description 1 Last Message |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not used. |  |
| &lt;MessageBody&gt; |  |  |  |  |  |  |  |
| 723 | LstUpdtTime | Y | 8 | 40 | UTCTimestamp | Last updated time. |  |
| 38 | OrderQty | Y | 8 | 48 | Qty |  | Total Order Quantity. |
| 44 | Price | N | 8 | 56 | PriceType |  |  |
| 14 | CumQty | Y | 8 | 64 | Qty | der. | Cumulated executed quantity of an or- |
| 1300 | MarketSegmentID | Y | 4 | 72 | signed int | Product identifier. |  |
| 30048 | SimpleSecurityID | Y | 4 | 76 | unsigned int | Instrument identifier for simple instru- ments. |  |
| 30048 |  |  |  |  |  | Instrument identifier for simple instru- ments. |  |
| 30048 |  |  |  |  |  | Instrument identifier for simple instru- ments. |  |
| 819 | TradingSessionID | Y | 2 | 80 | unsigned int |  | Session No. for the market type |
| 819 |  |  |  |  |  | Specified above for which open message is send. |  |
| 25056 | AuctionEnquiry- TradingStatus | Y | 1 5 | 82 | unsigned int Fixed String |  |  |
| 25056 | AuctionEnquiry- TradingStatus |  | 1 5 | 82 | unsigned int Fixed String | Value Description 0 |  |
| 25056 | AuctionEnquiry- TradingStatus |  | 1 5 | 82 | unsigned int Fixed String |  | Not_Defined |
| 25056 | AuctionEnquiry- TradingStatus |  | 1 5 | 82 | unsigned int Fixed String | 1 | Open |
| 25056 | AuctionEnquiry- TradingStatus |  | 1 5 | 82 | unsigned int Fixed String |  |  |
| 39050 | Pad5 | N | 1 5 | 83 |  | Not used. |  |

---

## 5.6.20 Inquire Pre-Trade Risk Limits Request-10311

This message is sent to the service “Order and Quote Management”.

Inquire Pre-Trade Risk Limits Request

Pre-Trade Risk Limit Response (Session Data)

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a  ETI mes-sage layout. Value: 10311 (PartyRisk-LimitsRequest, MsgType = CL) |
| 25028 | NetworkMsgID | U | 8 | 6 | Fixed String(0-terminable) | not used |
| 39020 | Pad2 | U | 2 | 14 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 34 | MsgSeqNum | Y | 4 | 16 | unsigned int | Message sequence number used by theparticipant for requests sent to thegateway. |
| 50 | SenderSubID | Y | 4 | 20 | unsigned int | User ID. |
|  |  |  |  |  |  |  |
| 1300 | MarketSegmentID | Y | 4 | 24 | signed int | Productidentifier. |
| 20412 | RootParty-IDExecutingTrader | N | 4 | 28 | unsigned int | Owning User ID. |
| 1533 | RiskLimitPlatform | Y | 1 | 32 | unsigned int | Scope for Pre-Trade risk limits |
| 1533 | RiskLimitPlatform | Y | 1 | 32 | unsigned int | Description |
| 1533 | RiskLimitPlatform | Y | 1 | 32 | unsigned int | On-Book |
| 1533 | RiskLimitPlatform | Y | 1 | 32 | unsigned int | Off-Book |
| 1533 | RiskLimitPlatform | Y | 1 | 32 | unsigned int |  |
| 22059 | PartyExecutingUnit | N | 5 | 33 | Fixed String(0-terminable) | Executing BusinessUnit nameValid characters: \x01-\x7E |
| 28775 | RiskLimitGroup | N | 3 | 38 | Fixed String | User Pre-Trade risk groupValid characters: A-Z, 0-9, \x20 |
| 39070 | Pad7 | N | 7 | 41 | Fixed String | Not used. |

---

## 5.6.21 Pre-Trade Risk Limit Response-10313

This message is sent to the service “Order and Quote Management”.

| Tag Field Name |  | Req’d Len |  | Ofs | Data Type   Description |  |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETImes-sage layout.Value:10313(PartyRiskLimitsReport, MsgType =CM) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
| <Re | sponseHeader> |  |  |  |  |  |
| 5979 | RequestTime | Y | 8 | 8 | UTCTimestamp | Gateway request in timestamp. |
| 52 | SendingTime | Y | 8 | 16 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |
| 34 | MsgSeqNum | Y | 4 | 24 | unsigned int | Message sequence number used bytheparticipant for requests sent tothe gateway. |
| 39040 | Pad4 | N | 4 | 28 | Fixed String | Not used. |
| <Me | ssageBody> |  |  |  |  |  |
| 1667 | RiskLimitReportID | Y | 8 | 32 | unsigned int | ID of a PartyRiskLimitReport |
| 20412 | RootParty-IDExecutingTrader | N | 4 | 40 | unsigned int | Owning User ID. |
| 710 F | iller2 | N | 4 | 44 | signed int |  |
| 1669 | NoRiskLimits | Y | 1 | 48 | Counter |  |

---

|  | Tag Field Name 1672 PartyDetailStatus | Req’d Y | Len | Ofs 49 | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  | Tag Field Name 1672 PartyDetailStatus | Req’d Y | 1 | Ofs 49 | unsigned int | Member status. Value Description 0 Active 1 Suspend |
|  | Tag Field Name 1672 PartyDetailStatus | Req’d Y | 1 | Ofs 49 | unsigned int | Member status. Value Description 0 Active 1 Suspend |
|  | Tag Field Name 1672 PartyDetailStatus | Req’d Y | 1 | Ofs 49 | unsigned int | Member status. Value Description 0 Active 1 Suspend |
|  | Tag Field Name 1672 PartyDetailStatus | Req’d Y | 1 | Ofs 49 | unsigned int | Member status. Value Description 0 Active 1 Suspend |
|  | 1533 RiskLimitPlatform | Y | 1 | 50 | unsigned int | Scope for Pre-Trade risk limits |
|  | 1533 RiskLimitPlatform |  | 1 | 50 |  | Value Description |
|  | 1533 RiskLimitPlatform |  | 1 | 50 |  | 0 On-Book |
|  | 1533 RiskLimitPlatform |  | 1 | 50 |  | 1 Off-Book |
|  | 1533 RiskLimitPlatform |  |  | 50 |  |  |
|  | 22259 PartyDetailExecuting- | Y | 5 | 51 | Fixed String | Detail BusinessUnit name |
|  | Unit |  |  |  |  | Valid characters: A-Z, 0-9, \x20 |
| &lt;RiskLimitsRptGrp&gt; |  |  |  |  |  | Cardinality:  0-64,  Record counter: NoRiskLimits |
|  | 28777 &gt;RiskLimitQty | N | 8 | 56 | Qty | Risk limit quantity |
|  | 1300 &gt;MarketSegmentID | Y | 4 | 68 | Signed int | Product Identifier. |
|  | 1530 &gt;RiskLimitType | Y | 1 | 68 | unsigned int | Type of risk limits |
|  | 1530 &gt;RiskLimitType | Y | 1 | 68 | unsigned int | Value Description |
|  | 1530 &gt;RiskLimitType | Y | 1 | 68 | unsigned int | 4 Long limit |
|  | 1530 &gt;RiskLimitType | Y | 1 | 68 | unsigned int | 5 Short limit |
|  | 461 &gt;ProductLine | Y | 1 | 69 | char | F – Future, O – Option Valid characters: A-Z |
|  | 39020 &gt;Pad2 | N | 2 | 70 | Fixed String | Not Used. |

---

**5.6.22 System Information Download-4100**

| Tag | Field Name | Req’d | Len | Ofs | Data Type | Description |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
| 9 | BodyLen | Y | 4 | 0 | unsigned int | Number of bytes for the message, in-cluding this field. |
| 28500 | TemplateID | Y | 2 | 4 | unsigned int | Unique identifier for a MCX ETI mes-sage layout.   Value:  4100(SystemInformation,MsgType = U79) |
| 39020 | Pad2 | U | 2 | 6 | Fixed String | not used |
|  |  |  |  |  |  |  |
| 52 | SendingTime | Y | 8 | 8 | UTCTimestamp | Outgoing timestamp; filled always bythe gateway |
| 1181 | ApplSeqNum | Y | 8 | 16 | unsigned int | Application sequencenumber  as-signed to a non-order related MCXETI data stream. |
| 28727 | ApplSubID | Y | 4 | 24 | unsigned int | Unique ID assigned by the MCX sys-tem during broadcast subscription inorder to link broadcasts to the relatedsubscription. |
| 5948 | PartitionID | Y | 2 | 28 | unsigned int | Grouping of MCX products.Belongs to the scope of Service Avail-ability andRetransmit requests. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Indicates a retransmission message. |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | Description |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | False |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int | True |
| 1352 | ApplResendFlag | Y | 1 | 30 | unsigned int |  |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Identifier for an ETI data stream. |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | Description |
| 1180 | ApplID | Y | 1 | 31 | unsigned int | News |

---

| Tag 893 | Field Name LastFragment | Req’d Y | Len | Ofs | Data Type unsigned int | Description Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tag 893 | Field Name LastFragment | Req’d Y | 1 | 32 | Data Type unsigned int | Description Indicates whether this message is the last fragment (part) of a sequence of messages belonging to one dedicated transaction. |  |
| Tag 893 | Field Name LastFragment | Req’d Y | 1 | 32 | Data Type unsigned int | Value | Description |
| Tag 893 | Field Name LastFragment | Req’d Y | 1 | 32 | Data Type unsigned int | 1 | Last Message |
| Tag 893 | Field Name LastFragment | Req’d Y | 1 | 32 | Data Type unsigned int |  |  |
| 39070 | Pad7 | N | 7 | 33 | Fixed String | Not used. |  |
| &lt;Message Body&gt; |  |  |  |  |  |  |  |
| 40075 | DisclosedQuantity- PercentAllowed | Y | 8 | 40 | Qty |  | Minimum Disclosed Quantity percent- |
| 40076 | TradeModification- TimeIndicator | Y | 2 | 48 | unsigned int | age allowed on current Quantity. 0-In case trade modification acceptance time has not elapsed. 1- In case trade modification acceptance time has elapsed. |  |
|  |  |  |  |  |  | age allowed on current Quantity. 0-In case trade modification acceptance time has not elapsed. 1- In case trade modification acceptance time has elapsed. |  |
| 40077 | DeliveryInstruction- ModificationTime- Indicator | Y | 2 | 50 | unsigned int | 0-In case acceptance time has not elapsed. 1- In case acceptance time has elapsed. |  |
| 818 | Status | Y | 2 2 | 52 | unsigned int Fixed String |  |  |
| 818 |  | Y | 2 2 |  | unsigned int Fixed String | Value Description |  |
| 818 |  | Y | 2 2 |  | unsigned int Fixed String | 1 | Open |
| 818 |  | Y | 2 2 |  | unsigned int Fixed String | 2 | Closed |
| 818 |  | Y | 2 2 |  | unsigned int Fixed String | 3 Halt |  |
| 39020 | Pad2 | N | 2 2 | 54 |  | Not used. |  |

---

## 6. Appendix

**Code Snippet - c++**

Password must be encrypted as per RSA token.

/** The sample code is provided *AS-IS* basis as an reference. The code is not be used in your application 
or system without* fully testing and validating the readiness of your application or system.*/

#include <openssl/pem.h>
#include <openssl/ssl.h>
#include <openssl/rsa.h>
#include <openssl/evp.h>
#include <openssl/bio.h>
#include <openssl/err.h>
#include <stdio.h>
#include <math.h>
#include <iostream>
#include <fstream>
using namespace std;
int padding = RSA_PKCS1_PADDING;
int Base64Encode(const char* message, char** buffer, int length) { //Encodes 
a string to base64
BIO *bio, *b64;
FILE* stream;
int encodedSize = 4*ceil((double)length/3);
*buffer = (char *)malloc(encodedSize+1);
stream = fmemopen(*buffer, encodedSize+1, "w");
b64 = BIO_new(BIO_f_base64());
bio = BIO_new_fp(stream, BIO_NOCLOSE);
bio = BIO_push(b64, bio);
BIO_set_flags(bio, BIO_FLAGS_BASE64_NO_NL); //Ignore newlines -write 
everything in one line
BIO_write(bio, message, length);
BIO_flush(bio);
BIO_free_all(bio);
fclose(stream);

---

**return** (**0**); //success
}
**int calcDecodeLength**(**const char*** b64input) { //Calculates the length of a 
decoded base64 string
**int** len = strlen(b64input);
**int** padding = **0**;
**if** (b64input[len-**1**] == ’=’ && b64input[len-**2**] == ’=’) //last two chars are = 
padding = 2;
**else if** (b64input[len-**1**] == ’=’) //last char is =
padding = **1**;
**return** (**int**)len***0.75**-padding;
}
int Base64Decode(char* b64message, char** buffer) { //Decodes a base64 
encoded string BIO *bio, *b64;
**int** decodeLen = calcDecodeLength(b64message),
len = **0**;
*buffer = (**char***)malloc(decodeLen+**1**);
**FILE*** stream = fmemopen(b64message, strlen(b64message), "r"); b64 = 
BIO_new(BIO_f_base64());
bio = BIO_new_fp(stream, BIO_NOCLOSE);
bio = BIO_push(b64, bio);
BIO_set_flags(bio, BIO_FLAGS_BASE64_NO_NL); //Do not use newlines to flush 
buffer len = BIO_read(bio, *buffer, strlen(b64message));
//Can test here if len == decodeLen -if not, then return an error
(*buffer)[len] = ’\**0**’;
BIO_free_all(bio);
fclose(stream);
**return** (**0**); //success
}
RSA * **createRSAWithFilename**(**char** * filename,**int** spublic)
{
**FILE** * fp = fopen(filename,"rb");
246
Confidential

---

**if**(fp == NULL)
{
printf("Unable to open file %s **\n**",filename);
**return** NULL;
}
RSA *rsa= RSA_new() ;
**if**(spublic)
{
rsa = PEM_read_RSA_PUBKEY(fp, &rsa,NULL, NULL);
}
**else**
{
rsa = PEM_read_RSAPrivateKey(fp, &rsa,NULL, NULL);
}
**return** rsa;
}
//Function encrypts the value returns length of encrypted text
**int public_encrypt**(**unsigned char** * data,**int** data_len, **unsigned char**
*encrypted)
{
RSA * rsa = 
createRSAWithFilename((**char***)"/home/a1783558/public.pem",**1**);//Path of public 
key int result = RSA_public_encrypt(data_len,data,encrypted,rsa,padding); 
return result;
}
**void printLastError**(**char** *msg)
{
**char** * err;
ERR_load_crypto_strings();
ERR_error_string(ERR_get_error(), err);
printf("%s ERROR: %s**\n**",msg, err);

---

free(err);
}
**int main**(){
//Text to be encrypted
**char** plainText[**2048**/**8**] = "Hello this is Ravi"; //key length : 2048 unsigned 
char encrypted[4098]={};
**int** encrypted_length= public_encrypt((**unsigned**
**char***)plainText,strlen(plainText),encrypted);
**if**(encrypted_length == -**1**){
printLastError("Public Encrypt failed ");
exit(**0**);
}
printf("Encrypted length =%d**\n**",encrypted_length);
//Copy char array to pointer
**char** * enc = (**char***)encrypted;
//Base64 Encoding
Base64Encode(enc ,&enc, encrypted_length);
cout<<enc<<"**\n**";
}
**Code Snippet - java**

Password has to be encrypted as per RSA token.

/** The sample code is provided *AS-IS* basis as an reference. The code is not be used in your application 
or system without* fully testing and validating the readiness of your application or system.*/

**package** com.mcx;
**import javax.crypto.BadPaddingException**;
**import javax.crypto.Cipher**;
**import javax.crypto.IllegalBlockSizeException**;
**import javax.crypto.NoSuchPaddingException**;
**import java.nio.charset.StandardCharsets**;
**import java.security.***;
**import java.security.spec.InvalidKeySpecException**; 
**import java.security.spec.PKCS8EncodedKeySpec**;
**import java.security.spec.X509EncodedKeySpec**;
**import java.util.Base64**;
**public class SampleCodeJava** {

**public static void main**(String[] args) **throws** NoSuchAlgorithmException, 
NoSuchPaddingException, InvalidKeySpecException, //Ciphper encoding

Confidential

---

```java
Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");

String clearTextPwd = "MyPassword";

PublicKey publicKey = loadPublicKey();

cipher.init(Cipher.ENCRYPT_MODE, publicKey);

byte[] encrypted =
cipher.doFinal(clearTextPwd.getBytes(StandardCharsets.UTF_8)); String
encryptedPwd = Base64.getEncoder().encodeToString(encrypted)

System.out.println("ClearText Password: " + clearTextPwd);

System.out.println("Encrypted Password: " + encryptedPwd);

}

private static PublicKey loadPublicKey() throws
InvalidKeySpecException, NoSuchAlgorithmException {

//      The below working sample public key provided as inline key. However
in production region, PEM encoded Public Key of MCX to be downloaded from
MCX FTP server.

String publicKeyPEM =
"MIIIiJiNBgkqkhjG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0gzs2VUrYlrnDPDrsSJf\n" +
"G2VGjos7/WuF5LWn28O0ET/RWkq4R1PiQCG8Dur15u+KuBt3vIQTE4n7S/Q5Ft1U\n" +
"p2/QWjqchKhnnIAeblpB1IXTdGdizZLakSSFN6+H661WFPkYL8d+F8Koizt2KBC/\n" +
"4D15RR8hHeIKneeSa23S8YsA06blqmFw5uL95LNtL9dtXNzw+sIt72iYcmM14cC2\n" +
"e8tNgcG0GWYqV4N4MGAAdX0gHaZtsEDM1UU4D6fM1tU5ddlfLtGPEqAcyaY13znSVW\n" +
"XU2jeORt9zdfJ2+48dylWc6slfJ003sQBVWytM1L09TIXZ/v+DK4kAeIQlxI3BaU\n" +
"bQIDAQAB";

publicKeyPEM = publicKeyPEM.replaceAll("\s", "");

// decode to get the binary DER representation

byte[] publicKeyDER = Base64.getDecoder().decode(publicKeyPEM); KeyFactory
keyFactory = KeyFactory.getInstance("RSA");

PublicKey publicKey = keyFactory.generatePublic(new
X509EncodedKeySpec(publicKeyDER)); return publicKey;

}
}
```

Code Spinnet - CShern

## Code Snippet - CSharp

Password has to be encrypted as per RSA token.

/** The sample code is provided *AS-IS* basis as an reference. The code is not be used in your application 
or system without* fully testing and validating the readiness of your application or system.*/

**using System**;
**using System.Text**;
**using Org.BouncyCastle.Crypto**;
**using Org.BouncyCastle.OpenSsl**;

---

```csharp
using Org.BouncyCastle.Crypto.Encodings;
using Org.BouncyCastle.Crypto.Engines;
using System.IO;

namespace SampleCodeCSharp

{

class Program

{

public static string publicKey = "-----BEGIN PUBLIC KEY----MIIBIjANB" +

"gkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0gzs2VUrYlrnDPDrsSJfG2VGjos7/WuF" +
"5LWn2T00ET/RWkq4R1PiQCG8Durl5u+KuBt3vIQTE4n7S/Q5Ft1Up2/QWjQchKhnnIAebl" +
"pB1IXTdGIdizZLakSSFN6+H661WFPkYL8d+F8Koizt2KBC/4D15RR8hHeIKneeSa23S8" +
"YsA06blqmFw5uL95LNtL9dtXNzw+sit72iYcmM14cC2e8tNGCG0GWYqV4N4MGAdX0gHaZt" +
"sEDM1UU4D6fM1tU5ddfLtGPEqAcYa13znSVWXU2jeORt9zdfJ2+48dylWc6slfJ003sQB" +
"VWytM1L09TIXZ/v+DK4kAeIQlxI3BaUbQIDAQAB----END PUBLIC KEY----";

public static string PasswordPlainText = "MyPassword";

static void Main(string[] args)

{

Program prg = new Program();

string EncryptedText = prg.RsaEncryptWithPublic(PasswordPlainText,
publicKey);

Console.WriteLine(EncryptedText);

Console.ReadLine();

}

public string RsaEncryptWithPublic(string clearText, string publicKey)

{

try

{

var bytesToEncrypt = Encoding.UTF8.GetBytes(clearText);

var encryptEngine = new PkcslEncoding(new RsaEngine());

using (var txtreader = new StringReader(publicKey))

{

var keyParameter = (AsymmetricKeyParameter)new
PemReader(txtreader).ReadObject(); encryptEngine.Init(true, keyParameter);

}

---

**var** encrypted = 
Convert.ToBase64String(encryptEngine.ProcessBlock(bytesToEncrypt, **0**, 
bytesToEncrypt.Length)) **return** encrypted;
}
**catch** (Exception ex)
{
**throw** ex;
}
}
}
}

## 7. Annexure for Encryption/Decryption

For symmetric encryption/decryption methodology –**A. Encryption:**

Initialization→
void encrypt_EVP_aes_256_gcm_init(EVP_CIPHER_CTX **ctx, unsigned char
*key, unsigned char *iv)
{
if(!(*ctx = EVP_CIPHER_CTX_new()))
handleErrors();
if(1 != EVP_EncryptInit_ex(*ctx, EVP_aes_256_gcm(), NULL, key, iv))
handleErrors();
}
Encryption→
void encrypt(EVP_CIPHER_CTX *ctx, unsigned char *plaintext, int
plaintext_len, unsigned char *ciphertext, int *ciphertext_len)
{
int len;
if(1 != EVP_EncryptUpdate(ctx, ciphertext, &len, plaintext, plaintext_len))
handleErrors();
*ciphertext_len = len;
}
**B. Decryption:**
Initialization→
void decrypt_EVP_aes_256_gcm_init(EVP_CIPHER_CTX **ctx, unsigned char
*key, unsigned char *iv)
{
if(!(*ctx = EVP_CIPHER_CTX_new()))
handleErrors();
if(1 != EVP_DecryptInit_ex(*ctx, EVP_aes_256_gcm(), NULL, key, iv))
handleErrors();
}

---

Decryption→
int decrypt(EVP_CIPHER_CTX *ctx, unsigned char *ciphertext, int
ciphertext_len, unsigned char *plaintext, int *plaintext_len)
{
int len;
if(1 != EVP_DecryptUpdate(ctx, plaintext, &len, ciphertext,
ciphertext_len))
handleErrors();
*plaintext_len = len;
}

## Example: Encryption using GCM mode

## Code Snippet - C++

int gcm_encrypt(unsigned char *plaintext, int plaintext_len,
unsigned char *aad, int aad_len,
unsigned char *key,
unsigned char *iv, int iv_len,
unsigned char *ciphertext,
unsigned char *tag)
{
EVP_CIPHER_CTX *ctx;
int len;
int ciphertext_len;
/* Create and initialise the context */
if(!(ctx = EVP_CIPHER_CTX_new()))
handleErrors();
/* Initialise the encryption operation. */
if(1!= EVP_EncryptInit_ex(ctx, EVP_aes_256_gcm(), NULL, NULL, NULL))
handleErrors();
/*
* Set IV length if default 12 bytes (96 bits) is not appropriate
*/
if(1!= EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_IVLEN, iv_len, NULL))
handleErrors();
/* Initialise key and IV */
if(1!= EVP_EncryptInit_ex(ctx, NULL, NULL, key, iv))
handleErrors();
/*
* Provide any AAD data. This can be called zero or more times as
* required
*/
if(1!= EVP_EncryptUpdate(ctx, NULL, &len, aad, aad_len))

---

handleErrors();
/*
* Provide the message to be encrypted, and obtain the encrypted output.
* EVP_EncryptUpdate can be called multiple times if necessary
*/
if(1!= EVP_EncryptUpdate(ctx, ciphertext, &len, plaintext, plaintext_len))
handleErrors();
ciphertext_len = len;
/*
* Finalise the encryption. Normally ciphertext bytes may be written at
* this stage, but this does not occur in GCM mode
*/
if(1!= EVP_EncryptFinal_ex(ctx, ciphertext + len, &len))
handleErrors();
ciphertext_len += len;
/* Get the tag */
if(1!= EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_GET_TAG, 16, tag))
handleErrors();
/* Clean up */
EVP_CIPHER_CTX_free(ctx);
return ciphertext_len;
}

## Code Snippet – C#

public byte[] AESGCMEncryptionNew(byte[] dataToEncrypt)
{
byte[] resultWithNoTag;
GcmBlockCipher cipher = new GcmBlockCipher(new AesFastEngine());
cipher.Init(true, new AeadParameters(new 
KeyParameter(Encoding.UTF8.GetBytes("ABCGFEDTHGYJSTFGABCGFEDTHGYJSTFG")), 128, 
Encoding.UTF8.GetBytes("dsfrehbndrft")));
byte[] encryptedResult = new 
byte[cipher.GetOutputSize(dataToEncrypt.Length)];
int lenRes = cipher.ProcessBytes(dataToEncrypt, 0, 
dataToEncrypt.Length, encryptedResult, 0);
cipher.DoFinal(encryptedResult, lenRes);
resultWithNoTag = new byte[encryptedResult.Length -16];
Array.Copy(encryptedResult, resultWithNoTag, 
resultWithNoTag.Length);
return resultWithNoTag;
}

## Example: Decryption using GCM mode

**Code Snippet - C++** int gcm_decrypt(unsigned char *ciphertext, int ciphertext_len,
unsigned char *aad, int aad_len,
unsigned char *tag,
unsigned char *key,
unsigned char *iv, int iv_len,
unsigned char *plaintext)
{
EVP_CIPHER_CTX *ctx;
int len;
int plaintext_len;
int ret;
/* Create and initialise the context */
if(!(ctx = EVP_CIPHER_CTX_new()))
handleErrors();
/* Initialise the decryption operation. */
if(!EVP_DecryptInit_ex(ctx, EVP_aes_256_gcm(), NULL, NULL, NULL))
handleErrors();
/* Set IV length. Not necessary if this is 12 bytes (96 bits) */
if(!EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_IVLEN, iv_len, NULL))
handleErrors();
/* Initialise key and IV */
if(!EVP_DecryptInit_ex(ctx, NULL, NULL, key, iv))
handleErrors();
/*
* Provide any AAD data. This can be called zero or more times as
* required
*/
if(!EVP_DecryptUpdate(ctx, NULL, &len, aad, aad_len))
handleErrors();
/*
* Provide the message to be decrypted, and obtain the plaintext output.
* EVP_DecryptUpdate can be called multiple times if necessary
*/
if(!EVP_DecryptUpdate(ctx, plaintext, &len, ciphertext, ciphertext_len))
handleErrors();
plaintext_len = len;
/* Set expected tag value. Works in OpenSSL 1.0.1d and later */
if(!EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_TAG, 16, tag))
handleErrors();
/*
* Finalise the decryption. A positive return value indicates success,
* anything else is a failure -the plaintext is not trustworthy.
*/
ret = EVP_DecryptFinal_ex(ctx, plaintext + len, &len);

---

/* Clean up */
EVP_CIPHER_CTX_free(ctx);
if(ret > 0) {
/* Success */
plaintext_len += len;
return plaintext_len;
} else {
/* Verify failed */
return -1;
}
}

## Code Snippet – C#

public byte[] AESGCMDecryptionNew(byte[] dataToDecrypt)
{
byte[] resultWithNoTag;
GcmBlockCipher cipher = new GcmBlockCipher(new AesFastEngine());
cipher.Init(true, new AeadParameters(new 
KeyParameter(Encoding.UTF8.GetBytes("ABCGFEDTHGYJSTFGABCGFEDTHGYJSTFG")), 128, 
Encoding.UTF8.GetBytes("dsfrehbndrft")));
byte[] decryptedResult = new 
byte[cipher.GetOutputSize(dataToDecrypt.Length)];
int lenRes = cipher.ProcessBytes(dataToDecrypt, 0, 
dataToDecrypt.Length, decryptedResult, 0);
cipher.DoFinal(decryptedResult, lenRes);
resultWithNoTag = new byte[decryptedResult.Length -16];
Array.Copy(decryptedResult, resultWithNoTag, 
resultWithNoTag.Length);
return resultWithNoTag;
}
