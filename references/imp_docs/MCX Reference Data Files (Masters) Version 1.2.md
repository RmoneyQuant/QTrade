Circular no.: MCX/CTCL/123/2026 March 10, 2026 ________________________________________________________________________

#### MCX Reference Data Files (Masters) Version 1.2

_________________________________________________________________________

In terms of provisions of the Rules, Bye-Laws and Business Rules of the Exchange and in continuation to Exchange circular no. MCX/CTCL/506/2021 dated August 10, 2021, and Circular no.: MCX/CTCL/078/2024 February 08, 2024. Members of the Exchange are notified as under:

The Exchange has released new MCX Reference Data Files (Masters) Version 1.2 with changes mentioned in document details. These changes apply with immediate effect

In case of any queries or clarification on Reference Data Files, trading members/vendors are requested to get in touch with the following contact details:

- Email – <u>ctcl@mcxindia.com</u>
- Phone: +91 22 – 6649 4040 / 6731 8888
Trading Members and Empanelled vendors are requested to take note of the same.

For and on behalf of Multi Commodity Exchange of India Ltd.

Abhay Angarkar VP-Technology Encl.: As above

Kindly contact Customer Service Team on 022 – 6649 4040 or send an email at customersupport@mcxindia.com for any clarification.

------------------------------------------------------- Corporate office ---------------------------------------------------------- Multi Commodity Exchange of India Limited Exchange Square, CTS No. 255, Suren Road, Chakala, Andheri (East), Mumbai – 400 093 Tel.: 022 – 6649 4000 Fax: 022 – 6649 4151 www.mcxindia.com email: customersupport@mcxindia.com

##### Public

##### MCX Reference Data Files

### Multi Commodity Exchange of India Limited

# Trading Interface-MCX Reference Data Files (Masters)

MCX

#### Version 1.2 March 10, 2026

##### Confidential Confidential Confidential

##### Public

All trademarks that appear in the document have been used for identification purposes only and belong to their

##### Copyright

respective companies.

##### Document details

##### Name Version no.

MCX_Trading_Interfaces-V 1.0 Reference_Data_Files

MCX_Trading_Interfaces-V 1.1 Reference_Data_Files

MCX_Trading_Interfaces-V 1.2 Reference_Data_Files

##### Description

Reference Data Files consist of file format of masters i.e. Instrument Master, participant master, asset master etc.

Section 1.1 - Instrument Master – Changes with respect to Daily Price range.

##### Section 1.1 - Instrument Master –

-Instrument PartitionID-Description updated-The PartitionID will also represents the EOBI and EMDI stream identifier. -Trading unit factor – Description updated. -Group Id-Description added. -Removed o Currency Master o Currency Pair master

##### MCX Contact Details

Multi Commodity Exchange of India Limited Exchange Square, Suren Road, Chakala, Andheri (East), Mumbai 400 093. <u>www.mcxindia.com</u>

purposes.

Tel: +91 – 22 – 66494000 / 67318888 Fax: +91 – 22 – 66494151 Email – apisupport@mcxindia.com

Confidential

##### Public

##### <u>The Exchange’s Member & Vendor may contact Technology Division to seek clarification at:</u>

##### Restriction on Use and Disclaimer of Information and Data

All The information contained in this document constitutes a trade secret and/or information that are commercial or financial and confidential or privileged. It is furnished in confidence with the understanding that it will not, without the prior written permission of MCX, be used or disclosed for other than allowed

The copyright in this work may be vested with MCX and / or its suppliers. No part of this document may be copied, reproduced, stored in a retrieval system, or transmitted, in any form or by any means whether, electronic, mechanical, or otherwise without the prior written permission of MCX.

The recipient acknowledges that MCX and its suppliers may have copyright in the work. The recipient further agrees that the work is confidential information and contains proprietary MCX information belonging to MCX and / or its suppliers. The recipient manifests, by its receipt of the work, its acknowledgment of MCX and / or its suppliers copyright in the work, its acceptance that the work is confidential information, and its compliance with the terms contained in this notice.

