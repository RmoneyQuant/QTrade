#include <sys/socket.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <CME_Feeder.h>
#include <qed_msg_extra_hf.h>
using namespace CME;

typedef struct {
  uint16_t lastHbSeqNo;
} MyQEDClosure;

void printQEDError(const char *info, int32_t err) {
  fprintf(stderr, "%s: QED error %d: %s\n", info, err, QED_error_string(err));
}

void printTime(uint64_t ts, const char *info) {
  time_t t = ts / 1000000;
  struct tm tmp;
  struct tm *gmRes = gmtime_r(&t, &tmp);
  char outstr[200];
  strftime(outstr, sizeof(outstr), "%Y%m%d-%H%M%S", gmRes);
  printf("%s = %s.%06lu\n", info, outstr, ts % 1000000);
}

void printSide(QED_msg msg, QED_Side side, uint32_t depth, int32_t numOrdersDepth) {

  const int32_t startLevel = (side == QED_ASK) ? (depth - 1) : 0;
  const int32_t incrementLevel = (side == QED_ASK) ? -1 : 1;
  const int32_t endLevel = (side == QED_ASK) ? -1 : depth;
  int32_t levelI;
  for (levelI = startLevel; levelI != endLevel; levelI += incrementLevel) {
    QED_Price price = QED_msg_get_level_price(msg, levelI, side);
    uint32_t qty = QED_msg_level_qty(msg, levelI, side);

    if (!false) { // todo test for price & qty overflow
      double priceD = QED_price_as_double(price);
      if( levelI < numOrdersDepth){
        uint16_t numOrders = QED_msg_level_number_of_orders(msg, levelI, side);
        printf("%s %d: %.5f %d (%d)\n", (side == QED_ASK) ? "ASK" : "BID",
            levelI, priceD, qty, numOrders);
      }
      else {
        printf("%s %d: %.5f %d\n", (side == QED_ASK) ? "ASK" : "BID", levelI, priceD, qty);
      }
    }
  }
}

void printStatus(QED_msg msg, uint8_t status) {
  if (status & QED_FIELD_OFLOW) {
    printf("overflow ");
  }
  if (status & QED_FEED_ERROR) {
    printf("feed_error ");
  }
  if (status & QED_BOOK_INCONSISTENT) {
    printf("book_inconsistent ");
  }
}

const char *getTrdStatusString(uint8_t trdStatus) {
  switch(trdStatus) {
  case QED_TRADING_UNKNOWN:
    return "unknown";
  case QED_TRADING_NOT_TRADING:
    return "not_trading";
  case QED_TRADING_TRADING:
    return "trading";
  case QED_TRADING_HALTED:
    return "halted";
  }
  return "error";
}

void printExtraData(QED_msg msg){
  if(QED_msg_get_extra_count(msg) != 0){
    int32_t openInterest = QED_msg_get_open_interest(msg);
    int32_t totalVol = QED_msg_get_total_volume(msg);
    QED_Price high = QED_msg_get_high_price(msg);
    QED_Price low = QED_msg_get_low_price(msg);
    QED_Price lastTrade = QED_msg_get_last_trade_price(msg);
    QED_Price settlement = QED_msg_get_settlement_price(msg);

    printf("----------\n");
    printf("open interest: %d\n", openInterest);
    printf("total volume:  %d\n", totalVol);
    printf("high:          %.5f\n", QED_price_as_double(high));
    printf("low:           %.5f\n", QED_price_as_double(low));
    printf("last trade:    %.5f\n", QED_price_as_double(lastTrade));
    printf("settlement:    %.5f\n", QED_price_as_double(settlement));
  }
}

void printHeartbeat(QED_parser parser, const char* packet, int32_t len){
  size_t numSymbols = 0;

  QED_heartbeat heartbeat =  QED_heartbeat_parse( parser, packet, len);

  if (NULL == heartbeat){
    printQEDError("QED_init error", QED_errno());
    return;
  }

  QED_closure_info** infos = QED_heartbeat_get_closure_infos(heartbeat, &numSymbols);

  printf("===================================\n");
  printf ("Heartbeat received id = %d\n", QED_heartbeat_get_id(heartbeat));

  for (uint32_t i = 0; i < numSymbols; i++){
    uint16_t seqno = QED_heartbeat_get_seq_no(heartbeat);
    MyQEDClosure* closure = (MyQEDClosure*)(infos[i]->closure);

    printf ("Heartbeat for %s\n",  infos[i]->symbol);
    printf("Sequence Number: %u (%s)\n", seqno,
        (closure->lastHbSeqNo + 1 != seqno ) ? "GAP" : "NO GAP");

    closure->lastHbSeqNo = seqno;

    printf("One way latency: %lu \n",
        QED_heartbeat_get_ds_timestamp(heartbeat) - QED_heartbeat_get_es_timestamp(heartbeat));

  }
}

