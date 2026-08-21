/*
 * feeder.h
 *
 *  Created on: 07-Apr-2014
 *      Author: nav
 */

#ifndef MCX_FEEDER_H_
#define MCX_FEEDER_H_

#include <includes.h>

namespace MCX{

#define SEPERATE_LOCAL_TIMESTAMP
#define FEED_ADVANCE_LEVEL_PROTECTION
#define FEED_LEVEL_DEPTH_1 (FEED_LEVEL_DEPTH-1)
#define MCX_NEW_API
#define MCX_PRICE_MULTIPLIER 1000000

#pragma pack(push,1)

// #ifdef MCX_NEW_API

// Structure: MessageHeader
typedef struct {
	uint16_t BodyLen;
    uint16_t TemplateID;
    uint32_t MsgSeqNum;
} MessageHeader;

typedef struct {
    uint32_t 	ApplSeqNum;
    uint32_t 	MarketSegmentID;
    uint8_t 	PartitionID;
    uint8_t 	CompletionIndicator;
    uint8_t	 	ApplSeqResetIndicator;
    char 		Pad5[5];
    uint64_t 	TransactTime;
} MessageBody;

//TemplateID : 13002
typedef struct {
	MessageHeader	Header;
	MessageBody		Body;
} PacketHeader;

//TemplateID : 13600
typedef struct {
	MessageHeader	Header;
	uint32_t	LastMsgSeqNumProcessed;
	uint8_t		TradingSessionID;
	uint8_t		TradingSessionSubID;
	uint8_t		TradSesStatus;
	uint8_t		MarketCOndition;
	uint8_t		FastMarketIndicator;
	char 		Pad7[7];
}SnapshotProductSummary;

typedef struct {
	int64_t		MDEntryPx;
	int64_t		MDEntrySize;
	uint16_t	MDEntryType;
	uint16_t	TradeCondition;
	char 		Pad4[4];
	int64_t		OILastUpdateTime;
}MDInstrumentEntryGrp;

//TemplateID : 13601
typedef struct {
	MessageHeader	Header;
	uint64_t		SecurityID;
	uint64_t		LastUpdateTime;
	uint64_t		TrdRegTSExecutionTime;
	uint16_t		TotNoOrders;
	uint8_t			SecurityStatus;
	uint8_t			SecurityTradingStatus;
	uint8_t			MarketCondition;
	uint8_t			FastMarketIndicator;
	uint8_t			SecurityTradingEvent;
	uint8_t			SoldOutIndicator;
	uint8_t			ProductComplex;
	uint8_t			NoMDEntries;
	char 			Pad6[6];
}SnapshotInstrumentSummary;

//TemplateID : 13602
typedef struct {
	MessageHeader	Header;
	uint64_t	TrdRegTSTimePriority;
	int64_t		DisplayQty;
	uint8_t		Side;
	uint8_t		OrderType;
	char 		Pad6[6];
	int64_t		Price;
}SnapshotOrder;

//TemplateID : 13100
typedef struct {
	MessageHeader	Header;
	uint64_t 				TrdRegTSTimeIn;
	int64_t					SecurityID;
	uint64_t				TrdRegTSTimePriority;
	int64_t					DisplayQty;
	uint8_t					Side;
	uint8_t					OrdType;
	char 					Pad6[6];
	int64_t					Price;
}OrderAdd;

//TemplateID : 13101
typedef struct {
	MessageHeader	Header;
	uint64_t 				TrdRegTSTimeIn;
	uint64_t 				TrdRegTSPrevTimePriority;
	int64_t	 				PrevPrice;
	int64_t	 				PrevDisplayQty;
	int64_t	 				SecurityID;
	uint64_t				TrdRegTSTimePriority;
	int64_t					DisplayQty;
	uint8_t					Side;
	uint8_t					OrdType;
	char 					Pad6[6];
	int64_t					Price;
}OrderModify;

//TemplateID : 13106
typedef struct {
	MessageHeader	Header;
	uint64_t 				TrdRegTSTimeIn;
	uint64_t 				TransactTime;
	int64_t	 				PrevDisplayQty;
	int64_t	 				SecurityID;
	uint64_t				TrdRegTSTimePriority;
	int64_t					DisplayQty;
	uint8_t					Side;
	uint8_t					OrdType;
	char 					Pad6[6];
	int64_t					Price;
}OrderModifySamePriority;

//TemplateID : 13102
typedef struct {
	MessageHeader	Header;
	uint64_t 				TrdRegTSTimeIn;
	uint64_t 				TransactTime;
	int64_t	 				SecurityID;
	uint64_t				TrdRegTSTimePriority;
	int64_t					DisplayQty;
	uint8_t					Side;
	uint8_t					OrdType;
	char 					Pad6[6];
	int64_t					Price;
}OrderDelete;

//TemplateID : 13103
typedef struct {
	MessageHeader	Header;
	int64_t 				SecurityID;
	uint64_t 				TransactTime;
}OrderMassDelete;

//TemplateID : 13104 , 13105
typedef struct {
	MessageHeader	Header;
	uint8_t					Side;
	uint8_t					OrderType;
	uint8_t					ATI;
	uint8_t					Pad1;
	int32_t	 				TrdMatchID;
	int64_t					Price;
	uint64_t 				TransactTime;
	int64_t	 				SecurityID;
	int64_t 				LastQty;
	int64_t	 				LastPx;
}Trade;

//TemplateID : 13202
typedef struct {
	MessageHeader	Header;
	int64_t	 				SecurityID;
	uint64_t 				reserve3;
	uint64_t 				reserve1;
	uint64_t 				ExecID;
	int64_t	 				LastQty;
	uint8_t 				AggressorSide;
	uint8_t 				Pad1;
	uint16_t 				TradeCOndition;
	char 					Pad4[4];
	int64_t					LastPx;
	int64_t	 				RestingHiddenQty;
	int64_t	 				RestingClxQty;

}ExecutionSummary;

//TemplateID : 13203
typedef struct {
	MessageHeader	Header;
	int64_t	 				SecurityID;
	int64_t	 				ClosePrice;
	int64_t	 				PrevClosePrice;
	int64_t	 				UpperCktLimit;
	int64_t 				LowerCktLimit;

}InstrumentInfo;

//TemplateID : 13504
typedef struct {
	MessageHeader	Header;
	uint64_t 		TrdRegTSTimeIn;
	int64_t			SecurityID;
	int64_t			BidPrice;
	int64_t			AskPrice;
	int64_t			BidQty;
	int64_t			AskQty;
	uint16_t		BidOrders;
	uint16_t		AskOrders;
	char 			Pad4[4];
}TopOfBook;

// #else

typedef struct Multicast_Stream_Parameters{
	char MulticastIP[16];
	uint32_t MulticastPort;
	char SourceIP[16];
}Multicast_Stream_Parameters;

typedef struct Product_Multicast_Stream_Parameters{
	uint32_t 					UniqueID;
	Multicast_Stream_Parameters StreamParam;
	int32_t						LastUpdatedTime;
}Product_Multicast_Stream_Parameters;

/**********************************************************/
/****************** STREAM PARAMETER **********************/
/**********************************************************/
typedef struct Price_Point_Stream_Parameters_Download_Request{
	uint16_t					MessageCode;
	uint16_t					RequestID;
	int32_t						LastUpdatedTime;

}Price_Point_Stream_Parameters_Download_Request;

typedef struct Price_Point_Stream_Parameters_Download_Start_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
	char		Reserved[20];
	Multicast_Stream_Parameters	MulticastParams;
	Multicast_Stream_Parameters	AncillaryMulticastParams;

}Price_Point_Stream_Parameters_Download_Start_Response;