Although MCX has made every effort to provide accurate information at the date of publication, it does not give any representations or warranties as to the accuracy, reliability or completeness of the information in this document. Accordingly, MCX, its subsidiaries and their employees, officers and contractors and its suppliers shall not, to the extent permitted by law, be liable for any direct or indirect loss arising in any way (including by way of negligence) from or in connection with anything provided in or omitted from this document or from any action taken, or inaction, in reliance on this document.

MCX reserves the right to amend details in this document at any time and without notice. Strictly for private circulation only. This document must not be circulated to other users without prior permission of MCX.

##### Public

## Contents

1 FILE LAYOUTS/FORMATS .................................................................................................................................. 6
INSTRUMENT MASTER...................................................................................................................................... 6
PARTICIPANT MASTER .................................................................................................................................... 13
ASSET MASTER ............................................................................................................................................. 14
ASSET UNDERLYING MAPPING ......................................................................................................................... 15

**Public**

#### 1 File Layouts/Formats

Exchange will upload following list of masters in SFTP drive, which members are required to connect and download.

##### Instrument Master

The instrument master shall be made available through FTP to the members connecting through Open Interface. No messaging download for instrument master shall be provided

##### File Name: MCXScrips.bcp File Type: Comma Separated File

Note: All values are captured in ASCII format. Data Type column indicates field value type like Integer, Char or

decimal

##### Description

|Field Description|Data Type|
|---|---|
|Filler2|Int 2 Bytes|
|Filler4|Int 4 Bytes|

Each MCX product is processed on exactly one partition; a partition is a grouping of products. To optimize the routing to the corresponding partition, the product identifier needs to be provided in each order and quote transaction by the participant.

The PartitionID will also represent the EOBI and EMDI stream identifier.

##### Instrument PartitionID

Int 2 Bytes Filler8 Char (8) Filler2 Int 2 Bytes

Instrument Identifier Int 4 Bytes

Symbol Char (12) Instrument Series Char (3)

##### Instrument Type

##### Int 2 Bytes

Permit Trading 1 Byte Filler4 Int 4 Bytes

ProductID

##### Int 4 Bytes

Bandhani Range (25) Char (25) Filler1 1 Byte Filler1 1 Byte

The U/L Asset / product Identifier is specified by Exchange for permitted U/L Assets / Products for trading The U/L Asset / product Code is specified by Exchange for permitted U/L Assets / Products for trading Series identification of the instrument. This is specified as “XX” 1-Underlying 2-Spot 3-Options 4-Futures 5-Auction 0-Trading not allowed 1-Trading allowed

Unique numeric Identifier for Product [Corresponds to field MarketSegmentID(1300) in the ETI API]. Example-FUTCOM GOLD has Product Name as FGOLD and ID as 21 Optional If defined, will indicate range of minimum low to maximum high a product can be quoted at.

##### Public

