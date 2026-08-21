/*
 * Contract_MCX.cpp
 *
 *  Created on: May 14, 2013
 *
 */

#include "Contract.h"
#include <map>
#include <string>
#include <vector>
#include <ctime>

#define EXCHG_MCX_OMS
#define EXCHG_BSE_CD_OMS
#define EXCHG_INX_OMS
#define EXCHG_CME_FEEDER
#define EXCHG_NSE_FO_OMS
#define EXCHG_NSE_CM_OMS
#define EXCHG_NSE_CD_OMS
#define EXCHG_DGCX_OMS

//std::map<std::pair<int, int>, GenericContract> ContractM;

int32_t Get_BSE_FO_ExpiryDate(std::string Date) {
    std::vector<std::string> parts; parts = split(Date, "-");
    int day = std::atoi(parts[0].c_str());
    std::string Month = parts[1];
    int Year = std::atoi(parts[2].c_str());
    int month = 1;
    if(Month == "JAN")month = 1;
    else if(Month == "FEB")month = 2;
    else if(Month == "MAR")month = 3;
    else if(Month == "APR")month = 4;
    else if(Month == "MAY")month = 5;
    else if(Month == "JUN")month = 6;
    else if(Month == "JUL")month = 7;
    else if(Month == "AUG")month = 8;
    else if(Month == "SEP")month = 9;
    else if(Month == "OCT")month = 10;
    else if(Month == "NOV")month = 11;
    else if(Month == "DEC")month = 12;

    //time_t t = time(NULL);
    tm dayofmonth= {0};
    dayofmonth.tm_mday = day;
    dayofmonth.tm_year = Year - 1900;
    dayofmonth.tm_mon = month - 1;
    int32_t ExpiryDate = 0;
    ExpiryDate = mktime(&dayofmonth);
	struct timespec todayTS;
	if( clock_gettime( CLOCK_REALTIME, &todayTS) == -1 ) {
		perror( "clock gettime @ strategy => " );
		exit( EXIT_FAILURE );
    }
	if(ExpiryDate+19800 < todayTS.tv_sec){
		return 0;
	}
    return ExpiryDate + 19800;
};