const char *tradeFlagToString(QED_trade_flag flag){
  switch (flag){
  case QED_SELL:
    return "ASK";
  case QED_BUY:
    return "BID";
  case QED_NONE:
    return "NONE";
  case QED_UNSUPPORTED:
    return "N/A";
  default:
    return "BAD FLAG";
  }
}

void printMsg(QED_msg msg, MyQEDClosure *myClosure) {
  uint8_t codec = QED_msg_get_codec_version(msg);
  uint8_t compat = QED_msg_get_compat_version(msg);
  uint64_t seqNo = QED_msg_get_src_seq_no(msg);
  printf("===================================\n");
  printf("codec = %hhu, compat = %hhu\n", codec, compat);
  printf("seqno = %lu\n", seqNo);

  uint64_t srcTime = QED_msg_get_src_time(msg);
  printTime(srcTime, "mkt time");

  uint64_t dsRcvTime = QED_msg_get_ds_rcv_time(msg);
  printTime(dsRcvTime, "DS recv time");

  uint64_t dsSendTime = QED_msg_get_ds_snd_time(msg);
  printTime(dsSendTime, "DS send time");
  printf("----------\n");

  uint8_t status = QED_msg_status_flags(msg);
  printStatus(msg, status);
  printf("trd status = %s\n", getTrdStatusString(QED_tradingStatus(status)));

  if (0 == (status & QED_BOOK_UNCHANGED)) {
    uint32_t depth = QED_msg_depth(msg);
    uint32_t numOrdersDepth = QED_msg_number_of_order_depth(msg);

    printf("depth = %d\n", depth);
    printf("number of orders depth = %d\n", numOrdersDepth);
    printf("----------\n");
    printSide(msg, QED_ASK, depth, numOrdersDepth);
    printf("----------\n");
    printSide(msg, QED_BID, depth, numOrdersDepth);
  } else {
    printf("order book unchanged\n");
  }
  printf("----------\n");

  const uint32_t nTradeLevels = QED_msg_trade_level_count(msg);
  uint32_t tradeLevelI;
  for (tradeLevelI = 0; tradeLevelI < nTradeLevels; tradeLevelI++) {
    QED_Price price = QED_msg_get_trade_price(msg, tradeLevelI);
    int32_t qty = QED_msg_get_trade_qty(msg, tradeLevelI);

    if (!false) { // todo test for price & qty overflow
      QED_trade_flag aggressor = QED_msg_get_trade_aggressor(msg, tradeLevelI);
      double priceD = QED_price_as_double(price);
      printf("Trades: %.5f %d (%s)\n", priceD, qty, tradeFlagToString(aggressor));
    }
  }
  if (QED_msg_get_unreported_qty(msg) != 0) {
    if (!false) { // todo test for qty overflow
      printf("Trades: %d\n", QED_msg_get_unreported_qty(msg));
    }
  }

  printExtraData(msg);
}

CME_Feeder::CME_Feeder(std::map<std::pair<int, int>, GenericContract> *contract):Contract(contract),TotalNodes(0),Data(NULL){

}

CME_Feeder::~CME_Feeder(){

}