|Field Description|Data Type|Description|
|---|---|---|
|Filler1|1 Byte||
|Filler1|1 Byte||
|Instrument Start Date|Int 4 Bytes|First Trading date of the product. Date in terms of seconds from 01- 01-1970 00:00:00 hrs. in IST|
|Filler4|Int 4 Bytes||
|Last Trading Date|Int 4 Bytes|Last Trading date of the product. Date in terms of seconds from 01- 01-1970 00:00:00 hrs. The time would result to 23:59:59 hrs|
|Lot Size|Int 4 Bytes|Size of Lot in whose multiple order should be placed.|
|Tick Size Instrument Description|Int 4 Bytes|Amount in paise in whose multiple price should be specified. Description of the instrument to give additional information to|
||Char (25)|Product Code|
|CapacityGroupID|Int 4 Bytes|ID is defined for set of commodities within which multi leg orders are allowed|
|Filler4|Int 4 Bytes||
|Filler4|Int 4 Bytes||
|Delivery Start Date Delivery End Date|Int 4 Bytes Int 4 Bytes|First Date from which delivery shall be accepted for the product. In terms of seconds from 01-01-1970 00:00:00 hrs. in IST Last Date from which delivery shall be accepted for the product. In terms of seconds from 01-01-1970 00:00:00 hrs. The time would result to 23:59:59 hrs in IST|
|Filler1|1 Byte||
|Trade2Trade Indicator|1 Byte|Will either have a value of 0 or 1 0 will imply not in T2T and 1 will imply that the product is in T2T|
|Index Flag|Int 2 Bytes|0-is an index participant 1-is not an index participant 1-Default Index to be displayed in TWS index bar (Where: Index|
|Default Index||Instrument is also set to 1)|
||Int 2 Bytes|Anything other than 1 means index is not the default index|
|Index Instrument Feed Flag|Int 2 Bytes Int 2 Bytes|0-is an index participant 1-is not an index participant 1-on, 0-Off External Feed Instrument Flag. When Instrument Deleted is marked as “Y” and Feed Flag is also marked as 1 then consider this instrument as non-tradable instrument and prices are shown for this instrument is been taken from other sources.|
|Filler1|1 Byte||
|Filler1|1 Byte||
|Filler1|1 Byte||
|Last Modified Date Instrument Status flag|Int 4 Bytes|Date and time when the instrument was last modified In terms of seconds from 01-01-1970 00:00:00 hrs. ‘N’ - Active ‘Y’ - Inactive or deleted. ‘S’ - Suspended|
||Char (1)|‘D’ – Delisted|
|Instrument Info|Char (40)|Description of the product as provided by the Exchange|

##### Public

##### Field Description Data Type Description

The minimum quantity for which order can be placed for the Minimum Lot instrument. The quantity should be incremented in multiple of this Int 2 Bytes lot First Date uptpo which delivery intention shall be accepted for the Tender Period Start Date Int 4 Bytes product. In terms of seconds from 01-01-1970 00:00:00 hrs. Last Date utpo which delivery intention shall be accepted for the Tender Period End Date product. In terms of seconds from 01-01-1970 00:00:00 hrs. The time Int 4 Bytes would result to 23:59:59 hrs Group under which U/L Asset is classified by the exchange

|U/L Asset Group|Char (25)|
|---|---|
|Name of Underlying U/L||
|Asset Identifier of the|Char (10)|
|underlying|Int 4 Bytes|
|Filler4|Int 4 Bytes|
|Filler4|Int 4 Bytes|
|Filler1|1 Byte|
|Filler1|1 Byte|
|Filler1|1 Byte|
|Filler1|1 Byte|
|Filler1|1 Byte|
|Instrument Name|Char (6)|

Name of U/L Asset n which instrument is created

##### The identifier code assigned by exchange

As defined by exchange Date of expiry by which product is created the date would be 01- 01- 1970 00:00:00 hrs. This depicts the expiry date which is shown along with the product. Original Expiry Date This date does not change even if the product last trading date is changed. Datetime Not to refer in case where Instrument Type is 2 Strike price Int 4 Bytes Applicable only for option instrument Applicable only for option instrument Option Type CA – Call option American, PA – put option American, CE – Call Char (2) option European, PE – Put option European Applicable only for option instrument

|CA level|Int 2 Bytes|
|---|---|
|Segment ID Additional Lean Period|Int 2 Bytes|
|Margin|Int 2 Bytes|
|Filler2|Int 2 Bytes|
|Price quote unit|Char (5)|

13 - Underlying, 12 - Products

1 - Margin in %age. 2 - Flat margin per quantity in terms of paise

Unit in which price for the product is quoted Price in which price for the product is quoted Price Quote quantity Quantity for which Price is being quoted. To be read with Price Int 4 Bytes Quote Unit.

