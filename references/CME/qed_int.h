/**
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_INT_H__
#define QED_INT_H__

/**
 * Internal header for unit testing.
 */

#include "Utils/QED_map.h"
#include "Utils/QED_config.h"
#include "Utils/QED_bitset.h"
#include "Utils/QED_sock.h"

#include "qed_compat.h"

#include "Quincy/qed.h"
#include "Quincy/qed_types.h"
#include "Quincy/qed_price.h"
#include "Quincy/qed_hb.h"

#include "qed_symbol_list.h"
#include "DsMacro.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

#ifdef __GNUC__
#define likely(x) __builtin_expect((x),1)
#define unlikely(x) __builtin_expect((x),0)
#else
#define likely(x) (x)
#define unlikely(x) (x)
#endif

#define QED_CFG_FILE "qed.cfg"

/**
 * Structs for processing packets.
 */
typedef struct {
  uint8_t codec_version;
  uint8_t compat_version;
  uint8_t instance_id;
} __attribute__((packed)) QED_PacketVersion;

typedef struct {
  uint8_t packet_type;
}__attribute__((packed)) QED_PacketType;

typedef struct {
  uint8_t channel_id;
  uint64_t pckt_seq_no;
  uint64_t receive_time;
  uint64_t send_time;
} __attribute__((packed)) QED_PacketInfo;

typedef struct {
  uint8_t message_type;
} __attribute__((packed)) QED_SnapshotHeader0;

typedef struct {
  uint16_t ds_id;
} __attribute__((packed)) QED_SnapshotHeader1V3;

typedef union {
  QED_SnapshotHeader1V3 v3;
} QED_SnapshotHeader1;

typedef struct {
  uint8_t extra_field_cnt;
  uint8_t  es_error;
  uint64_t exchg_time;
  uint16_t exchg_seq_no;
  uint8_t depth;
  uint8_t nbrOrderDepth;
} __attribute__((packed)) QED_SnapshotHeader2;

typedef uint32_t QED_BookSize;
typedef uint16_t QED_NumberOfOrders;
typedef int32_t QED_TradeV2;

/* From InstrumentSnapshot.h */
#define QED_MAX_BOOK_DEPTH 10
#define QED_MAX_BOOK_NUMBER_ORDERS 10
#define QED_MAX_TRADE_PRICES 200
/* From BaseExtra.h */
#define QED_MAX_EXTRA_VALUES 63
#define QED_MAX_SYMBOL_LIST_SIZE 8192
/* 8192 will hold 255 symbols 32 characters long. OPRA symbols are around 23
 * (don't recall the exact number), and they are the longest that I know of.
 */
#define QED_HB_CLOSURE_LIST_DEFAULT_SIZE 10

typedef struct  {
  const char* name;
  uint8_t channel_id;
  uint16_t heartbeat_id;
} QED_symbol_info_;

/**
 * Implementation for blind typdef in Quincy/ headers
 */
struct QED_ctx_ {
  char *symbolListBuffer;   /* For symbol list packet */
  size_t symbolListBufferLen; /* length of symbol list message */

  const char*      config_name;
  QED_MapStringKey symbol_to_id;
  QEDI_MapIntKey   id_to_symbol_info;
  const char **          symbols; /* array of the symbol names (char *) */
  uint16_t         num_symbols;
  size_t           symbols_capacity;
  QEDI_SymbolListContext* symbol_list_context;
};

/**
 * \brief QED_ExtraData
 *
 * This a union for the extra data fields. Not all messages contain extra
 * fields, nor do all channels send extra fields. For example, the QED ICE feed
 * sends extra fields only locally in Cermak. In all other POPs the ICE feed
 * omits the extra fields to conserve bandwidth.
 *
 * Clients access the extra fields through field specific accessors like
 * QED_msg_get_total_volume().
 *
 * The 2 low order bits of the id_and_size field is the size of the extra field
 * 1 (0x0),2(0x1),4(0x2), or 8(0x3)  bytes). The remaining 6 bits are the field
 * id (0 - 63):
 *    size = 1 << (id_and_size & 0x03);
 *    id = id_and_size >> 2;
 *
 */
