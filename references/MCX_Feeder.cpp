
#include <MCX_Feeder.h>

//extern bool StartPrint;
using namespace MCX;

//#define RAW_FEED_NOTIF
//#define RAW_FEED_NOTIF_LOGS


MCX_Feeder::MCX_Feeder(std::map<std::pair<int, int>, GenericContract> *contract, std::string file):_Contract(contract), TotalNodes(0),BaseFileName(file){
	Reset_Token_Sequence_And_Orderbook(NULL, 0);
}

MCX_Feeder::~MCX_Feeder(){

}

int32_t MCX_Feeder::Load_New_DPR_Settings(TOKEN_ORDER_BOOK *TokenInfo, uint32_t &Tkn, int32_t &Price){

	int32_t DPR_Gap,Depth,AllocLen,sx,index,ShiftPosition;
	// Store All The Old Info
	int32_t OldMax=TokenInfo->BUY_SELL_DPR_RANGE_MAX_PRICE, OldMin=TokenInfo->BUY_SELL_DPR_RANGE_MIN_PRICE, OldThousandDepth=TokenInfo->ThousandSize;

	/*********************************************************/
	/********** Auto Align The DPR Boundry ************/
	/*********************************************************/
	// cout << "Load_New_DPR_Settings " << endl;
	int32_t NewMax = TokenInfo->BUY_SELL_DPR_RANGE_MAX_PRICE, NewMin = TokenInfo->BUY_SELL_DPR_RANGE_MIN_PRICE;

	if(Price > NewMax){
		NewMax = (Price + (Price/4)); NewMax += (10 - (NewMax % 10)); NewMax -= (NewMax%TokenInfo->TickSize);// increase by 25%

		DPR_Gap = NewMax - NewMin;	DPR_Gap/=TokenInfo->TickSize; Depth = (DPR_Gap/TokenInfo->PaisaDepthRange)+1; TokenInfo->ThousandSize = (Depth/1000)+1;
		TokenInfo->BUY_SELL_DPR_RANGE_MAX_PRICE =  NewMax;
		DPR_Gap = (DPR_Gap+(TokenInfo->PaisaDepthRange-(DPR_Gap%TokenInfo->PaisaDepthRange))); AllocLen = sizeof(ORDER_QTY_INFO)*DPR_Gap;
		TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO	= (ORDER_QTY_INFO*)realloc(TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO,AllocLen);
		TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO	= (ORDER_QTY_INFO*)realloc(TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO,AllocLen);
		// initialize the Virtual Memory
		// cout << __LINE__ << " DPR_GAP: " << DPR_Gap << endl;
		for(sx=((OldMax - OldMin)/TokenInfo->TickSize);sx<DPR_Gap;++sx){
			// cout << __LINE__ << " sx: " << sx << endl;
			TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0; TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0;
			TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0; TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0;
		}
		AllocLen = sizeof(ORDER_BOOK_THOUSAND_RANGE)*TokenInfo->ThousandSize;
		TokenInfo->BUY_INFO_TILL_RUPEE  = (ORDER_BOOK_THOUSAND_RANGE*)realloc(TokenInfo->BUY_INFO_TILL_RUPEE,AllocLen);
		TokenInfo->SELL_INFO_TILL_RUPEE = (ORDER_BOOK_THOUSAND_RANGE*)realloc(TokenInfo->SELL_INFO_TILL_RUPEE,AllocLen);
		// initialize the Virtual Memory
		for(Depth=OldThousandDepth;Depth<TokenInfo->ThousandSize;++Depth){
			TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Avaliable = 0;
			TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Avaliable = 0;
			for(sx=0;sx<10;++sx){
				TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].Avaliable=0;
				TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].Avaliable=0;
				for(index=0;index<10;++index){
					TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].Avaliable=0;
					TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].Avaliable=0;
					for(ShiftPosition=0;ShiftPosition<10;++ShiftPosition){
						TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].OneRupee[ShiftPosition].Avaliable = 0;
						TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].OneRupee[ShiftPosition].Avaliable = 0;
					}
				}
			}
		}
	}
	else if(Price < NewMin){
		ORDER_QTY_INFO *OLD_BUY_PAISA_QTY_INFO = TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO, *OLD_SELL_PAISA_QTY_INFO = TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO;
		ORDER_BOOK_THOUSAND_RANGE *OLD_BUY_INFO_TILL_RUPEE=TokenInfo->BUY_INFO_TILL_RUPEE, *OLD_SELL_INFO_TILL_RUPEE = TokenInfo->SELL_INFO_TILL_RUPEE;
		NewMin = (Price - (Price/4)); NewMin -= ((OldMin-NewMin)%100); NewMin -= (NewMin%TokenInfo->TickSize); if(NewMin < 0)NewMin = 0; // decrease by 25%
		DPR_Gap = NewMax - NewMin;	DPR_Gap/=TokenInfo->TickSize; Depth = (DPR_Gap/TokenInfo->PaisaDepthRange)+1; TokenInfo->ThousandSize = (Depth/1000)+1;
		TokenInfo->BUY_SELL_DPR_RANGE_MAX_PRICE =  NewMax; TokenInfo->BUY_SELL_DPR_RANGE_MIN_PRICE =  NewMin;
		DPR_Gap = (DPR_Gap+(TokenInfo->PaisaDepthRange-(DPR_Gap%TokenInfo->PaisaDepthRange))); AllocLen = sizeof(ORDER_QTY_INFO)*DPR_Gap;
		TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO	= (ORDER_QTY_INFO*)malloc(AllocLen);//= (int32_t*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
		TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO	= (ORDER_QTY_INFO*)malloc(AllocLen);//= (int32_t*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
		//sprintf(output,"\n==========================33 =====================> StreamID:%d Token:%d,  Old:%p | New:%p..\n\n",StreamID,Tkn,OLD_SELL_PAISA_QTY_INFO,TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO);fwrite(output, 1, strlen(output), feedLog);
		// initialize the Virtual Memory
		// cout << __LINE__ << " DPR_GAP: " << DPR_Gap << endl;
		for(sx=0;sx<DPR_Gap;++sx){
			// cout << __LINE__ << " sx: " << sx << endl;
			TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0; TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0;
			TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0; TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0;
		}
		AllocLen = sizeof(ORDER_BOOK_THOUSAND_RANGE)*TokenInfo->ThousandSize;
		TokenInfo->BUY_INFO_TILL_RUPEE  = (ORDER_BOOK_THOUSAND_RANGE*)malloc(AllocLen);//= (ORDER_BOOK_THOUSAND_RANGE*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
		TokenInfo->SELL_INFO_TILL_RUPEE = (ORDER_BOOK_THOUSAND_RANGE*)malloc(AllocLen);// = (ORDER_BOOK_THOUSAND_RANGE*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
		// initialize the Virtual Memory
		for(Depth=0;Depth<TokenInfo->ThousandSize;++Depth){
			TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Avaliable = 0;
			TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Avaliable = 0;
			for(sx=0;sx<10;++sx){
				TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].Avaliable=0;
				TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].Avaliable=0;
				for(index=0;index<10;++index){
					TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].Avaliable=0;
					TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].Avaliable=0;
					for(ShiftPosition=0;ShiftPosition<10;++ShiftPosition){
						TokenInfo->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].OneRupee[ShiftPosition].Avaliable = 0;
						TokenInfo->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].OneRupee[ShiftPosition].Avaliable = 0;
					}
				}
			}
		}

		int32_t L1,L2,L3,L4,L5,P1,P2,P3,sPrice,ThousandDepth,HundredDepth,TenRupeeDepth,RupeeDepth,cPrice;
		for(L1=OldThousandDepth-1; L1>=0; --L1){
			if(OLD_BUY_INFO_TILL_RUPEE[L1].Avaliable){
				P1 = (L1*1000);
				for(L2=9; L2>=0; --L2){
					if(OLD_BUY_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
						P2 = P1 + (L2*100);
						for(L3=9; L3>=0; --L3){
							if(OLD_BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
								P3 = P2 + (L3*10);
								for(L4=9; L4>=0; --L4){
									if(OLD_BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
										sPrice = ((P3 + L4)*TokenInfo->PaisaDepthRange);
										for(L5=(TokenInfo->PaisaDepthRange-1); L5>=0; --L5){
											if(OLD_BUY_PAISA_QTY_INFO[sPrice+L5].Quantity)
											{
												cPrice = (OldMin + ((sPrice+L5)*TokenInfo->TickSize));
												if(cPrice>TokenInfo->BUY_SELL_DPR_RANGE_MAX_PRICE || cPrice<TokenInfo->BUY_SELL_DPR_RANGE_MIN_PRICE)break;
												DPR_Gap = ((cPrice - TokenInfo->BUY_SELL_DPR_RANGE_MIN_PRICE)/TokenInfo->TickSize);
												TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count = OLD_BUY_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity = OLD_BUY_PAISA_QTY_INFO[sPrice+L5].Quantity;
												// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " sPrice: " << sPrice << " L5: " << L5 << " Qty: " << TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;
												Depth = (DPR_Gap/TokenInfo->PaisaDepthRange);
												ThousandDepth = Depth/1000; TenRupeeDepth = Depth - (ThousandDepth*1000);
												HundredDepth = (TenRupeeDepth)/100; RupeeDepth = TenRupeeDepth - (HundredDepth*100);
												TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10); if(ThousandDepth>TokenInfo->ThousandSize)break;
												TokenInfo->BUY_INFO_TILL_RUPEE[ThousandDepth].Hundred[HundredDepth].TenRupee[TenRupeeDepth].OneRupee[RupeeDepth].Avaliable += OLD_BUY_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->BUY_INFO_TILL_RUPEE[ThousandDepth].Hundred[HundredDepth].TenRupee[TenRupeeDepth].Avaliable += OLD_BUY_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->BUY_INFO_TILL_RUPEE[ThousandDepth].Hundred[HundredDepth].Avaliable += OLD_BUY_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->BUY_INFO_TILL_RUPEE[ThousandDepth].Avaliable += OLD_BUY_PAISA_QTY_INFO[sPrice+L5].Count;
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		for(L1=0; L1<OldThousandDepth; ++L1){
			if(OLD_SELL_INFO_TILL_RUPEE[L1].Avaliable){
				P1 = (L1*1000);
				for(L2=0; L2<10; ++L2){
					if(OLD_SELL_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
						P2 = P1 + (L2*100);
						for(L3=0; L3<10; ++L3){
							if(OLD_SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
								P3 = P2 + (L3*10);
								for(L4=0; L4<10; ++L4){
									if(OLD_SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
										sPrice = ((P3 + L4)*TokenInfo->PaisaDepthRange);
										for(L5=0; L5<TokenInfo->PaisaDepthRange; ++L5){
											if(OLD_SELL_PAISA_QTY_INFO[sPrice+L5].Quantity){
												cPrice = (OldMin + ((sPrice+L5)*TokenInfo->TickSize)); if(cPrice>TokenInfo->BUY_SELL_DPR_RANGE_MAX_PRICE || cPrice<TokenInfo->BUY_SELL_DPR_RANGE_MIN_PRICE)break;
												DPR_Gap = ((cPrice - TokenInfo->BUY_SELL_DPR_RANGE_MIN_PRICE)/TokenInfo->TickSize);
												TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count = OLD_SELL_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity = OLD_SELL_PAISA_QTY_INFO[sPrice+L5].Quantity;
												// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " sPrice: " << sPrice << " L5: " << L5 << " Qty: " << TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;
												Depth = (DPR_Gap/TokenInfo->PaisaDepthRange);
												ThousandDepth = Depth/1000; TenRupeeDepth = Depth - (ThousandDepth*1000);
												HundredDepth = (TenRupeeDepth)/100; RupeeDepth = TenRupeeDepth - (HundredDepth*100);
												TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10); if(ThousandDepth>TokenInfo->ThousandSize)break;
												TokenInfo->SELL_INFO_TILL_RUPEE[ThousandDepth].Hundred[HundredDepth].TenRupee[TenRupeeDepth].OneRupee[RupeeDepth].Avaliable += OLD_SELL_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->SELL_INFO_TILL_RUPEE[ThousandDepth].Hundred[HundredDepth].TenRupee[TenRupeeDepth].Avaliable += OLD_SELL_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->SELL_INFO_TILL_RUPEE[ThousandDepth].Hundred[HundredDepth].Avaliable += OLD_SELL_PAISA_QTY_INFO[sPrice+L5].Count;
												TokenInfo->SELL_INFO_TILL_RUPEE[ThousandDepth].Avaliable += OLD_SELL_PAISA_QTY_INFO[sPrice+L5].Count;
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}

		//sprintf(output,"\n==========================free =====================> StreamID:%d Token:%d,  Old:%p | New:%p..\n\n",StreamID,Tkn,OLD_SELL_PAISA_QTY_INFO,TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO);fwrite(output, 1, strlen(output), feedLog);
		free(OLD_BUY_PAISA_QTY_INFO);
		free(OLD_BUY_INFO_TILL_RUPEE);
		free(OLD_SELL_PAISA_QTY_INFO);
		free(OLD_SELL_INFO_TILL_RUPEE);
	}
	// cout << __LINE__ << " DPR_GAP: " << DPR_Gap << endl;
	// 	for(sx=0;sx<DPR_Gap;++sx){
	// 		cout << __LINE__ << " sx: " << sx << " Qty: " << TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Quantity << endl;
	// 		TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0; TokenInfo->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0;
	// 		TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0; TokenInfo->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0;
	// 	}
//	printf("Loaded New DPR Settings => Token:%d, [ OldMax:%d, OldMin:%d ---> Price:%d <---- NewMax:%d ,NewMin:%d ], "
//			"..\n",Tkn,OldMax,OldMin,Price,NewMax,NewMin); //cout << FeedProcessingLogger[fLoggerIndex].Message << endl;;
	return 0;
}

void MCX_Feeder::add_token_to_orderbook(uint32_t Token, TOKEN_ORDER_BOOK *TknOrdBk, int32_t StartSequence, int32_t HighPriceRange, int32_t LowPriceRange, int32_t TickSize, int32_t ConversionRatio, TOKEN_SNAPSHOT_BUY_SELL*Snapshot){
	int32_t ThousandDepth=0,Depth=0, ShiftPosition=0, DPR_Gap;
	unsigned long AllocLen = 0;// int32_t OrderBookSize = sizeof(ORDER_BOOK);
	TknOrdBk->TickSize = TickSize;
	TknOrdBk->PaisaDepthRange = 20;//(ConversionRatio/TickSize); if(TknOrdBk->PaisaDepthRange<=0)TknOrdBk->PaisaDepthRange=20;	//	TknOrdBk->PaisaDepthRange = 20;
	if(LowPriceRange < TickSize)LowPriceRange = 0;else if((LowPriceRange % TickSize))LowPriceRange -= (LowPriceRange % TickSize);
	if(HighPriceRange == 0)HighPriceRange = TickSize*100;
	DPR_Gap = HighPriceRange - LowPriceRange;	DPR_Gap/=TknOrdBk->TickSize; Depth = (DPR_Gap/TknOrdBk->PaisaDepthRange)+1; ThousandDepth = (Depth/1000)+1;
	TknOrdBk->BUY_SELL_DPR_RANGE_MAX_PRICE =  HighPriceRange; TknOrdBk->BUY_SELL_DPR_RANGE_MIN_PRICE =  LowPriceRange;
	DPR_Gap = (DPR_Gap+(TknOrdBk->PaisaDepthRange-(DPR_Gap%TknOrdBk->PaisaDepthRange))); AllocLen = sizeof(ORDER_QTY_INFO)*DPR_Gap;
	if(TknOrdBk->BUY_ORDER_PAISA_QUANTITY_INFO)free(TknOrdBk->BUY_ORDER_PAISA_QUANTITY_INFO);
	TknOrdBk->BUY_ORDER_PAISA_QUANTITY_INFO	  = (ORDER_QTY_INFO*)malloc(AllocLen);//= (int32_t*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
	if(TknOrdBk->SELL_ORDER_PAISA_QUANTITY_INFO)free(TknOrdBk->SELL_ORDER_PAISA_QUANTITY_INFO);
	TknOrdBk->SELL_ORDER_PAISA_QUANTITY_INFO  = (ORDER_QTY_INFO*)malloc(AllocLen);//= (int32_t*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
	// initialize the Virtual Memory
	for(int32_t sx=0;sx<DPR_Gap;sx++){
		TknOrdBk->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0; TknOrdBk->BUY_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0;
		TknOrdBk->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Quantity = 0; TknOrdBk->SELL_ORDER_PAISA_QUANTITY_INFO[sx].Count = 0;
	}
	AllocLen = sizeof(ORDER_BOOK_THOUSAND_RANGE)*ThousandDepth;
	if(TknOrdBk->BUY_INFO_TILL_RUPEE)free(TknOrdBk->BUY_INFO_TILL_RUPEE);
	TknOrdBk->BUY_INFO_TILL_RUPEE  = (ORDER_BOOK_THOUSAND_RANGE*)malloc(AllocLen);//= (ORDER_BOOK_THOUSAND_RANGE*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
	if(TknOrdBk->SELL_INFO_TILL_RUPEE)free(TknOrdBk->SELL_INFO_TILL_RUPEE);
	TknOrdBk->SELL_INFO_TILL_RUPEE = (ORDER_BOOK_THOUSAND_RANGE*)malloc(AllocLen);// = (ORDER_BOOK_THOUSAND_RANGE*)(&((int32_t*) mmap(0,AllocLen,PROT_READ | PROT_WRITE , MAP_PRIVATE | MAP_ANONYMOUS ,0,0))[1]);
	TknOrdBk->ThousandSize = ThousandDepth;
	// initialize the Virtual Memory
	for(Depth=0;Depth<ThousandDepth;Depth++){
		TknOrdBk->BUY_INFO_TILL_RUPEE[Depth].Avaliable = 0;
		TknOrdBk->SELL_INFO_TILL_RUPEE[Depth].Avaliable = 0;
		for(int32_t sx=0;sx<10;sx++){
			TknOrdBk->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].Avaliable=0;
			TknOrdBk->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].Avaliable=0;
			for(int32_t index=0;index<10;index++){
				TknOrdBk->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].Avaliable=0;
				TknOrdBk->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].Avaliable=0;
				for(ShiftPosition=0;ShiftPosition<10;ShiftPosition++){
					TknOrdBk->BUY_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].OneRupee[ShiftPosition].Avaliable = 0;
					TknOrdBk->SELL_INFO_TILL_RUPEE[Depth].Hundred[sx].TenRupee[index].OneRupee[ShiftPosition].Avaliable = 0;
				}
			}
		}
	}

	TknOrdBk->OrderBook.LastTradedPrice = 0;
	TknOrdBk->OrderBook.LastTradedQty = 0;
	for(uint32_t y=0;y<FEED_LEVEL_DEPTH;y++){
		TknOrdBk->OrderBook.BuyPrice	[y] 	= 0;
		TknOrdBk->OrderBook.BuyQty[y]		= 0;
		TknOrdBk->OrderBook.NoOfBuyOrds[y] 	= 0;
		TknOrdBk->OrderBook.SellPrice	[y] 	= 0;
		TknOrdBk->OrderBook.SellQty[y] 	= 0;
		TknOrdBk->OrderBook.NoOfSellOrds[y] 	= 0;
	}

	// if(StartSequence > 1){
	// 	int32_t idx = 0, Price = 0;
	// 	std::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator citr = contract_file->instrument_umap[EXCHG_MCX].find(Token);
	// 	if(citr != contract_file->instrument_umap[EXCHG_MCX].end()){
	// 		TknOrdBk->OrderBook.LastTradedPrice = ((Price_Point_Snapshot_Download_Start_Response*)citr->second->InitRes)->LastTradedPrice;
	// 		TknOrdBk->OrderBook.LastTradedQty = ((Price_Point_Snapshot_Download_Start_Response*)citr->second->InitRes)->LastTradedQuantity;
	// 	}
	// 	for(std::map<int32_t,TOKEN_SNAPSHOT_INFO*>::reverse_iterator itr = Snapshot->SnapshotBuy.rbegin(); itr != Snapshot->SnapshotBuy.rend(); itr++){
	// 		Price = itr->first;
	// 		if(Price < 0){
	// 			printf("\n\nPrice ERROR => Token:%u, Price:%d\n\n",Token,itr->first);exit(0);
	// 		}
	// 		if(Price > TknOrdBk->BUY_SELL_DPR_RANGE_MAX_PRICE || Price < TknOrdBk->BUY_SELL_DPR_RANGE_MIN_PRICE){
	// /*			fprintf(feedLog, "------------------------ BEFORE BUY : %d , %d -------------------------\n\n",Token, Price);
	// 			PrintNodes(feedLog, TknOrdBk, 0, 1); PrintNodes(feedLog, TknOrdBk, 0, 2);*/
	// 			if(Load_New_DPR_Settings(TknOrdBk, Token, Price)){
	// 				printf("\n\nSNAPSHOT_BUY Load_New_DPR_Settings ERROR => Token:%u, Price:%u, [ Quantity:%u BUY_SELL_DPR_RANGE_MAX_PRICE:%d | BUY_SELL_DPR_RANGE_MIN_PRICE:%d ]\n\n",
	// 						Token,itr->first,itr->second->Quantity,TknOrdBk->BUY_SELL_DPR_RANGE_MAX_PRICE,TknOrdBk->BUY_SELL_DPR_RANGE_MIN_PRICE);
	// 				exit(0);
	// 			}
	// /*			fprintf(feedLog, "------------------------ AFTER -------------------------\n\n");
	// 			PrintNodes(feedLog, TknOrdBk, 0, 1); PrintNodes(feedLog, TknOrdBk, 0, 2);*/
	// 		}

	// 		int32_t DPR_Gap = ((Price - TknOrdBk->BUY_SELL_DPR_RANGE_MIN_PRICE)/TknOrdBk->TickSize);;
	// 		int32_t Depth = (DPR_Gap/TknOrdBk->PaisaDepthRange);
	// 		int32_t ThousandDepth = Depth/1000;
	// 		int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
	// 		int32_t HundredDepth = (TenRupeeDepth)/100;
	// 		int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
	// 		TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
	// 		//cout << "UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
	// 		// Step 1
	// 		ORDER_BOOK_THOUSAND_RANGE *tptr=NULL; ORDER_BOOK_HUNDRED_RANGE *hptr=NULL; ORDER_BOOK_TEN_RUPEE_RANGE *tnptr=NULL;
	// 		tptr = &TknOrdBk->BUY_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable += itr->second->Orders;
	// 		hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable += itr->second->Orders;
	// 		tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable += itr->second->Orders;
	// 		tnptr->OneRupee[RupeeDepth].Avaliable += itr->second->Orders;
	// 		// Step 2
	// 		TknOrdBk->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity += itr->second->Quantity;
	// 		TknOrdBk->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count += itr->second->Orders;
	// 		// Step 3
	// 		if(idx < FEED_LEVEL_DEPTH){
	// 			TknOrdBk->OrderBook.BuyPrice	[idx] 	= itr->first;
	// 			TknOrdBk->OrderBook.BuyQty[idx]	= itr->second->Quantity;
	// 			TknOrdBk->OrderBook.NoOfBuyOrds[idx]	= itr->second->Orders;
	// 			++idx;
	// 		}
	// 	}
	// 	idx = 0;
	// 	for(std::map<int32_t,TOKEN_SNAPSHOT_INFO*>::iterator itr = Snapshot->SnapshotSell.begin(); itr != Snapshot->SnapshotSell.end(); itr++){
	// 		Price = itr->first;
	// 		if(Price < 0){
	// 			printf("\n\nPrice ERROR => Token:%u, Price:%d\n\n",Token,itr->first);exit(0);
	// 		}
	// 		if(Price > TknOrdBk->BUY_SELL_DPR_RANGE_MAX_PRICE || Price < TknOrdBk->BUY_SELL_DPR_RANGE_MIN_PRICE){
	// /*			fprintf(feedLog, "------------------------ BEFORE SELL: %d , %d -------------------------\n\n",Token, Price);
	// 			PrintNodes(feedLog, TknOrdBk, 0, 1); PrintNodes(feedLog, TknOrdBk, 0, 2);*/
	// 			if(Load_New_DPR_Settings(TknOrdBk, Token, Price)){
	// 				printf("\n\nSNAPSHOT_SELL Load_New_DPR_Settings ERROR => Token:%u, Price:%u, [ Quantity:%u BUY_SELL_DPR_RANGE_MAX_PRICE:%d | BUY_SELL_DPR_RANGE_MIN_PRICE:%d ]\n\n",
	// 						Token,itr->first,itr->second->Quantity,TknOrdBk->BUY_SELL_DPR_RANGE_MAX_PRICE,TknOrdBk->BUY_SELL_DPR_RANGE_MIN_PRICE);
	// 				exit(0);
	// 			}
	// /*			fprintf(feedLog, "------------------------ AFTER -------------------------\n\n");
	// 			PrintNodes(feedLog, TknOrdBk, 0, 1); PrintNodes(feedLog, TknOrdBk, 0, 2);*/
	// 		}

	// 		int32_t DPR_Gap = ((Price - TknOrdBk->BUY_SELL_DPR_RANGE_MIN_PRICE)/TknOrdBk->TickSize);;
	// 		int32_t Depth = (DPR_Gap/TknOrdBk->PaisaDepthRange);
	// 		int32_t ThousandDepth = Depth/1000;
	// 		int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
	// 		int32_t HundredDepth = (TenRupeeDepth)/100;
	// 		int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
	// 		TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
	// 		//cout << "UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
	// 		// Step 1
	// 		ORDER_BOOK_THOUSAND_RANGE *tptr=NULL; ORDER_BOOK_HUNDRED_RANGE *hptr=NULL; ORDER_BOOK_TEN_RUPEE_RANGE *tnptr=NULL;
	// 		tptr = &TknOrdBk->SELL_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable += itr->second->Orders;
	// 		hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable += itr->second->Orders;
	// 		tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable += itr->second->Orders;
	// 		tnptr->OneRupee[RupeeDepth].Avaliable += itr->second->Orders;
	// 		// Step 2
	// 		TknOrdBk->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity += itr->second->Quantity;
	// 		TknOrdBk->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count += itr->second->Orders;
	// 		// Step 3
	// 		if(idx < FEED_LEVEL_DEPTH){
	// 			TknOrdBk->OrderBook.SellPrice	[idx] 	= itr->first;
	// 			TknOrdBk->OrderBook.SellQty[idx]	= itr->second->Quantity;
	// 			TknOrdBk->OrderBook.NoOfSellOrds[idx]	= itr->second->Orders;
	// 			++idx;
	// 		}
	// 	}

	// 	for(int32_t Depth=0;Depth<FEED_LEVEL_DEPTH;Depth++){
	// 		if((TknOrdBk->OrderBook.BuyPrice[Depth] && TknOrdBk->OrderBook.BuyQty[Depth]==0) || (TknOrdBk->OrderBook.SellPrice[Depth] && TknOrdBk->OrderBook.SellQty[Depth]==0)
	// 				|| (TknOrdBk->OrderBook.BuyPrice[Depth] && TknOrdBk->OrderBook.SellPrice[Depth] && TknOrdBk->OrderBook.BuyPrice[Depth] > TknOrdBk->OrderBook.SellPrice[Depth])){
	// 			printf("Wrong OrderBook Init => Token:%d , HighPriceRange:%d, LowPriceRange:%d\n\n",Token,HighPriceRange,LowPriceRange);
	// 			printf("%0.2f--%u \t\t\t\t %0.2f--%u\n",((double)TknOrdBk->OrderBook.BuyPrice[Depth]/(double)100.00), TknOrdBk->OrderBook.BuyQty[Depth],
	// 					((double)TknOrdBk->OrderBook.SellPrice[Depth]/(double)100.00), TknOrdBk->OrderBook.SellQty[Depth] );
	// 			printf("\n");
	// 			 PrintNodes(stdout, TknOrdBk, 0, 1); PrintNodes(stdout, TknOrdBk, 0, 2); fprintf(stdout, "\n\n"); exit(0);
	// 		}
	// 	}
	// }

	TknOrdBk->CurrentSequence = StartSequence;
}

int MCX_Feeder::Add_ReplayFile(Exchange_Type Segment, int8_t StreamID, const char *file){
	if(!file)return -1;
    FILE *fp = fopen(file, "r");
    if (fp == NULL) {
         printf("\ncouldn't open FEED_DATA_FILE :\033[1;31m %s \033[0mfor reading.\n\n",file);
         fflush((FILE *)stdout); return -2;
     }
    printf("Loaded FEED_DATA_FILE :\033[1;31m %s \033[0mfor reading.\n",file);

	//Data = (DATA_HOLDER*)realloc(Data, (sizeof(DATA_HOLDER) * (TotalNodes + 1)));
	DATA_HOLDER *pData = new (&Data[TotalNodes]) DATA_HOLDER;
	std::vector<std::string> _fname; _fname = split(file,"/");
	char filen[50]; sprintf(filen,"Feeder_%s.log",((_fname.size())?_fname[_fname.size()-1].c_str() : file));
	pData->feed_log = fopen(filen, "w");  if(!pData->feed_log){cout << "Error Creating Output File => "<< filen << endl; exit(0);} if (setvbuf(pData->feed_log, NULL,_IONBF, 0) < 0){printf("setvbuf error in TokenOrderbook\n");return 0;}
	pData->fp =fp;
	pData->MSG_LEN=0;
	pData->sz=0;
	pData->SEQ_NO =0;
	pData->ReceiveSequence=1;
	pData->Processed_Bytes = 0;
	//pData->mint = 0;
	pData->Pos = 0;
	pData->StreamID = StreamID;
	pData->Buffer = (unsigned char*)numa_alloc_local(BYTES_READ_SIZE + 1048576);
	pData->BytesToRead = BYTES_READ_SIZE;
	pData->FirstOrderID = 0;
	pData->_Segment = Segment;
	/*if(Segment == EXCHG_MCX){
		char rFile[1024]; strcpy(rFile, BaseFileName.c_str()); *strstr(rFile, "feeder_capture_tokens_sync_point_") = '\0';
		sprintf(rFile + strlen(rFile), "feeder_capture_tokens_sync_point_%s", (strstr(BaseFileName.c_str(), "feeder_capture_tokens_sync_point_") + 33));
		pData->ResetSeqFile = fopen(rFile, "rb");
		if (pData->ResetSeqFile == NULL) {
			 printf("\nCouldn't Open MCX Init File:%s For Reading..\n\n", rFile); fflush((FILE *)stdout);
		 }else{
				//printf("\nUsing File %s For MCX Recovery...\n\n", rFile);
		 }
	}*/

	pData->PRODUCT = new PRODUCT_INFO_MCX[(contract_file->Max_Product_ID_MCX + 1)];
	fprintf(pData->feed_log,"\nStreamID:%d , Total_Products:%d\n", pData->StreamID, contract_file->Max_Product_ID_MCX); //ABORT;
	for(uint32_t p=0; p < contract_file->Max_Product_ID_MCX + 1; p++){
		pData->PRODUCT[p].SkipSequence = 0;
		pData->PRODUCT[p].BackLogProcessing = 0;
		pData->PRODUCT[p].BackLogProcessingIdx = -1;
		pData->PRODUCT[p].CurrentSequence = 0;
		pData->PRODUCT[p].BufferedSequence = 0;
		pData->PRODUCT[p].BackLogSequence = 0;
		pData->PRODUCT[p].BackLogIndex = 0;
		// PRODUCT[p].PRIMARY_DATA_BUFFER = NULL;
		pData->PRODUCT[p].ProductID = 0;
	}

	for(auto sidITR = contract_file->StreamIDsProduct_MCX[pData->StreamID].begin(); sidITR != contract_file->StreamIDsProduct_MCX[pData->StreamID].end(); ++sidITR){
		fprintf(pData->feed_log, "ProductID:%d\n", *sidITR);
		pData->PRODUCT[*sidITR].ProductID = *sidITR;
	}
	
	std::map<std::pair<int, int>, GenericContract>::iterator contract_itr;
	for( contract_itr= _Contract->begin(); contract_itr != _Contract->end(); contract_itr++ ){
		if(contract_itr->first.first == Segment){
			TOKEN_ORDER_BOOK *TknOrd = new TOKEN_ORDER_BOOK;
			pData->OrderBook.insert(std::make_pair(contract_itr->first.second, TknOrd));
			TknOrd->BUY_SELL_DPR_RANGE_MAX_PRICE = 0;
			TknOrd->BUY_INFO_TILL_RUPEE = NULL;
			TknOrd->BUY_ORDER_PAISA_QUANTITY_INFO = NULL;
			TknOrd->SELL_INFO_TILL_RUPEE = NULL;
			TknOrd->SELL_ORDER_PAISA_QUANTITY_INFO = NULL;
			TknOrd->BUY_SELL_DPR_RANGE_MIN_PRICE = 0;	
			memset(&TknOrd->OrderBook,0,sizeof(TknOrd->OrderBook));
			TknOrd->ThousandSize = 0;
			TknOrd->TickSize = contract_itr->second.TickSize;
			TknOrd->PaisaDepthRange = 0; 
			TknOrd->PriceExponent = __builtin_powi(10,contract_itr->second.PriceExponent);
			std::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator citr = contract_file->instrument_umap[EXCHG_MCX].find(contract_itr->first.second);
			if(citr != contract_file->instrument_umap[EXCHG_MCX].end()){
				// printf("# Token => %d [%d - %d]\n", contract_itr->first.second, citr->second->Contract.HighPriceRange, citr->second->Contract.LowPriceRange);
				add_token_to_orderbook(contract_itr->first.second, TknOrd, citr->second->StartSequence, citr->second->Contract.HighPriceRange, citr->second->Contract.LowPriceRange, contract_itr->second.TickSize, TknOrd->PriceExponent, (TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR);
			}
			else{
				printf("@ Token => %d [%d - %d]\n", contract_itr->first.second, contract_itr->second.HighPriceRange, contract_itr->second.LowPriceRange);
				add_token_to_orderbook(contract_itr->first.second, TknOrd, 1, contract_itr->second.HighPriceRange,contract_itr->second.LowPriceRange, contract_itr->second.TickSize, TknOrd->PriceExponent, NULL);
			}
		}
	}

	TotalNodes++;
	return 0;
}

int MCX_Feeder::generate_feed(DATA_HOLDER *pData, uint64_t *timestamp){

	unsigned char *StreamBufferPtr = (unsigned char *)(pData->Buffer + pData->Processed_Bytes);
	int ProcessedMsgLen = 0,ProcessMsgSize=0,ordsz=0;PRODUCT_INFO_MCX *Product = NULL;
	int OrderBookUpdated=0; int Price=0/*, Quantity=0*/, OrderType ='-', QtyMatch=0, ShiftPosition=0, sx=0, tOrders=0;
// #ifdef PRINT_LOGS
// 	printf("[timestamp:%ld][%d] => pData->sz=%d,\tProcessed_Bytes:%d,\tMsgLength:%d,\tSequenceNo:%d,\tMsgCode:%d,\tToken:%d,\tAP:%d\n",
// 			*timestamp, ORDER->LastUpdatedTime, pData->sz, pData->Processed_Bytes,pData->MSG_LEN,ORDER->SequenceNo, ORDER->MsgCode, ORDER->UniqueIdentifier,ORDER->ActivePassive_Flg);
// #endif

	TOKEN_ORDER_BOOK  *OrderBookPtr=NULL;uint32_t Token;
	ORDER_BOOK_THOUSAND_RANGE *tptr=NULL; ORDER_BOOK_HUNDRED_RANGE *hptr=NULL; ORDER_BOOK_TEN_RUPEE_RANGE *tnptr=NULL;
	while(ProcessedMsgLen < (pData->MSG_LEN)){
		MessageHeader *Header = (MessageHeader*)(StreamBufferPtr);
		if(Header->TemplateID == 13003){
			PacketHeader *pHDR = (PacketHeader*)StreamBufferPtr;
			Product = &(pData->PRODUCT[pHDR->Body.MarketSegmentID]);
			if(Product->ProductID && Product->BackLogProcessingIdx == -1){
				// if(pHDR->Body.MarketSegmentID != 401){
				// 	cout << "Kya h re" << endl;
				// }
				// fprintf(pData->feed_log, "\nPacket Header[nTot:%ld, ProcessedBytes:%d] => BodyLen:%u, TemplateID:%u, MsgSeqNum:%u, ReceivedApplSeqNum:%u, ApplSeqResetIndicator:%u, MarketSegmentID:%u, PartitionID:%u, CompletionIndicator:%u, TransactTime:%lu, SkipSequence:%d\n",
				// 		pData->MSG_LEN,ProcessedMsgLen,pHDR->Header.BodyLen, pHDR->Header.TemplateID, pHDR->Header.MsgSeqNum, pHDR->Body.ApplSeqNum, pHDR->Body.ApplSeqResetIndicator, pHDR->Body.MarketSegmentID, pHDR->Body.PartitionID, pHDR->Body.CompletionIndicator, pHDR->Body.TransactTime,
				// 		Product->SkipSequence);
			}
		}
		else if(Header->TemplateID != 13001 && Product && Product->ProductID && Product->BackLogProcessingIdx == -1){
			if(!Product->CurrentSequence)Product->CurrentSequence=Header->MsgSeqNum;
			else if(Header->MsgSeqNum != Product->CurrentSequence+1){
			 	// ABORT;
			}
			Product->CurrentSequence = Header->MsgSeqNum;

			if(Header->TemplateID == 13101){
				OrderModify *ord = (OrderModify*)StreamBufferPtr;
				std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(ord->SecurityID);
				// fprintf(pData->feed_log,"[%d] Modify\t=> %u, %ld, %d, %ld, %ld, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->PrevPrice, ord->PrevDisplayQty, ord->Price, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
				if(tITR != pData->OrderBook.end()){
					// fprintf(pData->feed_log,"[%d] Modify\t=> %u, %ld, %d, %ld, %ld, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->PrevPrice, ord->PrevDisplayQty, ord->Price, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
					int32_t Price = 0, pPrice = 0, Quantity = 0, pQuantity = 0, OrderType = 0, ShiftPosition=0, sx=0;
					Price = ord->Price / MCX_PRICE_MULTIPLIER; Quantity = ord->DisplayQty / 10000; pPrice = ord->PrevPrice / MCX_PRICE_MULTIPLIER; pQuantity = ord->PrevDisplayQty / 10000;  OrderType = ord->Side;
					OrderBookPtr = tITR->second;
					Token = ord->SecurityID;

					// if(Header->MsgSeqNum >= 10157 &&  Header->MsgSeqNum >= 10177)
					{
						// PrintNodes(pData->feed_log, OrderBookPtr, 0, 1); PrintNodes(pData->feed_log, OrderBookPtr, 0, 2);
					}

					// cout << "Price: " << Price << " BUY_SELL_DPR_RANGE_MAX_PRICE:" <<  OrderBookPtr->BUY_SELL_DPR_RANGE_MAX_PRICE << " BUY_SELL_DPR_RANGE_MIN_PRICE: " << OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE << endl;
					if(Price > OrderBookPtr->BUY_SELL_DPR_RANGE_MAX_PRICE || Price < OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE){
						if(Load_New_DPR_Settings(OrderBookPtr,Token, Price)){
							fprintf(pData->feed_log,"\n\nStream ID:%d, MSG:%c, OrderType:%c, Token:%ld, Price:%u, [ Quantity:%u BUY_SELL_DPR_RANGE_MAX_PRICE:%d | BUY_SELL_DPR_RANGE_MIN_PRICE:%d ] =>  Can Not Handle This Modify Order .. Price & DPR Mismatch\n\n",
									pData->StreamID,StreamBufferPtr[8],OrderType, Token,Price,Quantity,OrderBookPtr->BUY_SELL_DPR_RANGE_MAX_PRICE,OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
							// ABORT;
						}
					}

                    int pDPR_Gap = ((pPrice - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE)/OrderBookPtr->TickSize);
					int pDepth = (pDPR_Gap/OrderBookPtr->PaisaDepthRange);
					int pThousandDepth = pDepth/1000;
					int pTenRupeeDepth = pDepth - (pThousandDepth*1000);
					int pHundredDepth = (pTenRupeeDepth)/100;
					int pRupeeDepth = pTenRupeeDepth - (pHundredDepth*100);
					pTenRupeeDepth = (pRupeeDepth)/10; pRupeeDepth = pRupeeDepth - (pTenRupeeDepth*10);
					
					int DPR_Gap = ((Price - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE)/OrderBookPtr->TickSize);;
					int Depth = (DPR_Gap/OrderBookPtr->PaisaDepthRange);
					int ThousandDepth = Depth/1000;
					int TenRupeeDepth = Depth - (ThousandDepth*1000);
					int HundredDepth = (TenRupeeDepth)/100;
					int RupeeDepth = TenRupeeDepth - (HundredDepth*100);
					TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);

					if(OrderType == 1){	//fprintf(ptr->feedLog,"LINE => %d, Price:%d , Quantity:%d\n", __LINE__,Price,Quantity);	 // BUY

						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity -= pQuantity;
						// cout << __LINE__ << " pDPR_Gap: " << pDPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity << endl;

						tptr = &OrderBookPtr->BUY_INFO_TILL_RUPEE[pThousandDepth]; tptr->Avaliable -= 1; hptr = &tptr->Hundred[pHundredDepth]; hptr->Avaliable -= 1;
						tnptr = &hptr->TenRupee[pTenRupeeDepth]; tnptr->Avaliable -= 1; tnptr->OneRupee[pRupeeDepth].Avaliable -= 1;
						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Count -= 1;

						tptr = &OrderBookPtr->BUY_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable += 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable += 1;
						tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable += 1; tnptr->OneRupee[RupeeDepth].Avaliable += 1;
						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count += 1;
						
						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity += Quantity;
						// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;

						//if(QtyMatch <= lQty){
						if(pPrice >= OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1]){
							for(sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
								if(OrderBookPtr->OrderBook.BuyPrice[sx]==pPrice){
									OrderBookPtr->OrderBook.BuyQty[sx] -= pQuantity;
									OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookUpdated = 1;
									if(Price==pPrice){OrderBookPtr->OrderBook.BuyQty[sx] += Quantity; goto TOKEN_UPDATED;}
									if(OrderBookPtr->OrderBook.BuyQty[sx]==0){
										ShiftPosition = 1;
										for(; sx<FEED_LEVEL_DEPTH_1; ++sx){
											OrderBookPtr->OrderBook.BuyPrice[sx] 	= OrderBookPtr->OrderBook.BuyPrice[sx+1];
											OrderBookPtr->OrderBook.BuyQty[sx]	= OrderBookPtr->OrderBook.BuyQty[sx+1];
											OrderBookPtr->OrderBook.NoOfBuyOrds[sx]	= OrderBookPtr->OrderBook.NoOfBuyOrds[sx+1];
											if(OrderBookPtr->OrderBook.BuyPrice[sx+1]==0){ShiftPosition=0;break;}
										}
										if(Price >= OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1]){
											for(sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
												if(OrderBookPtr->OrderBook.BuyPrice[sx]==Price){
													OrderBookPtr->OrderBook.BuyQty[sx] += Quantity;
													OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
													if(ShiftPosition)break; else goto TOKEN_UPDATED; // vUpdate=0;
												}else if(Price > OrderBookPtr->OrderBook.BuyPrice[sx]){
													Depth = sx;
													for(sx=FEED_LEVEL_DEPTH_1; sx>Depth; --sx){
														OrderBookPtr->OrderBook.BuyPrice[sx] 	= OrderBookPtr->OrderBook.BuyPrice[sx-1];
														OrderBookPtr->OrderBook.BuyQty[sx] = OrderBookPtr->OrderBook.BuyQty[sx-1];
														OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->OrderBook.NoOfBuyOrds[sx-1];
													}
													OrderBookPtr->OrderBook.BuyPrice[sx] = Price;
													OrderBookPtr->OrderBook.BuyQty[sx] = Quantity;
													OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
													goto TOKEN_UPDATED;
												}
											}
										}
										if(ShiftPosition){
											int32_t Depth = (OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
											OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.BuyQty[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.NoOfBuyOrds[FEED_LEVEL_DEPTH_1] = 0;
											int32_t PaisaDepth = ((Depth/OrderBookPtr->TickSize)%(OrderBookPtr->PaisaDepthRange))-1; Depth /=(OrderBookPtr->TickSize*OrderBookPtr->PaisaDepthRange);
											int32_t ThousandDepth = Depth/1000;
											int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
											int32_t HundredDepth = (TenRupeeDepth)/100;
											int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
											TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
											//cout << "Ord Update UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
											int32_t L1=ThousandDepth;
											for(ORDER_BOOK_THOUSAND_RANGE *L1_PTR = &OrderBookPtr->BUY_INFO_TILL_RUPEE[ThousandDepth]; L1>=0; --L1, --L1_PTR){
												if(L1_PTR->Avaliable){
													int32_t L2=HundredDepth;
													for(ORDER_BOOK_HUNDRED_RANGE *L2_PTR = &L1_PTR->Hundred[HundredDepth]; L2>=0; --L2, --L2_PTR){
														if(L2_PTR->Avaliable){
															int32_t L3=TenRupeeDepth;
															for(ORDER_BOOK_TEN_RUPEE_RANGE *L3_PTR = &L2_PTR->TenRupee[TenRupeeDepth]; L3>=0; --L3, --L3_PTR){
																if(L3_PTR->Avaliable){
																	int32_t L4=RupeeDepth;
																	for(ORDER_BOOK_RUPEE_RANGE *L4_PTR = &L3_PTR->OneRupee[RupeeDepth]; L4>=0; --L4, --L4_PTR){
																		if(L4_PTR->Avaliable){
																			//P1 = (L1*1000); P2 = P1 + (L2*100); P3 = P2 + (L3*10); sPrice = ((P3 + L4)*20);
																			int32_t sPrice = ((((((L1*1000)) + (L2*100)) + (L3*10)) + L4)*OrderBookPtr->PaisaDepthRange);
																			for(int32_t L5=PaisaDepth; L5>=0; --L5){
																				if(OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
																					OrderBookPtr->OrderBook.BuyQty[FEED_LEVEL_DEPTH_1] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
																					OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] = (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
																					OrderBookPtr->OrderBook.NoOfBuyOrds[FEED_LEVEL_DEPTH_1] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count;
																					goto TOKEN_UPDATED;
																				}
																			}
																		}
																		PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
																	}
																}
																RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
															}
														}
														TenRupeeDepth = 9; RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
													}
												}
												HundredDepth = 9; TenRupeeDepth = 9; RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
											}
											goto TOKEN_UPDATED;
										}
									}
									break;
								}
							}
						}
						if(Price >= OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1]){ // vUpdate &&
							for(sx=0; sx<FEED_LEVEL_DEPTH; sx++){
								if(OrderBookPtr->OrderBook.BuyPrice[sx]==Price){
									OrderBookPtr->OrderBook.BuyQty[sx] += Quantity;
									OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									break;
								}else if(Price > OrderBookPtr->OrderBook.BuyPrice[sx]){
									ShiftPosition = sx;
									for(sx=FEED_LEVEL_DEPTH_1; sx>ShiftPosition; --sx){
										OrderBookPtr->OrderBook.BuyPrice[sx] 	= OrderBookPtr->OrderBook.BuyPrice[sx-1];
										OrderBookPtr->OrderBook.BuyQty[sx] = OrderBookPtr->OrderBook.BuyQty[sx-1];
										OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->OrderBook.NoOfBuyOrds[sx-1];
									}
									OrderBookPtr->OrderBook.BuyPrice[sx] = Price;
									OrderBookPtr->OrderBook.BuyQty[sx] = Quantity;
									OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									break;
								}
							}
							OrderBookUpdated = 1;
						}
					}
					else if(OrderType == 2){ //fprintf(ptr->feedLog,"LINE => %d, Price:%d , Quantity:%d\n", __LINE__,Price,Quantity);// SELL

						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity -= pQuantity;
						// cout << __LINE__ << " DPR_Gap: " << pDPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity << endl;

						tptr = &OrderBookPtr->SELL_INFO_TILL_RUPEE[pThousandDepth]; tptr->Avaliable -= 1; hptr = &tptr->Hundred[pHundredDepth]; hptr->Avaliable -= 1;
						tnptr = &hptr->TenRupee[pTenRupeeDepth]; tnptr->Avaliable -= 1; tnptr->OneRupee[pRupeeDepth].Avaliable -= 1;
						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Count -= 1;

						tptr = &OrderBookPtr->SELL_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable += 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable += 1;
						tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable += 1; tnptr->OneRupee[RupeeDepth].Avaliable += 1;
						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count += 1;

						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity += Quantity;
						// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;

						if(pPrice <= OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] || OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1]==0){
							for(sx=0; sx<FEED_LEVEL_DEPTH; sx++){
								if(OrderBookPtr->OrderBook.SellPrice[sx]==pPrice){
									OrderBookPtr->OrderBook.SellQty[sx] -= pQuantity;
									OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookUpdated = 1;
									if(Price==pPrice){OrderBookPtr->OrderBook.SellQty[sx] += Quantity;	goto TOKEN_UPDATED;}
									if(OrderBookPtr->OrderBook.SellQty[sx]==0){
										ShiftPosition = 1;
										for(; sx<FEED_LEVEL_DEPTH_1; sx++){
											OrderBookPtr->OrderBook.SellPrice[sx] 		= OrderBookPtr->OrderBook.SellPrice[sx+1];
											OrderBookPtr->OrderBook.SellQty[sx]	= OrderBookPtr->OrderBook.SellQty[sx+1];
											OrderBookPtr->OrderBook.NoOfSellOrds[sx]	= OrderBookPtr->OrderBook.NoOfSellOrds[sx+1];
											if(OrderBookPtr->OrderBook.SellPrice[sx+1]==0){ShiftPosition=0;break;}
										}
										if(Price <= OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] || OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1]==0){
											for(sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
												if(OrderBookPtr->OrderBook.SellPrice[sx]==Price){
													OrderBookPtr->OrderBook.SellQty[sx] += Quantity;
													OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
													if(ShiftPosition)break; else goto TOKEN_UPDATED; //vUpdate=0;
												}else if(Price < OrderBookPtr->OrderBook.SellPrice[sx]){
													Depth = sx;
													for(sx=FEED_LEVEL_DEPTH_1; sx>Depth; --sx){
														OrderBookPtr->OrderBook.SellPrice[sx] 	= OrderBookPtr->OrderBook.SellPrice[sx-1];
														OrderBookPtr->OrderBook.SellQty[sx] = OrderBookPtr->OrderBook.SellQty[sx-1];
														OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->OrderBook.NoOfSellOrds[sx-1];
													}
													OrderBookPtr->OrderBook.SellPrice[sx] = Price;
													OrderBookPtr->OrderBook.SellQty[sx] = Quantity;
													OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
													goto TOKEN_UPDATED;
												}
												else if(OrderBookPtr->OrderBook.SellPrice[sx]==0){
													OrderBookPtr->OrderBook.SellPrice[sx] = Price;
													OrderBookPtr->OrderBook.SellQty[sx] = Quantity;
													OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
													goto TOKEN_UPDATED;
												}
											}
										}
										if(ShiftPosition){
											int32_t Depth = (OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
											OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.SellQty[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.NoOfSellOrds[FEED_LEVEL_DEPTH_1] = 0;
											int32_t PaisaDepth = ((Depth/OrderBookPtr->TickSize)%(OrderBookPtr->PaisaDepthRange))+1; Depth /=(OrderBookPtr->TickSize*OrderBookPtr->PaisaDepthRange);
											int32_t ThousandDepth = Depth/1000;
											int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
											int32_t HundredDepth = (TenRupeeDepth)/100;
											int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
											TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
											//cout << "Ord Update UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
											int32_t L1=ThousandDepth;
											for(ORDER_BOOK_THOUSAND_RANGE *L1_PTR = &OrderBookPtr->SELL_INFO_TILL_RUPEE[ThousandDepth]; L1<OrderBookPtr->ThousandSize; ++L1,++L1_PTR){
												if(L1_PTR->Avaliable){
													int32_t L2=HundredDepth;
													for(ORDER_BOOK_HUNDRED_RANGE *L2_PTR = &L1_PTR->Hundred[HundredDepth]; L2<10; ++L2,++L2_PTR){
														if(L2_PTR->Avaliable){
															int32_t L3=TenRupeeDepth;
															for(ORDER_BOOK_TEN_RUPEE_RANGE *L3_PTR = &L2_PTR->TenRupee[TenRupeeDepth]; L3<10; ++L3,++L3_PTR){
																if(L3_PTR->Avaliable){
																	int32_t L4=RupeeDepth;
																	for(ORDER_BOOK_RUPEE_RANGE *L4_PTR = &L3_PTR->OneRupee[RupeeDepth]; L4<10; ++L4,++L4_PTR){
																		if(L4_PTR->Avaliable){
																			//P1 = (L1*1000); P2 = P1 + (L2*100); P3 = P2 + (L3*10); sPrice = ((P3 + L4)*20);
																			int32_t sPrice = ((((((L1*1000)) + (L2*100)) + (L3*10)) + L4)*OrderBookPtr->PaisaDepthRange);
																			for(int32_t L5=PaisaDepth; L5<OrderBookPtr->PaisaDepthRange; ++L5){
																				if(OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
																					OrderBookPtr->OrderBook.SellQty[FEED_LEVEL_DEPTH_1] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
																					OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] = (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
																					OrderBookPtr->OrderBook.NoOfSellOrds[FEED_LEVEL_DEPTH_1] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count;
																					goto TOKEN_UPDATED;
																				}
																			}
																		}
																		PaisaDepth = 0;
																	}
																}
																RupeeDepth = 0; PaisaDepth = 0;
															}
														}
														TenRupeeDepth = 0; RupeeDepth = 0; PaisaDepth = 0;
													}
												}
												HundredDepth = 0; TenRupeeDepth = 0; RupeeDepth = 0; PaisaDepth = 0;
											}
											goto TOKEN_UPDATED; // Not Before vUpdate
										}
									}
									break;
								}
							}
						}
						if((Price <= OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] || OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1]==0)){ //vUpdate &&
							for(sx=0; sx<FEED_LEVEL_DEPTH; sx++){
								if(OrderBookPtr->OrderBook.SellPrice[sx]==Price){
									OrderBookPtr->OrderBook.SellQty[sx] += Quantity;
									OrderBookPtr->OrderBook.NoOfSellOrds[sx] =  OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									break;
								}else if(Price < OrderBookPtr->OrderBook.SellPrice[sx]){
									ShiftPosition = sx;
									for(sx=FEED_LEVEL_DEPTH_1; sx>ShiftPosition; --sx){
										OrderBookPtr->OrderBook.SellPrice[sx] 	= OrderBookPtr->OrderBook.SellPrice[sx-1];
										OrderBookPtr->OrderBook.SellQty[sx] = OrderBookPtr->OrderBook.SellQty[sx-1];
										OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->OrderBook.NoOfSellOrds[sx-1];
									}
									OrderBookPtr->OrderBook.SellPrice[sx] = Price;
									OrderBookPtr->OrderBook.SellQty[sx] = Quantity;
									OrderBookPtr->OrderBook.NoOfSellOrds[sx] =  OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									break;
								}
								else if(OrderBookPtr->OrderBook.SellPrice[sx]==0){
									OrderBookPtr->OrderBook.SellPrice[sx] = Price;
									OrderBookPtr->OrderBook.SellQty[sx] = Quantity;
									OrderBookPtr->OrderBook.NoOfSellOrds[sx] =  OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									break;
								}
							}
							OrderBookUpdated = 1;
						}
					}
				}
			}
			else if(Header->TemplateID == 13106){
				OrderModifySamePriority *ord = (OrderModifySamePriority*)StreamBufferPtr;
				std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(ord->SecurityID);
				// fprintf(pData->feed_log,"[%d] ModifyPri\t=> %u, %ld, %d, %ld, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->PrevDisplayQty, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
				if(tITR != pData->OrderBook.end()){
					// fprintf(pData->feed_log,"[%d] ModifyPri\t=> %u, %ld, %d, %ld, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->PrevDisplayQty, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
					int32_t Price = 0, pPrice = 0, Quantity = 0, pQuantity = 0, OrderType = 0;
					Price = ord->Price / MCX_PRICE_MULTIPLIER; Quantity = ord->DisplayQty / 10000; pPrice = Price; pQuantity = ord->PrevDisplayQty / 10000;  OrderType = ord->Side;
					Token = ord->SecurityID;
					OrderBookPtr = tITR->second;

					// {
						// PrintNodes(pData->feed_log, OrderBookPtr, 0, 1); PrintNodes(pData->feed_log, OrderBookPtr, 0, 2);
					// }

					int pDPR_Gap = ((pPrice - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE)/OrderBookPtr->TickSize);
					int pDepth = (pDPR_Gap/OrderBookPtr->PaisaDepthRange);
					int pThousandDepth = pDepth/1000;
					int pTenRupeeDepth = pDepth - (pThousandDepth*1000);
					int pHundredDepth = (pTenRupeeDepth)/100;
					int pRupeeDepth = pTenRupeeDepth - (pHundredDepth*100);
					pTenRupeeDepth = (pRupeeDepth)/10; pRupeeDepth = pRupeeDepth - (pTenRupeeDepth*10);

					if(OrderType==1){
						if((int32_t)Price >= OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1]){
							for(uint32_t sx=0; sx<FEED_LEVEL_DEPTH; sx++){
								if(OrderBookPtr->OrderBook.BuyPrice[sx]==(int32_t)Price){
									OrderBookPtr->OrderBook.BuyQty[sx] -= pQuantity;
									OrderBookPtr->OrderBook.BuyQty[sx] += Quantity;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = 1;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookUpdated = 1; break;
								}
							}
						}
						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity -= pQuantity;
						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity += Quantity;
						// cout << __LINE__ << " pDPR_Gap: " << pDPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity << endl;
					}
					else if(OrderType==2){
						if(((int32_t)Price <= OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] || OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1]==0)){
							for(uint32_t sx=0; sx<FEED_LEVEL_DEPTH; sx++){
								if(OrderBookPtr->OrderBook.SellPrice[sx]==(int32_t)Price){
									OrderBookPtr->OrderBook.SellQty[sx] -= pQuantity;
									OrderBookPtr->OrderBook.SellQty[sx] += Quantity;
									OrderBookPtr->OrderBook.FeedEventReason = 1;
									OrderBookPtr->OrderBook.FeedEventAggressor = 2;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookUpdated = 1; break;
								}
							}
						}
						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity -= pQuantity;
						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity += Quantity;
						// cout << __LINE__ << " pDPR_Gap: " << pDPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Quantity << endl;
					}
				}
			}
			else if(Header->TemplateID == 13100){
				OrderAdd *ord = (OrderAdd*)StreamBufferPtr;
				std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(ord->SecurityID);
				// fprintf(pData->feed_log,"[%d] Add\t=> %u, %ld, %d, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
				if(tITR != pData->OrderBook.end())
				{
					// fprintf(pData->feed_log,"[%d] Add\t=> %u, %ld, %d, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
					int32_t Price = ord->Price / MCX_PRICE_MULTIPLIER, Quantity = ord->DisplayQty / 10000, OrderType = ord->Side, ShiftPosition=0, sx=0;
					Token = ord->SecurityID;
					OrderBookPtr = tITR->second;
					// {
						// PrintNodes(pData->feed_log, OrderBookPtr, 0, 1); PrintNodes(pData->feed_log, OrderBookPtr, 0, 2);
					// }
					if(OrderBookPtr->BUY_SELL_DPR_RANGE_MAX_PRICE){
						if(Price > OrderBookPtr->BUY_SELL_DPR_RANGE_MAX_PRICE || Price < OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE){
							if(Load_New_DPR_Settings(OrderBookPtr,Token, Price)){
								fprintf(pData->feed_log,"\n\nStream ID:%d, MSG:%c, OrderType:%c, Token:%ld, Price:%u, [ Quantity:%u BUY_SELL_DPR_RANGE_MAX_PRICE:%d | BUY_SELL_DPR_RANGE_MIN_PRICE:%d ] =>  Can Not Handle This New Order .. Price & DPR Mismatch\n\n",pData->StreamID,StreamBufferPtr[8],OrderType,
										Token,Price,Quantity,OrderBookPtr->BUY_SELL_DPR_RANGE_MAX_PRICE,OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
								// ABORT;
							}
						}
						int DPR_Gap = ((Price - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE)/OrderBookPtr->TickSize);
						int Depth = (DPR_Gap/OrderBookPtr->PaisaDepthRange);
						int ThousandDepth = Depth/1000;
						int TenRupeeDepth = Depth - (ThousandDepth*1000);
						int HundredDepth = (TenRupeeDepth)/100;
						int RupeeDepth = TenRupeeDepth - (HundredDepth*100);
						TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
						//cout << "UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;

						if(OrderType==1){		// BUY
							tptr = &OrderBookPtr->BUY_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable += 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable += 1;
							tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable += 1; tnptr->OneRupee[RupeeDepth].Avaliable += 1;
							OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count += 1;

							OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity += Quantity;
							// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;

							if(Price >= OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1]){
								for(sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
									if(OrderBookPtr->OrderBook.BuyPrice[sx]==Price){
										OrderBookPtr->OrderBook.BuyQty[sx] += Quantity;
										OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
										OrderBookPtr->OrderBook.LevelChangeIndex = sx;
										break;
									}else if(Price > OrderBookPtr->OrderBook.BuyPrice[sx]){
										ShiftPosition = sx;
										for(sx=FEED_LEVEL_DEPTH_1; sx>ShiftPosition; --sx){
											OrderBookPtr->OrderBook.BuyPrice[sx] 	= OrderBookPtr->OrderBook.BuyPrice[sx-1];
											OrderBookPtr->OrderBook.BuyQty[sx] = OrderBookPtr->OrderBook.BuyQty[sx-1];
											OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->OrderBook.NoOfBuyOrds[sx-1];
										}
										OrderBookPtr->OrderBook.BuyPrice[sx] = Price;
										OrderBookPtr->OrderBook.BuyQty[sx] = Quantity;
										OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
										OrderBookPtr->OrderBook.LevelChangeIndex = sx;
										break;
									}
								}
								OrderBookPtr->OrderBook.FeedEventReason = 0;
								OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
								OrderBookUpdated = 1;
							}
						}
						else if(OrderType==2){	// SELL


							tptr = &OrderBookPtr->SELL_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable += 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable += 1;
							tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable += 1; tnptr->OneRupee[RupeeDepth].Avaliable += 1;
							OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count += 1;

							OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity += Quantity;
							// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;

							if(Price <= OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] || OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1]==0){
								for(sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
									if(OrderBookPtr->OrderBook.SellPrice[sx]==Price){
										OrderBookPtr->OrderBook.SellQty[sx] += Quantity;
										OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
										OrderBookPtr->OrderBook.LevelChangeIndex = sx;
										break;
									}else if(Price < OrderBookPtr->OrderBook.SellPrice[sx]){
										ShiftPosition = sx;
										for(sx=FEED_LEVEL_DEPTH_1; sx>ShiftPosition; --sx){
											OrderBookPtr->OrderBook.SellPrice[sx] 	= OrderBookPtr->OrderBook.SellPrice[sx-1];
											OrderBookPtr->OrderBook.SellQty[sx] = OrderBookPtr->OrderBook.SellQty[sx-1];
											OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->OrderBook.NoOfSellOrds[sx-1];
										}
										OrderBookPtr->OrderBook.SellPrice[sx] = Price;
										OrderBookPtr->OrderBook.SellQty[sx] = Quantity;
										OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
										OrderBookPtr->OrderBook.LevelChangeIndex = sx;
										break;
									}
									else if(OrderBookPtr->OrderBook.SellPrice[sx]==0){
										OrderBookPtr->OrderBook.SellPrice[sx] = Price;
										OrderBookPtr->OrderBook.SellQty[sx] = Quantity;
										OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
										OrderBookPtr->OrderBook.LevelChangeIndex = sx;
										break;
									}
								}
								OrderBookPtr->OrderBook.FeedEventReason = 0;
								OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
								OrderBookUpdated = 1;
							}
						}
					}
				}
			}
			else if(Header->TemplateID == 13102){
				OrderDelete *ord = (OrderDelete*)StreamBufferPtr;
				std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(ord->SecurityID);
				// fprintf(pData->feed_log,"[%d] Delete\t=> %u, %ld, %d, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
				if(tITR != pData->OrderBook.end())
				{
					// fprintf(pData->feed_log,"[%d] Delete\t=> %u, %ld, %d, %ld, %ld, %ld, %ld,  \n", Product->ProductID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->DisplayQty, ord->TrdRegTSTimeIn, ord->TrdRegTSTimePriority);
					int32_t Price = ord->Price / MCX_PRICE_MULTIPLIER, Quantity = ord->DisplayQty / 10000, OrderType = ord->Side, ShiftPosition=0, sx=0;
					Token = ord->SecurityID;
					OrderBookPtr = tITR->second;
					// {
						// PrintNodes(pData->feed_log, OrderBookPtr, 0, 1); PrintNodes(pData->feed_log, OrderBookPtr, 0, 2);
					// }
					int DPR_Gap = ((Price - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE)/OrderBookPtr->TickSize);;
					int Depth = (DPR_Gap/OrderBookPtr->PaisaDepthRange);
					int ThousandDepth = Depth/1000;
					int TenRupeeDepth = Depth - (ThousandDepth*1000);
					int HundredDepth = (TenRupeeDepth)/100;
					int RupeeDepth = TenRupeeDepth - (HundredDepth*100);
					TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
					if(OrderType == 1){		// BUY

						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity -= Quantity;
						// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;
						tptr = &OrderBookPtr->BUY_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable -= 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable -= 1;
						tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable -= 1; tnptr->OneRupee[RupeeDepth].Avaliable -= 1;
						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count -= 1;

						if(Price >= OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1]){
							for(sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
								if(OrderBookPtr->OrderBook.BuyPrice[sx]==Price){
									OrderBookPtr->OrderBook.BuyQty[sx] -= Quantity;
									OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 2;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookUpdated = 1;
									if(OrderBookPtr->OrderBook.BuyQty[sx]==0){
										ShiftPosition = 1;
										for(; sx<FEED_LEVEL_DEPTH_1; ++sx){
											OrderBookPtr->OrderBook.BuyPrice[sx] 	= OrderBookPtr->OrderBook.BuyPrice[sx+1];
											OrderBookPtr->OrderBook.BuyQty[sx]	= OrderBookPtr->OrderBook.BuyQty[sx+1];
											OrderBookPtr->OrderBook.NoOfBuyOrds[sx]	= OrderBookPtr->OrderBook.NoOfBuyOrds[sx+1];
											if(OrderBookPtr->OrderBook.BuyPrice[sx+1]==0){ShiftPosition=0;break;}
										}
										if(ShiftPosition){
											int32_t Depth = (OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
											OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.BuyQty[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.NoOfBuyOrds[FEED_LEVEL_DEPTH_1] = 0;
											int32_t PaisaDepth = ((Depth/OrderBookPtr->TickSize)%(OrderBookPtr->PaisaDepthRange))-1; Depth /=(OrderBookPtr->TickSize*OrderBookPtr->PaisaDepthRange);
											int32_t ThousandDepth = Depth/1000;
											int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
											int32_t HundredDepth = (TenRupeeDepth)/100;
											int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
											TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
											//cout << "Ord Update UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
											int32_t L1=ThousandDepth;
											for(ORDER_BOOK_THOUSAND_RANGE *L1_PTR = &OrderBookPtr->BUY_INFO_TILL_RUPEE[ThousandDepth]; L1>=0; --L1, --L1_PTR){
												if(L1_PTR->Avaliable){
													int32_t L2=HundredDepth;
													for(ORDER_BOOK_HUNDRED_RANGE *L2_PTR = &L1_PTR->Hundred[HundredDepth]; L2>=0; --L2, --L2_PTR){
														if(L2_PTR->Avaliable){
															int32_t L3=TenRupeeDepth;
															for(ORDER_BOOK_TEN_RUPEE_RANGE *L3_PTR = &L2_PTR->TenRupee[TenRupeeDepth]; L3>=0; --L3, --L3_PTR){
																if(L3_PTR->Avaliable){
																	int32_t L4=RupeeDepth;
																	for(ORDER_BOOK_RUPEE_RANGE *L4_PTR = &L3_PTR->OneRupee[RupeeDepth]; L4>=0; --L4, --L4_PTR){
																		if(L4_PTR->Avaliable){
																			//P1 = (L1*1000); P2 = P1 + (L2*100); P3 = P2 + (L3*10); sPrice = ((P3 + L4)*20);
																			int32_t sPrice = ((((((L1*1000)) + (L2*100)) + (L3*10)) + L4)*OrderBookPtr->PaisaDepthRange);
																			for(int32_t L5=PaisaDepth; L5>=0; --L5){
																				if(OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
																					OrderBookPtr->OrderBook.BuyQty[FEED_LEVEL_DEPTH_1] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
																					OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] = (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
																					OrderBookPtr->OrderBook.NoOfBuyOrds[FEED_LEVEL_DEPTH_1] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count;
																					goto TOKEN_UPDATED;
																				}
																			}
																		}
																		PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
																	}
																}
																RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
															}
														}
														TenRupeeDepth = 9; RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
													}
												}
												HundredDepth = 9; TenRupeeDepth = 9; RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
											}
										}
									}
									break;
								}
							}
						}
					}
					else if(OrderType == 2){	// SELL

						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity -= Quantity;
						// cout << __LINE__ << " DPR_Gap: " << DPR_Gap << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity << endl;
						tptr = &OrderBookPtr->SELL_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable -= 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable -= 1;
						tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable -= 1; tnptr->OneRupee[RupeeDepth].Avaliable -= 1;
						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count -= 1;

						if(Price <= OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] || OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1]==0){
							for(sx=0; sx<FEED_LEVEL_DEPTH; sx++){
								if(OrderBookPtr->OrderBook.SellPrice[sx]==Price){
									OrderBookPtr->OrderBook.SellQty[sx] -= Quantity;
									OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 2;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookUpdated = 1;
									if(OrderBookPtr->OrderBook.SellQty[sx]==0){
										ShiftPosition = 1;
										for(; sx<FEED_LEVEL_DEPTH_1; sx++){
											OrderBookPtr->OrderBook.SellPrice[sx] 		= OrderBookPtr->OrderBook.SellPrice[sx+1];
											OrderBookPtr->OrderBook.SellQty[sx]	= OrderBookPtr->OrderBook.SellQty[sx+1];
											OrderBookPtr->OrderBook.NoOfSellOrds[sx]	= OrderBookPtr->OrderBook.NoOfSellOrds[sx+1];
											if(OrderBookPtr->OrderBook.SellPrice[sx+1]==0){ShiftPosition=0;break;}
										}
										if(ShiftPosition){
											int32_t Depth = (OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
											OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.SellQty[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.NoOfSellOrds[FEED_LEVEL_DEPTH_1] = 0;
											int32_t PaisaDepth = ((Depth/OrderBookPtr->TickSize)%(OrderBookPtr->PaisaDepthRange))+1; Depth /=(OrderBookPtr->TickSize*OrderBookPtr->PaisaDepthRange);
											int32_t ThousandDepth = Depth/1000;
											int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
											int32_t HundredDepth = (TenRupeeDepth)/100;
											int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
											TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
											//cout << "Ord Update UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
											int32_t L1=ThousandDepth;
											for(ORDER_BOOK_THOUSAND_RANGE *L1_PTR = &OrderBookPtr->SELL_INFO_TILL_RUPEE[ThousandDepth]; L1<OrderBookPtr->ThousandSize; ++L1,++L1_PTR){
												if(L1_PTR->Avaliable){
													int32_t L2=HundredDepth;
													for(ORDER_BOOK_HUNDRED_RANGE *L2_PTR = &L1_PTR->Hundred[HundredDepth]; L2<10; ++L2,++L2_PTR){
														if(L2_PTR->Avaliable){
															int32_t L3=TenRupeeDepth;
															for(ORDER_BOOK_TEN_RUPEE_RANGE *L3_PTR = &L2_PTR->TenRupee[TenRupeeDepth]; L3<10; ++L3,++L3_PTR){
																if(L3_PTR->Avaliable){
																	int32_t L4=RupeeDepth;
																	for(ORDER_BOOK_RUPEE_RANGE *L4_PTR = &L3_PTR->OneRupee[RupeeDepth]; L4<10; ++L4,++L4_PTR){
																		if(L4_PTR->Avaliable){
																			//P1 = (L1*1000); P2 = P1 + (L2*100); P3 = P2 + (L3*10); sPrice = ((P3 + L4)*20);
																			int32_t sPrice = ((((((L1*1000)) + (L2*100)) + (L3*10)) + L4)*OrderBookPtr->PaisaDepthRange);
																			for(int32_t L5=PaisaDepth; L5<OrderBookPtr->PaisaDepthRange; ++L5){
																				if(OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
																					OrderBookPtr->OrderBook.SellQty[FEED_LEVEL_DEPTH_1] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
																					OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] = (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
																					OrderBookPtr->OrderBook.NoOfSellOrds[FEED_LEVEL_DEPTH_1] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count;
																					goto TOKEN_UPDATED;
																				}
																			}
																		}
																		PaisaDepth = 0;
																	}
																}
																RupeeDepth = 0; PaisaDepth = 0;
															}
														}
														TenRupeeDepth = 0; RupeeDepth = 0; PaisaDepth = 0;
													}
												}
												HundredDepth = 0; TenRupeeDepth = 0; RupeeDepth = 0; PaisaDepth = 0;
											}
										}
									}
									break;
								}
							}
						}
					}
				}
			}

			// else if(Header->TemplateID == 13202){
			// 	ExecutionSummary *ord = (ExecutionSummary*)StreamBufferPtr;
			// 	Token = ord->SecurityID;
			// 	std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(ord->SecurityID);
			// 	// fprintf(pData->feed_log,"ExecutionSummary[%d]\t=> %u, %ld, %d, %ld, %ld, %ld, %ld, %d,  \n", Product->ProductID,  ord->Header.MsgSeqNum, ord->SecurityID, ord->AggressorSide, ord->LastPx, ord->LastQty, ord->RestingHiddenQty, ord->RestingClxQty, ord->TradeCOndition);
			// 	if(tITR != pData->OrderBook.end())
			// 	{
			// 		fprintf(pData->feed_log,"ExecutionSummary[%d]\t=> %u, %ld, %d, %ld, %ld, %ld, %ld, %d,  \n", Product->ProductID,  ord->Header.MsgSeqNum, ord->SecurityID, ord->AggressorSide, ord->LastPx, ord->LastQty, ord->RestingHiddenQty, ord->RestingClxQty, ord->TradeCOndition);
			// 		int32_t Price = ord->LastPx / MCX_PRICE_MULTIPLIER, Quantity = ord->LastQty / 10000, OrderType = ord->AggressorSide;
			// 		TOKEN_ORDER_BOOK  *OrderBookPtr = tITR->second;
			// 		// {
			// 			PrintNodes(pData->feed_log, OrderBookPtr, 0, 1); PrintNodes(pData->feed_log, OrderBookPtr, 0, 2);
			// 		// }
			// 		if(OrderType == 2){
			// 			int8_t fillOB = -1; OrderBookUpdated = 1;
			// 			for(int32_t L1=OrderBookPtr->ThousandSize - 1; L1>=0; L1--){
			// 				if(OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Avaliable){
			// 					int32_t P1 = (L1*1000);
			// 					for(int32_t L2=9; L2>=0; L2--){
			// 						if(OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
			// 							int32_t P2 = P1 + (L2*100);
			// 							for(int32_t L3=9; L3>=0; L3--){
			// 								if(OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
			// 									int32_t P3 = P2 + (L3*10);
			// 									for(int32_t L4=9; L4>=0; L4--){
			// 										if(OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
			// 											int32_t sPrice = ((P3 + L4)*OrderBookPtr->PaisaDepthRange);
			// 											for(int32_t L5=(OrderBookPtr->PaisaDepthRange-1); L5>=0; L5--){
			// 												if(OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
			// 													//fprintf(ptr->feedLog," %d:%d:%d:%d::%d => Price:%d Qty:%d\n",L1,L2,L3,L4,L5,(OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize)),OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity);
			// 													if(fillOB == -1){
			// 														if(Price <= (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize))){
			// 															if(Quantity <= OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
			// 																OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity -= Quantity;
			// 																// cout << __LINE__ << " DPR_Gap: " << sPrice+L5 << " Qty: " << OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[ sPrice+L5].Quantity << endl;
			// 																Quantity = 0;
			// 															}
			// 															else{
			// 																Quantity -= OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
			// 																OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity = 0;
			// 															}

			// 															if(OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity < 1){
			// 																OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Avaliable -= 1;
			// 																OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable -= 1;
			// 																OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable -= 1;
			// 																OrderBookPtr->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable -= 1;
			// 																OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count = 0;
			// 															}

			// 															if(Quantity < 1){
			// 																for(int sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
			// 																	OrderBookPtr->OrderBook.BuyPrice[sx] 	= 0;
			// 																	OrderBookPtr->OrderBook.BuyQty[sx]	= 0;
			// 																	OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = 0;
			// 																}
			// 																L5 +=1;
			// 																fillOB = 0;
			// 															}
			// 														}
			// 														else{
			// 															if(Quantity){
			// 																for(int sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
			// 																	OrderBookPtr->OrderBook.BuyPrice[sx] 	= 0;
			// 																	OrderBookPtr->OrderBook.BuyQty[sx]	= 0;
			// 																	OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = 0;
			// 																}
			// 																L5 +=1;
			// 																fillOB = 0;
			// 															}
			// 														}
			// 													}
			// 													else{
			// 														OrderBookPtr->OrderBook.BuyPrice[fillOB] 	= (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
			// 														OrderBookPtr->OrderBook.BuyQty[fillOB]	= OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
			// 														// OrderBookPtr->OrderBook.NoOfBuyOrds[fillOB]	= OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].NoOfBuyOrds;
			// 														++fillOB; if(fillOB >= FEED_LEVEL_DEPTH){goto TOKEN_UPDATED;}
			// 													}
			// 												}
			// 											}
			// 										}
			// 									}
			// 								}
			// 							}
			// 						}
			// 					}
			// 				}
			// 			}
			// 		}
			// 		else if(OrderType == 1){
			// 			int8_t fillOB = -1; OrderBookUpdated = 1;
			// 			for(int32_t L1=0; L1<OrderBookPtr->ThousandSize; L1++){
			// 				if(OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Avaliable){
			// 					int32_t P1 = (L1*1000);
			// 					for(int32_t L2=0; L2<10; L2++){
			// 						if(OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
			// 							int32_t P2 = P1 + (L2*100);
			// 							for(int32_t L3=0; L3<10; L3++){
			// 								if(OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
			// 									int32_t P3 = P2 + (L3*10);
			// 									for(int32_t L4=0; L4<10; L4++){
			// 										if(OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
			// 											int32_t sPrice = ((P3 + L4)*OrderBookPtr->PaisaDepthRange);
			// 											for(int32_t L5=0; L5<OrderBookPtr->PaisaDepthRange; L5++){
			// 												if(OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
			// 													//fprintf(ptr->feedLog," %d:%d:%d:%d::%d => Price:%d Qty:%d\n",L1,L2,L3,L4,L5, (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize)), OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity);
			// 													if(fillOB == -1){
			// 														if(Price >= (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize))){
			// 															if(Quantity <= OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
			// 																OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity -= Quantity;
			// 																Quantity = 0;
			// 															}
			// 															else{
			// 																Quantity -= OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
			// 																OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity = 0;
			// 															}

			// 															if(OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity < 1){
			// 																OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Avaliable -= 1;
			// 																OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable -= 1;
			// 																OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable -= 1;
			// 																OrderBookPtr->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable -= 1;
			// 																OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count = 0;
			// 															}

			// 															if(Quantity < 1){
			// 																for(int sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
			// 																	OrderBookPtr->OrderBook.SellPrice[sx] 	= 0;
			// 																	OrderBookPtr->OrderBook.SellQty[sx]	= 0;
			// 																	OrderBookPtr->OrderBook.NoOfSellOrds[sx] = 0;
			// 																}
			// 																L5 -=1;
			// 																fillOB = 0;
			// 															}
			// 														}
			// 														else{
			// 															if(Quantity){
			// 																for(int sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
			// 																	OrderBookPtr->OrderBook.SellPrice[sx] 	= 0;
			// 																	OrderBookPtr->OrderBook.SellQty[sx]	= 0;
			// 																	OrderBookPtr->OrderBook.NoOfSellOrds[sx] = 0;
			// 																}
			// 																L5 -=1;
			// 																fillOB = 0;
			// 															}
			// 														}
			// 													}
			// 													else{
			// 														OrderBookPtr->OrderBook.SellPrice[fillOB] 	= (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
			// 														OrderBookPtr->OrderBook.SellQty[fillOB]	= OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
			// 														// OrderBookPtr->OrderBook.NoOfSellOrds[fillOB]	= OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].NoOfSellOrds;
			// 														++fillOB; if(fillOB >= FEED_LEVEL_DEPTH){goto TOKEN_UPDATED;}
			// 													}
			// 												}
			// 											}
			// 										}
			// 									}
			// 								}
			// 							}
			// 						}
			// 					}
			// 				}
			// 			}
			// 		}
			// 	}
			// }

			else if(Header->TemplateID == 13104 || Header->TemplateID == 13105){
				Trade *ord = (Trade*)StreamBufferPtr;
				std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(ord->SecurityID);
				// fprintf(pData->feed_log,"Trade[%d]\t %d => %u, %ld, %d, %ld, %ld, %ld, %ld,  \n", Product->ProductID, Header->TemplateID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->LastPx, ord->LastQty, ord->TransactTime);
				if(tITR != pData->OrderBook.end())
				{
					// fprintf(pData->feed_log,"Trade[%d]\t %d => %u, %ld, %d, %ld, %ld, %ld, %ld,  \n", Product->ProductID, Header->TemplateID, ord->Header.MsgSeqNum, ord->SecurityID, ord->Side, ord->Price, ord->LastPx, ord->LastQty, ord->TransactTime);
					int32_t Price = ord->LastPx / MCX_PRICE_MULTIPLIER, Quantity = ord->LastQty / 10000, OrderType = ord->Side;
					Token = ord->SecurityID;
					OrderBookPtr = tITR->second;
					// {
						// PrintNodes(pData->feed_log, OrderBookPtr, 0, 1); PrintNodes(pData->feed_log, OrderBookPtr, 0, 2);
					// }

					int DPR_Gap = ((Price - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE)/OrderBookPtr->TickSize);;
					int Depth = (DPR_Gap/OrderBookPtr->PaisaDepthRange);
					int ThousandDepth = Depth/1000;
					int TenRupeeDepth = Depth - (ThousandDepth*1000);
					int HundredDepth = (TenRupeeDepth)/100;
					int RupeeDepth = TenRupeeDepth - (HundredDepth*100);
					TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);

					if(OrderType == 1){

						if(Header->TemplateID == 13104){
							tptr = &OrderBookPtr->BUY_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable -= 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable -= 1;
							tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable -= 1; tnptr->OneRupee[RupeeDepth].Avaliable -= 1;
							OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count -= 1;
						}
						OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity -= Quantity;

						if(Price >= OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1]){
							for(sx=0; sx<FEED_LEVEL_DEPTH; sx++){
								if(OrderBookPtr->OrderBook.BuyPrice[sx]==Price){
									OrderBookPtr->OrderBook.BuyQty[sx] -= Quantity;
									OrderBookPtr->OrderBook.NoOfBuyOrds[sx] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 3;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookPtr->OrderBook.LastTradedPrice = Price;
									OrderBookPtr->OrderBook.LastTradedQty = Quantity;
									OrderBookUpdated = 1;
									if(OrderBookPtr->OrderBook.BuyQty[sx]==0){
										ShiftPosition = 1;
										for(; sx<FEED_LEVEL_DEPTH_1; sx++){
											OrderBookPtr->OrderBook.BuyPrice[sx] 	= OrderBookPtr->OrderBook.BuyPrice[sx+1];
											OrderBookPtr->OrderBook.BuyQty[sx]	= OrderBookPtr->OrderBook.BuyQty[sx+1];
											OrderBookPtr->OrderBook.NoOfBuyOrds[sx]	= OrderBookPtr->OrderBook.NoOfBuyOrds[sx+1];
											if(OrderBookPtr->OrderBook.BuyPrice[sx+1]==0){ShiftPosition=0;break;}
										}
										if(ShiftPosition){
											int32_t Depth = (OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
											OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.BuyQty[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.NoOfBuyOrds[FEED_LEVEL_DEPTH_1] = 0;
											int32_t PaisaDepth = ((Depth/OrderBookPtr->TickSize)%(OrderBookPtr->PaisaDepthRange))-1; Depth /=(OrderBookPtr->TickSize*OrderBookPtr->PaisaDepthRange);
											int32_t ThousandDepth = Depth/1000;
											int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
											int32_t HundredDepth = (TenRupeeDepth)/100;
											int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
											TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
											//cout << "Ord Update UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
											int32_t L1=ThousandDepth;
											for(ORDER_BOOK_THOUSAND_RANGE *L1_PTR = &OrderBookPtr->BUY_INFO_TILL_RUPEE[ThousandDepth]; L1>=0; --L1, --L1_PTR){
												if(L1_PTR->Avaliable){
													int32_t L2=HundredDepth;
													for(ORDER_BOOK_HUNDRED_RANGE *L2_PTR = &L1_PTR->Hundred[HundredDepth]; L2>=0; --L2, --L2_PTR){
														if(L2_PTR->Avaliable){
															int32_t L3=TenRupeeDepth;
															for(ORDER_BOOK_TEN_RUPEE_RANGE *L3_PTR = &L2_PTR->TenRupee[TenRupeeDepth]; L3>=0; --L3, --L3_PTR){
																if(L3_PTR->Avaliable){
																	int32_t L4=RupeeDepth;
																	for(ORDER_BOOK_RUPEE_RANGE *L4_PTR = &L3_PTR->OneRupee[RupeeDepth]; L4>=0; --L4, --L4_PTR){
																		if(L4_PTR->Avaliable){
																			//P1 = (L1*1000); P2 = P1 + (L2*100); P3 = P2 + (L3*10); sPrice = ((P3 + L4)*20);
																			int32_t sPrice = ((((((L1*1000)) + (L2*100)) + (L3*10)) + L4)*OrderBookPtr->PaisaDepthRange);
																			for(int32_t L5=PaisaDepth; L5>=0; --L5){
																				if(OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
																					OrderBookPtr->OrderBook.BuyQty[FEED_LEVEL_DEPTH_1] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
																					OrderBookPtr->OrderBook.BuyPrice[FEED_LEVEL_DEPTH_1] = (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
																					OrderBookPtr->OrderBook.NoOfBuyOrds[FEED_LEVEL_DEPTH_1] = OrderBookPtr->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count;
																					goto TOKEN_UPDATED;
																				}
																			}
																		}
																		PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
																	}
																}
																RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
															}
														}
														TenRupeeDepth = 9; RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
													}
												}
												HundredDepth = 9; TenRupeeDepth = 9; RupeeDepth = 9; PaisaDepth = (OrderBookPtr->PaisaDepthRange-1);
											}
										}
									}
									break;
								}
							}
						}
					}
					else if(OrderType == 2){
						if(Header->TemplateID == 13104){
							tptr = &OrderBookPtr->SELL_INFO_TILL_RUPEE[ThousandDepth]; tptr->Avaliable -= 1; hptr = &tptr->Hundred[HundredDepth]; hptr->Avaliable -= 1;
							tnptr = &hptr->TenRupee[TenRupeeDepth]; tnptr->Avaliable -= 1; tnptr->OneRupee[RupeeDepth].Avaliable -= 1;
							OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count -= 1;
						}
						OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Quantity -= Quantity;

						if(Price <= OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] || OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1]==0){
							for(int32_t sx=0; sx<FEED_LEVEL_DEPTH; ++sx){
								if(OrderBookPtr->OrderBook.SellPrice[sx]==Price){
									OrderBookPtr->OrderBook.SellQty[sx] -= Quantity;
									OrderBookPtr->OrderBook.NoOfSellOrds[sx] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[DPR_Gap].Count;
									OrderBookPtr->OrderBook.FeedEventReason = 3;
									OrderBookPtr->OrderBook.FeedEventAggressor = ord->Side;
									OrderBookPtr->OrderBook.LevelChangeIndex = sx;
									OrderBookPtr->OrderBook.LastTradedPrice = Price;
									OrderBookPtr->OrderBook.LastTradedQty = Quantity;
									OrderBookUpdated = 1;
									if(OrderBookPtr->OrderBook.SellQty[sx]==0){
										ShiftPosition = 1;
										for(; sx<FEED_LEVEL_DEPTH_1; ++sx){
											OrderBookPtr->OrderBook.SellPrice[sx] 		= OrderBookPtr->OrderBook.SellPrice[sx+1];
											OrderBookPtr->OrderBook.SellQty[sx]	= OrderBookPtr->OrderBook.SellQty[sx+1];
											OrderBookPtr->OrderBook.NoOfSellOrds[sx]	= OrderBookPtr->OrderBook.NoOfSellOrds[sx+1];
											if(OrderBookPtr->OrderBook.SellPrice[sx+1]==0){ShiftPosition=0;break;}
										}
										if(ShiftPosition){
											int32_t Depth = (OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] - OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE);
											OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.SellQty[FEED_LEVEL_DEPTH_1] = 0;
											OrderBookPtr->OrderBook.NoOfSellOrds[FEED_LEVEL_DEPTH_1] = 0;
											int32_t PaisaDepth = ((Depth/OrderBookPtr->TickSize)%(OrderBookPtr->PaisaDepthRange))+1; Depth /=(OrderBookPtr->TickSize*OrderBookPtr->PaisaDepthRange); //PaisaDepth =((Depth%100)/5)+1;
											int32_t ThousandDepth = Depth/1000;
											int32_t TenRupeeDepth = Depth - (ThousandDepth*1000);
											int32_t HundredDepth = (TenRupeeDepth)/100;
											int32_t RupeeDepth = TenRupeeDepth - (HundredDepth*100);
											TenRupeeDepth = (RupeeDepth)/10; RupeeDepth = RupeeDepth - (TenRupeeDepth*10);
											//cout << "Ord Update UP=> Price:"<< Price << " ThousandDepth:" << ThousandDepth << " HundredDepth:"<< HundredDepth << " TenRupeeDepth:" << TenRupeeDepth << " RupeeDepth:" << RupeeDepth<< endl;
											int32_t L1=ThousandDepth;
											for(ORDER_BOOK_THOUSAND_RANGE *L1_PTR = &OrderBookPtr->SELL_INFO_TILL_RUPEE[ThousandDepth]; L1<OrderBookPtr->ThousandSize; ++L1,++L1_PTR){
												if(L1_PTR->Avaliable){
													int32_t L2=HundredDepth;
													for(ORDER_BOOK_HUNDRED_RANGE *L2_PTR = &L1_PTR->Hundred[HundredDepth]; L2<10; ++L2,++L2_PTR){
														if(L2_PTR->Avaliable){
															int32_t L3=TenRupeeDepth;
															for(ORDER_BOOK_TEN_RUPEE_RANGE *L3_PTR = &L2_PTR->TenRupee[TenRupeeDepth]; L3<10; ++L3,++L3_PTR){
																if(L3_PTR->Avaliable){
																	int32_t L4=RupeeDepth;
																	for(ORDER_BOOK_RUPEE_RANGE *L4_PTR = &L3_PTR->OneRupee[RupeeDepth]; L4<10; ++L4,++L4_PTR){
																		if(L4_PTR->Avaliable){
																			//P1 = (L1*1000); P2 = P1 + (L2*100); P3 = P2 + (L3*10); sPrice = ((P3 + L4)*20);
																			int32_t sPrice = ((((((L1*1000)) + (L2*100)) + (L3*10)) + L4)*OrderBookPtr->PaisaDepthRange);
																			for(int32_t L5=PaisaDepth; L5<OrderBookPtr->PaisaDepthRange; ++L5){
																				if(OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
																					OrderBookPtr->OrderBook.SellQty[FEED_LEVEL_DEPTH_1] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity;
																					OrderBookPtr->OrderBook.SellPrice[FEED_LEVEL_DEPTH_1] = (OrderBookPtr->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPtr->TickSize));
																					OrderBookPtr->OrderBook.NoOfSellOrds[FEED_LEVEL_DEPTH_1] = OrderBookPtr->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count;
																					goto TOKEN_UPDATED;
																				}
																			}
																		}
																		PaisaDepth = 0;
																	}
																}
																RupeeDepth = 0; PaisaDepth = 0;
															}
														}
														TenRupeeDepth = 0; RupeeDepth = 0; PaisaDepth = 0;
													}
												}
												HundredDepth = 0; TenRupeeDepth = 0; RupeeDepth = 0; PaisaDepth = 0;
											}
										}
									}
									break;
								}
							}
						}
					}
				}
			}

			else if(Header->TemplateID == 13103){
				OrderMassDelete *ord = (OrderMassDelete*)StreamBufferPtr; Token = ord->SecurityID;
				// fprintf(pData->feed_log,"OrderMassDelete sec: %d,%d,\n", ord->SecurityID,Header->MsgSeqNum);
				std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(ord->SecurityID);
				if(tITR != pData->OrderBook.end()){
					OrderBookPtr = tITR->second;
					fprintf(pData->feed_log,"OrderMassDelete sec: %d,%d,\n", ord->SecurityID,Header->MsgSeqNum);
					Purger_Market_Depth(OrderBookPtr , 0); OrderBookUpdated = 1;
				}
			}
			else{
				// fprintf(pData->feed_log,"TemplateID:%d,%d\n",Header->TemplateID,Header->MsgSeqNum);
			}
			// Product->CurrentSequence = Header->MsgSeqNum;
		}

		TOKEN_UPDATED:

		ProcessedMsgLen += Header->BodyLen; StreamBufferPtr += Header->BodyLen; 
		// pData->Processed_Bytes += Header->BodyLen;
	}

		
		// std::tr1::unordered_map<uint32_t,TOKEN_ORDER_BOOK *>::iterator tITR = pData->OrderBook.find(Token);
			if(/*ORDER->ActivePassive_Flg == 0 && */ OrderBookUpdated /*&& OrderBookPtr->OrderBook.OrdType == 'T'*/ && OrderBookPtr != NULL/*tITR != pData->OrderBook.end()*/){
				// TOKEN_ORDER_BOOK  *OrderBookPtr = tITR->second;
				//if(ORDER->UniqueIdentifier == 211500  && ORDER->SequenceNo >= 1337950 && ORDER->SequenceNo <= 1338155)
//				{
				if((OrderBookPtr->OrderBook.SellPrice[0] == 0 || (OrderBookPtr->OrderBook.SellPrice[0] && (OrderBookPtr->OrderBook.BuyPrice[0] < OrderBookPtr->OrderBook.SellPrice[0]))))
				{

				fprintf(pData->feed_log,"\n## Token:%u : (%d,%d) LTP:%0.2f, %d, %ld ##\n", Token, OrderBookPtr->OrderBook.FeedEventReason, OrderBookPtr->OrderBook.FeedEventAggressor,
							((double)OrderBookPtr->OrderBook.LastTradedPrice/(double)100.00), OrderBookPtr->OrderBook.LastTradedQty, *timestamp);
					for(int p=0;p<FEED_LEVEL_DEPTH;p++){
						fprintf(pData->feed_log,"[%u]--%u--%0.2f \t\t %0.2f--%u--[%u]\n",OrderBookPtr->OrderBook.NoOfBuyOrds[p], OrderBookPtr->OrderBook.BuyQty[p], ((double)OrderBookPtr->OrderBook.BuyPrice[p]/(double)100.00),
								((double)OrderBookPtr->OrderBook.SellPrice[p]/(double)100.00), OrderBookPtr->OrderBook.SellQty[p], OrderBookPtr->OrderBook.NoOfSellOrds[p] );
					}

				// fprintf(pData->feed_log,"\n## Token:%u : (%d,%d) LTP:%0.2f, %d, %ld ##\n", Token, OrderBookPtr->OrderBook.FeedEventReason, OrderBookPtr->OrderBook.FeedEventAggressor,
				// 			((double)OrderBookPtr->OrderBook.LastTradedPrice/(double)100.00), OrderBookPtr->OrderBook.LastTradedQty, *timestamp);
				// 	for(int p=0;p<FEED_LEVEL_DEPTH;p++){
				// 		fprintf(pData->feed_log,"%u--%0.2f \t\t %0.2f--%u\n", OrderBookPtr->OrderBook.BuyQty[p], ((double)OrderBookPtr->OrderBook.BuyPrice[p]/(double)100.00),
				// 				((double)OrderBookPtr->OrderBook.SellPrice[p]/(double)100.00), OrderBookPtr->OrderBook.SellQty[p]);
				// 	}
				

				// for(int Depth=0;Depth<FEED_LEVEL_DEPTH;Depth++){
				// 	if((OrderBookPtr->OrderBook.BuyPrice[Depth] && OrderBookPtr->OrderBook.BuyQty[Depth]==0) || (OrderBookPtr->OrderBook.SellPrice[Depth] && OrderBookPtr->OrderBook.SellQty[Depth]==0)
				// 			|| (OrderBookPtr->OrderBook.BuyPrice[Depth] && OrderBookPtr->OrderBook.SellPrice[Depth] && OrderBookPtr->OrderBook.BuyPrice[Depth] > OrderBookPtr->OrderBook.SellPrice[Depth])){
				// 		PrintNodes(stdout, OrderBookPtr, 0, 1); PrintNodes(stdout, OrderBookPtr, 0, 2); fprintf(stdout, "\n\n");
				// 		fprintf(stdout, "FEED ERROR EXIT\n\n");printf("Line => %d\n", __LINE__);exit(0);
				// 	}
				// }

				Token_Data fdata;
				fdata.Token = Token;
				fdata.Timestamp = *timestamp;
				// fdata.ExchgTimestamp = ord->LastUpdatedTime - 19800;
				//fdata.OpenInterest = OrderBookPtr->OrderBook.OpenInterest;
				fdata.FeedEventReason = OrderBookPtr->OrderBook.FeedEventReason;
				fdata.FeedEventAggressor = OrderBookPtr->OrderBook.FeedEventAggressor;
				fdata.LevelChangeIndex = OrderBookPtr->OrderBook.LevelChangeIndex;
				fdata.LastTradedPrice = OrderBookPtr->OrderBook.LastTradedPrice;
				fdata.LastTradedQty = OrderBookPtr->OrderBook.LastTradedQty;
				fdata.Delayed = 0;
				#ifdef PACKET_SEQ_NO_ENABLED
					fdata.StreamID = pData->StreamID;
					// fdata.PacketSeqNo = ORDER->SequenceNo;
				#endif

				for(int p=0;p<FEED_LEVEL_DEPTH;p++){
					fdata.BuyPrice[p] = OrderBookPtr->OrderBook.BuyPrice[p];
					fdata.BuyQty[p] = OrderBookPtr->OrderBook.BuyQty[p];
					fdata.NoOfBuyOrds[p] = OrderBookPtr->OrderBook.NoOfBuyOrds[p];
					fdata.SellPrice[p] = OrderBookPtr->OrderBook.SellPrice[p];
					fdata.SellQty[p] = OrderBookPtr->OrderBook.SellQty[p];
					fdata.NoOfSellOrds[p] = OrderBookPtr->OrderBook.NoOfSellOrds[p];
				}
				feed_func(EXCHG_MCX, &fdata);
			}
			}
			
	no_action:

	return 0;
}