0 - Not applicable, 1 - DPR in % age, 2 - DPR as flat in per quantity in Terms of daily price paise term, 3 - DPR for option products, 4 - Statistical DPR for options range Int 2 Bytes product Upper Daily price range to be computed with previous days close Upper Daily price range Numeric (20.4) price.

##### Public

##### Field Description Data Type Description

For existing option contracts (statistical DPR), Upper Daily Price Range is computed using End Of Day RPF values. For new option contracts/strikes added, provisional upper price range will be available. The same will be updated at the beginning of the day through market data. Lower Daily price range to be computed-with previous days close price. For existing option contracts (statistical DPR), Lower Daily Price

|Lower Daily price range||Range is computed using End of Day RPF values. For new option|
|---|---|---|
||Numeric (20.4)|contracts/strikes added, provisional lower price range will be available. The same will be updated at the beginning of the day through market data.|
|Tender Period Indicator||1 - Tender period available,|
||Int 2 Bytes|2 - Tender period not available|
|Settlement method||1 – Delivery settled,|
||Int 2 Bytes|2 – Cash settled|
|Terms of Initial Margin||1 – Margin in %age,|
||Int 2 Bytes|2 – Flat margin per quantity in terms of paise|
|Buy Initial margin rate|Numeric (20,4)|Buy initial margin to be computed for the product|
|Base Price Maximum single|Int 4 Bytes|Base price of the product for first day DPR|
|transaction quantity|Int 4 Bytes|Maximum quantity permitted for single order for product|
|Maximum single|Numeric||
|transaction value|Pwd (20,4)|Maximum order value permitted for single order for product based on last traded price|
|Instrument class|Int 2 Bytes|Classifier identification for the group of products to be read with instrument name and instrument type|
|Near month instrument||Applicable only for spread instruments, indicating the near month|
|identifier|Int 4 Bytes|product for the spread. - 1 will indicate for non-spread \ non instrument|
|Far month instrument||Applicable only for spread instruments, indicating the far month|
|identifier|Int 4 Bytes|product for the spread. - 1 will indicate for non-spread \ non instrument|
|Trading unit||Unit in which trading is done.|
||Char (5)|For example, Gold is quoted per 10 Grams but traded in Kgs. Factor by which the Trading unit should be multiplied to arrive at|
|Trading unit factor||Quote unit. For example, 1000 is trading Unit to convert Kgs to|
||Numeric (20,4)|Grams.|
|Delivery Unit||Unit in which delivery shall be affected|
||Char (5)|E.g.: Gold is quoted per 10 Grams, but is delivered in Kgs.|
|Delivery unit factor|Numeric (20,4)|Factor by which each lot should be multiplied to arrive at delivery unit|
|Price Numerator|Numeric (20,4)|Used for deriving the trade value|
|Specification|Char (100)|Brief product specification|
|Price denominator|Numeric (20,4)|Used for deriving the trade value|
|General Numerator|Numeric (20,4)|Used for deriving the trade value|
|General denominator|Numeric (20,4)|Used for deriving the trade value|
|Lot Numerator|Numeric (20,4)|Used for deriving the trade quantity|

##### Public

