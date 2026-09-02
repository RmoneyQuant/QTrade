#include<bits/stdc++.h>
#include<stdio.h>
#include<stdint.h>
#include<stdlib.h>

#pragma pack(push,1)
#define FEED_LEVEL_DEPTH 5
typedef int32_t PriceType;

typedef enum Exchange_Type: int8_t {
        EXCHG_CME=0,
        EXCHG_NONE
}Exchange_Type;

typedef enum Instrument_Type:int8_t{
        EQUITY,
        INDEX,
        FUTIDX,
        FUTSTK,
        FUTCOM,
        FUTCUR,
        FUTINT,
        FUTIVX,
        FUTIRC,
        FUTIRD,
        FUTIRT,
        FUTENR,
        FUTBLN,
        FUTBAS,
        COM,

        OPTIDX,
        OPTSTK,
        OPTFUT,
        OPTCOM,
        OPTCUR,
        OPTINT,
        OPTIVX,
        OPTIRC,
        OPTIRD,
        OPTENR,
        OPTBLN,
        OPTBAS,

        UNDINT ,
        UNDCUR,
        UNDIRC,
        UNDIRT,

        INST_ERROR
}Instrument_Type;

typedef enum Option_Type:int8_t{ CA, PA, CE, PE, OPTION_NONE }Option_Type;

typedef struct ContractMargin{
        uint32_t                InitialMargin;
        uint32_t                AdditionalMargin;
        uint32_t                RegulatoryMargin;
        uint32_t                LongMargin;
        uint32_t                ShortMargin;
}ContractMargin;

typedef struct  GenericContract {
        Exchange_Type           Exchange;
        int32_t                                         Index;
        int32_t                                         Token;
        int32_t                                         ExpiryDate;
        PriceType                                       StrikePrice;
        int32_t                                         LotSize;
        int32_t                                         TickSize;
        PriceType                                       LowPriceRange;
        PriceType                                       HighPriceRange;
        PriceType                                       BasePrice;
        PriceType                                       Multiplier;
        ContractMargin                          Margin;
        char                                            Symbol [50];
        char                                            SymbolCode [50];
        Instrument_Type                         InstrumentType;
        Option_Type                                     OptionType;
        uint8_t                                         PriceExponent;
        uint8_t                                         PriceExponentDisplay;
}GenericContract;

#pragma pack(pop)

typedef struct CONTRACT_DETAILS
{
        PriceType                       BuyPrice[FEED_LEVEL_DEPTH];
        uint32_t                        BuyQty[FEED_LEVEL_DEPTH];
        PriceType                       SellPrice[FEED_LEVEL_DEPTH];
        uint32_t                        SellQty[FEED_LEVEL_DEPTH];
        int32_t                         NoOfBuyOrds[FEED_LEVEL_DEPTH];
        int32_t                         NoOfSellOrds[FEED_LEVEL_DEPTH];
        PriceType                       LastTradedPrice;
        uint16_t                        LastTradedQty:12;
        uint8_t                         FeedEventAggressor:2;
        GenericContract                 Contract;

}CONTRACT_DETAILS;

int main(int argc, char **argv)
{

    char pfile[256];sprintf(pfile, "%s", argv[1]); //Feed_FO_StreamID_1_03_10_2023.bin
    FILE *fp = fopen(pfile, "r");
    if (fp == NULL) {
        printf("\ncouldn't open FEED_DATA_FILE :\033[1;31m %s \033[0mfor reading.\n\n",pfile); return -1;
    }
        uint64_t PacketCount =0;
        unsigned char Buffer[8192], *MulticastBuffer = NULL;int64_t ApplSeqNum = 0; int64_t Counter=0;
        while(1){
                ssize_t nRead;
                if (fread(&nRead, sizeof(nRead), 1, fp) != 1){
                        std::cout << "EOF reached ApplSeqNum: " << ApplSeqNum << std::endl;
                        std::cout << "Total Packet Count:" << PacketCount <<" Missed:" << Counter << std::endl;
                        break;              // EOF safely reached
                }
                if (nRead < (ssize_t)sizeof(uint64_t)) {
                        printf("Corrupted record found. Exiting.\n");
                        break;
                }
                //std::cout << "nRead:" << nRead << std::endl;

                ssize_t payloadSize = nRead - sizeof(uint64_t);
                uint64_t RcTime = 0;
                int sz=0;
                if (sz=fread(Buffer, nRead, 1, fp) != 0) {
                        RcTime = *((uint64_t *)Buffer);
                        std::cout << "RcTime:" << RcTime << std::endl;
                        MulticastBuffer = Buffer+sizeof(uint64_t);
                        PacketCount++;
                        CONTRACT_DETAILS *ct = (CONTRACT_DETAILS*)MulticastBuffer;
                        std::cout << "sz:" << nRead << " payloadSize:" << payloadSize <<  std::endl;
                        std::cout << "----------" << ct->Contract.SymbolCode << "----------" << std::endl;
                        for(int k=0; k<FEED_LEVEL_DEPTH; k++){
                                std::cout << ct->BuyQty[k] << " - " << ct->BuyPrice[k] << " | " << ct->SellPrice[k] << " - " << ct->SellQty[k] << std::endl;
                        }
                }
        }
        return 0;
}