int32_t Get_CME_ExpiryDate(int32_t Month, int32_t Year) {
	if(Month == 'F')Month = 1; //January	= 	'F',
	else if(Month == 'G')Month = 2; // February 	=	'G',
	else if(Month == 'H')Month = 3;//March	=	'H',
	else if(Month == 'J')Month = 4;//April		=	'J',
	else if(Month == 'K')Month = 5;//May 	=	'K',
	else if(Month == 'M')Month = 6;//June	=	'M',
	else if(Month == 'N')Month = 7;//July 	=	'N',
	else if(Month == 'Q')Month = 8;//August	=	'Q',
	else if(Month == 'U')Month = 9;//September	=	'U',
	else if(Month == 'V')Month = 10;//October	=	'V',
	else if(Month == 'X')Month = 11;//November	=	'X',
	else if(Month == 'Z')Month = 12;//December	=	'Z',
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

int32_t Get_LME_ExpiryDate(int32_t Month, int32_t Year) {

    tm dayofmonth= {0}; dayofmonth.tm_year = Year; dayofmonth.tm_mon = Month; dayofmonth.tm_mday = 0; mktime(&dayofmonth);
	char date_buffer[50]; sprintf(date_buffer, "%d-%02d-%02d-00:00:00", dayofmonth.tm_year, (dayofmonth.tm_mon+1), dayofmonth.tm_mday); int32_t ExpiryDate = 0;
	struct tm tmTime; memset(&tmTime,0,sizeof(struct tm)); strptime(date_buffer,"%Y-%m-%d:%H:%M:%S", &tmTime);ExpiryDate = mktime(&tmTime);
	//cout << endl << " TimeBuff:" << date_buffer << " ExpiryDate:" << ExpiryDate << endl << endl;
	return ExpiryDate;
};

Contract::Contract() {
	for(uint32_t x=0;x<EXCHG_NONE;x++){
		min_token[x] = 0x7FFFFFFF; max_token[x]=0; max_index[x] = 0;
	}
	Max_Product_ID_BSE_CD = Max_Product_ID_INX = 0;
}

Contract::~Contract() {
	// TODO Auto-generated destructor stub
}

void Contract::set_min_max_token(Exchange_Type Exchange,uint32_t token)
{
	if (token < min_token[Exchange])
		min_token[Exchange]=token;
	if (token > max_token[Exchange])
		max_token[Exchange]=token;
}

void Contract::load_exchanges_contract(map<Exchange_Type, std::string> &cpath)
{
	CONTRACT_DETAILS *ct = NULL; int32_t printlog = 0;
	if(printlog){cout << "\nContract Log Printing Enabled..\n\n"; }
	for(uint32_t x=0;x<EXCHG_NONE;x++){
		//printf("\n******************** Gonna Load & Process %s Contract File **********************\n",Exchange_Type_To_String((Exchange_Type)x));
		switch(x)
		{
			case EXCHG_MCX: {
#if defined(EXCHG_MCX_FEEDER) || defined(EXCHG_MCX_OMS)
				std::string file = cpath[(Exchange_Type)x]; file.append("MCXScrips.bcp");
				std::ifstream in_stream(file.c_str(),std::ifstream::in);
				if (in_stream.is_open())
				{
					char Token_DPR_High[50],Token_DPR_Low[50]; int32_t tIndex = 0;
					for(std::string line;std::getline(in_stream,line);)
					{
						std::vector<std::string> parts; parts = split(line, ","); int32_t key = atoi(parts[5].c_str());
						if (key == 0)// || key !=207536)
							continue;

						if (atoi(parts[9].c_str())==1 && atoi(parts[108/*Spread Type*/].c_str()) == 0 && (parts[38].c_str())[0] == 'N'
//							&& (
//								strncmp(parts[53].c_str(),"COM",3)	  == 0 ||
//								strncmp(parts[53].c_str(),"FUTCOM",6) == 0 ||
//								strncmp(parts[53].c_str(),"FUTIDX",6) == 0 ||
//								(
//									(
//										strncmp(parts[53].c_str(),"OPTIDX",6) == 0 ||
//										strncmp(parts[53].c_str(),"OPTCOM",6) == 0 ||
//										strncmp(parts[53].c_str(),"OPTFUT",6) == 0
//									)
//									&&
//									(
//										strstr(parts[6].c_str(), "CRUDEOIL")
////										strstr(parts[6].c_str(), "GOLD") ||
////										strstr(parts[6].c_str(), "SILVER")
//									)
//								)
//							)
						)
						{

							ct = new CONTRACT_DETAILS;
							memset(&ct->Contract, 0, sizeof(GenericContract));
							pthread_spin_init(&ct->ContractLock, PTHREAD_PROCESS_PRIVATE);
							ct->PTR = NULL;
							ct->StreamID = atoi(parts[2].c_str());
							ct->StartSequence = 1;
							ct->Contract.Exchange = EXCHG_MCX;
							ct->Contract.Index = tIndex++;
							ct->Contract.Token = key;

							// if(457532 == ct->Contract.Token || 457533 == ct->Contract.Token){
							// 	printf("\n\nToken:%d, Symbol:%s, Expiry:%s, Strike:%s, OptionType:%s\n\n", ct->Contract.Token, parts[6].c_str(), parts[54].c_str(), parts[55].c_str(), parts[56].c_str());
							// }
							if(atoi(parts[54].c_str()) > 0){
								ct->Contract.ExpiryDate = atoi(parts[54].c_str()) - 19800;
							}else ct->Contract.ExpiryDate = 0;
							ct->Contract.StrikePrice = atoi(parts[55].c_str());
							//imt->iBasePrice = (uint32_t)(((((double)imt->iLotSize * (double)imt->dTradingUnitFactor) / (double)imt->iPriceQuoteQty)) * 100);
							ct->Contract.BasePrice = (PriceType)(((((double)(atoi(parts[20].c_str())) * (double)(atof(parts[77].c_str()))) / (double)(atoi(parts[62].c_str())))) * 100);
							ct->Contract.TickSize = atoi(parts[21].c_str());
							if(ct->Contract.TickSize < 5){
								ct->Contract.TickSize = 5;
							}
							ct->Contract.LotSize=atoi(parts[20].c_str());
							sprintf(Token_DPR_High,"%d_High_DPR_Value",key); sprintf(Token_DPR_Low,"%d_Low_DPR_Value",key);
							ct->Contract.LowPriceRange = 5;
							ct->Contract.HighPriceRange = atof(parts[64].c_str());
                            if(ct->Contract.HighPriceRange < 0){
                                ct->Contract.HighPriceRange = ct->Contract.TickSize * 10;
                            }
							ct->Contract.LowPriceRange = atof(parts[65].c_str());
                            if(ct->Contract.LowPriceRange < 0){
                                ct->Contract.LowPriceRange = ct->Contract.TickSize;
                            }
							if(ct->Contract.LowPriceRange < ct->Contract.TickSize)ct->Contract.LowPriceRange = 0;else if((ct->Contract.LowPriceRange % ct->Contract.TickSize))ct->Contract.LowPriceRange -= (ct->Contract.LowPriceRange % ct->Contract.TickSize);
							ct->Contract.Multiplier = (PriceType)((((double)atoi(parts[20].c_str()) * atof(parts[77].c_str())) / atoi(parts[62].c_str())) * 100);
							ct->Contract.PriceExponent = 2; ct->Contract.PriceExponentDisplay = 2;
							parts[6].erase(remove(parts[6].begin(), parts[6].end(), ' '), parts[6].end());
							strcpy(ct->Contract.Symbol ,parts[6].c_str()); strcpy(ct->Contract.SymbolCode ,"-");
							char inst[256]; int32_t ix = 0; strcpy(inst,parts[53].c_str());while(inst[ix]){if(inst[ix] == ' '){inst[ix]='\0';break;}else ix++;}
							ct->Contract.InstrumentType = String_To_Instrument_Type(inst);
							if(ct->Contract.InstrumentType == Instrument_Type::INST_ERROR){
								cout << "\nInstrumentType ::" << parts[53].c_str() << " Not Defined...\n" << endl; ABORT;
							}
//							if(!strncmp(parts[53].c_str(), "COM", 3)){
//
//							}
							ct->Contract.OptionType = String_To_Option_Type(parts[56].c_str());
							
							//From 11thMarch, 2026 to 29thMarch, 2026
							// if(ct->Contract.InstrumentType == FUTCOM)
							// {
							// 	if(!memcmp(ct->Contract.Symbol,"ALUMINI", 7)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"ALUMINIUM", 9)){
							// 		ct->StreamID = 5;
 							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"CARDAMOM", 8)){
                            //     	ct->StreamID = 2;
                            //     }
							// 	else if(!memcmp(ct->Contract.Symbol,"COPPER", 6)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"COTTON", 6)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"COTTONCNDY", 10)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"COTTONOIL", 9)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"CRUDEOIL", 8)){
							// 		ct->StreamID = 3;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"CRUDEOILM", 9)){
							// 		ct->StreamID = 1;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"ELECDMBL", 8)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLD", 4)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLDGUINEA", 10)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLDM", 5)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLDPETAL", 9)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLDPETAL", 9)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLDTEN", 7)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"KAPAS", 5)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"LEAD", 4)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"LEADMINI", 8)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"MCXBULLDEX", 10)){
							// 		ct->StreamID = 1;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"MCXMETLDEX", 10)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"MENTHAOIL", 9)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"NATGASMINI", 10)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"NATURALGAS", 10)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"NICKEL", 6)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"SILVER", 6)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"SILVERM", 7)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"SILVERMIC", 9)){
							// 		ct->StreamID = 3;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"STEELREBAR", 10)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"ZINC", 4)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"ZINCMINI", 8)){
							// 		ct->StreamID = 5;
							// 	}
							// }
							// else
							// {
							// 	if(!memcmp(ct->Contract.Symbol,"COPPERM", 7)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"COTTONREF", 9)){
							// 		ct->StreamID = 2;
 							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"CPO", 3)){
                            //     	ct->StreamID = 1;
                            //     }
							// 	else if(!memcmp(ct->Contract.Symbol,"MCXBULLDEX", 10)){
							// 		ct->StreamID = 1;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"MCXMETLDEX", 10)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"NICKELM", 7)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"COPPER", 6)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"CRUDEOIL", 8)){
							// 		ct->StreamID = 4;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"CRUDEOILM", 9)){
							// 		ct->StreamID = 1;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLD", 4)){
							// 		ct->StreamID = 1;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"GOLDM", 5)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"NATGASMINI", 10)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"NATURALGAS", 10)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"SILVER", 6)){
							// 		ct->StreamID = 2;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"SILVERM", 7)){
							// 		ct->StreamID = 3;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"ZINC", 4)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"OGMCXBULLDEX", 12)){
							// 		ct->StreamID = 5;
							// 	}
							// 	else if(!memcmp(ct->Contract.Symbol,"RUBBER", 6)){
							// 		ct->StreamID = 2;
							// 	}
							// } 
														// Start to Jan23rd, 2026
														// if(!memcmp(ct->Contract.Symbol,"GOLDM", 5)){
                                                        //         if(ct->Contract.InstrumentType == FUTCOM){ct->StreamID = 1;}
                                                        //         else{ct->StreamID = 2;}
                                                        // }
                                                        // else if(!memcmp(ct->Contract.Symbol,"GOLD", 4) || !memcmp(ct->Contract.Symbol,"CRUDEOILM", 9)){
                                                        //         ct->StreamID = 1;
                                                        // }
                                                        // else if(!memcmp(ct->Contract.Symbol,"SILVERMIC", 9)){
                                                        //         ct->StreamID = 4; //(StreamID 3 for Start to 23rdJan, 2026) //change to StreamID 4 from Jan27th, 2026 to Mar10th, 2026
                                                        // }
                                                        // else if(!memcmp(ct->Contract.Symbol,"SILVERM", 7)){
                                                        //         if(ct->Contract.InstrumentType == FUTCOM){ct->StreamID = 5;}
                                                        //         else{
                                                        //         ct->StreamID = 3;
                                                        //         }
                                                        // }
                                                        // else if(!memcmp(ct->Contract.Symbol,"SILVER", 6)){
                                                        //         if(ct->Contract.InstrumentType == FUTCOM){ct->StreamID = 5;}
                                                        //         else{
                                                        //         ct->StreamID = 2;
                                                        //         }
                                                        // }
                                                        // else if(!memcmp(ct->Contract.Symbol,"CRUDE", 5)){
                                                        //         ct->StreamID = 4;
                                                        // }
                                                        // else if(!memcmp(ct->Contract.Symbol,"NATURALGAS", 10) || !memcmp(ct->Contract.Symbol,"NATGAS", 6)){
                                                        //         ct->StreamID = 5;
                                                        // }
                                                        // else{
                                                        //         if(ct->Contract.InstrumentType == FUTIDX){
                                                        //                 ct->StreamID = 1;
                                                        //         }
                                                        //         else if(ct->Contract.InstrumentType == OPTIDX){
                                                        //                 ct->StreamID = 5;
                                                        //         }
                                                        //         else{
                                                        //                 ct->StreamID = 2;
                                                        //                 if(atoi(parts[2].c_str()) == 4){
                                                        //                         ct->StreamID = 5;
                                                        //                 }
                                                        //         }
                                                        // }

			                // From 30thMarch, 2026 to 24thApril, 2026
							// if(atoi(parts[2].c_str()) == 1){
							//  ct->StreamID = 1;
							//  }else if(atoi(parts[2].c_str()) == 2){
							//  ct->StreamID = 3;
							//  }else if(atoi(parts[2].c_str()) == 3){
							//  ct->StreamID = 4;
							//  }else if(atoi(parts[2].c_str()) == 4){
							//  ct->StreamID = 5;
							//  }else if(atoi(parts[2].c_str()) == 5){
							//  ct->StreamID = 2;
							//  }else{
							//  	ct->StreamID = atoi(parts[2].c_str());
							//  }//Stream Id From Bcp FIle

							//From 25thApril, 2026 onwards
							ct->StreamID = atoi(parts[2].c_str());

							#ifdef DEMO_MODEdoes not exists...does not exists...
							//printf("Token:%d (MarginType :%s)=> Buy Initial Margin :%s, Sell Initial Margin :%s\n", key, parts[68].c_str(), parts[69].c_str(), parts[94].c_str());
							ct->Contract.Margin.InitialMargin = 4;
							ct->Contract.Margin.AdditionalMargin = 0;
							ct->Contract.Margin.RegulatoryMargin = 0;
							ct->Contract.Margin.LongMargin = 0;
							ct->Contract.Margin.ShortMargin = 0;
							#endif

							set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tIndex;
							instrument_umap[(Exchange_Type)x][key] = ct;
							StreamIds[(Exchange_Type)x].insert(ct->StreamID);
							ProductID_MCX.insert(atoi(parts[11].c_str()));
							StreamIDsProduct_MCX[ct->StreamID].insert(atoi(parts[11].c_str()));
							TokenProductID_MCX[key] = atoi(parts[11].c_str());
							ProductID_Streams_MCX[atoi(parts[11].c_str())] = ct->StreamID;
							if(Max_Product_ID_MCX < (uint32_t)atoi(parts[11].c_str())){
								Max_Product_ID_MCX = atoi(parts[11].c_str());
							}
							if(Max_Partition_ID_MCX < (uint32_t)atoi(parts[2].c_str())){
								Max_Partition_ID_MCX = atoi(parts[2].c_str());
							}

							if(printlog == 1){
								std::cout<< key << " => StreamID:" << ct->StreamID <<", Partition:"<< parts[2] << ", " <<  parts[6] << ", Instrument:" << parts[53] << "::" << Instrument_Type_To_String(ct->Contract.InstrumentType) << ", ProductID:" << atoi(parts[11].c_str())
										<<  " , LotSize:" << parts[20] <<  " , Ticksize:" << parts[21] <<  " , HDPR:" << ct->Contract.HighPriceRange
								<<  " , LDPR:" << ct->Contract.LowPriceRange <<  " , Multiplier:" << ct->Contract.Multiplier  <<  " , StrikePrice:" << ct->Contract.StrikePrice << " , InstrumentType:" << atoi(parts[8].c_str()) << " , OptionType:" << Option_Type_To_String(ct->Contract.OptionType)
								<< " , GroupID:" << atoi(parts[106].c_str()) << " , Expiry:" << ct->Contract.ExpiryDate << endl;
							}
						}
					}
				}else{
					cout<< endl << "MCX Contract Not Found..." << endl << endl;
				}
#endif
			}break;

			case EXCHG_BSE_CD: {
#if defined(EXCHG_BSE_CD_FEEDER) || defined(EXCHG_BSE_CD_OMS)
				int32_t ix; char Name[256]; set<string> workingscripts;
				char filename[512]; strcpy(filename, cpath[(Exchange_Type)x].c_str());
				if(!strstr(filename,".csv")){
					if(strstr(filename,"BFX_CO")){
						time_t t = time(NULL); struct tm tm = *localtime(&t);
						sprintf((filename + strlen(filename)),"%d.csv",(((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + tm.tm_mday));
						if(access(filename, F_OK ) != 0){sprintf((strstr(filename, "BFX_CO") +  strlen("BFX_CO")),"%d.csv",(((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + (tm.tm_mday-1)));}
					}
					else {
						time_t t = time(NULL); struct tm tm = *localtime(&t);
						sprintf((filename + strlen(filename)),"BFX_CO%d.csv",  (((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + tm.tm_mday));
						if(access(filename, F_OK ) != 0){sprintf((strstr(filename, "BFX_CO") +  strlen("BFX_CO")),"%d.csv",(((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + (tm.tm_mday-1)));}
					}
				}
				std::ifstream in_stream(filename,std::ifstream::in);
				if (in_stream.is_open())
				{
					 int32_t tIndex = 0; std::string startline; getline(in_stream,startline);
					for(std::string line;std::getline(in_stream,line);)
					{
						std::vector<std::string> parts; parts = split(line, ","); int64_t key = atol(parts[0].c_str());
						if (key == 0)continue;
						ix = 0; strcpy(Name,parts[3].c_str());while(Name[ix]){if(Name[ix] == ' '){Name[ix]='\0';break;}else ix++;}
						//if (strncmp(parts[0].c_str(),"FUT",3)==0)
						{
							ct = new CONTRACT_DETAILS;
							memset(&ct->Contract, 0, sizeof(GenericContract));

							ct->PTR = NULL;
							ct->StreamID = atoi(parts[12].c_str());
							ct->StartSequence = 1;
							ct->Contract.Exchange = EXCHG_BSE_CD;
							ct->Contract.Index = tIndex++;
							ct->Contract.Token = key;
							if(parts[7].size() == 0)ct->Contract.StrikePrice = -1; else ct->Contract.StrikePrice = atoi(parts[7].c_str());
							for(uint32_t p=0; p< parts.size(); p++){
								if(parts[p].size() == 0){
									parts[p] = "0";
								}
							}
							ct->Contract.TickSize = (((double)atoi(parts[32].c_str())/(double)10000000) * 10000);
							ct->Contract.LotSize = atoi(parts[30].c_str()) ;
							ct->Contract.HighPriceRange = 0x7FFFFFFF;
							ct->Contract.LowPriceRange = 5;
							strcpy(ct->Contract.Symbol ,Name);
							parts[53].erase(remove(parts[53].begin(), parts[53].end(), ' '), parts[53].end()); strcpy(ct->Contract.SymbolCode ,parts[53].c_str());
							ct->Contract.BasePrice = (PriceType)atoi(parts[67].c_str());
							//roun (Rate * (Price numerator / Price Denominator) * Quantity * Tradable lot *	(General numerator / General Denominator) * (Lot numerator/ Lot denominator), 2);
//							if(atoi(parts[47].c_str()) && atoi(parts[49].c_str()) && atoi(parts[51].c_str()))
//								ct->Contract.Multiplier =  (PriceType)((ct->Contract.TickSize * ( atoi(parts[46].c_str()) /  atoi(parts[47].c_str())) * atoi(parts[18].c_str()) * ( atoi(parts[48].c_str()) /  atoi(parts[49].c_str())) * ( atoi(parts[50].c_str()) / atoi(parts[51].c_str()))) * 100);
//							else ct->Contract.Multiplier = 0;
							ct->Contract.Multiplier =  atoi(parts[51].c_str());
#if __PRICE_SIZE == 8
							ct->Contract.PriceExponent = 8; ct->Contract.PriceExponentDisplay = 4;
#else
							ct->Contract.PriceExponent = 4; ct->Contract.PriceExponentDisplay = 4;
#endif
							/*if(!strcmp(parts[0].c_str(),"COM"))ct->Contract.InstrumentType = Instrument_Type::COM;
							else if(!strcmp(parts[0].c_str(),"FX"))ct->Contract.InstrumentType = Instrument_Type::FX;
							else if(!strcmp(parts[0].c_str(),"INDEX"))ct->Contract.InstrumentType = Instrument_Type::INDEX;*/
							ct->Contract.InstrumentType = String_To_Instrument_Type(parts[2].c_str());
							if(ct->Contract.InstrumentType == Instrument_Type::INST_ERROR){
								cout << "\nInstrumentType ::" << parts[2].c_str() << " Not Defined...\n" << endl; ABORT;
							}
							ct->Contract.OptionType = String_To_Option_Type(parts[8].c_str());
							if(atoi(parts[6].c_str()) > 0){
								struct tm tmTime; memset(&tmTime,0,sizeof(struct tm)); strptime(parts[6].c_str(),"%d-%b-%Y", &tmTime);
								ct->Contract.ExpiryDate = mktime(&tmTime);
							}else ct->Contract.ExpiryDate = 0;

							if (atoi(parts[28].c_str()) > 1000 /***** PRODUCT_ID *****/){
								std::cout << "[ SKIPPING ] @ "<< key <<" => " <<  Name << " , Expiry:" << parts[6].c_str() << " ("<< ct->Contract.ExpiryDate << ") , InstrumentType:" << parts[2].c_str() <<  " , LotSize: " << ct->Contract.LotSize <<  " , Ticksize:" << ct->Contract.TickSize << " , Multiplier:" << ct->Contract.Multiplier  <<  " , OptionType:" << parts[8].c_str()
												  <<  " , BasePrice:" << ct->Contract.BasePrice <<  " , ProductID:" << atoi(parts[28].c_str())   <<  " , PartitionID:" << atoi(parts[12].c_str())  <<  " , SpreadInsID:" << atol(parts[15].c_str())  << endl;
								delete ct; continue;
							}

							if(instrument_umap[(Exchange_Type)x].find(key) == instrument_umap[(Exchange_Type)x].end()){
								set_min_max_token((Exchange_Type)x, key); max_index[x] = tIndex;
								instrument_umap[(Exchange_Type)x][key] = ct;
								ProductID_BSE_CD[key] = atoi(parts[28].c_str());
								if(Max_Product_ID_BSE_CD < (uint32_t)atoi(parts[28].c_str())){Max_Product_ID_BSE_CD = atoi(parts[28].c_str());	}
							}
							else{
								printf("\n\nSame Unique ID\n\n"); ABORT;
							}
						}
					}

				}else{
					//cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< filename << " Not Found..." << endl << endl; fflush(stdout);
				}
#endif
			}break;

			case EXCHG_INX: {
#if defined(EXCHG_INX_FEEDER) || defined(EXCHG_INX_OMS)
				int32_t ix; char Name[256]; set<string> workingscripts;

				char filename[512]; strcpy(filename, cpath[(Exchange_Type)x].c_str());
				if(!strstr(filename,".csv")){
					if(strstr(filename,"INX_CO_")){
						time_t t = time(NULL); struct tm tm = *localtime(&t);
						sprintf((filename + strlen(filename)),"%d.csv",(((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + tm.tm_mday));
						if(access(filename, F_OK ) != 0){sprintf((strstr(filename, "INX_CO_") +  strlen("INX_CO_")),"%d.csv",(((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + (tm.tm_mday-1)));}
					}
					else {
						time_t t = time(NULL); struct tm tm = *localtime(&t);
						sprintf((filename + strlen(filename)),"INX_CO_%d.csv",  (((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + tm.tm_mday));
						if(access(filename, F_OK ) != 0){sprintf((strstr(filename, "INX_CO_") +  strlen("INX_CO_")),"%d.csv",(((((tm.tm_year+1900)*100) +(tm.tm_mon+1)) * 100) + (tm.tm_mday-1)));}
					}
				}
				std::ifstream in_stream(filename,std::ifstream::in);
				if (in_stream.is_open())
				{
					 int32_t tIndex = 0;
					for(std::string line;std::getline(in_stream,line);)
					{
						std::vector<std::string> parts; parts = split(line, ","); int32_t key = atoi(parts[2].c_str());
						if (key == 0)continue;
						ix = 0; strcpy(Name,parts[4].c_str());while(Name[ix]){if(Name[ix] == ' '){Name[ix]='\0';break;}else ix++;}

						if(!strcmp(parts[0].c_str(),"FUTCOM") || !strcmp(parts[0].c_str(),"FUTIDX"))
						{
							ct = new CONTRACT_DETAILS;
							memset(&ct->Contract, 0, sizeof(GenericContract));

							ct->PTR = NULL;
							ct->StreamID = atoi(parts[17].c_str());
							ct->StartSequence = 1;
							ct->Contract.Exchange = EXCHG_INX;
							ct->Contract.Index = tIndex++;
							ct->Contract.Token = key;
							if(parts[8].size() == 0)ct->Contract.StrikePrice = -1; else ct->Contract.StrikePrice = (atof(parts[8].c_str())*10000);
							for(uint32_t p=0; p< parts.size(); p++){
								if(parts[p].size() == 0){
									parts[p] = "0";
								}
							}
							ct->Contract.TickSize = (atof(parts[19].c_str())*10000); //printf("%s, %s\n", parts[8].c_str(), parts[19].c_str());
							ct->Contract.LotSize = (atoi(parts[18].c_str()) * atoi(parts[12].c_str()));
							ct->Contract.HighPriceRange = 0x7FFFFFFF;
							ct->Contract.LowPriceRange = 5;
							strcpy(ct->Contract.Symbol ,Name);
							parts[45].erase(remove(parts[45].begin(), parts[45].end(), ' '), parts[45].end()); strcpy(ct->Contract.SymbolCode ,parts[45].c_str());
							ct->Contract.BasePrice = (PriceType)(atof(parts[10].c_str())*10000);
							//roun (Rate * (Price numerator / Price Denominator) * Quantity * Tradable lot *	(General numerator / General Denominator) * (Lot numerator/ Lot denominator), 2);
//							if(atoi(parts[47].c_str()) && atoi(parts[49].c_str()) && atoi(parts[51].c_str()))
//								ct->Contract.Multiplier =  (PriceType)((ct->Contract.TickSize * ( atoi(parts[46].c_str()) /  atoi(parts[47].c_str())) * atoi(parts[18].c_str()) * ( atoi(parts[48].c_str()) /  atoi(parts[49].c_str())) * ( atoi(parts[50].c_str()) / atoi(parts[51].c_str()))) * 10000);
//							else ct->Contract.Multiplier = 0;
							ct->Contract.Multiplier = 0;
#if __PRICE_SIZE == 8
							ct->Contract.PriceExponent = 8; ct->Contract.PriceExponentDisplay = 4;
#else
							ct->Contract.PriceExponent = 4; ct->Contract.PriceExponentDisplay = 4;
#endif
							//if(!strcmp(parts[0].c_str(),"COM") || !strcmp(parts[0].c_str(),"FX") || !strcmp(parts[0].c_str(),"INDEX"))continue;
							ct->Contract.InstrumentType = String_To_Instrument_Type(parts[0].c_str());
							if(ct->Contract.InstrumentType == Instrument_Type::INST_ERROR){
								cout << "\nInstrumentType ::" << parts[0].c_str() << " Not Defined...\n" << endl; ABORT;
							}
							ct->Contract.OptionType = String_To_Option_Type(parts[7].c_str());
							if(atoi(parts[9].c_str()) > 0){
								struct tm tmTime; memset(&tmTime,0,sizeof(struct tm)); strptime(parts[9].c_str(),"%d%b%Y", &tmTime);
								ct->Contract.ExpiryDate = mktime(&tmTime);
							}else ct->Contract.ExpiryDate = 0;

							#ifdef DEMO_MODE
							ct->Contract.PriceExponent = 4;
							ct->Contract.PriceExponentDisplay = 4;
							ct->Contract.Multiplier = 320000;
							ct->Contract.Margin.InitialMargin = 4;
							ct->Contract.Margin.AdditionalMargin = 0;
							ct->Contract.Margin.RegulatoryMargin = 0;
							ct->Contract.Margin.LongMargin = 0;
							ct->Contract.Margin.ShortMargin = 0;
							#endif

							if (atoi(parts[28].c_str()) > 1000 /***** PRODUCT_ID *****/){
								std::cout << "[ SKIPPING ] @ " << key <<" => " <<  Name << " , Expiry:" << parts[9].c_str() << " ("<< ct->Contract.ExpiryDate << ") , InstrumentType:" << parts[0].c_str() <<  " , LotSize: " << ct->Contract.LotSize <<  " , Ticksize:" << ct->Contract.TickSize << " , Multiplier:" << ct->Contract.Multiplier  <<  " , OptionType:" << parts[7].c_str()
													 <<  " , Multiplier:" << ct->Contract.Multiplier   <<  " , BasePrice:" << ct->Contract.BasePrice <<  " , ProductID:" << atoi(parts[16].c_str())   <<  " , PartitionID:" << atoi(parts[17].c_str())  <<  " , SpreadInsID:" << atol(parts[15].c_str())  << endl;
								delete ct; continue;
							}

							if(instrument_umap[(Exchange_Type)x].find(key) == instrument_umap[(Exchange_Type)x].end()){
								set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tIndex;
								instrument_umap[(Exchange_Type)x][key] = ct;
								ProductID_INX[key] = atoi(parts[16].c_str());
								if(Max_Product_ID_INX < (uint32_t)atoi(parts[16].c_str()))Max_Product_ID_INX = atoi(parts[16].c_str());
							}
							else{
								printf("\n\nSame Unique ID\n\n"); ABORT;
							}
						}
					}
				}else{
					//cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< filename << " Not Found..." << endl << endl; fflush(stdout);
				}
#endif
			}break;

			case EXCHG_CME:
			case EXCHG_LME:{
#if defined(EXCHG_CME_FEEDER) || defined(EXCHG_CME_OMS) || defined(EXCHG_LME_FEEDER) || defined(EXCHG_LME_OMS)
					//std::ifstream in_stream((cpath[(Exchange_Type)x] + ( x==  EXCHG_CME ? "CME_Contract": "LME_Contract")).c_str(),std::ifstream::in);
					std::ifstream in_stream("/home/rishav/workspace/CME_latest/backtesting/CME_Contract", std::ifstream::in);
					if (in_stream.is_open())
					{
						 int32_t tIndex = 0;
						for(std::string line;std::getline(in_stream,line);)
						{
							std::vector<std::string> parts; parts = split(line, ","); if(line.size() ==0)continue;
							parts[0].erase(remove(parts[0].begin(), parts[0].end(), ' '), parts[0].end());
							parts[1].erase(remove(parts[1].begin(), parts[1].end(), ' '), parts[1].end());
							if(parts.size()>2){parts[2].erase(remove(parts[2].begin(), parts[2].end(), ' '), parts[2].end());}
							if(parts.size()>3){parts[3].erase(remove(parts[3].begin(), parts[3].end(), ' '), parts[3].end());}
							if(parts.size()>4){parts[4].erase(remove(parts[4].begin(), parts[4].end(), ' '), parts[4].end());}

							ct = new CONTRACT_DETAILS;
							memset(&ct->Contract, 0, sizeof(GenericContract));

							ct->PTR = NULL;
							if(parts.size()>2)ct->StartSequence = atoi(parts[2].c_str());else ct->StartSequence = 1;
							if(parts.size()>3)ct->StreamID = atoi(parts[3].c_str());else ct->StreamID = 0;
							ct->Contract.Exchange = (Exchange_Type)x;
							ct->Contract.Index = tIndex++;
							ct->Contract.Token = tIndex;
							if((Exchange_Type)x == EXCHG_LME || strstr(parts[0].c_str(), "LME") || strstr(parts[0].c_str(), "R.")){
								ct->Contract.ExpiryDate = Get_LME_ExpiryDate(4,2018);
							}
							else if((Exchange_Type)x == EXCHG_CME){
								ct->Contract.ExpiryDate = Get_CME_ExpiryDate( *(parts[0].c_str()+strlen(parts[0].c_str()) - 2) , ( *(parts[0].c_str()+strlen(parts[0].c_str()) - 1)-48) );
							}
							ct->Contract.StrikePrice = 0;
							ct->Contract.BasePrice = 0;
							ct->Contract.TickSize = 100;
							ct->Contract.LotSize = 0;
							ct->Contract.Multiplier =  0;
							ct->Contract.HighPriceRange = 0x7FFFFFFF;
							ct->Contract.LowPriceRange = 5;
							strcpy(ct->Contract.SymbolCode ,parts[0].c_str());
							char *sym = (strstr(ct->Contract.SymbolCode,".")+1); while(strstr(sym,"."))sym =(strstr(sym,".")+1);
							strcpy(ct->Contract.Symbol , sym);
							ct->Contract.PriceExponent = atoi(parts[1].c_str());
							ct->Contract.PriceExponentDisplay = ct->Contract.PriceExponent;
							if(parts.size() > 4){
								if(atoi(parts[4].c_str()) > 0 && atoi(parts[4].c_str()) < 9){
									ct->Contract.PriceExponentDisplay = atoi(parts[4].c_str());
								}
								else{
									ct->Contract.PriceExponentDisplay = atoi(parts[1].c_str());
								}
							}
							ct->Contract.InstrumentType = Instrument_Type::FUTCOM;
							ct->Contract.OptionType = Option_Type::OPTION_NONE;

							set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tIndex;
							if(instrument_umap[(Exchange_Type)x].find(tIndex) == instrument_umap[(Exchange_Type)x].end()){
								instrument_umap[(Exchange_Type)x][tIndex] = ct;
								if(printlog){
									cout << "CME => Added :" << "Code:"  <<ct->Contract.SymbolCode << " , Symbol:" << ct->Contract.Symbol << " , Token:" << tIndex  << " , Expiry:" <<  ct->Contract.ExpiryDate  << ", Instrument:" << Instrument_Type_To_String(ct->Contract.InstrumentType) << "\n";
								}
							}
							else{
								printf("\n\nSame Unique ID\n\n"); ABORT;
							}
						}
//						// Update Tokens
//						{
//							QED_initializeLogs();
//							QED_ctx ctx = QED_initWithPath(cfg.getValueOfKey<std::string>("EXCHG_CME_CONFIG_SECTION","-",true).c_str(), cfg.getValueOfKey<std::string>("EXCHG_CME_CONFIG_DIR_PATH","-",true).c_str());
//							if (ctx == NULL) {
//								printf("\nQED_init error:%s \n\n", QED_error_string(QED_errno())); ABORT;
//							}
//							char *sdata; unsigned long int slen=0; QED_getSymbolListPacket(ctx, (const char**)&sdata, &slen);
//							//const char** symbols;  uint16_t  count = QED_getSymbolList(ctx, &symbols);	for(int32_t i = 0; i < count; i++){ printf("Registered Symbol %s, SymbolChannelID:%d\n", symbols[i], QED_getSymbolChannelID(ctx,symbols[i]));}for(int32_t i = 0; i <slen; i++){printf("%02x ", sdata[i]);}printf("\n---------------------- %d --------------------\n\n", slen);
//							slen -=8; uint32_t cnt = 8; uint16_t Index = 0; uint8_t ChannelID = 0; char Symbol[100];
//							while(cnt < slen){
//								Index = *(uint16_t*)(sdata + cnt); cnt += 2;
//								ChannelID = *(uint8_t*)(sdata + cnt); cnt += 1;
//								cnt += 2;
//								strcpy(Symbol, (sdata + cnt)); cnt += (strlen(Symbol) + 1);
//								std::tr1::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator contract_itr;
//								for( contract_itr= contract_file->instrument_umap[EXCHG_CME].begin(); contract_itr != contract_file->instrument_umap[EXCHG_CME].end(); contract_itr++ ){
//									if(!strcmp(contract_itr->second->Contract.SymbolCode,Symbol)){
//										contract_itr->second->Contract.Token = Index;
//										contract_itr->second->Contract.Multiplier = ChannelID;
//										printf("Setting => Index:%d, ChannelID:%d, Symbol:%s\n", Index, ChannelID, Symbol);
//									}
//								}
//								//printf("Index:%d, ChannelID:%d, Symbol:%s\n", Index, ChannelID, Symbol);
//							}
//							QED_close(ctx);
//						}
					}else{
						//cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< cpath[(Exchange_Type)x].c_str() << " Not Found..." << endl << endl;
					}
#endif
			}break;

			case EXCHG_NSE_FO: {
#if defined(EXCHG_NSE_FO_FEEDER) || defined(EXCHG_NSE_FO_OMS)
				std::string file = cpath[(Exchange_Type)x]; file.append("fo_contract_stream_info.csv");
				string line; ifstream ContractFile (file.c_str());
				if (ContractFile.is_open())
				{
					int32_t Date, TotalTokens, tindex = 0, StreamID, Token, ExpiryDate, StrikePrice;
					char __SYMBOL__[20],_InstrumentName_[10],_OptionType_[3]; char MsgType;
					getline (ContractFile,line); sscanf(line.c_str(), "%d,%d",&Date, &TotalTokens);

					while ( getline (ContractFile,line) )
					{
						sscanf(line.c_str(), "%c,%d,%u,%[^,],%[^,],%u,%u,%[^,]",&MsgType, &StreamID, &Token, _InstrumentName_, __SYMBOL__, &ExpiryDate, &StrikePrice, _OptionType_); _OptionType_[2] = '\0';

						if(MsgType=='C'){
							ct = new CONTRACT_DETAILS;
							memset(&ct->Contract, 0, sizeof(GenericContract));
							pthread_spin_init(&ct->ContractLock, PTHREAD_PROCESS_PRIVATE);
							ct->PTR = NULL;
							ct->StartSequence = 1;
							ct->StreamID = StreamID;
							ct->Contract.Index = tindex++;
							ct->Contract.Token = Token;
							ct->Contract.StrikePrice = StrikePrice;
							ct->Contract.ExpiryDate = ExpiryDate + (315532800 - 19800);
							ct->Contract.Exchange = EXCHG_NSE_FO;
							strcpy(ct->Contract.Symbol ,__SYMBOL__); strcpy(ct->Contract.SymbolCode ,"-");
							ct->Contract.InstrumentType = String_To_Instrument_Type(_InstrumentName_);
							if(ct->Contract.InstrumentType == Instrument_Type::INST_ERROR){
								cout << "\nInstrumentType ::" << _InstrumentName_ << " Not Defined...\n" << endl; ABORT;
							}
							ct->Contract.OptionType = String_To_Option_Type(_OptionType_);
//							if(ct->Contract.OptionType == Option_Type::OPTION_NONE){
//								printf("\n\nToken:%d Option_Type :: %s Not Defined..\n\n", Token, _OptionType_); ABORT;
//							}
							ct->Contract.PriceExponent = 2; ct->Contract.PriceExponentDisplay = 2;
							ct->Contract.BasePrice = 0;
							ct->Contract.LotSize = 0;
							ct->Contract.Multiplier =  100;

							set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tindex;
							instrument_umap[(Exchange_Type)x][ct->Contract.Token]  = ct;
							StreamIds[(Exchange_Type)x].insert(StreamID);
							if(printlog == 2){
								printf("MsgType:%c, StreamID:%d, Token:%u, Instrument:%s, Symbol:%s, Expiry:%u, Strike:%u, Option:%s\n\n",MsgType, StreamID, Token, _InstrumentName_, __SYMBOL__, ExpiryDate, StrikePrice, _OptionType_);
							}
						}
					}
					ContractFile.close();

					char neatfo[20],neatfo_ver[20];
					std::string file = cpath[(Exchange_Type)x]; file.append("contract.txt");
					ifstream ContractTxt (file.c_str());
					if (ContractTxt.is_open())
					{
						vector<string> tokens;getline (ContractTxt,line); sscanf(line.c_str(), "%[^|]|%[^|]",neatfo, neatfo_ver);
						//cout << "\nName : "<< neatfo << " , Version : " << neatfo_ver << endl << endl;
						int32_t high,low;
						while ( getline (ContractTxt,line) )
						{
						   tokens = split(line, "|"); // copy(tokens.begin(), tokens.end(), ostream_iterator<string>(cout, "\n"));
						   if((tokens[2].size() == 0 && atoi(tokens[3].c_str()) == 0)){
	                            std::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator itr = instrument_umap[(Exchange_Type)x].find(atoi(tokens[0].c_str()));
	                            if(itr != instrument_umap[(Exchange_Type)x].end()){
	                                //itr->second->Deleted = true;
	                            }
							   //instrument_umap[(Exchange_Type)x].erase(atoi(tokens[0].c_str()));
							   continue;
						   }

						   //if(memcmp(tokens[2].c_str(),"FUTCUR",6))continue; // <<---- Skip
							std::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator itr = instrument_umap[(Exchange_Type)x].find(atoi(tokens[0].c_str()));
							if(itr != instrument_umap[(Exchange_Type)x].end()){
								//itr->second->Contract.StrikePrice = atoi(tokens[7].c_str());
								itr->second->Contract.LotSize = atoi(tokens[31].c_str()); //37
								itr->second->Contract.Multiplier =  itr->second->Contract.LotSize * 100; // fixing GUI netposition calculation
								itr->second->Contract.TickSize = atoi(tokens[32].c_str());
								high = atoi(tokens[43].c_str());  low = atoi(tokens[42].c_str()); itr->second->Contract.HighPriceRange = high; if(low==0)low=high/4; itr->second->Contract.LowPriceRange = low;
								if(itr->second->Contract.LowPriceRange < itr->second->Contract.TickSize)itr->second->Contract.LowPriceRange = 0;else if((itr->second->Contract.LowPriceRange % itr->second->Contract.TickSize))itr->second->Contract.LowPriceRange -= (itr->second->Contract.LowPriceRange % itr->second->Contract.TickSize);
								if(printlog == 1){
								   printf("Instrument:%s, TokenNum:%d, Symbol:%s, ExpiryDate:%d, StrikePrice:%d, BoardLotQuantity:%d, TickSize:%d DPR_HIGH:%d DPR_LOW:%d\n",tokens[2].c_str(), itr->second->Contract.Token, itr->second->Contract.Symbol,
										   itr->second->Contract.ExpiryDate, itr->second->Contract.StrikePrice, itr->second->Contract.LotSize, itr->second->Contract.TickSize, itr->second->Contract.HighPriceRange, itr->second->Contract.LowPriceRange);
								}
							}
						}
						ContractTxt.close();
					}else{
						cout<< endl << "EXCHG_NSE_FO_CONTRACT_TXT_PATH Not Found..." << endl << endl; ABORT;
					}
				}else{
					cout<< endl << "fo_contract_stream_info Not Found..." << endl << endl;
				}
#endif
			}break;

			case EXCHG_NSE_CM:{
#if defined(EXCHG_NSE_CM_FEEDER) || defined(EXCHG_NSE_CM_OMS)
				std::string file = cpath[(Exchange_Type)x]; file.append("cm_contract_stream_info.csv");
				string line; ifstream ContractFile (file.c_str());
				if (ContractFile.is_open())
                                {
					int32_t Date, TotalTokens, tindex = 0, StreamID, Token, ExpiryDate, StrikePrice;
                                        char __SYMBOL__[20],_InstrumentName_[10],_OptionType_[3]; char MsgType;
                                        getline (ContractFile,line); sscanf(line.c_str(), "%d,%d",&Date, &TotalTokens);

					while ( getline (ContractFile,line) )
                                        {
                                                sscanf(line.c_str(), "%c,%d,%u,%[^,],%[^,],%u,%u,%[^,]",&MsgType, &StreamID, &Token, _InstrumentName_, __SYMBOL__, &ExpiryDate, &StrikePrice, _OptionType_); _OptionType_[2] = '\0';

                                                if(MsgType=='C' && strstr(_OptionType_,"EQ")){
                                                        ct = new CONTRACT_DETAILS;
                                                        memset(&ct->Contract, 0, sizeof(GenericContract));
                                                        pthread_spin_init(&ct->ContractLock, PTHREAD_PROCESS_PRIVATE);
                                                        ct->PTR = NULL;
                                                        ct->StartSequence = 1;
                                                        ct->StreamID = StreamID;
                                                        ct->Contract.Index = tindex++;
                                                        ct->Contract.Token = Token;
                                                        ct->Contract.StrikePrice = StrikePrice;
                                                        ct->Contract.Exchange = EXCHG_NSE_CM;
                                                        strcpy(ct->Contract.Symbol ,__SYMBOL__); strcpy(ct->Contract.SymbolCode ,"-");
                                                        ct->Contract.InstrumentType = String_To_Instrument_Type(_InstrumentName_);
                                                        if(ct->Contract.InstrumentType == Instrument_Type::INST_ERROR){
                                                                cout << "\nInstrumentType ::" << _InstrumentName_ << " Not Defined...\n" << endl; ABORT;
                                                        }
                                                        ct->Contract.OptionType = String_To_Option_Type(_OptionType_);
//                                                      if(ct->Contract.OptionType == Option_Type::OPTION_NONE){
//                                                              printf("\n\nToken:%d Option_Type :: %s Not Defined..\n\n", Token, _OptionType_); ABORT;
//                                                      }
                                                        ct->Contract.PriceExponent = 2; ct->Contract.PriceExponentDisplay = 2;
                                                        ct->Contract.BasePrice = 0;
                                                        ct->Contract.LotSize = 1;
							ct->Contract.TickSize = 5;
                                                        ct->Contract.Multiplier =  100;

                                                        set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tindex;
                                                        instrument_umap[(Exchange_Type)x][ct->Contract.Token]  = ct;
                                                        StreamIds[(Exchange_Type)x].insert(StreamID);
                                                        if(printlog == 2){
                                                                printf("MsgType:%c, StreamID:%d, Token:%u, Instrument:%s, Symbol:%s, Expiry:%u, Strike:%u, Option:%s\n\n",MsgType, StreamID, Token, _InstrumentName_, __SYMBOL__, ExpiryDate, StrikePrice, _OptionType_);
                                                        }
                                                }
                                        }
				
					ContractFile.close();
					char neatfo[20],neatfo_ver[20];
                                        std::string file = cpath[(Exchange_Type)x]; file.append("security.txt");
                                        ifstream ContractTxt (file.c_str());
                                        if (ContractTxt.is_open())
                                        {
                                                vector<string> tokens;getline (ContractTxt,line); sscanf(line.c_str(), "%[^|]|%[^|]",neatfo, neatfo_ver);
                                                int32_t high,low;
                                                while ( getline (ContractTxt,line) )
                                                {
                                                   tokens = split(line, "|");
                                                   if((tokens[1].size() == 0 && atoi(tokens[2].c_str()) == 0))continue;
                                                           if(strcmp(tokens[2].c_str(),"EQ"))continue;
                                                           std::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator itr = instrument_umap[(Exchange_Type)x].find(atoi(tokens[0].c_str()));
                                                           if(itr == instrument_umap[(Exchange_Type)x].end()){
                                                                continue;
                                                           }

                                                           if(itr != instrument_umap[(Exchange_Type)x].end()){
                                                                itr->second->Contract.LotSize = atoi(tokens[19].c_str());
                                                                itr->second->Contract.Multiplier =  itr->second->Contract.LotSize * 100;
                                                                itr->second->Contract.TickSize = atoi(tokens[20].c_str());
                                                                vector<string> dpr = split(tokens[6].c_str(), "-"); int32_t high,low;
                                                                high = atof(dpr[1].c_str()) * 100; low = atof(dpr[0].c_str()) * 100;
                                                                itr->second->Contract.HighPriceRange = high;
                                                                itr->second->Contract.LowPriceRange = low;
                                                                if(itr->second->Contract.LowPriceRange < itr->second->Contract.TickSize)itr->second->Contract.LowPriceRange = 0;else if((itr->second->Contract.LowPriceRange % itr->second->Contract.TickSize))itr->second->Contract.LowPriceRange -= (itr->second->Contract.LowPriceRange % itr->second->Contract.TickSize);
                                                                if(printlog == 1)
                                                                {
                                                                printf("Instrument:%s, TokenNum:%d, Symbol:%s, StrikePrice:%d, BoardLotQuantity:%d, TickSize:%d DPR_HIGH:%d DPR_LOW:%d\n",tokens[2].c_str(), itr->second->Contract.Token, itr->second->Contract.Symbol,
                                                                                   itr->second->Contract.StrikePrice, itr->second->Contract.LotSize, itr->second->Contract.TickSize, itr->second->Contract.HighPriceRange, itr->second->Contract.LowPriceRange);
                                                                }
                                                        }
                                                }
                                                ContractTxt.close();
                                        }else{
                                                cout<< endl << "EXCHG_NSE_CM_SECURITY_TXT_PATH Not Found..." << endl << endl; ABORT;
                                        }
				}
				else{
                                        cout<< endl << "EXCHG_NSE_CM_CONTRACT_TXT_PATH Not Found..." << endl << endl; //ABORT;
                                }
#endif
					  
			}break;

			case EXCHG_NSE_CD: {
#if defined(EXCHG_NSE_CD_FEEDER) || defined(EXCHG_NSE_CD_OMS)
				string line; ifstream ContractFile (cpath[(Exchange_Type)x].c_str());
				if (ContractFile.is_open())
				{
					char neatfo[20],neatfo_ver[20]; int32_t tindex = 0;
					vector<string> tokens;getline (ContractFile,line); sscanf(line.c_str(), "%[^|]|%[^|]",neatfo, neatfo_ver);
					cout<< "\nName : "<< neatfo << " , Version : " << neatfo_ver << endl << endl;

					while ( getline (ContractFile,line) )
					{
					   tokens = split(line, "|"); // copy(tokens.begin(), tokens.end(), ostream_iterator<string>(cout, "\n"));
					   if(tokens[2].size()==0 && atoi(tokens[3].c_str())==0)continue;
					   //if(memcmp(tokens[2].c_str(),"FUTCUR",6))continue; // <<---- Skip

						ct = new CONTRACT_DETAILS;
						memset(&ct->Contract, 0, sizeof(GenericContract));

						ct->PTR = NULL;
						ct->StreamID = 0;
						ct->StartSequence = 1;
						ct->Contract.Exchange = EXCHG_NSE_CD;
						ct->Contract.Index = tindex++;

						ct->Contract.Token = atoi(tokens[0].c_str());;
						if(atoi(tokens[6].c_str()) > 0){
							ct->Contract.ExpiryDate = atoi(tokens[6].c_str()) + (315532800 - 19800);
						}else ct->Contract.ExpiryDate = 0;
						ct->Contract.StrikePrice = atoi(tokens[7].c_str());

						ct->Contract.BasePrice = 0;
						ct->Contract.TickSize = atoi(tokens[32].c_str());
						ct->Contract.LotSize = 0;
						//high = atoi(tokens[43].c_str());  low = atoi(tokens[42].c_str());
						ct->Contract.HighPriceRange = 0x7FFFFFFF;
						ct->Contract.LowPriceRange = 5;

						ct->Contract.Multiplier =  0;
						ct->Contract.PriceExponent = 8; ct->Contract.PriceExponentDisplay = 4;
						tokens[3].erase(remove(tokens[3].begin(), tokens[3].end(), ' '), tokens[3].end());
						strcpy(ct->Contract.Symbol ,tokens[3].c_str()); strcpy(ct->Contract.SymbolCode ,"-");
						ct->Contract.InstrumentType = String_To_Instrument_Type(tokens[2].c_str());
						if(ct->Contract.InstrumentType == Instrument_Type::INST_ERROR){
							cout << "\nInstrumentType ::" << tokens[2].c_str() << " Not Defined...\n" << endl; ABORT;
						}
						ct->Contract.OptionType = String_To_Option_Type(tokens[8].c_str());

					    set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tindex;
					   instrument_umap[(Exchange_Type)x][ct->Contract.Token]  = ct;
					   if(printlog){
						   printf("Instrument:%s, TokenNum:%s, Symbol:%s, ExpiryDate:%s, StrikePrice:%s, MinimumLotQuantity:%s, BoardLotQuantity:%s, TickSize:%s DPR_HIGH:%s DPR_LOW:%s\n",tokens[2].c_str(),tokens[0].c_str(),tokens[3].c_str(),
								   tokens[6].c_str(),tokens[7].c_str(),tokens[30].c_str(), tokens[31].c_str(), tokens[32].c_str(), tokens[43].c_str(),tokens[42].c_str());
					   }
					}
					ContractFile.close();
				}else{
					//cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< cpath[(Exchange_Type)x].c_str() << " Not Found..." << endl << endl;
				}
#endif
			}break;

			case EXCHG_DGCX: {
#if defined(EXCHG_DGCX_FEEDER) || defined(EXCHG_DGCX_OMS)
				std::ifstream in_stream((cpath[(Exchange_Type)x] + "DGCX_Contract").c_str(),std::ifstream::in);
				if (in_stream.is_open())
				{
					 int32_t tIndex = 0;
					for(std::string line;std::getline(in_stream,line);)
					{
						std::vector<std::string> parts; parts = split(line, ","); if(line.size() ==0)continue;
						for(int iz=0; iz<5; iz++)parts[iz].erase(remove(parts[iz].begin(), parts[iz].end(), ' '), parts[iz].end());

						ct = new CONTRACT_DETAILS;
						memset(&ct->Contract, 0, sizeof(GenericContract));

						ct->PTR = NULL;
						ct->StreamID = 0;
						ct->StartSequence = 1;
						ct->Contract.Exchange = EXCHG_DGCX;
						ct->Contract.Index = tIndex++;
						ct->Contract.Token = atoi(parts[0].c_str());
						strcpy(ct->Contract.Symbol ,parts[1].c_str());
						{
							struct tm tmTime; memset(&tmTime,0,sizeof(struct tm)); strptime(parts[2].c_str(),"%Y%m%d 00:00:00", &tmTime);
							ct->Contract.ExpiryDate = mktime(&tmTime) + 19800;
						}
						ct->Contract.PriceExponent = atoi(parts[3].c_str());
						ct->Contract.PriceExponentDisplay = atoi(parts[4].c_str());
						strcpy(ct->Contract.SymbolCode ,"-");
						ct->Contract.InstrumentType = Instrument_Type::FUTCUR;
						ct->Contract.OptionType = Option_Type::OPTION_NONE;

					    set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tIndex;
						if(instrument_umap[(Exchange_Type)x].find(tIndex) == instrument_umap[(Exchange_Type)x].end()){
							instrument_umap[(Exchange_Type)x][ct->Contract.Token]  = ct;
							if(printlog){
								cout << "DGCX => Added : " << " Symbol:" << ct->Contract.Symbol << " , Token:" << ct->Contract.Token  << " , Expiry:" <<  ct->Contract.ExpiryDate  << ", Instrument:" << Instrument_Type_To_String(ct->Contract.InstrumentType) << "\n";
							}
						}
						else{
							printf("\n\nSame Unique ID\n\n"); ABORT;
						}
					}
				}else{
					//cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< cpath[(Exchange_Type)x].c_str() << " Not Found..." << endl << endl;
				}
#endif
			}break;

			case EXCHG_ICX: {
#if defined(EXCHG_ICX_FEEDER) || defined(EXCHG_ICX_OMS)
				char filename[512]; strcpy(filename, cfg.getValueOfKey<std::string>(cparamname,"-",true).c_str());
				if(!strstr(filename,".csv")){
					if(strstr(filename,"ContractMaster_")){
						time_t t = time(NULL); struct tm tm = *localtime(&t);
						sprintf((filename + strlen(filename)),"%d.csv", ((((tm.tm_mday *100) + (tm.tm_mon+1))*10000)+(tm.tm_year+1900)));
						if(access(filename, F_OK ) != 0){
							cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< filename << " Not Found => Searching Backdate Contract." << endl << endl;
							sprintf((strstr(filename, "ContractMaster_") +  strlen("ContractMaster_")),"%d.csv",  (((((tm.tm_mday-1) *100) + (tm.tm_mon+1))*10000)+(tm.tm_year+1900)));
						}
					}
					else {
						time_t t = time(NULL); struct tm tm = *localtime(&t);
						sprintf((filename + strlen(filename)),"ContractMaster_%d.csv",  ((((tm.tm_mday *100) + (tm.tm_mon+1))*10000)+(tm.tm_year+1900)));
						if(access(filename, F_OK ) != 0){
							cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< filename << " Not Found => Searching Backdate Contract." << endl << endl;
							sprintf((strstr(filename, "ContractMaster_") +  strlen("ContractMaster_")),"%d.csv",  (((((tm.tm_mday-1) *100) + (tm.tm_mon+1))*10000)+(tm.tm_year+1900)));
						}
					}
				}
				string line; ifstream ContractFile (filename);
				if (ContractFile.is_open())
				{
					int32_t tindex = 0; vector<string> tokens; getline (ContractFile,line);
					std::vector<std::string> scripts; scripts = split(cfg.getValueOfKey<std::string>(sparamname,"-",false), ",");
					std::vector<std::string> inst; inst = split(cfg.getValueOfKey<std::string>(iparamname,"-",false), ",");
					while ( getline (ContractFile,line) )
					{
					   tokens = split(line, ",");
					   tokens[1].erase(remove(tokens[1].begin(), tokens[1].end(), ' '), tokens[1].end());
					   if(tokens[1].size()==0 || tokens[2].c_str()==0)continue;
					   if(std::find(scripts.begin(), scripts.end(), "-") == scripts.end()){
						   bool found = false;
						   for(std::vector<std::string>::iterator itr = scripts.begin(); itr!=scripts.end(); itr++){
							   if(strstr(itr->c_str(), tokens[2].c_str())){
								   found = true;break;
							   }
						   }
						   if(!found)continue;
					   }
						if(std::find(inst.begin(), inst.end(), "-") == inst.end() && std::find(inst.begin(), inst.end(), tokens[0].c_str()) == inst.end()){
							continue;
						}

						ct = new CONTRACT_DETAILS;
						memset(&ct->Contract, 0, sizeof(GenericContract));

						ct->PTR = NULL;
						ct->StreamID = 1;
						ct->StartSequence = 1;
						ct->Contract.Exchange = EXCHG_ICX;
						ct->Contract.Index = tindex++;
						ct->Contract.Token = tindex;
						tokens[2].erase(remove(tokens[2].begin(), tokens[2].end(), ' '), tokens[2].end());
						tokens[3].erase(remove(tokens[3].begin(), tokens[3].end(), ' '), tokens[3].end());
//						if(tokens[3].size() > 0)
//							strcpy(ct->Contract.Symbol ,tokens[3].c_str());
//						else
							strcpy(ct->Contract.Symbol ,tokens[2].c_str());
						strcpy(ct->Contract.SymbolCode , tokens[2].c_str());
						if(atoi(tokens[0].c_str()) == 0){
							ct->Contract.InstrumentType = Instrument_Type::FUTCOM;
						}
						else if(atoi(tokens[0].c_str()) == 1){
							ct->Contract.InstrumentType = Instrument_Type::EQUITY;
						}
						else {cout << "\nInstrumentType ::" << tokens[2].c_str() << " Not Defined...\n\n" << endl;  delete ct; continue;;}

						if(atoi(tokens[9].c_str()) > 0){
							struct tm tmTime; memset(&tmTime,0,sizeof(struct tm)); strptime(tokens[9].c_str(),"%d%m%Y", &tmTime);
							ct->Contract.ExpiryDate = mktime(&tmTime);
						}else ct->Contract.ExpiryDate = 0;

						ct->Contract.OptionType = Option_Type::OPTION_NONE;
						ct->Contract.StrikePrice = 0;
						ct->Contract.BasePrice = (atof(tokens[6].c_str())*10000);
						ct->Contract.TickSize = (atof(tokens[8].c_str()) * 100);
						ct->Contract.Multiplier =  (atof(tokens[10].c_str()) * 100);
						ct->Contract.LotSize = (atof(tokens[12].c_str()) * 100);
						uint32_t marExpo = 1000; //100
						ct->Contract.Margin.InitialMargin = (atof(tokens[26].c_str()) * marExpo);
						ct->Contract.Margin.RegulatoryMargin = (atof(tokens[27].c_str()) * marExpo);
						ct->Contract.Margin.AdditionalMargin = (atof(tokens[31].c_str()) * marExpo);
						ct->Contract.Margin.LongMargin = (atof(tokens[37].c_str()) * marExpo);
						ct->Contract.Margin.ShortMargin = (atof(tokens[38].c_str()) * marExpo);
						ct->Contract.HighPriceRange = 0;
						ct->Contract.LowPriceRange = 0;
						ct->Contract.PriceExponent = 2;
						ct->Contract.PriceExponentDisplay = 2;

					    set_min_max_token((Exchange_Type)x,ct->Contract.Token); max_index[x] = tindex;
						if(instrument_umap[(Exchange_Type)x].find(tindex) == instrument_umap[(Exchange_Type)x].end()){
							instrument_umap[(Exchange_Type)x][ct->Contract.Token]  = ct;
						   if(printlog)
						   {
							   printf("Instrument:%s, TokenNum:%s(%d), Symbol:%s, ExpiryDate:%s : %d, TickSize:%s::%d, LotSize:%s, Multiplier:%s, Margin:%0.2f \n",tokens[0].c_str(),tokens[1].c_str(),tindex,
								   ct->Contract.Symbol, tokens[9].c_str(), ct->Contract.ExpiryDate,tokens[8].c_str(), ct->Contract.TickSize,tokens[9].c_str(), tokens[50].c_str(), atof(tokens[23].c_str()));
						   }
						}
					}
					ContractFile.close();
				}else{
					cout<< endl << Exchange_Type_To_String((Exchange_Type)x) << " :: "<< filename << " Not Found..." << endl << endl;
				}
#endif
			}break;

			default:
				cout << "No Contract Handler Installed For Exchange : " <<  Exchange_Type_To_String((Exchange_Type)x) << endl;

		}
	}
}

GenericContract* get_contract(Exchange_Type ex, int32_t tkn){
	CONTRACT_DETAILS *crt = contract_file->get_contract((int32_t)ex, tkn);
	if(crt){
		return &crt->Contract;
	}
	return NULL;
}