|Field Description|Data Type|Description|
|---|---|---|
|Lot Denominator|Numeric (20,4)|Used for deriving the trade quantity|
|Decimal Locator|Numeric (10,5)|Multiplier with price to get the value in quote currency|
|Filler2|Char (2)||
|Filler15|Numeric (15,0)||
|Filler4|Int 4 Bytes||
|Filler50|Char (50)||
|Additional Lean Period|||
|Margin (Sell)|Numeric (15,4)|Additional Lean Period Margin (Sell) to be computed for the Product|
|Spread Benefit on|||
|Additional Lean Period||Spread Benefit on Additional Lean Period Margin as|
|Margin|Numeric (15,4)|percentage/amount|
|Sell Initial margin rate ProductName|Numeric (20,4)|Sell initial margin to be computed for the product Short code of the futures or options product|
||Char (12)|Example FGOLD (for Future on Gold), OGOLD ( for Option on GOLD)|
|Filler2|Int 2 Bytes||
|Terms of special margin||1-Margin in %age.|
||Int 2 Bytes|2-Flat margin per quantity in terms of paise|
|Buy Special Margin Rate|Numeric (20,4)|Buy Special Margin to be computed for the Product|
|Sell Special Margin Rate|Numeric (20,4)|Sell Special Margin to be computed for the Product|
|Initial Margin Spread||0 – Off|
|Benefit Flag||1 - On|
||Int 2 Bytes|This is applicable at the underlying level.|
|Instrument End||Date – Time Combination depicting Instrument End|
|Date – Time Trading Currency|Int 4 Bytes Char (3)|Date – Time Currency ISO Code in which trading will take place i.e. USD, INR etc.|
|Filler3|Char (3)||
|Product Month|Char (7)||
|Pre Open Allowed||0 – Pre Open not Allowed.|
||Int 2 Bytes|1 – Pre Open-Allowed. Group ID of the trading schedule to which the product is tagged to. 0 – AUCTION/Miscellaneous|
|Group Id||1 – AGRI-Closes 5 PM 2 – BULLION, METAL, ENERGY, OTHERS-Closes 11:30 / 11:55 PM (Day Light Saving)|
||Int 2 Bytes|3 – OIL-Closes 9 PM|
|Matching Type|Int 2 Bytes|0 = Normal (Price Time Priority )|
|Spread Type|Int 2 Bytes|Please refer to the description Rules for Spread Type Combination|
|Filler16|Char (16)||
|Value Method||1 – Trade Value should be computed as existing|
||Char|(Calculate according to Method 1)|
|Additional Lean Period|||
|Margin (Buy)|Numeric (20,4)|Additional Lean Period Margin (Buy) to be computed for the Product|
|SLBM Eligibility|Byte|Should always be 0|

##### Public

##### Field Description Data Type Description

Terms of Extreme Loss Margin Int 2 Bytes 1 – Margin in %age Buy Extreme Loss Margin Rate Numeric (20,4) Buy Extreme Loss Margin to be computed for the Product Sell Extreme Loss Numeric (20,4) Sell Extreme Loss Margin to be computed for the Product For Instrument Type 'Options' (ie Instrument Type = 3) Following Value will be provided:

#### 0-Black Scholes

Options Pricing Model 3 - Black76 4 - Bachelier For Instrument Type other than Options, Value will be – 1 Int 2 Bytes (i.e. Not Applicable) This will contain the delivery mode of the product It will contain the following values: 0 – Both Delivery Mode 1 – Sellers Option 2 – Compulsory Delivery Int 2 Bytes-1 – Not Applicable ie. Delivery is NOT allowed for this product.

- **Rules for Spread Type Combination:**
1) First and second bit will determine the Spread product B/S Anchor Leg.
st 0

- 1 bit - 2 – Near month means
- Buy would Buy Near month and Sell Far month
- Sell would Buy Far month and Sell Near month
nd 0

- 2 bit - 2 – Far month means
- Buy would Buy Far month and Sell Near month
- Sell would Buy Near month and Sell Far month
2) Third and fourth bit will determine whether the Spread product B/S Anchor Leg Trade Price calculation will be based on LTP or Closing Price.
rd 2

- 3 bit - 2 – Last Traded Price th 3
- 4 bit - 2 – Closing Price
3) Fifth and sixth bit will determine whether the Spread product calculation will be based on Near month or Far month.
th 4

- 5 bit - 2 – Near month would refer Near Month’s LTP/Closing Price based on
rd th defined 3 /4 bit.

th 5 rd th 6 bit - 2 – Far Month refer Far Month’s LTP/Closing Price based on defined 3 /4 bit.

- **The Trade Value would be derived as under:**
- For MTM purpose, it should be derived considering Actual Price and for margin and turnover purpose it should be derived considering Absolute Price.
##### Public

