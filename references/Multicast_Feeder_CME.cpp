
#include <ConfigReader.h>
#include <includes.h>
#include <qed.h>
#include <qed_parser.h>
#include <qed_msg.h>
#include <qed_msg_extra.h>
#include <qed_hb.h>
#include <qed_symbology.h>
#include <qedc_context.h>
#include <qed_msg_extra_hf.h>
#include <Multicast_Feeder_CME.h>
using namespace QED;
using namespace CME;

//#define EXANIC_KERNEL_BYPASS_CME
#define ENCRYPTION_ON

#ifdef EXANIC_KERNEL_BYPASS_CME
#include <exanic/exanic.h>
#include <exanic/time.h>
#include <exanic/fifo_rx.h>
#include <exanic/filter.h>
#endif


#define NUMA_MEMEORY_ALLOCATION

 extern char *MulticastFileName;
 vector<string> split(const string& s, const string& delim, const bool keep_empty = true) {
    vector<string> result;
    if (delim.empty()) {
        result.push_back(s);
        return result;
    }
    string::const_iterator substart = s.begin(), subend;
    while (true) {
        subend = search(substart, s.end(), delim.begin(), delim.end());
        string temp(substart, subend);
        if (keep_empty || !temp.empty()) {
            result.push_back(temp);
        }
        if (subend == s.end()) {
            break;
        }
        substart = subend + delim.size();
    }
    return result;
}
#define QED_MTU 1520

int32_t Get_CME_ExpiryDate(int32_t Month, int32_t Year) {
        if(Month == 'F')Month = 1; //January    =       'F',
        else if(Month == 'G')Month = 2; // February     =       'G',
        else if(Month == 'H')Month = 3;//March  =       'H',
        else if(Month == 'J')Month = 4;//April          =       'J',
        else if(Month == 'K')Month = 5;//May    =       'K',
        else if(Month == 'M')Month = 6;//June   =       'M',
        else if(Month == 'N')Month = 7;//July   =       'N',
        else if(Month == 'Q')Month = 8;//August =       'Q',
        else if(Month == 'U')Month = 9;//September      =       'U',
        else if(Month == 'V')Month = 10;//October       =       'V',
        else if(Month == 'X')Month = 11;//November      =       'X',
        else if(Month == 'Z')Month = 12;//December      =       'Z',
        else{
                printf("\n\nInvalid CME Month Code:%c\n\n", Month);
                //ABORT;
        }
time_t t = time(NULL); tm* timePtr = localtime(&t); Year = 2000 + Year; while(Year < (timePtr->tm_year + 1900)){Year +=10;}
    tm dayofmonth= {0}; dayofmonth.tm_year = Year; dayofmonth.tm_mon = Month; dayofmonth.tm_mday = 0; mktime(&dayofmonth);
        char date_buffer[50]; sprintf(date_buffer, "%d-%02d-%02d-00:00:00", dayofmonth.tm_year, (dayofmonth.tm_mon+1), dayofmonth.tm_mday); int32_t ExpiryDate = 0;
        struct tm tmTime; memset(&tmTime,0,sizeof(struct tm)); strptime(date_buffer,"%Y-%m-%d:%H:%M:%S", &tmTime);ExpiryDate = mktime(&tmTime);
        //cout << endl << " TimeBuff:" << date_buffer << " ExpiryDate:" << ExpiryDate << endl << endl;
        return ExpiryDate;
};

typedef struct {
  uint16_t lastHbSeqNo;
} MyQEDClosure;

 static void printQEDError(const char *info, int32_t err) {
   fprintf(stderr, "%s: QED error %d: %s\n", info, err, QED_error_string(err));
 }

int32_t OrderRequestHandler::StartPrimaryFeed(){
	pthread_attr_t attr2; pthread_attr_init (&attr2);	pthread_attr_setdetachstate (&attr2, PTHREAD_CREATE_DETACHED);
	if(pthread_attr_setinheritsched (&attr2, PTHREAD_EXPLICIT_SCHED)!=0){perror("pthread_attr_setinheritsched:");ABORT;}
	int32_t s = pthread_create(&PrimaryStreamPThreadID, &attr2, &Stream_Processing_Thread_Func, this);
	if (s != 0)
	{
		char output[256];
		sprintf(output,"Error: %d  Stream_Processing_Thread_Func pthread_create( CME ) error\n",s);
		fwrite(output, 1, strlen(output), feedLog); cout << output;
	}
	return 0;
}

