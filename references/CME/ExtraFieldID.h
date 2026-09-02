/**
 * File:   ExtraFieldID.h
 *
 * Copyright (c) 2014 Quincy Data LLC -- All Rights Reserved
 */

#ifndef DS_EXTRA_FIELD_ID_H
#define DS_EXTRA_FIELD_ID_H

/******************************************************************************
                       This is Necessary for C Client API

We want to be able to include this file in the C Client API so we don't have to
maintain the enum in more than one place.
******************************************************************************/

/**
 * ID's for Extra Fields in QED Messages
 *
 * For ICE, the local QED feed in Cermak sends some of these extra fields.
 */
enum ExtraFielID {
  ID_LOW                           = 0,
  ID_HIGH                          = 1,
  ID_TRADE_LAST_PRICE              = 2,
  ID_SETTLEMENT_PRICE              = 3,
  ID_TOTAL_TRADE_QTY               = 4,
  ID_OPEN_INTEREST                 = 5,
  ID_NET_ORDER_IMBALANCE_PRICE     = 6,
  ID_NET_ORDER_IMBALANCE_QTY       = 7,
  ID_DETAILED_INFO                 = 8,
  ID_RECV_TIMESTAMP                = 9,
  ID_FULL_MARKET_SEQ_NUM           = 10,
  ID_HF_PROTOCOL_ID_AND_VERSION    = 11,
  ID_HF_SIDE_AND_THRESHOLD         = 12,

  /// from 32 to 63 -> internal usage not forwarded
  ID_FULL_MARKET_TIMESTAMP         = 32,

  ID_LAST                          = 63 // used in test case
};

/**
 * Field size encoding
 */
enum ExtraFieldSize {
  ExtraFieldSize_1Byte = 0,
  ExtraFieldSize_2Bytes = 1,
  ExtraFieldSize_4Bytes = 2,
  ExtraFieldSize_8Bytes = 3
};

/**
 * Field size encoding mask
 */
#define EXTRAFIELD_SIZE_MASK 0x03

/*
 * Detailed Info extra data fields
 */
#define EXTRAFIELD_DETAILED_INFO_COMPLETION_INDICATOR 0x01

/*
 * Mask to determine if extra id is private or public
 */
#define EXTRA_FIELD_INTERNAL_USAGE_MASK 0x20

/*****************************************************************************/

#endif	/* DS_EXTRA_FIELD_ID_H */