typedef struct {
  uint8_t id_and_size;
  union {
    uint8_t  ui8;
    uint16_t ui16;
    uint32_t ui32;
    uint64_t ui64;
    int8_t   i8;
    int16_t  i16;
    int32_t  i32;
    int64_t  i64;
  } data;
} __attribute__((packed)) QED_ExtraData;

#define QED_MAX_EXTRA_ID 63

struct QED_msg_ {
  const QED_PacketVersion* cur_packet_version;
  const QED_PacketType* cur_packet_type;
  const QED_PacketInfo* cur_packet;
  const QED_SnapshotHeader0*  cur_snapshot0;
  const QED_SnapshotHeader1*  cur_snapshot1;
  const QED_SnapshotHeader2*  cur_snapshot2;
  uint16_t ds_id;
  QED_Price* price_levels[2];
  QED_BookSize* book_qtys[2];
  QED_NumberOfOrders* book_number_orders[2];
  QED_Price* trade_prices;

  QED_TradeV2* trade_qtys;

  uint8_t* trade_flags;

  const char* extra_start;
  uint64_t extra_fields; /* presence map for extra fields */
  QED_ExtraData* extra_data_ptrs[QED_MAX_EXTRA_ID + 1];

  void* closure;
  const char* topic;
  uint8_t num_trades;
  bool extra_fields_processed;
};

typedef struct {
  uint8_t channel_id;
  uint16_t heartbeat_id;
  uint64_t es_timestamp;
  uint64_t ds_timestamp;
  uint16_t seq_no;
} __attribute__((packed)) QED_hb_msg_;

struct QED_hb_ {
  const QED_PacketVersion* cur_packet_version;
  const QED_PacketType* cur_packet_type;
  const QED_hb_msg_* cur_hb_msg;
  const struct QED_hb_closure_list_info_* cur_hb_msg_info;
};

struct QED_hb_closure_list_info_ {
  size_t numOfSymbols; /* Number of stored symbols */
  size_t sizeOfArray; /* Available space in closure array*/
  QED_closure_info** closures;
};

struct QED_parser_ {
  struct QED_ctx_* ctx;
  QEDI_MapIntKey closures;
  QEDI_MapIntKey hb_key_to_closures; //call QEDI_getHBClosureKey to get the lookup
  QED_BitSet snapshot_filter;
  size_t cur_packet_len;
  struct QED_msg_ msg;
  struct QED_hb_ hb;
};

/**
 * \brief Inline function to check packet and library versions
 *
 * The QED client library has two versions associated with it:
 *   QED_CLIENT_MAX_VERSION: Greatest codec version (inclusive) the library supports
 *   QED_CLIENT_MIN_VERSION: Oldest codec version (inclusive) the library supports
 *
 * QED packets have two pieces of version information associated with them:
 *   codec_version: The codec version of the packet
 *   compat_version: The oldest client version which can parse the message
 *
 *   Caveat: Not all features of new messages will be parsed if using an older
 *           but still compatible client library
 *
 * @param packet_version
 * @return returns true if the packet can be parsed by this library
 */
static inline bool QEDI_checkPacketAndCodecVersion(QED_PacketVersion* packet_version) {
  if( ((QED_CLIENT_MAX_VERSION >= packet_version->compat_version) &&
       (QED_CLIENT_MIN_VERSION <= packet_version->compat_version)) &&
       (packet_version->codec_version >= packet_version->compat_version) ) {
    return true;
  }

  return false;
}

/**
 * Process Extra Fields
 *
 * Inline method to setup pointers for extra fields. Only invoked if client
 * attempts to access extra fields.
 */
static inline bool QEDI_processExtraFields(struct QED_msg_* msg){
  if (!msg->extra_fields_processed){
    uint8_t extra_field_cnt = 0;
    const char *pos = msg->extra_start;

    msg->extra_fields_processed = true;
    msg->extra_fields = 0; /* clear presense map */

    extra_field_cnt = msg->cur_snapshot2->extra_field_cnt;

    for (int i = 0; i < extra_field_cnt; i++){
      uint8_t field_size = 0x01 << (uint8_t)(*pos & 0x03);
      uint8_t id = (uint8_t)(*pos) >> 2;

      msg->extra_data_ptrs[id] = (QED_ExtraData*)pos;
      msg->extra_fields |= 1 << id;

      pos = pos + field_size + 1;
    }
  }
  return true;
}