int CME_Feeder::Add_ReplayFile(Exchange_Type Segment, int8_t StreamID, const char *file, const char *keys){
	if(!file)return -1;
    FILE *fp = fopen(file, "r");
    if (fp == NULL) {
         printf("\ncouldn't open FEED_DATA_FILE :\033[1;31m %s \033[0mfor reading.\n\n",file);
         fflush((FILE *)stdout); return -2;
     }

	Data = (DATA_HOLDER*)realloc(Data, (sizeof(DATA_HOLDER) * (TotalNodes + 1)));
	DATA_HOLDER *pData = new (&Data[TotalNodes]) DATA_HOLDER;
	std::vector<std::string> _fname; _fname = split(file,"/");
	char filen[50]; sprintf(filen,"Feeder_%s.log",((_fname.size())?_fname[_fname.size()-1].c_str() : file));
	pData->feed_log = fopen(filen, "w");  if(!pData->feed_log){cout << "Error Creating Output File => "<< filen << endl; ABORT;} if (setvbuf(pData->feed_log, NULL,_IONBF, 0) < 0){printf("setvbuf error in TokenOrderbook\n");return 0;}
	pData->fp =fp;
	pData->MSG_LEN=0;
	pData->StreamID = StreamID;
	pData->sz=0; pData->SEQ_NO =0;
	pData->ReceiveSequence=1;
	pData->Processed_Bytes = 0;
	pData->Pos = 0;
	pData->Buffer = (unsigned char*)numa_alloc_local(BYTES_READ_SIZE + 1024);
	pData->BytesToRead = BYTES_READ_SIZE;
	pData->_Segment = Segment;

	//Process Contract & init Quincy Objects
	std::unordered_set<std::string> ContractSymbols;  QED_ctx ctx = NULL;
	{
		/*ssize_t contract_sz = 0; if(fread(&contract_sz, sizeof(ssize_t), 1, pData->fp) < 0){printf("Error Taking Contract Size..\n\n"); ABORT;}
		unsigned char Buffer[5196]; if((int32_t)fread(Buffer, 1, contract_sz, pData->fp) != contract_sz){printf("Error Taking Contract Bytes:%d..\n\n", contract_sz); ABORT;}
		printf("\n[ContractBuffer:%d]\n",contract_sz);for(int i=0; i<contract_sz; i++){printf("0x%02x,", Buffer[i]);}printf("\n");
		char *sdata = (char*)Buffer; unsigned long int slen=contract_sz;
		slen -=8; uint32_t cnt = 8; char Symbol[100]; uint16_t __attribute__((unused)) Index = 0; uint8_t __attribute__((unused)) ChannelID = 0;
		while(cnt < slen){
			Index = *(uint16_t*)(sdata + cnt); cnt += 2;
			ChannelID = *(uint8_t*)(sdata + cnt); cnt += 1;
			cnt += 2;
			strcpy(Symbol, (sdata + cnt)); cnt += (strlen(Symbol) + 1);
			ContractSymbols.insert(Symbol);
			printf("Index:%d, ChannelID:%d, Symbol:%s\n", Index, ChannelID, Symbol);
		}*/
		//for(std::tr1::unordered_set<std::string>::iterator contract_itr= ContractSymbols.begin(); contract_itr != ContractSymbols.end(); contract_itr++ ){	std::cout << *contract_itr << ' '  << endl;}
		//Initialize Quincy Lib
		QED_initializeLogs(); QED_setLogFile(stdout);
		/*ctx = QED_initWithUserSymbolPackets("CME", "./");
		if (ctx == NULL) {
			printf("\nQED_init error:%s \n\n", QED_error_string(QED_errno())); ABORT;
		}*/
		ctx = QED_initWithPath("CME", "./");
		if (ctx == NULL) {
                        printf("\nQED_init error:%s \n\n", QED_error_string(QED_errno())); ABORT;
                }
		/*int32_t qret = QED_processSymbolPacket(ctx, sdata, contract_sz);
		if(qret != QED_EOK){
			printf("\nQED_processSymbolPacket error:%s , %d\n\n", QED_error_string(QED_errno()), qret); //ABORT;
		}
		const char** symbols;  uint16_t  count = QED_getSymbolList(ctx, &symbols);
		printf("Count %d\n", count);
		for(int32_t i = 0; i < count; i++){
		  printf("Registered Symbol %s, SymbolChannelID:%d\n", symbols[i], QED_getSymbolChannelID(ctx,symbols[i]));
		}*/
//		sdata = NULL; slen=0; QED_getSymbolListPacket(ctx, (const char**)&sdata, &slen);printf("\n---------------------- %ld --------------------\n\n", slen);for(uint32_t i = 0; i <slen; i++){printf("0x%02x,", ((unsigned char*)sdata)[i]);}
		pData->parser = QED_create_parser(ctx);
	}

	std::map<std::pair<int, int>, GenericContract>::iterator contract_itr;
	for( contract_itr= Contract->begin(); contract_itr != Contract->end(); contract_itr++ ){
		if(contract_itr->first.first == Segment){
			//printf("\n\n%s\n\n",contract_itr->second.SymbolCode);
			//if(ContractSymbols.find(contract_itr->second.SymbolCode) != ContractSymbols.end())
			{
				std::tr1::unordered_map<std::string,CONTRACT_DETAILS*>::iterator citr = instrument_umap.find(contract_itr->second.SymbolCode);
				if(citr == instrument_umap.end()){
					CONTRACT_DETAILS *cntr = new CONTRACT_DETAILS;
					memcpy(&cntr->Contract, &contract_itr->second, sizeof(GenericContract));
					instrument_umap[cntr->Contract.SymbolCode] = cntr;
					MyQEDClosure closure; QED_Status s;
					if ((s = QED_parser_register_symbol(pData->parser, cntr->Contract.SymbolCode, true, &closure)) == QED_EOK) {
						printf("Added @ Token => %d::%s\n",cntr->Contract.Token, cntr->Contract.SymbolCode);
					}
					else{
						//printf("\nCME Symbol:%s Not Found..\n", cntr->Contract.SymbolCode); ABORT;
					}
				}
			}/*else{
				printf("Requested SymbolCode Not Present In The CME Contract File @ Token => %d:: [ %s ]\n",contract_itr->second.Token, contract_itr->second.SymbolCode);
				ABORT;
			}*/
		}
	}

	TotalNodes++;

	cryptCtx = NULL;
	QED_Status s = QEDC_init_context(&cryptCtx, keys);
	if(s != QED_EOK) {
		printQEDError("\nQEDC_init_context error", s);
		printf("keys:%s\n", keys);
		ABORT;
	}
	return 0;
}