typedef struct Price_Point_Stream_Parameters_Download_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
	Product_Multicast_Stream_Parameters ProductStreamParams;

}Price_Point_Stream_Parameters_Download_Response;

typedef struct Price_Point_Stream_Parameters_Download_End_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
}Price_Point_Stream_Parameters_Download_End_Response;

/**************************************************/
/****************** SNAPSHOT **********************/
/**************************************************/
typedef struct Price_Point_Snapshot_Download_Request{
	uint16_t	MessageCode;
	uint16_t	RequestID;
	uint32_t 	UniqueID;
}Price_Point_Snapshot_Download_Request;

typedef struct Price_Point_Snapshot_Download_Start_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
	int32_t		LastUpdatedTime;
	int32_t		LastSequenceNo;
	uint8_t		LastActivePassiveFlag;
	uint8_t		Terms;
	int32_t		LastTradedTime;
	int32_t		LastTradedPrice;
	uint32_t	LastTradedQuantity;
	int32_t		Open;
	int32_t		High;
	int32_t		Low;
	int32_t		Close;
	uint32_t	TotalNonSpreadQuantity;
	double		TotalNonSpreadValue;
	uint32_t	NumberOfNonSpreadTrades;
	uint32_t	TotalQuantityTraded;
	double		TotalValueTraded;
	uint32_t	NumberOfTrades;
	uint32_t	OpenInterest;
	int32_t		LifeTimeHigh;
	int32_t		LifeTimeLow;
}Price_Point_Snapshot_Download_Start_Response;

