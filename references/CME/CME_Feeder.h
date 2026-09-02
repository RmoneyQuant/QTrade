/*
 * feeder.h
 *
 *  Created on: 07-Apr-2014
 *      Author: nav
 */

#ifndef CME_FEEDER_H_
#define CME_FEEDER_H_

#include <includes.h>
#include <qed.h>
#include <qed_parser.h>
#include <qed_msg.h>
#include <qed_msg_extra.h>
#include <qed_hb.h>
#include <qed_symbology.h>
#include <qedc_context.h>
using namespace QED;

namespace CME{
typedef struct DATA_HOLDER: public DATA_HOLDER_BASE
{
    unsigned int SEQ_NO,ReceiveSequence;
    unsigned char *StreamBufferPtr;
    QED_parser parser;

}DATA_HOLDER;

typedef struct CONTRACT_DETAILS
{
	PriceType			BuyPrice[FEED_LEVEL_DEPTH];
	uint32_t	 		BuyQty[FEED_LEVEL_DEPTH];
	PriceType			SellPrice[FEED_LEVEL_DEPTH];
	uint32_t	 		SellQty[FEED_LEVEL_DEPTH];
	int32_t			 	NoOfBuyOrds[FEED_LEVEL_DEPTH];
	int32_t			 	NoOfSellOrds[FEED_LEVEL_DEPTH];
	PriceType		 	LastTradedPrice;
	uint16_t			LastTradedQty:12;
	uint8_t				FeedEventAggressor:2;
	GenericContract	 							Contract;
	void 													*PTR;

}CONTRACT_DETAILS;

class CME_Feeder
{
private:
	std::map<std::pair<int, int>, GenericContract> *Contract;
public:

	uint32_t						TotalNodes;
	DATA_HOLDER			*Data;
	QEDC_context cryptCtx;
	std::tr1::unordered_map<std::string,CONTRACT_DETAILS*> instrument_umap;
	std::function<void(Exchange_Type, Token_Data*)> feed_func;
	int generate_feed(DATA_HOLDER *pData, uint64_t *timestamp);
	int Add_ReplayFile(Exchange_Type Segment, int8_t StreamID, const char *file=NULL, const char *keys=NULL);
	int Start_FileReplay(void);
	explicit CME_Feeder(std::map<std::pair<int, int>, GenericContract> *contract);
	~CME_Feeder();
};
}
#endif /* CME_FEEDER_H_ */