void *OrderRequestHandler::Stream_Processing_Thread_Func(void *arg)
{
	OrderRequestHandler *ptr = (OrderRequestHandler*)arg; char output[2048];//int32_t FeedLoggerIndex=0;
	if(ptr->Primary_Multicast_Core  > 0){
		cpu_set_t cpuset;
		CPU_ZERO(&cpuset);
		CPU_SET(ptr->Primary_Multicast_Core, &cpuset);
		int32_t ret = pthread_setaffinity_np( pthread_self(), sizeof(cpu_set_t), &cpuset);
    	sprintf(output,"Primary Processing Thread Of CME Started On Core:%d With Error Code :%d\n\n",(int32_t)ptr->Primary_Multicast_Core,ret);fwrite(output, 1, strlen(output), ptr->feedLog);
    	printf("Primary Processing Thread Of CME Started On Core:%d With Error Code :%d\n\n",(int32_t)ptr->Primary_Multicast_Core,ret);fwrite(output, 1, strlen(output), ptr->feedLog);
	}
	else{
		sprintf(output,"Primary Processing Thread Of CME Started On Core:%d\n\n",ptr->Primary_Multicast_Core);fwrite(output, 1, strlen(output), ptr->feedLog);
	}
    if (pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, NULL) != 0){
    	sprintf(output,"CME => core:%d pthread_setcancelstate error\n\n",ptr->Primary_Multicast_Core);fwrite(output, 1, strlen(output), ptr->feedLog);
    }
#ifdef NUMA_MEMEORY_ALLOCATION
    numa_set_strict(1);
#endif
	ConfigFile cfg(MulticastFileName, __FUNCTION__,__LINE__);
	QED_Status s;  QED_initializeLogs(); QED_setLogFile(ptr->feedLog); QED_ctx ctx = NULL;
#ifdef ENCRYPTION_ON
	QEDC_context cryptCtx = NULL;
	s = QEDC_init_context(&cryptCtx, "./keys");
	if(s != QED_EOK) {
		printQEDError("QEDC_init_context error", s);ABORT;
	}