o **Method 1**: Trade Value (For MTM) = Qty * (Price / Decimal Locator)*Lot Size * (Gen Numerator/Gen Denominator)* (Price Numerator/Price Denominator)

- Trade Unit: 100 GMS
- Price Quote: Paise per 10 grams
- Price=-59200.00 Paise (per 10 gram)
- Lot Size = 5 (in terms of trading units)
- Price Numerator = 1 and Price Denominator = 100 (multiplier to convert value from Paise to Rupees)
- General Numerator = 10 and General Denominator = 1 (multiplier to convert value from price [per 10 grams] to Trading Unit [per 100 grams]
- The actual Value for 1 LOT of Silver would be (-59200.00 * 5 * 1/100 * 10/1) =-29,600.00 Rupees
We would arrive it as ROUND[(-59200.00 * 5 * (1 / 100) * (10/1),2] = - 29600.00 Rupees

o **Method 1**: Trade Value (For Turnover and Margin) = Qty * ABS(Price / Decimal Locator)*Lot Size * (Gen Numerator/Gen Denominator)* (Price Numerator/Price Denominator)

- Trade Unit: 100 GMS
- Price Quote: Paise per 10 grams
- Price=-59200.00 Paise (per 10 gram)
- Lot Size = 5 (in terms of trading units)
- Price Numerator = 1 and Price Denominator = 100 (multiplier to convert value from Paise to Rupees)
- General Numerator = 10 and General Denominator = 1 (multiplier to convert value from price [per 10 grams] to Trading Unit [per 100 grams]
- The actual Value for 1 LOT of Silver would be ABS (-59200.00 * 5 * 1/100
* 10/1) =29,600.00 Rupees
We would arrive it as ROUND[(59200.00 * 5 * (1 / 100) * (10/1),2] = 29600.00 Rupees

##### Public

##### Participant Master

The participant master shall be made available through FTP to the members connecting through Open Interface. No messaging download for participant master shall be provided.

##### File Name: MCX_PART.bcp

##### File Type: Comma Separated File

|Field Description|Data Type|Description|
|---|---|---|
|Participant ID|Char (12)|Institution Participant ID|
|Participant Name|Char (40)|Institution Participant Name A-Active|
|Status|Char (1)|D-De-active Date and time when the instrument was last modified In terms of|
|Last Modified Date|Int 4 Bytes|seconds from 01-01-1970 00:00:00 hrs. in IST|
|Filler2|Int 2 Bytes||

##### Public

##### Asset Master

The asset master shall be made available through FTP to the members connecting through Open Interface. No messaging download for asset master shall be provided.

##### File Name: MCX_ASSET_MASTER.bcp

##### File Type: Comma Separated File

|Field Description|Data Type|Description|
|---|---|---|
|Asset Instrument|||
|Identifier|Int 4 Bytes|Unique Asset Instrument Identifier.|
|Asset Name|Char (50)|Asset Name Date and time when the asset was last modified in terms of seconds|
|Last Updated Date|Int 4 Bytes|from 01-01-1970 00:00:00 hrs.|

##### Public

##### Asset Underlying Mapping

The asset underlying map shall be made available through FTP to the members connecting through Open Interface. No messaging download for asset underlying map shall be provided.

##### File Name: MCX_ASSET_UNDERLYING_MAP.bcp

##### File Type: Comma Separated File

|Field Description|Data Type|Description|
|---|---|---|
|Asset Instrument|||
|Identifier|Int 4 Bytes|Unique Asset Instrument Identifier.|
|Asset Underlying|||
|Instrument Identifier|Int 4 Bytes|Asset Underlying Instrument Identifier|
|Price Numerator|Numeric (20,4)|Numerator value|
|Price Denominator|Numeric (20,4)|Denominator value Date and time when the asset underlying was last modified in terms|
|Last Updated Date|Int 4 Bytes|of seconds from 01-01-1970 00:00:00 hrs.|

##### Public