/**
 * \brief Inline Functions To Get Extra Fields
 *
 * Client applications should use the normalized accessors like
 * QED_msg_get_total_volume().
 *
 * @return NULL if the field is not present.
 *
 */
static inline QED_ExtraData* QEDI_getExtraField(struct QED_msg_* msg, uint8_t id){
  if( unlikely(!QEDI_processExtraFields(msg))) {
    return NULL;
  }

  if (msg->extra_fields & (1 << id)) {
    return msg->extra_data_ptrs[id];
  } else {
   return NULL;
  }
}

/* This works for GCC but >> behavior for signed values is implementation
 * defined in the ISO C Standard.
 */
#define GCC_MAN(price) (int64_t)((price) >> 8);

#define ISO_MAN(price)                                             \
  (price) < 0 ?   (int64_t)(((price) >> 8) | 0xFF00000000000000LL) \
              :   (int64_t)((price) >> 8)

/**
 * \brief Inline Function To Get Extra Price Field
 *
 * Client applications should use the normalized accessors like
 * QED_msg_get_total_volume().
 *
 * @return The price. The price will invalid (INT8_MIN for exponent and
 * INT64_MIN for mantissa) if an error occurs. qed_last_error will be set to
 * descibe the error.
 *
 */
static inline QED_Price QEDI_getExtraPrice(struct QED_msg_* msg, uint8_t id){
  QED_Price p;
  qed_last_error = QED_EOK;
  int64_t man;
  uint8_t exp;

  QED_ExtraData* extra = QEDI_getExtraField(msg, id);

  if (NULL == extra ||
     ( 3 != (extra->id_and_size & 0x3))) {  // Not 64 bits
    p.man = INT64_MIN;
    p.exp = INT8_MIN;
    qed_last_error = QED_EINVAL;
    return p;
  }

  /*
   * Extra price encoding is 8 bit unsigned exponent (implied negative) with 56
   * bit mantissa.
   */
  exp = (int8_t)(extra->data.ui64 & 0xFF);

#ifdef __GNUC__
  man = GCC_MAN(extra->data.i64);
#else
  man = ISO_MAN(extra->data.i64);
#endif

  if (INT64_MIN == extra->data.i64 ||
      -exp < INT8_MIN + 1 ||
      man < INT64_MIN + 1) { // overflow
    p.exp = INT8_MIN;
    p.man = INT64_MIN;
  }
  else {
    p.exp = -(int8_t)exp;
    p.man = (int64_t)man;
  }

  return p;
}

/**
 * \brief Inline function To create hb id key
 *
 * @return A uint32_t where the 16-24 bits are a channel id and the 0-16
 * bits are a heartbeat_id.
 *
 */
static inline uint32_t QEDI_getHBClosureKey(uint8_t channel_id, uint16_t heartbeat_id) {
  uint32_t ret = heartbeat_id;
  ret |= channel_id << (sizeof(uint16_t) * CHAR_BIT);

  return ret;
};

/* In header for unit tests */
uint8_t QEDI_msg_get_trade_flag(struct QED_msg_* msg, uint8_t level);

/*
 * @brief Adds a symbol parsed from the symbol list to the context
 * @param ctx QED_ctx to be updated
 * @param symbol symbol name (null terminated string)
 * @param ds_id the symbol id
 * @param hb_id the heartbeat id for the symbol
 * @param channel_id the channel id
 * @return QED_EOK on success, an error status in case of failure
 */
extern QED_Status QEDI_addSymbol(QED_ctx ctx, const char *symbol, uint16_t ds_id,
                                 uint16_t hb_id, uint8_t channel_id);

#if defined(__cplusplus)
} // extern C
} // namespace QED
#endif

#endif /* QED_INT_H__ */