#endif
	if(cfg.getMultiValueOfKey<std::string>("EXCHG_CME_GET_SYMBOL_LIST_FROM_HEX","-",true).size() < 5)
	{
		ctx = QED_initWithPath(cfg.getValueOfKey<std::string>("EXCHG_CME_CONFIG_SECTION","-",true).c_str(), cfg.getValueOfKey<std::string>("EXCHG_CME_CONFIG_DIR_PATH","-",true).c_str());
		if (ctx == NULL) {
			printf("\n@ QED_init error:%s \n\n", QED_error_string(QED_errno())); ABORT;
		}
		char *sdata; unsigned long int slen=0; QED_getSymbolListPacket(ctx, (const char**)&sdata, &slen);printf("\n---------------------- %ld --------------------\n\n", slen);for(uint32_t i = 0; i <slen; i++){printf("0x%02x,", ((unsigned char*)sdata)[i]);}
	}
	else{
		ctx = QED_initWithUserSymbolPackets("CME", "./");
		if (ctx == NULL) {
			printf("\nQED_init error:%s \n\n", QED_error_string(QED_errno())); ABORT;
		}
		if(!strcmp(cfg.getMultiValueOfKey<std::string>("EXCHG_CME_GET_SYMBOL_LIST_FROM_HEX","-",true).c_str(), "PATCHED")){
			/*unsigned char *trPTR, rbuffer[2048]; memset(rbuffer, 0, sizeof(rbuffer));
			rbuffer[0]=0x03; rbuffer[1]=0x03; rbuffer[2]=0x01; rbuffer[3]=0x00; rbuffer[4]=0x01; rbuffer[5]=0x00; rbuffer[6]=0x00; rbuffer[7]=0x00;
			trPTR = (rbuffer + 8);
			for(std::unordered_map<int32_t,FEED_DETAILS*>::iterator contract_itr= contract_file->instrument_umap[EXCHG_CME].begin(); contract_itr != contract_file->instrument_umap[EXCHG_CME].end(); contract_itr++ ){
				*((qint16*)trPTR) = rcnt; trPTR += 2; // Counter
				//if(strstr(it->second->SymbolCode, "XNYM.CLH7") == NULL)continue;
				*((int16_t*)trPTR) = contract_itr->second->StartSequence; trPTR += 2;
				*((int16_t*)trPTR) = contract_itr->second->StreamID;  trPTR += 1;
				*((int16_t*)trPTR) = 0x1e;     trPTR += 1; // ??
				*((int16_t*)trPTR) = 0;     trPTR += 1; // ??
				 strcpy((char*)trPTR, contract_itr->second->Contract.SymbolCode); trPTR += (strlen(contract_itr->second->Contract.SymbolCode) + 1); // Symbol
			}
			int32_t qret = QED_processSymbolPacket(ctx, (char*)(rbuffer), (trPTR - rbuffer));
			if(qret != QED_EOK){
				printf("\nQED_processSymbolPacket error:%s , %d\n\n", QED_error_string(QED_errno()), qret); ABORT;
			}*/
		}
		else{
			std::vector<std::string> parts; parts = split(cfg.getMultiValueOfKey<std::string>("EXCHG_CME_GET_SYMBOL_LIST_FROM_HEX","-",true).c_str(), ",");
			int32_t cnt=0, slen = parts.size(); char *sdata = new char[slen + 1];for(std::vector<std::string>::iterator itr= parts.begin(); itr!= parts.end(); ++itr){sscanf((*itr).c_str(), "%x", (unsigned int*)&sdata[cnt]);cnt++;	}//for(int32_t i = 0; i <slen; i++){printf("%02x ", sdata[i]);}printf("\n---------------------- %d --------------------\n\n", slen);
			int32_t qret = QED_processSymbolPacket(ctx, sdata, slen);
			if(qret != QED_EOK){
				printf("\nQED_processSymbolPacket error:%s , %d\n\n", QED_error_string(QED_errno()), qret); ABORT;
			}
		}
	}

	QED_parser parser = QED_create_parser(ctx);	MyQEDClosure closure;


	/*std::unordered_map<std::string,SymbolCodeInfo*> SymbolCodeToIndex; std::unordered_map<uint32_t,SymbolCodeInfo*> TokenToIndex[2];

	std::unordered_map<int32_t,FEED_DETAILS*>::iterator contract_itr;
	for( contract_itr= contract_file->instrument_umap[EXCHG_CME].begin(); contract_itr != contract_file->instrument_umap[EXCHG_CME].end(); contract_itr++ ){
		SymbolCodeInfo *info = (SymbolCodeInfo*)malloc(sizeof(SymbolCodeInfo));
		info->Exchange = EXCHG_CME;
		info->Index = contract_itr->second->Contract.Index;
		info->SendUpdates = false;
		memset(&info->OrderBook.Feed, 0, sizeof(Token_Data));
		info->OrderBook.Feed.Token = contract_itr->second->Contract.Token;
		info->OrderBook.Queued = 0;
		info->OrderBook.Lock.LockVal = 0;
		info->OrderBook.Lock.UnlockVal = 1;
		SymbolCodeToIndex[contract_itr->second->Contract.SymbolCode] = info;
		TokenToIndex[0][contract_itr->second->Contract.Token] = info;
		  if ((s = QED_parser_register_symbol(parser, contract_itr->second->Contract.SymbolCode, true, &closure)) == QED_EOK) {
		  } else {
			printQEDError("QED_parser_register_symbol error ", s);
			const char** symbols;  uint16_t  count = QED_getSymbolList(ctx, &symbols);
			for(int32_t i = 0; i < count; i++){
			  printf("Registered Symbol %s, SymbolChannelID:%d\n", symbols[i], QED_getSymbolChannelID(ctx,symbols[i]));
			  fprintf(ptr->feedLog, "Registered Symbol %s, SymbolChannelID:%d\n", symbols[i], QED_getSymbolChannelID(ctx,symbols[i]));
			}
			
			fprintf(ptr->feedLog, "\nCME Symbol:%s Not Found..\n", contract_itr->second->Contract.SymbolCode);
			printf("\nCME Symbol:%s Not Found..\n", contract_itr->second->Contract.SymbolCode); ABORT;
		  }
	}
	for( contract_itr= contract_file->instrument_umap[EXCHG_LME].begin(); contract_itr != contract_file->instrument_umap[EXCHG_LME].end(); contract_itr++ ){
		SymbolCodeInfo *info = (SymbolCodeInfo*)malloc(sizeof(SymbolCodeInfo));
		info->Exchange = EXCHG_LME;
		info->Index = contract_itr->second->Contract.Index;
		info->SendUpdates = false;
		memset(&info->OrderBook.Feed, 0, sizeof(Token_Data));
		info->OrderBook.Feed.Token = contract_itr->second->Contract.Token;
		info->OrderBook.Queued = 0;
		info->OrderBook.Lock.LockVal = 0;
		info->OrderBook.Lock.UnlockVal = 1;
		SymbolCodeToIndex[contract_itr->second->Contract.SymbolCode] = info;
		TokenToIndex[1][contract_itr->second->Contract.Token] = info;
		  if ((s = QED_parser_register_symbol(parser, contract_itr->second->Contract.SymbolCode, true, &closure)) == QED_EOK) {
		  } else {
			printQEDError("QED_parser_register_symbol error ", s);
			const char** symbols;  uint16_t  count = QED_getSymbolList(ctx, &symbols);
			for(int32_t i = 0; i < count; i++){
			  printf("Registered Symbol %s, SymbolChannelID:%d\n", symbols[i], QED_getSymbolChannelID(ctx,symbols[i]));
			}
			fprintf(ptr->feedLog, "\nLME Symbol:%s Not Found..\n", contract_itr->second->Contract.SymbolCode); printf("\nLME Symbol:%s Not Found..\n", contract_itr->second->Contract.SymbolCode); ABORT;
		  }
	}*/

	const char** symbols;  uint16_t  count = QED_getSymbolList(ctx, &symbols);printf("\n[ Quincy ]\n");
	std::unordered_map<std::string,FEED_DETAILS> SymbolData;
	for(int32_t i = 0; i < count; i++){
	  printf("Symbol %s, SymbolChannelID:%d\n", symbols[i], QED_getSymbolChannelID(ctx,symbols[i]));
	  if(QED_getSymbolChannelID(ctx,symbols[i]) == 69 || QED_getSymbolChannelID(ctx,symbols[i]) == 89){
		  printf("Registering symbol:%s, symbolChannelID:%d\n", symbols[i], QED_getSymbolChannelID(ctx,symbols[i]));
	  	if ((s = QED_parser_register_symbol(parser, symbols[i], true, &closure)) == QED_EOK) {
			FEED_DETAILS data;// = new Token_Data;
			memset(&data,0,sizeof(FEED_DETAILS));
			data.ExpiryDate = Get_CME_ExpiryDate( *(symbols[i]+9) , ( *(symbols[i]+strlen(symbols[i]) - 1)-48) );//Get_CME_ExpiryDate( *(symbols[i]+strlen(symbols[i]) - 2) , ( *(symbols[i]+strlen(symbols[i]) - 1)-48) );
			data.Index = QED_getSymbolChannelID(ctx,symbols[i]);
			strcpy(data.SymbolCode, symbols[i]);
			SymbolData[symbols[i]] = data;
                  } else {
			  printQEDError("QED_parser_register_symbol error ", s);
			  printf("\nCME Symbol:%s Not Found..\n");ABORT;
		  }
	  }
	}printf("\n");

	printf("Registered Symbols:%d\n",SymbolData.size());

	char pfile[256];sprintf(pfile, "/home/ashok/CME_DecodeOldData/Debug_Exanic_Quincy/cme_feeder_capture_Tier_1_02_09_2026.bin");
	FILE *fp = fopen(pfile, "r");
	if (fp == NULL) {
        	printf("\ncouldn't open FEED_DATA_FILE :\033[1;31m %s \033[0mfor reading.\n\n",pfile); ABORT;
    	}
	unsigned char Buffer[8192], *buffer = NULL;

	  while (true) {
		  ssize_t nRead;
                if (fread(&nRead, sizeof(nRead), 1, fp) != 1){
			break;exit(0);
		}
		unsigned char buffer[2048];
		uint64_t RcTime=0;
		int sz=0;
                if (sz=fread(Buffer, nRead, 1, fp) != 0) {
			RcTime = *((uint64_t *)Buffer);
			ssize_t res = nRead - sizeof(uint64_t);
			memcpy(buffer, (char*)(Buffer+sizeof(uint64_t)) , res);
			if(QEDC_is_encrypted_packet((uint8_t*)buffer, res)) {
      				size_t len = res;
      				QED_Status s = QEDC_decrypt_packet(cryptCtx, (uint8_t*)(buffer), (uint8_t*)(buffer), &len);
     				if(s != QED_EOK) {
        				printQEDError("QEDC_decrypt_packet error", s);
        				ABORT;
      				}
				res = len;

			}
			else{
				continue;
			}
		    int offset = QEDC_PAYLOAD_OFFSET;
	

	    QED_msg msg = QED_parser_begin(parser, (char*)(buffer + offset), res - offset);
	    //if(res>0) printf("Token: %d\n", *(int16_t*)(buffer + 30));
	    if (msg == NULL) {
			if (QED_EHB == QED_errno()){  /*printHeartbeat(parser, buffer, res);*/ continue;}
			//printf("---> %d , %d\n", QED_parser_get_pckt_seq_no( (char*)buffer, res), QED_errno());
			if(QED_EOK == QED_errno())continue;
			fprintf(ptr->feedLog,"\nrecv_len:%ld \n",res);for(int32_t x=0; x < res; x++){fprintf(ptr->feedLog,"%02x ",buffer[x]);	}fprintf(ptr->feedLog,"\n\n");
			printQEDError("QED_parser_begin error", QED_errno()); //break;
			abort();
	    }
	    else{
			while(true) {
				MyQEDClosure *myClosure = (MyQEDClosure *)QED_msg_get_closure(msg);
				if (myClosure == NULL) {printQEDError("QED_msg_get_closure error", QED_errno());goto CLEANRETURN;}
				std::unordered_map<std::string,FEED_DETAILS>::iterator itr = SymbolData.find(QED_msg_get_topic(msg));
				//fprintf(ptr->feedLog,"Searching symbol:%s\n",QED_msg_get_topic(msg));
				if(itr != SymbolData.end())
				{

					QED_Direction direction; uint64_t threshold; QED_msg_get_hf_direction_and_threshold(msg, &direction, &threshold); uint8_t tflg = 0;
					if( QED_EOK == QED_errno()){
						tflg = 0x7f;
					}
					else{
					  const unsigned int nTradeLevels = QED_msg_trade_level_count(msg); unsigned int tradeLevelI;
					  for (tradeLevelI = 0; tradeLevelI < nTradeLevels; tradeLevelI++) {
						QED_Price price = QED_msg_get_trade_price(msg, tradeLevelI);
						if (/*qty != QED_INVALID_TRADE_QTY && */QED_price_is_valid(price)) {
						  QED_trade_flag aggressor = QED_msg_get_trade_aggressor(msg, tradeLevelI);
						  //double priceD = QED_price_as_double(price); fprintf(ptr->feedLog,"Trades: %ld::%.5f : %d @ (%d)\n\n", price, priceD, qty, aggressor);
							itr->second.LastTradedPrice = (int32_t)price.man;
							itr->second.LastTradedQty = QED_msg_get_trade_qty(msg, tradeLevelI);
							itr->second.FeedEventAggressor = aggressor + 1;
							tflg = 3;
						}
					  }
					  if(QED_msg_depth(msg)){
						for(int32_t i=0; i < FEED_LEVEL_DEPTH; i++){
							//printf("%d\n",__LINE__);
							itr->second.BuyPrice[i] 	=	(int32_t)QED_price_mantissa(QED_msg_get_level_price(msg, i, QED_BID ));
							itr->second.SellPrice[i]	= 	QED_price_mantissa(QED_msg_get_level_price(msg, i, QED_ASK ));
							itr->second.BuyQty[i] 		=  QED_msg_level_qty(msg, i, QED_BID );
							itr->second.SellQty[i] 		=  QED_msg_level_qty(msg, i, QED_ASK);
							itr->second.NoOfBuyOrds[i] 	=  QED_msg_level_number_of_orders(msg, i, QED_BID );
							itr->second.NoOfSellOrds[i] 	=  QED_msg_level_number_of_orders(msg, i, QED_ASK);
						}
						tflg = 1;
					  }
					  	itr->second.SrcTimestamp = QED_msg_get_src_time(msg);
						itr->second.DS_rcv_timestamp = QED_msg_get_ds_rcv_time(msg);
						itr->second.DS_snd_timestamp = QED_msg_get_ds_snd_time(msg);
					}

					if(tflg < 0x7f){
						/*fprintf(ptr->feedLog,"\n-----------%s @ LTP:%d----------\n", QED_msg_get_topic(msg), itr->second.LastTradedPrice);
						for(int32_t i=0; i<5; ++i){
						 fprintf(ptr->feedLog,"%d\t-\t%d\t-\t%d\t-\t%d\n", itr->second.BuyQty[i], itr->second.BuyPrice[i], itr->second.SellPrice[i], itr->second.SellQty[i]);
						}*/
						ssize_t nTot = sizeof(FEED_DETAILS) +  sizeof(uint64_t);
						fwrite(&nTot, 1, sizeof(ssize_t), ptr->feedCapture);
						fwrite(&RcTime, 1, sizeof(uint64_t), ptr->feedCapture);
						fwrite(&itr->second, 1, sizeof(FEED_DETAILS), ptr->feedCapture);
					}
				}
				msg = QED_parser_next(parser);
                                if (msg == NULL) {
                                	if (QED_errno() != QED_EOK) {
                                  		printQEDError("QED_parser_next error", QED_errno());
                                  		goto CLEANRETURN;
                                	}
                                	break;
                                
				}
	    		}
		}
		}

			__MEMORY_BARRIER__;
	  }

