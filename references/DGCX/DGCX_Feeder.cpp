#include <sys/socket.h>
#include <arpa/inet.h>
#include <DGCX_Feeder.h>
#include <netdb.h>
using namespace DGCX;

#pragma pack(push,1)
typedef struct DGCX_FEED_DATA{
	uint32_t 		Token;
	uint32_t 		BuyP[5];
	uint32_t 		BuyQ[5];
	uint32_t 		SellP[5];
	uint32_t 		SellQ[5];
	int8_t          Delayed;
}DGCX_FEED_DATA;
#pragma pack(pop)

DGCX_Feeder::DGCX_Feeder(std::map<std::pair<int, int>, GenericContract> *contract):Contract(contract),TotalNodes(0),Data(NULL){

}

DGCX_Feeder::~DGCX_Feeder(){

}

int DGCX_Feeder::Add_ReplayFile(Exchange_Type Segment, int8_t StreamID, const char *file){
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
	pData->feed_log = fopen(filen, "w");  if(!pData->feed_log){cout << "Error Creating Output File => "<< filen << endl; exit(0);} if (setvbuf(pData->feed_log, NULL,_IONBF, 0) < 0){printf("setvbuf error in TokenOrderbook\n");return 0;}
	pData->fp =fp;
	pData->MSG_LEN=0;
	pData->sz=0;
	pData->SEQ_NO =0;
	pData->StreamID = StreamID;
	pData->ReceiveSequence=1;
	pData->Processed_Bytes = 0;
	pData->Pos = 0;
	pData->Buffer = (unsigned char*)numa_alloc_local(BYTES_READ_SIZE + 1024);
	pData->BytesToRead = BYTES_READ_SIZE;
	pData->_Segment = Segment;

	std::map<std::pair<int, int>, GenericContract>::iterator contract_itr;
	for( contract_itr= Contract->begin(); contract_itr != Contract->end(); contract_itr++ ){
		if(contract_itr->first.first == Segment){
			std::tr1::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator citr = instrument_umap.find(contract_itr->second.Token);
			if(citr == instrument_umap.end()){
				CONTRACT_DETAILS *cntr = new CONTRACT_DETAILS;
				memcpy(&cntr->Contract, &contract_itr->second, sizeof(GenericContract));
				instrument_umap[cntr->Contract.Token] = cntr;
			}
		}
	}

	TotalNodes++;
	return 0;
}

int DGCX_Feeder::generate_feed(DATA_HOLDER *pData,uint64_t *timestamp){
	DGCX_FEED_DATA *feed = (DGCX_FEED_DATA*)(pData->Buffer + pData->Processed_Bytes);

	std::tr1::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator citr = instrument_umap.find(feed->Token);
	if(citr != instrument_umap.end()){
		fprintf(pData->feed_log,"\n## Token:%u , %ld\n",feed->Token, *timestamp);
		for(int p=0;p<1;p++){
			fprintf(pData->feed_log,"%u--%0.2f \t\t %0.2f--%u\n", feed->BuyQ[p], ((double)feed->BuyP[p]/(double)100.00),
					((double)feed->SellP[p]/(double)100.00), feed->SellQ[p]);
		}
		Token_Data fdata; memset(&fdata, 0 , sizeof(Token_Data));
		fdata.Token = feed->Token;
		fdata.Delayed = 0;
		fdata.Timestamp = *timestamp;
		for(int p=0;p<FEED_LEVEL_DEPTH;p++){
			fdata.BuyPrice[p] = feed->BuyP[p];
			fdata.BuyQty[p] = feed->BuyQ[p];
			fdata.SellPrice[p] = feed->SellP[p];
			fdata.SellQty[p] = feed->SellQ[p];
		}
		if(fdata.BuyPrice[0] && fdata.SellPrice[0] && (fdata.SellPrice[0] > fdata.BuyPrice[0]))feed_func(EXCHG_DGCX, &fdata);

	}
	return 0;
}

int DGCX_Feeder::Start_FileReplay(void)
{
	uint64_t Timestamp[TotalNodes]; for(unsigned int nav=0; nav<TotalNodes; ++nav)Timestamp[nav] = 0; uint64_t *timestamp = NULL;
    //setvbuf(stdout, NULL,_IONBF, 0);

    while(1){
    	//if(TotalNodes > 1)
    	{
    		uint64_t min_time = 0x7FFFFFFFFFFFFFFF; int32_t nav = -1; DATA_HOLDER *pData = NULL;
			for(unsigned int i=0; i<TotalNodes; ++i){
				//printf("LINE => %d, [%d]=> %ld\n", __LINE__, i, Timestamp[i]);
				if(Timestamp[i] > 0){
					if(min_time > Timestamp[i]){min_time = Timestamp[i]; nav = i; pData = &Data[nav];}
				}else if(Timestamp[i] == 0){
					nav = i;  pData = &Data[nav];
					if(pData->Processed_Bytes < pData->sz){goto start;/*printf("LINE => %d, %d, %d, %d\n", __LINE__, nav, pData->Processed_Bytes , pData->sz);*/}else break;
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
									timestamp = (uint64_t *)(pData->Buffer + pData->Processed_Bytes);
									pData->MSG_LEN = *(int32_t *)(pData->Buffer + pData->Processed_Bytes + 8); pData->Processed_Bytes += (4 + 8);
								}
								else if(pData->MSG_LEN==0){
									pData->Pos = pData->sz; pData->BytesToRead = ((4 + 8) - (pData->sz - pData->Processed_Bytes));
									//printf("---- [ Length Alignment Started => pData->sz:%d pData->BytesToRead:%d MsgLength:%d pData->Processed_Bytes:%d ]---- \n",pData->sz,pData->BytesToRead,pData->MSG_LEN,pData->Processed_Bytes); fflush((FILE *)stdout);
									goto beginRead;
								}
								if((pData->MSG_LEN+pData->Processed_Bytes) > pData->sz){
									pData->Pos = pData->sz; pData->BytesToRead = (pData->MSG_LEN - (pData->sz - pData->Processed_Bytes));
									//printf("---- [ Byte Alignment Started => pData->sz:%d pData->BytesToRead:%d MsgLength:%d pData->Processed_Bytes:%d ]---- \n",pData->sz,pData->BytesToRead,pData->MSG_LEN,pData->Processed_Bytes); fflush((FILE *)stdout);
									goto beginRead;
								}

								if(Timestamp[nav] == 0){
									Timestamp[nav] = *timestamp; goto end;
								}

								//printf("[timestamp:%ld][%d] => pData->sz=%d,\tProcessed_Bytes:%d,\tMsgLength:%d\n",Timestamp[nav], nav, pData->sz, pData->Processed_Bytes,pData->MSG_LEN);

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
    printf("\nReturning From DGCX FilePlay...\n\n");
    return 0;
}
//
//extern DGCX_Feeder *dfobj;
//
//int Init_DGCX_Feed_Replay() {
//
//	if(dfobj)delete dfobj; dfobj = new DGCX_Feeder(); //mfobj->BaseFileName = file;
//
//	return 0;
//}
//
//int Add_DGCX_ReplayFile(uint8_t Segment, const char *file=NULL){
//	return dfobj->Add_ReplayFile(Segment,  file);
//}
//
//void Start_DGCX_Feed_Replay(void){
//	dfobj->Start_FileReplay();
//}