typedef struct Snapshot_Price_Point{
	uint8_t		DataFlag;
	uint8_t		BuySell;
	int32_t		Price;
	uint32_t	Quantity;
	uint32_t	Orders;
}Snapshot_Price_Point;

typedef struct Price_Point_Snapshot_Download_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
}Price_Point_Snapshot_Download_Response;

typedef struct Price_Point_Snapshot_Download_End_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
}Price_Point_Snapshot_Download_End_Response;

typedef struct ERROR_RESPONSE{
	uint16_t	MessageCode;
	int16_t		RequestID;
	int16_t		ErrorCode;
}ERROR_RESPONSE;

typedef struct Price_Point_Information
{
	uint16_t	MsgCode;
	uint32_t	UniqueIdentifier;
	uint32_t	LastUpdatedTime;
	uint32_t	SequenceNo;
	char			ActivePassive_Flg;

}Price_Point_Information;

typedef struct Price_Point_Information_Order_EMCTS
{
	char						OrderType;
	char						DataFlag;
	char						BuySellFlg;
	int32_t					Price;			// 4 Bytes
	uint32_t				Quantity;		// 4 Bytes
	uint32_t				Orders;			// 4 Bytes
	uint32_t				AffectedQty;	// 4 Bytes

}Price_Point_Information_Order_EMCTS;

typedef struct Price_Point_Update2
{
	uint8_t				OperationType;			// 4 Bytes
	uint32_t			OpenInterest;			// 4 Bytes

}Price_Point_Update2;

typedef struct Price_Point_Information_Reset_Sequence_Numbers
{
	uint16_t	MsgCode;
	uint32_t	UniqueIdentifier;
	uint32_t	LastUpdatedTime;
	uint32_t	LastSequenceNo;

}Price_Point_Information_Reset_Sequence_Numbers;

/**************************************************/
/****************** HISTORY **********************/
/**************************************************/
typedef struct Price_Point_History_Download_Request{
uint16_t	MessageCode;
uint16_t	RequestID;
int32_t		UniqueID;
int32_t		StartSeq;
int32_t		EndSeq;
}Price_Point_History_Download_Request;

typedef struct Price_Point_History_Download_Start_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
}Price_Point_History_Download_Start_Response;

typedef struct Price_Point_History_Download_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
	Price_Point_Information PricePointInfo;
}Price_Point_History_Download_Response;

typedef struct Price_Point_History_Download_End_Response{
	uint16_t	MessageCode;
	uint16_t	RequestID;
}Price_Point_History_Download_End_Response;

// #endif

#pragma pack(pop)


/***************************************** END *******************************************/

typedef struct ORDER_BOOK_RUPEE_RANGE
{
	int32_t						Avaliable;

}ORDER_BOOK_RUPEE_RANGE;

typedef struct ORDER_BOOK_TEN_RUPEE_RANGE
{
	int32_t						Avaliable;
	ORDER_BOOK_RUPEE_RANGE	OneRupee[10];

}ORDER_BOOK_TEN_RUPEE_RANGE;

typedef struct ORDER_BOOK_HUNDRED_RANGE
{
	int32_t							Avaliable;
	ORDER_BOOK_TEN_RUPEE_RANGE	TenRupee[10];

}RDER_BOOK_HUNDRED_RANGE;

typedef struct ORDER_BOOK_THOUSAND_RANGE
{
	int32_t							Avaliable;
	ORDER_BOOK_HUNDRED_RANGE	Hundred[10];

} ORDER_BOOK_THOUSAND_RANGE;

typedef struct ORDER_BOOK_FINDER_BID_ASK
{
	int16_t		ThousandDepth;
	int16_t		HundredDepth;
	int16_t		TenRupeeDepth;
	int16_t		RupeeDepth;
	int32_t		PriceIndex;
	int32_t 		Price;
	int32_t 		Quantity;
	uint32_t	Token;

}ORDER_BOOK_FINDER_BID_ASK ;

typedef struct ORDER_QTY_INFO
{
	int32_t Count;
	int32_t Quantity;

}ORDER_QTY_INFO;

#pragma pack(push,1)