CLEANRETURN:
	QED_parser_free(parser);
	QED_close(ctx);
	return NULL;
}

OrderRequestHandler::OrderRequestHandler():feedLog(NULL) , feedCapture(NULL), PrimaryStreamPThreadID(0), Primary_Multicast_Core(0){
	if(!feedLog){
		char filelogger[256]; time_t now; char the_date[50]; the_date[0] = '\0'; now = time(NULL); if (now != -1){ strftime(the_date, 50, "%d_%m_%Y", localtime(&now));} else{printf("\nDate error...\n\n");ABORT;}
		sprintf(filelogger,"cme_feeder_%s.%d.log",the_date,getpid()); feedLog = fopen(filelogger, "wb"); setvbuf(feedLog, NULL,_IONBF, 0); if(feedLog==NULL){printf("\nfile opening error :%s...\n\n",filelogger);ABORT;}
	}
	if(!feedCapture){
		char filelogger[256]; time_t now; char the_date[50]; the_date[0] = '\0'; now = time(NULL); if (now != -1){ strftime(the_date, 50, "%d_%m_%Y", localtime(&now));} else{printf("\nDate error...\n\n");ABORT;}
                sprintf(filelogger,"cme_feeder_%s.bin",the_date); feedCapture = fopen(filelogger, "wb"); setvbuf(feedCapture, NULL,_IONBF, 0); if(feedCapture==NULL){printf("\nfile opening error :%s...\n\n",filelogger);ABORT;}
	}
}