int MCX_Feeder::Start_FileReplay(void)
{
    int64_t Timestamp[TotalNodes]; for(unsigned int nav=0; nav<TotalNodes; ++nav)Timestamp[nav] = 0;
    //setvbuf(stdout, NULL,_IONBF, 0);

    while(1){
    	//if(TotalNodes > 1)
    	{
    		int64_t min_time = 0x7FFFFFFFFFFFFFFF; int32_t nav = -1; DATA_HOLDER *pData = NULL;
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

								if(pData->MSG_LEN==0 && (pData->sz - pData->Processed_Bytes) >=(5 + 8)){
									pData->Timestamp = *(int64_t *)(pData->Buffer + pData->Processed_Bytes); if(*(int64_t *)(pData->Buffer + pData->Processed_Bytes)==0)pData->Timestamp=1;
									char len[10]; memcpy(len, (pData->Buffer + pData->Processed_Bytes + 8), 5);
									pData->MSG_LEN = atoi(len); pData->Processed_Bytes += (5 + 8);
								}
								else if(pData->MSG_LEN==0){
									pData->Pos = pData->sz; pData->BytesToRead = ((5 + 8) - (pData->sz - pData->Processed_Bytes));
									//printf("---- [ Length Alignment Started => pData->sz:%d pData->BytesToRead:%d MsgLength:%d pData->Processed_Bytes:%d ]---- \n",pData->sz,pData->BytesToRead,pData->MSG_LEN,pData->Processed_Bytes); fflush((FILE *)stdout);
									goto beginRead;
								}
								if((pData->MSG_LEN+pData->Processed_Bytes) > pData->sz){
									pData->Pos = pData->sz; pData->BytesToRead = (pData->MSG_LEN - (pData->sz - pData->Processed_Bytes));
									//printf("---- [ Byte Alignment Started => pData->sz:%d pData->BytesToRead:%d MsgLength:%d pData->Processed_Bytes:%d ]---- \n",pData->sz,pData->BytesToRead,pData->MSG_LEN,pData->Processed_Bytes); fflush((FILE *)stdout);
									goto beginRead;
								}

								if(Timestamp[nav] == 0){
									Timestamp[nav] = pData->Timestamp; goto end;
								}
								//printf("Line:%d\n", __LINE__);
								generate_feed(pData, &pData->Timestamp);

								pData->Processed_Bytes += pData->MSG_LEN; pData->MSG_LEN = 0; if(Timestamp[nav]>0){Timestamp[nav]=0; goto end;}

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
    printf("\nReturning From MCX FilePlay...\n\n");
    return 0;
}


void MCX_Feeder::Purger_Market_Depth(TOKEN_ORDER_BOOK *OrderBookPTR, uint32_t ResetSeq)
{
	int32_t L1=0,L2=0,L3=0,L4=0,L5,P1,P2,P3,sPrice;
	if(OrderBookPTR){
		OrderBookPTR->OrderBook.LastTradedPrice = 0;
		OrderBookPTR->OrderBook.LastTradedQty = 0;
		for(int32_t i=0;i<FEED_LEVEL_DEPTH;i++){
			OrderBookPTR->OrderBook.BuyPrice[i] = 0; OrderBookPTR->OrderBook.BuyQty[i] = 0; OrderBookPTR->OrderBook.NoOfBuyOrds[i] = 0;
			OrderBookPTR->OrderBook.SellPrice[i] = 0; OrderBookPTR->OrderBook.SellQty[i] = 0; OrderBookPTR->OrderBook.NoOfSellOrds[i] = 0;
		}
		//Lock for receiver thread
		OrderBookPTR->CurrentSequence = ResetSeq + 1;
		// unlock
		for(L1=OrderBookPTR->ThousandSize; L1>=0; L1--){
			if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Avaliable){
				OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Avaliable = 0;
				P1 = (L1*1000);
				for(L2=9; L2>=0; L2--){
					if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
						OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable = 0;
						P2 = P1 + (L2*100);
						for(L3=9; L3>=0; L3--){
							if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
								OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable = 0;
								P3 = P2 + (L3*10);
								for(L4=9; L4>=0; L4--){
									if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
										OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable = 0;
										sPrice = ((P3 + L4)*OrderBookPTR->PaisaDepthRange);
										for(L5=(OrderBookPTR->PaisaDepthRange-1); L5>=0; L5--){
											if(OrderBookPTR->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
												OrderBookPTR->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity = 0;
												OrderBookPTR->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count = 0;
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		for(L1=0; L1<OrderBookPTR->ThousandSize; L1++){
			if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Avaliable){
				OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Avaliable = 0;
				P1 = (L1*1000);
				for(L2=0; L2<10; L2++){
					if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
						OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable = 0;
						P2 = P1 + (L2*100);
						for(L3=0; L3<10; L3++){
							if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
								OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable = 0;
								P3 = P2 + (L3*10);
								for(L4=0; L4<10; L4++){
									if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
										OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable = 0;
										sPrice = ((P3 + L4)*OrderBookPTR->PaisaDepthRange);
										for(L5=0; L5<OrderBookPTR->PaisaDepthRange; L5++){
											if(OrderBookPTR->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
												OrderBookPTR->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity = 0;
												OrderBookPTR->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Count = 0;
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
}

void MCX_Feeder::PrintNodes(FILE *fp, TOKEN_ORDER_BOOK *OrderBookPTR, int32_t mode, char BS)
{
	int L1=0,L2=0,L3=0,L4=0,L5,P1,P2,P3,sPrice;
	if(BS==1){
		fprintf(fp,"\n***************** BUY *********************\n");
		for(L1=OrderBookPTR->ThousandSize; L1>=0; L1--){
			if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Avaliable){
				P1 = (L1*1000);
				for(L2=9; L2>=0; L2--){
					if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
						P2 = P1 + (L2*100);
						for(L3=9; L3>=0; L3--){
							if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
								P3 = P2 + (L3*10);
								for(L4=9; L4>=0; L4--){
									if(OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
										fprintf(fp,"# %d:%d:%d:%d #\n",OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Avaliable,OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable,OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable,
												OrderBookPTR->BUY_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable);
										sPrice = ((P3 + L4)*OrderBookPTR->PaisaDepthRange);
										for(L5=(OrderBookPTR->PaisaDepthRange-1); L5>=0; L5--){
											if(OrderBookPTR->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
												fprintf(fp," %d:%d:%d:%d::%d => sPrice:%d [Price:%d Qty:%d]\n",L1,L2,L3,L4,L5,sPrice,(OrderBookPTR->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPTR->TickSize)),OrderBookPTR->BUY_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity);
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		//fprintf(fp,"**************************************\n\n");
	}
	else if(BS==2){
		fprintf(fp,"\n***************** SELL *********************\n");
		for(L1=0; L1<OrderBookPTR->ThousandSize; L1++){
			if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Avaliable){
				P1 = (L1*1000);
				for(L2=0; L2<10; L2++){
					if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable){
						P2 = P1 + (L2*100);
						for(L3=0; L3<10; L3++){
							if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable){
								P3 = P2 + (L3*10);
								for(L4=0; L4<10; L4++){
									if(OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable){
										fprintf(fp,"# %d:%d:%d:%d #\n",OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Avaliable,OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].Avaliable,OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].Avaliable,
												OrderBookPTR->SELL_INFO_TILL_RUPEE[L1].Hundred[L2].TenRupee[L3].OneRupee[L4].Avaliable);
										sPrice = ((P3 + L4)*OrderBookPTR->PaisaDepthRange);
										for(L5=0; L5<OrderBookPTR->PaisaDepthRange; L5++){
											if(OrderBookPTR->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity){
												fprintf(fp," %d:%d:%d:%d::%d => sPrice:%d [Price:%d Qty:%d]\n",L1,L2,L3,L4,L5,sPrice,(OrderBookPTR->BUY_SELL_DPR_RANGE_MIN_PRICE+((sPrice+L5)*OrderBookPTR->TickSize)),OrderBookPTR->SELL_ORDER_PAISA_QUANTITY_INFO[sPrice+L5].Quantity);
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
		//fprintf(fp,"**************************************\n\n");
	}
}

int MCX_Feeder::Reset_Token_Sequence_And_Orderbook(DATA_HOLDER *pData, uint32_t Token) {
	// Read & Init Orderbook
	FILE *fp = NULL;
	if(pData == NULL){
		fp = fopen(BaseFileName.c_str(), "rb");
		if (fp == NULL) {
			//  printf("\nCouldn't Open MCX Init File:%s For Reading..\n\n", BaseFileName.c_str());
			 fflush((FILE *)stdout); return (-1 * __LINE__);
		 }
	}
	else if (pData->ResetSeqFile == NULL) {
		 // printf("\nCouldn't Open MCX Init File For Recovery..\n\n"); exit(0);
	 }
	else{
		fp = pData->ResetSeqFile;
	}

	int32_t MSG_LEN=0,sz,Pos=0,Processed_Bytes=0,HMax=0,LMin=0;
	 unsigned char Buffer[5120]; int32_t BytesToRead=1024; uint32_t tMsgLen=0;
	 std::unordered_map<int32_t,CONTRACT_DETAILS*>::iterator citr = contract_file->instrument_umap[EXCHG_MCX].end();

		while((sz=fread(Buffer + Pos, 1, BytesToRead, fp)) != 0){

			if(sz > 0){

				sz += Pos;

				do{

					if(MSG_LEN==0 && (sz - Processed_Bytes) >=5){
						//Token = (uint64_t *)(Buffer + Processed_Bytes);
						char len[10]; memcpy(len, (Buffer + Processed_Bytes), 5);
						MSG_LEN = atoi(len); Processed_Bytes += 5;
					}
					else if(MSG_LEN==0){
						Pos = sz; BytesToRead = (5 - (sz - Processed_Bytes));
						//printf("---- [ Length Alignment Started => sz:%d BytesToRead:%d MsgLength:%d Processed_Bytes:%d ]---- \n",sz,BytesToRead,MSG_LEN,Processed_Bytes); fflush((FILE *)stdout);
						goto beginRead;
					}
					if((MSG_LEN+Processed_Bytes) > sz){
						Pos = sz; BytesToRead = (MSG_LEN - (sz - Processed_Bytes));
						//printf("---- [ Byte Alignment Started => sz:%d BytesToRead:%d MsgLength:%d Processed_Bytes:%d ]---- \n",sz,BytesToRead,MSG_LEN,Processed_Bytes); fflush((FILE *)stdout);
						goto beginRead;
					}

					switch(*(uint16_t*)(Buffer + Processed_Bytes)){

						case 10013:{
#ifdef PRINT_LOGS
							   Price_Point_Stream_Parameters_Download_Start_Response *response = (Price_Point_Stream_Parameters_Download_Start_Response*)(Buffer + Processed_Bytes);
								printf("\nSTART => MsgCode:%d, ReqID:%d\n\tMulticastIP:%s , MulticastPort:%d , SourceIP:%s\n\tAncMulticastIP:%s , AncMulticastPort:%d , AncSourceIP:%s\n\n",
									response->MessageCode,response->RequestID,
									response->MulticastParams.MulticastIP,response->MulticastParams.MulticastPort,response->MulticastParams.SourceIP,
									response->AncillaryMulticastParams.MulticastIP,response->AncillaryMulticastParams.MulticastPort,response->AncillaryMulticastParams.SourceIP);
#endif
						}break;

						case 10014:{
							Price_Point_Stream_Parameters_Download_Response *response = (Price_Point_Stream_Parameters_Download_Response*)(Buffer + Processed_Bytes);
#ifdef PRINT_LOGS
							printf("DATA => MsgCode:%d, ReqID:%d @\n", response->MessageCode,response->RequestID);
#endif
							tMsgLen = 0; Product_Multicast_Stream_Parameters *info = (Product_Multicast_Stream_Parameters*)(&response->ProductStreamParams);
							do{
#ifdef PRINT_LOGS
									printf("\tToken:%d , LastUpdatedTime:%d , MulticastIP:%s , MulticastPort:%d , SourceIP:%s\n",
										info->UniqueID,info->LastUpdatedTime,info->StreamParam.MulticastIP,info->StreamParam.MulticastPort,info->StreamParam.SourceIP);
#endif
								tMsgLen += sizeof(Product_Multicast_Stream_Parameters); ++info;
							}while( tMsgLen < (MSG_LEN - (sizeof(Price_Point_Stream_Parameters_Download_Response) - sizeof(Product_Multicast_Stream_Parameters))));
						}break;

						case 10015:{
#ifdef PRINT_LOGS
							Price_Point_Stream_Parameters_Download_End_Response *response = (Price_Point_Stream_Parameters_Download_End_Response*)(Buffer + Processed_Bytes);
							printf("\nEND => MsgCode:%d, ReqID:%d\n\n",response->MessageCode,response->RequestID);
#endif
						}break;

						case 10107:{
							Price_Point_Snapshot_Download_Request *request = (Price_Point_Snapshot_Download_Request*)(Buffer + Processed_Bytes);
#ifdef PRINT_LOGS
								printf("\nREQUEST => MsgCode:%d, ReqID:%d, Token:%d\n\n",request->MessageCode,request->RequestID,request->UniqueID);
#endif
								if(Token && request->UniqueID != Token){
									printf("\nRequested Token:%d Reset Data Not Matching Current Token :%d\n\n", Token, request->UniqueID); return (-1 * __LINE__);
								}
								citr = contract_file->instrument_umap[EXCHG_MCX].find(request->UniqueID);
								if(citr == contract_file->instrument_umap[EXCHG_MCX].end()){
									CONTRACT_DETAILS *cntr = new CONTRACT_DETAILS; cntr->Contract.Token = request->UniqueID; cntr->PTR = NULL;
									std::map<std::pair<int, int>, GenericContract>::iterator contract_itr = _Contract->find(std::make_pair(EXCHG_MCX, request->UniqueID));
									if(contract_itr != _Contract->end()){
										cntr->Contract.TickSize = contract_itr->second.TickSize; cntr->Contract.HighPriceRange = contract_itr->second.HighPriceRange; cntr->Contract.LowPriceRange = contract_itr->second.LowPriceRange;
									}else{
										cntr->Contract.TickSize = 1; cntr->Contract.HighPriceRange = 0; cntr->Contract.LowPriceRange = 0;
									}
									contract_file->instrument_umap[EXCHG_MCX][request->UniqueID] = cntr;	citr = contract_file->instrument_umap[EXCHG_MCX].find(request->UniqueID);
								}
						}break;

						case 10109:{
							Price_Point_Snapshot_Download_Start_Response *response = (Price_Point_Snapshot_Download_Start_Response*)(Buffer + Processed_Bytes);
#ifdef PRINT_LOGS
								printf("START => MsgCode:%d, ReqID:%d, LastSequenceNo:%d , %d\n\n",response->MessageCode,response->RequestID,response->LastSequenceNo,
									response->LastUpdatedTime);
#endif
							HMax = LMin = 0;
							if(citr != contract_file->instrument_umap[EXCHG_MCX].end()){
								memcpy(&citr->second->InitRes, response, sizeof(Price_Point_Snapshot_Download_Start_Response));
								citr->second->StartSequence = response->LastSequenceNo+1;
								if(citr->second->PTR != NULL){
									for(std::map<int32_t,TOKEN_SNAPSHOT_INFO*>::reverse_iterator itr = ((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotBuy.rbegin(); itr != ((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotBuy.rend(); itr++){
										delete itr->second;
									}
									for(std::map<int32_t,TOKEN_SNAPSHOT_INFO*>::reverse_iterator itr = ((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotSell.rbegin(); itr != ((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotSell.rend(); itr++){
										delete itr->second;
									}
									delete (TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR;
								}
								citr->second->PTR = new TOKEN_SNAPSHOT_BUY_SELL;
							}else return (-1 * __LINE__);
						}break;

						case 10110:{
#ifdef PRINT_LOGS
							Price_Point_Snapshot_Download_Response *response = (Price_Point_Snapshot_Download_Response*)(Buffer + Processed_Bytes);
							printf("\nDATA => MsgCode:%d, ReqID:%d\n\n",response->MessageCode,response->RequestID);
#endif
							tMsgLen = 0; Snapshot_Price_Point *info = (Snapshot_Price_Point*)(Buffer + Processed_Bytes + sizeof(Price_Point_Snapshot_Download_Response));
							if(citr != contract_file->instrument_umap[EXCHG_MCX].end()){
								do{
									if(info->Price < 0) info->Price *= -1;
#ifdef PRINT_LOGS
									printf("\tBuySell:%d , Price:%d , Quantity:%d , Orders:%d, DataFlag:%d\n",info->BuySell,info->Price,info->Quantity,info->Orders, info->DataFlag);
#endif
									if(info->BuySell==1){
										if(((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotBuy.find(info->Price) == ((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotBuy.end()){
											TOKEN_SNAPSHOT_INFO *detail = new TOKEN_SNAPSHOT_INFO; detail->Orders = info->Orders; detail->Quantity = info->Quantity;
											((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotBuy[info->Price] = detail;
										}else{
											((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotBuy[info->Price]->Orders += info->Orders;
											((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotBuy[info->Price]->Quantity += info->Quantity;
										}
									}
									else if(info->BuySell==2){
										if(((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotSell.find(info->Price) == ((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotSell.end()){
											TOKEN_SNAPSHOT_INFO *detail = new TOKEN_SNAPSHOT_INFO; detail->Orders = info->Orders; detail->Quantity = info->Quantity;
											((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotSell[info->Price] = detail;
										}else{
											((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotSell[info->Price]->Orders += info->Orders;
											((TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR)->SnapshotSell[info->Price]->Quantity += info->Quantity;
										}
									}else{
										printf("\nError => BuySell:%d , Price:%d , Quantity:%d , Orders:%d, DataFlag:%d\n",info->BuySell,info->Price,info->Quantity,info->Orders, info->DataFlag); exit(0);
									}
									// Calculate DPR Ranges
									if((int32_t)info->Price > HMax){
										HMax = info->Price;
									}
									else if((int32_t)info->Price < LMin || LMin == 0){
										LMin = info->Price;
									}
									tMsgLen += sizeof(Snapshot_Price_Point); ++info;
								}while(tMsgLen < (MSG_LEN - sizeof(Price_Point_Snapshot_Download_Response)));
							}else return (-1 * __LINE__);
						}break;

						case 10111:{
							if(citr != contract_file->instrument_umap[EXCHG_MCX].end()){
								if(HMax || LMin){
									// Update DPR Ranges
									citr->second->Contract.HighPriceRange = (HMax + (citr->second->Contract.TickSize*25));citr->second->Contract.LowPriceRange = (LMin - (citr->second->Contract.TickSize*25));
									if(citr->second->Contract.LowPriceRange < citr->second->Contract.TickSize)citr->second->Contract.LowPriceRange = 0;
									else if((citr->second->Contract.LowPriceRange % citr->second->Contract.TickSize))citr->second->Contract.LowPriceRange -= (citr->second->Contract.LowPriceRange % citr->second->Contract.TickSize);
									if(citr->second->Contract.HighPriceRange % citr->second->Contract.TickSize || citr->second->Contract.LowPriceRange %citr->second->Contract.TickSize){
										printf("\nWrong HDPR, LDPR Calculation => HMax:%d, LMin:%d, TickSize:%d, => HighPriceRange:%d, LowPriceRange:%d\n\n",
												HMax,LMin,citr->second->Contract.TickSize,citr->second->Contract.HighPriceRange,citr->second->Contract.LowPriceRange);// exit(0);
									}
								}
							}else return (-1 * __LINE__);
#ifdef PRINT_LOGS
							Price_Point_Snapshot_Download_End_Response *response = (Price_Point_Snapshot_Download_End_Response*)(Buffer + Processed_Bytes);
							printf("\nEND => MsgCode:%d, ReqID:%d\n\n",response->MessageCode,response->RequestID);
#endif
							if(Token){
								add_token_to_orderbook(Token,  pData->OrderBook.find(Token)->second, citr->second->StartSequence, citr->second->Contract.HighPriceRange,citr->second->Contract.LowPriceRange,
										citr->second->Contract.TickSize, __builtin_powi(10, citr->second->Contract.PriceExponent), (TOKEN_SNAPSHOT_BUY_SELL*)citr->second->PTR);
								return 0;
							}citr = contract_file->instrument_umap[EXCHG_MCX].end();
						}break;

						case 10009:{
						}break;

						case 10010:{
							ERROR_RESPONSE *error = (ERROR_RESPONSE*)(Buffer + Processed_Bytes);
							printf("\nERROR => MsgCode:%d, ErrorCode:%d\n\n",error->MessageCode,error->ErrorCode); return (-1 * __LINE__);
						}break;

						default:{
							printf("\nUNKNOWN ERROR => MsgCode:%d\n\n",*(uint16_t*)(Buffer + Processed_Bytes)); return (-1 * __LINE__);
						}
					}

					Processed_Bytes += MSG_LEN; MSG_LEN = 0;
				}while(Processed_Bytes < sz);
				if(Processed_Bytes == sz){
					Processed_Bytes = Pos = 0; BytesToRead = BYTES_READ_SIZE;
				}
				beginRead:{}
			}
		}

	return 0;
}
//
//extern MCX_Feeder *mfobj;
//
//int Init_MCX_Feed_Replay(std::string file) {
//
//	if(mfobj)delete mfobj; mfobj = new MCX_Feeder(file); //mfobj->BaseFileName = file;
//
//	return 0;
//}
//
//int Add_MCX_ReplayFile(uint8_t Segment, const char *file=NULL){
//	return mfobj->Add_ReplayFile(Segment,  file);
//}
//
//void Start_MCX_Feed_Replay(void){
//	mfobj->Start_FileReplay();
//}
//