int CME_Feeder::generate_feed(DATA_HOLDER *pData, uint64_t *timestamp){
	//fprintf(pData->feed_log,"[timestamp:%ld] => pData->sz=%d,\tProcessed_Bytes:%d,\tMsgLength:%d\n",*timestamp, pData->sz, pData->Processed_Bytes,pData->MSG_LEN);

	unsigned char buffer[BYTES_READ_SIZE]; int offset = 0; memcpy(buffer, (char*)(pData->Buffer + pData->Processed_Bytes), pData->MSG_LEN); ssize_t res = pData->MSG_LEN;
    if(QEDC_is_encrypted_packet((uint8_t*)buffer, res)) {
      size_t len = res;
      QED_Status s = QEDC_decrypt_packet(cryptCtx, (uint8_t*)(buffer), (uint8_t*)(buffer), &len);
      if(s != QED_EOK) {
        printQEDError("QEDC_decrypt_packet error", s);
        return -1;
      }

      offset = QEDC_PAYLOAD_OFFSET; res = len;

//	      if(init[1] == false){end1[1] = end2;init[1]=true;}
//	      uint64_t tym = (((end2.tv_sec*1000000000)+end2.tv_nsec) - ((end1[1].tv_sec*1000000000)+end1[1].tv_nsec))/1000; avg[1] += tym; ++acnt[1];
//	      fprintf(ptr->feedLog,"[%ld.%ld] => seq:%ld # DIFF:%ld micros\n",end2.tv_sec, (end2.tv_nsec/1000), QED_parser_get_pckt_seq_no((char*)(buffer + offset), res), tym);
//	      end1[1] = end2;
    }

    QED_msg msg = QED_parser_begin(pData->parser, (char*)(buffer + offset), res - offset);
    if (msg == NULL) {
		if (QED_EHB == QED_errno()){  /*printHeartbeat(parser, buffer, res);*/ goto Next;}
		if(QED_EOK == QED_errno())goto Next;
		fprintf(pData->feed_log,"\nrecv_len:%ld \n",res);for(int32_t x=0; x < res; x++){fprintf(pData->feed_log,"0x%02x,",buffer[x]);}fprintf(pData->feed_log,"\n\n");
		for(int i=0; i<pData->MSG_LEN; i++){fprintf(pData->feed_log,"0x%02x,", pData->Buffer[i]);}printf("\n");
		printQEDError("QED_parser_begin error", QED_errno()); goto Next;
    }

    //else if(QED_msg_get_completion_indicator(msg) != QED_EOK)continue;

    while(true) {
		MyQEDClosure *myClosure = (MyQEDClosure *)QED_msg_get_closure(msg);
		if (myClosure == NULL) {printQEDError("QED_msg_get_closure error", QED_errno());ABORT;}
		//printMsg(msg, myClosure);
		//if(QED_price_as_double(QED_msg_get_last_trade_price(msg)) > 0)ABORT;

		std::tr1::unordered_map<std::string,CONTRACT_DETAILS*>::iterator citr = instrument_umap.find(QED_msg_get_topic(msg));
		if(citr != instrument_umap.end()){// && QED_msg_depth(msg)){

			QED_Direction direction; uint64_t threshold; QED_msg_get_hf_direction_and_threshold(msg, &direction, &threshold);
			if( QED_EOK == QED_errno()){
				fprintf(pData->feed_log,"\n## %s => direction:%d, threshold:%ld, Timestamp:%ld::%ld ##\n", QED_msg_get_topic(msg), direction, threshold, QED_msg_get_src_time(msg), QED_msg_get_ds_snd_time(msg));
				Token_Data fdata; memset(&fdata, 0 , sizeof(fdata));
				fdata.Token = citr->second->Contract.Token;
				fdata.Timestamp = *timestamp;
				fdata.FeedEventReason = 0;//0x7f;
				fdata.Direction = direction;
				fdata.Threshold = threshold;
				fdata.Delayed = 0;
				//feed_func(pData->_Segment, &fdata);
			}
			else{
				//fprintf(pData->feed_log,"\n%s, %0.2f, %d, %d, %ld\n",  QED_msg_get_topic(msg), QED_price_as_double(QED_msg_get_last_trade_price(msg)), QED_msg_get_open_interest(msg), QED_price_exponent(QED_msg_get_level_price(msg, 0, QED_ASK )), *timestamp);
				fprintf(pData->feed_log,"\t--------------------------%s--------------------------\n",QED_msg_get_topic(msg));
				for(int32_t i=0; i<FEED_LEVEL_DEPTH; ++i){
					fprintf(pData->feed_log,"[%d]\t%0.8f\t-\t%d\t|\t%0.8f\t-\t%d\t[%d]\n", ((QED_msg_number_of_order_depth(msg) > i)?QED_msg_level_number_of_orders(msg, i, QED_BID ):0), QED_price_as_double(QED_msg_get_level_price(msg, i, QED_BID )), QED_msg_level_qty(msg, i, QED_BID ),
							QED_price_as_double(QED_msg_get_level_price(msg, i, QED_ASK )), QED_msg_level_qty(msg, i, QED_ASK ), ((QED_msg_number_of_order_depth(msg) > i)?QED_msg_level_number_of_orders(msg, i, QED_ASK ):0));
					//fprintf(pData->feed_log,"[%d]\t%d\t-\t%d\t|\t%d\t-\t%d\n", i, QED_price_mantissa(QED_msg_get_level_price(msg, i, QED_BID )), QED_msg_level_qty(msg, i, QED_BID ), QED_price_mantissa(QED_msg_get_level_price(msg, i, QED_ASK )), QED_msg_level_qty(msg, i, QED_ASK ));
				}

				 const unsigned int nTradeLevels = QED_msg_trade_level_count(msg); unsigned int tradeLevelI;  uint8_t tflg = 0;
				  for (tradeLevelI = 0; tradeLevelI < nTradeLevels; tradeLevelI++) {
					QED_Price price = QED_msg_get_trade_price(msg, tradeLevelI);
					int qty = QED_msg_get_trade_qty(msg, tradeLevelI);
					if (/*qty != QED_INVALID_TRADE_QTY && */QED_price_is_valid(price)) {
					  QED_trade_flag aggressor = QED_msg_get_trade_aggressor(msg, tradeLevelI);
					  //double priceD = QED_price_as_double(price); fprintf(pData->feed_log,"Trades: %ld::%.5f : %d @ (%d)\n\n", price, priceD, qty, aggressor);
					  citr->second->LastTradedPrice = (PriceType)price.man;
					  citr->second->FeedEventAggressor = aggressor + 1;
					  citr->second->LastTradedQty = qty;
					  tflg = 3;
					}
				  }
				  if(QED_msg_depth(msg)){
					for(int32_t i=0; i<FEED_LEVEL_DEPTH; i++){
						citr->second->BuyPrice[i] 	=	QED_price_mantissa(QED_msg_get_level_price(msg, i, QED_BID ));
						citr->second->SellPrice[i]	= 	QED_price_mantissa(QED_msg_get_level_price(msg, i, QED_ASK ));
						citr->second->BuyQty[i] 		=  QED_msg_level_qty(msg, i, QED_BID );
						citr->second->SellQty[i] 	=  QED_msg_level_qty(msg, i, QED_ASK);
						citr->second->NoOfBuyOrds[i] 		=  QED_msg_level_number_of_orders(msg, i, QED_BID );
						citr->second->NoOfSellOrds[i] 	=  QED_msg_level_number_of_orders(msg, i, QED_ASK);
						fprintf(pData->feed_log,"[%d]\t%0.8f\t-\t%d\t|\t%0.8f\t-\t%d\t[%d]\n",citr->second->NoOfBuyOrds[i],citr->second->BuyPrice[i],citr->second->BuyQty[i],citr->second->SellPrice[i],citr->second->SellQty[i],citr->second->NoOfSellOrds[i]);
					}
				  }

				Token_Data fdata; memset(&fdata, 0 , sizeof(fdata));
				fdata.Token = citr->second->Contract.Token;
				fdata.Timestamp = *timestamp;
				fdata.LastTradedPrice = citr->second->LastTradedPrice;
				fdata.LastTradedQty = citr->second->LastTradedQty;
				fdata.FeedEventAggressor = citr->second->FeedEventAggressor;
				fdata.FeedEventReason = tflg;
				fdata.Delayed = 0;

				for(int p=0;p<FEED_LEVEL_DEPTH;p++){
	//				fdata.BuyPrice[p] = QED_price_mantissa(QED_msg_get_level_price(msg, p, QED_BID ));
	//				fdata.BuyQty[p] = QED_msg_level_qty(msg, p, QED_BID );
	//				fdata.NoOfBuyOrds[p] = ((QED_msg_number_of_order_depth(msg) > p)?QED_msg_level_number_of_orders(msg, p, QED_BID ):0);
	//				fdata.SellPrice[p] = QED_price_mantissa(QED_msg_get_level_price(msg, p, QED_ASK ));
	//				fdata.SellQty[p] = QED_msg_level_qty(msg, p, QED_ASK );
	//				fdata.NoOfSellOrds[p] = ((QED_msg_number_of_order_depth(msg) > p)?QED_msg_level_number_of_orders(msg, p, QED_ASK ):0);
					fdata.BuyPrice[p] 	=	citr->second->BuyPrice[p];
					fdata.SellPrice[p]	= 	citr->second->SellPrice[p];
					fdata.BuyQty[p] 		=  	citr->second->BuyQty[p];
					fdata.SellQty[p] 	=  	citr->second->SellQty[p];
					fdata.NoOfBuyOrds[p] 		=  	citr->second->NoOfBuyOrds[p];
					fdata.NoOfSellOrds[p] 	=  	citr->second->NoOfSellOrds[p];
				}
				if(fdata.BuyPrice[0] && fdata.SellPrice[0] && (fdata.SellPrice[0] > fdata.BuyPrice[0]))feed_func(pData->_Segment, &fdata);
			}

		}
		msg = QED_parser_next(pData->parser);
		if (msg == NULL) {
			if (QED_errno() != QED_EOK) {
			  printQEDError("QED_parser_next error", QED_errno()); ABORT;
			}
			goto Next;
		}
    }
    Next:
	return 0;
}

