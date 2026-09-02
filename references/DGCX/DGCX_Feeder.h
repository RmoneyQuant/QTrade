/*
 * feeder.h
 *
 *  Created on: 07-Apr-2014
 *      Author: nav
 */

#ifndef DGCX_FEEDER_H_
#define DGCX_FEEDER_H_

#include <includes.h>
#include <qed.h>
#include <qed_parser.h>
#include <qed_msg.h>
#include <qed_msg_extra.h>
#include <qed_hb.h>
#include <qed_symbology.h>

namespace DGCX{

typedef struct DATA_HOLDER: public DATA_HOLDER_BASE
{
    unsigned int SEQ_NO,ReceiveSequence;
    unsigned char *StreamBufferPtr;

}DATA_HOLDER;

typedef struct CONTRACT_DETAILS
{
	GenericContract	 							Contract;
	void 													*PTR;

}CONTRACT_DETAILS;

class DGCX_Feeder
{
private:
	std::map<std::pair<int, int>, GenericContract> *Contract;
public:
	uint32_t						TotalNodes;
	DATA_HOLDER			*Data;
	std::tr1::unordered_map<int32_t,CONTRACT_DETAILS*> instrument_umap;
	int generate_feed(DATA_HOLDER *pData, uint64_t *timestamp);
	std::function<void(Exchange_Type, Token_Data*)> feed_func;
	int Add_ReplayFile(Exchange_Type Segment, int8_t StreamID, const char *file=NULL);
	int Start_FileReplay(void);
	explicit DGCX_Feeder(std::map<std::pair<int, int>, GenericContract> *contract);
	~DGCX_Feeder();
};
}
#endif /* DGCX_FEEDER_H_ */