typedef struct TOKEN_ORDER_BOOK
{
	uint32_t													CurrentSequence;
	uint32_t													BackLogProcessing;

	int32_t				 										BUY_SELL_DPR_RANGE_MAX_PRICE;
	ORDER_BOOK_THOUSAND_RANGE  	*BUY_INFO_TILL_RUPEE;
	ORDER_QTY_INFO   								*BUY_ORDER_PAISA_QUANTITY_INFO;
	ORDER_BOOK_THOUSAND_RANGE  	*SELL_INFO_TILL_RUPEE;
	ORDER_QTY_INFO   								*SELL_ORDER_PAISA_QUANTITY_INFO;
	int32_t				 										BUY_SELL_DPR_RANGE_MIN_PRICE;
	ORDER_BOOK													OrderBook;
	int32_t														TokenIndex;
	int32_t														ThousandSize;
	int32_t														TickSize;
	int32_t														PaisaDepthRange;
	int32_t														PriceExponent;

	int32_t				 										ActivePassiveB;
	int32_t				 										ActivePassiveS;

}TOKEN_ORDER_BOOK ;



typedef struct PRODUCT_INFO_MCX
{
	uint32_t													ProductID;
	uint32_t													SkipSequence;
	uint32_t													CurrentSequence;
	uint32_t													BufferedSequence;
	uint32_t													BackLogProcessing;
	int32_t														BackLogProcessingIdx;
	uint32_t													BackLogSequence;
	uint32_t													BackLogIndex;
	uint32_t													PRIMARY_DATA_BUFFER_INDEX;
	// STREAM_INDEX_BUFFER											*PRIMARY_DATA_BUFFER;
#ifdef ERROR_CHECKING_MCX
	uint32_t													ProcessingSequence;
#endif
}PRODUCT_INFO_MCX;

#pragma pack(pop)

typedef struct DATA_HOLDER: public DATA_HOLDER_BASE
{
    unsigned int SEQ_NO,ReceiveSequence;
    unsigned char *StreamBufferPtr;
    uint64_t FirstOrderID;
	PRODUCT_INFO_MCX *PRODUCT;
    std::tr1::unordered_map<uint64_t, ORDER_BOOK_FINDER_BID_ASK*> ORDER_INDEX;
    std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *> OrderBook;
    FILE 	*ResetSeqFile;

}DATA_HOLDER;

typedef struct TOKEN_SNAPSHOT_INFO
{
	int32_t	Orders;
	int32_t	Quantity;

}TOKEN_SNAPSHOT_INFO;

typedef struct TOKEN_SNAPSHOT_BUY_SELL
{
	std::map<int32_t,TOKEN_SNAPSHOT_INFO*> SnapshotBuy;
	std::map<int32_t,TOKEN_SNAPSHOT_INFO*> SnapshotSell;

}TOKEN_SNAPSHOT_BUY_SELL;

class MCX_Feeder
{
private:
	std::map<std::pair<int, int>, GenericContract> *_Contract;
	int32_t Load_New_DPR_Settings(TOKEN_ORDER_BOOK *TokenInfo, uint32_t &Tkn, int32_t &Price);
	void add_token_to_orderbook(uint32_t Token, TOKEN_ORDER_BOOK *TknOrdBk, int32_t StartSequence, int32_t HighPriceRange, int32_t LowPriceRange, int32_t TickSize, int32_t ConversionRatio, TOKEN_SNAPSHOT_BUY_SELL*Snapshot);
	void PrintNodes(FILE *fp, TOKEN_ORDER_BOOK *idxPTR, int32_t mode, char BS);
	int Reset_Token_Sequence_And_Orderbook(DATA_HOLDER *pData, uint32_t Token);
	void Purger_Market_Depth(TOKEN_ORDER_BOOK *OrderBookPTR, uint32_t ResetSeq);
public:
	uint32_t						TotalNodes;
	DATA_HOLDER			Data[100];
	//std::tr1::unordered_map<int32_t,CONTRACT_DETAILS*> instrument_umap;
	std::function<void(Exchange_Type, Token_Data*)> feed_func;
	int generate_feed(DATA_HOLDER *pData, uint64_t *timestamp);
	int Add_ReplayFile(Exchange_Type Segment, int8_t StreamID, const char *file=NULL);
	int Start_FileReplay(void);
	explicit MCX_Feeder(std::map<std::pair<int, int>, GenericContract> *contract, std::string file);
	~MCX_Feeder();
	std::string						BaseFileName;
};
}

#endif /* MCX_FEEDER_H_ */