int CME_Feeder::Start_FileReplay(void)
{
    int64_t Timestamp[TotalNodes]; for(unsigned int nav=0; nav<TotalNodes; ++nav)Timestamp[nav] = 0; int64_t *timestamp = NULL;
    //setvbuf(stdout, NULL,_IONBF, 0);

    while(1){
    	//if(TotalNodes > 1)
    	{
    		int64_t min_time = 0x7FFFFFFFFFFFFFFF; int32_t nav = -1; DATA_HOLDER *pData = NULL;
			for(unsigned int i=0; i<TotalNodes; ++i){
				//fprintf(pData->feed_log,"LINE => %d, [%d]=> %ld\n", __LINE__, i, Timestamp[i]);
				if(Timestamp[i] > 0){
					if(min_time > Timestamp[i]){min_time = Timestamp[i]; nav = i; pData = &Data[nav];}
				}else if(Timestamp[i] == 0){
					nav = i;  pData = &Data[nav];
					if(pData->Processed_Bytes < pData->sz){goto start;/*fprintf(pData->feed_log,"LINE => %d, %d, %d, %d\n", __LINE__, nav, pData->Processed_Bytes , pData->sz);*/}else break;
				}
			}
			if(nav > -1 && Timestamp[nav] >0)	goto start;
			else if(nav == -1)	break;
			else
			{
					if(pData->Processed_Bytes == pData->sz){
						pData->Processed_Bytes = pData->Pos = 0; pData->BytesToRead = BYTES_READ_SIZE;
					}

					while((pData->sz=fread(pData->Buffer + pData->Pos, 1, pData->BytesToRead, pData->fp)) != 0){

						if(pData->sz > 0){

							pData->sz += pData->Pos;

							do{
								start:

								if(pData->MSG_LEN==0 && (pData->sz - pData->Processed_Bytes) >=(4 + 8)){
									timestamp = (int64_t *)(pData->Buffer + pData->Processed_Bytes);
									pData->MSG_LEN = *(int32_t *)(pData->Buffer + pData->Processed_Bytes + 8); pData->Processed_Bytes += (4 + 8);
								}
								else if(pData->MSG_LEN==0){
									pData->Pos = pData->sz; pData->BytesToRead = ((4 + 8) - (pData->sz - pData->Processed_Bytes));
									//fprintf(pData->feed_log,"---- [ Length Alignment Started => pData->sz:%d pData->BytesToRead:%d MsgLength:%d pData->Processed_Bytes:%d ]---- \n",pData->sz,pData->BytesToRead,pData->MSG_LEN,pData->Processed_Bytes); fflush((FILE *)stdout);
									goto beginRead;
								}
								if((pData->MSG_LEN+pData->Processed_Bytes) > pData->sz){
									pData->Pos = pData->sz; pData->BytesToRead = (pData->MSG_LEN - (pData->sz - pData->Processed_Bytes));
									//fprintf(pData->feed_log,"---- [ Byte Alignment Started => pData->sz:%d pData->BytesToRead:%d MsgLength:%d pData->Processed_Bytes:%d ]---- \n",pData->sz,pData->BytesToRead,pData->MSG_LEN,pData->Processed_Bytes); fflush((FILE *)stdout);
									goto beginRead;
								}

								if(Timestamp[nav] == 0){
									Timestamp[nav] = *timestamp; goto end;
								}

								generate_feed(pData, timestamp);

							    pData->Processed_Bytes += pData->MSG_LEN; pData->MSG_LEN = 0; timestamp = NULL; if(Timestamp[nav]>0){Timestamp[nav]=0; goto end;}
							}while(pData->Processed_Bytes < pData->sz);
							if(pData->Processed_Bytes == pData->sz){
								pData->Processed_Bytes = pData->Pos = 0; pData->BytesToRead = BYTES_READ_SIZE;
							}
							beginRead:{}
						}
					}

				if(pData->sz <= 0){
					fprintf(pData->feed_log, "\nEOF Reached ...\n\n"); fclose(pData->fp); fclose(pData->feed_log); pData->fp = NULL; pData->feed_log = NULL; pData->sz = 0; Timestamp[nav] = -1;
				}
				end:{}
			}

    	}
    	//While(DataPresent)
    }
    printf("\nReturning From CME FilePlay...\n\n");
    return 0;
}
//
//extern CME_Feeder *cfobj;
//
//int Init_CME_Feed_Replay() {
//
//	if(cfobj)delete cfobj; cfobj = new CME_Feeder(); //mfobj->BaseFileName = file;
//
//	return 0;
//}
//
//int Add_CME_ReplayFile(uint8_t Segment, const char *file=NULL){
//	return cfobj->Add_ReplayFile(Segment,  file);
//}
//
//void Start_CME_Feed_Replay(void){
//	cfobj->Start_FileReplay();
//}
