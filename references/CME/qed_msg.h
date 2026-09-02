/**
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_MSG_H__
#define QED_MSG_H__

#include "qed_compat.h"
#include "qed.h"
#include "qed_price.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

/**
 * Overflow and values.
 */
#define QED_INVALID_SEQNO UINT64_MAX
#define QED_INVALID_TIME UINT64_MAX
#define QED_INVALID_DEPTH UINT8_MAX
#define QED_INVALID_QUOTE_QTY UINT32_MAX
#define QED_INVALID_NUM_ORDERS UINT16_MAX
#define QED_INVALID_TRADE_QTY INT32_MIN
#define QED_INVALID_QTY INT32_MIN
#define QED_INVALID_BOOK_TRADE_INDEX UINT8_MAX
#define QED_INVALID_NUM_LEVELS UINT8_MAX
#define QED_INVALID_TRADE_FLAG UINT8_MAX
#define QED_INVALID_FLAGS 0xFF
#define QED_TRADE_SUMMARY_FLAG 0x80


/**
 * QED message type. The message structure refers to and accesses data in the
 * packet so the packet memory must remain in scope while using the message.
 */
typedef struct QED_msg_* QED_msg;

/**
 * Return the application supplied closure for the message. Clients specify a
 * closure for each topic when they register the topic with the \p QED_parser.
 * This method returns the closure associated with the message's topic.
 *
 * \param msg The message
 * \return The closure. If the msg is not valid, \p QED_msg_get_closure returns
 *         NULL and sets \p QED_errno() to QED_EINVAL.
 *
 * \see QED_parser_register_symbol
 *
 * ERRORS:
 *  - QED_EOK: success
 *  - QED_EINVAL: invalid argument
 */
extern void* QED_msg_get_closure(QED_msg msg);

/**
 * Return the codec version of the message.
 *
 * \param msg The message
 *
 * \return The integer version number identifying the codec used to encode the packet
 *
 * ERROR:
 *  - QED_EOK: No error
 *  - QED_EINVAL: The message is not valid.
 */
extern uint8_t QED_msg_get_codec_version(QED_msg msg);

/**
 * Return the compatibility version of the message. This is the oldest codec version
 * which may be used to interpret this message.
 *
 * \param msg The message
 *
 * \return The integer version number identifying the oldest compatible codec version
 *
 * ERROR:
 *  - QED_EOK: No error
 *  - QED_EINVAL: The message is not valid.
 */
extern uint8_t QED_msg_get_compat_version(QED_msg msg);

/**
 * Return the 16 least significant bits of the exchange sequence number if it is
 * available. In conjunction with the exchange timestamp, the low order bytes of
 * the sequence number allow users to correlate messages with the raw feed from
 * the exchange.
 *
 * Since QED updates are conflated, the sequence number is for the most recent
 * trade or book update applied to the message.
 *
 * For CME, the source exchange sequence number used is the MsgSeqNum field of
 * the last Globex MDP message.
 *
 * \param msg The message
 *
 * \return The 2 low order bytes of the exchange sequence number.
 *
 * ERROR:
 *  - QED_EOK: No error
 *  - QED_EINVAL: The message is not valid.
 */
extern uint16_t QED_msg_get_src_seq_no(QED_msg msg);

/**
 * Return the market time time stamp in microseconds since the epoch.
 *
 * For CME, this is the SendingTime field of the last Globex MDP
 * taken into account.
 *
 * \param msg The message.
 *
 * \return The time stamp since the epoch in microseconds. If an error occurs,
 *         the method returns QED_INVALID_TIME and sets QED_errno() to the
 *         appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid message.
 */
extern uint64_t QED_msg_get_src_time(QED_msg msg);

/**
 * Return the time that the Distribution Server received the packet with the
 * current message from the Encoding Server in microseconds since the epoch.
 *
 * \param msg The message.
 *
 * \return The time stamp since the epoch in microseconds. If an error occurs,
 *         the method returns QED_INVALID_TIME and sets \p QED_errno() to the
 *         appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid message.
 */
extern uint64_t QED_msg_get_ds_rcv_time(QED_msg msg);

/**
 * Return the time that the Distribution Server sent the packet with the
 * current message in microseconds since the epoch.
 *
 * \param msg The message.
 *
 * \return The time stamp since the epoch in microseconds. If an error occurs,
 *         the method returns QED_INVALID_TIME and sets \p QED_errno() to the
 *         appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid message.
 */
extern uint64_t QED_msg_get_ds_snd_time(QED_msg msg);

/**
 * Return the number of visible prices levels in the book. The actual book depth
 * may be smaller or larger. QED sends fixed depth snapshots for all
 * instruments.
 *
 * \param msg The message.
 *
 * \return The number of levels. If the message is not valid the return value
 *         will be \p QED_INVALID_DEPTH and \p QED_errno() set to QED_EINVAL.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL: Invalid message.
 */
extern uint8_t QED_msg_depth(QED_msg msg);

/**
 * Return the number of visible levels in the book for number of orders. This
 * value is independent of the number of price levels in the book, but will
 * generally be less than the number of visible price levels. Currently for
 * exchanges that provide the number orders QED, sends it for the top 2 levels.
 *
 * \param msg The message.
 *
 * \return The number of levels. If the message is not valid the return value
 *         will be \p QED_INVALID_DEPTH and \p QED_errno() set to QED_EINVAL.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL: Invalid message.
 */
extern uint8_t QED_msg_number_of_order_depth(QED_msg msg);

/**
 * Enumeration to specify the side of the book.
 */
typedef enum {
  QED_BID = 0,
  QED_ASK = 1
} QED_Side;

/**
 * Return the price for the specified level and side. If the quantity for the
 * level is 0, indicating that the level is not present, the return value is not
 * specified.
 *
 * \param msg The message.
 * \param level The level.
 * \param side The side (\p QED_BID or \p QED_ASK)
 *
 * \return The price as a \p QED_Price. If an error occurs,
 *         \p QED_price_is_valid(price) will return false, and \p QED_errno()
 *         will be set to the appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 *  - QED_EIDX The level is out of range.
 */
extern QED_Price QED_msg_get_level_price(QED_msg msg, uint8_t level, QED_Side side);

/**
 * Return the quantity for a level and side.
 *
 * \param msg The message.
 * \param level The level.
 * \param side The side (\p QED_BID or \p QED_ASK)
 *
 * \return The quantity as a \p uint32_t. If an error occurs, the value will be
 *         \p QED_INVALID_QUOTE_QTY \p QED_errno() will be set to the appropriate error
 *         code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 *  - QED_EIDX The level is out of range.
 */
extern uint32_t QED_msg_level_qty(QED_msg msg, uint8_t level, QED_Side side);

/**
 * Return the number of orders for a level and side.
 *
 * \param msg The message.
 * \param level The level
 * \param side The side (\p QED_BID or \p QED_ASK)
 *
 * \return The number of orders as a \p uint16_t. If an error occurs, the value will be
 *         \p QED_INVALID_NUM_ORDERS \p QED_errno() will be set to the appropriate error
 *         code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 *  - QED_EIDX The level is out of range.
 */
extern uint16_t QED_msg_level_number_of_orders(QED_msg msg, uint8_t level, QED_Side side);

/**
 * Return the book trade index to mark the latest book update.
 *
 * \param msg The message.
 *
 * \return The book trade index. If the message is not valid the return value
 *         will be \p QED_INVALID_BOOK_TRADE_INDEX and \p QED_errno() set to QED_EINVAL.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid message.
 */
extern uint8_t QED_msg_book_trade_index(QED_msg msg);

/**
 * Return the number of trade levels in the message.
 *
 * \param msg The message.
 *
 * \return The number of levels. If the message is not valid the return value
 *         will be \p QED_INVALID_NUM_LEVELS and \p QED_errno() set to QED_EINVAL.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid message.
 */
extern uint8_t QED_msg_trade_level_count(QED_msg msg);

/**
 * Return the trade price for the specified level.
 *
 * \param msg The message.
 * \param level The level.
 *
 * \return The price as a \p QED_Price. If an error occurs,
 *         \p QED_price_is_valid(price) will return false, and \p QED_errno()
 *         will be set to the appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 *  - QED_EIDX The level is out of range.
 */
extern QED_Price QED_msg_get_trade_price(QED_msg msg, uint8_t level);

/**
 * Return the trade quantity for a level.
 *
 * \param msg The message.
 * \param level The level.
 *
 * \return The quantity as a \p int16_t. If an error occurs, the value will be
 *         \p QED_INVALID_TRADE_QTY \p QED_errno() will be set to the
 *         appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 *  - QED_EIDX The level is out of range.
 */
extern int32_t QED_msg_get_trade_qty(QED_msg msg, uint8_t level);

/**
 * Enumeration to specify the aggressor side of a trade
 *
 * 0x00 QED_SELL
 * 0x01 QED_BUY
 * 0x02 QED_NONE - Aggressor side is neither buy nor sell
 * 0x03 QED_UNSUPPORTED - Aggressor side is not available/unsupported
 * 0x80 QED_SUMMARY_SELL - summary aggressor sell
 * 0x81 QED_SUMMARY_BUY - summary aggressor buy
 * 0x82 QED_SUMMARY_NONE - summary aggressor none
 * 0x04-0x7F and 0x83-0xFF Reserved for future use
 */
typedef enum {
  QED_SELL = 0,
  QED_BUY = 1,
  QED_NONE = 2,
  QED_UNSUPPORTED = 3,

  QED_SUMMARY_SELL = QED_TRADE_SUMMARY_FLAG | QED_SELL, // 0x80
  QED_SUMMARY_BUY  = QED_TRADE_SUMMARY_FLAG | QED_BUY,  // 0x81
  QED_SUMMARY_NONE = QED_TRADE_SUMMARY_FLAG | QED_NONE  // 0x82
} QED_trade_flag;

/**
 * Returns the aggressor side of a trade as a QED_trade_flag.
 *
 * \param msg The message.
 * \param level The level.
 *
 * \return The aggressor side as a \p QED_trade_flag. If an error occurs, the value will be
 *         \p QED_UNSUPPORTED \p QED_errno() will be set to the
 *         appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 *  - QED_EIDX The level is out of range.
 */
extern QED_trade_flag QED_msg_get_trade_aggressor(QED_msg msg, uint8_t level);

/**
 * Return the unreported trade quantity. This is the quantity not included in
 * the aggregated trade levels. If all the volume is reported this method
 * returns 0.
 *
 * \param msg The message.
 *
 * \return The quantity as a \p int16_t. If an error occurs, the value will be
 *         \p QED_INVALID_TRADE_QTY \p QED_errno() will be set to the
 *         appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 */
extern int32_t QED_msg_get_unreported_qty(QED_msg msg);

/**
 * Return the message topic.
 *
 * \param msg The message.
 *
 * \return The topic as a null-terminated c-string \p const char*. If an error occurs,
 *         the value will be \p NULL \p QED_errno() will be set to the
 *         appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid argument.
 */
extern const char* QED_msg_get_topic(QED_msg msg);

/**
 * Return the message status. Use the \p QED_statusXxx() macros to examine the
 * message status which is a bitmap described below.
 *
 * \param msg The message
 *
 * \return The status flags or QED_INVALID_FLAGS with \p QED_errno() set to QED_EINVAL on
 * error.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL Invalid message.
 */
extern uint8_t QED_msg_status_flags(QED_msg msg);

/**
 * Market data status macros.
 *
 * The first 4 bits describe the message status with the flags below.
 *
 * Bits 5 and 6 contain the trading status:
 *  - 00 Unknown status QED_TRADING_UNKNOWN
 *  - 01 Not trading    QED_TRADING_NOT_TRADING
 *  - 10 Trading        QED_TRADING_TRADING
 *  - 11 Halted         QED_TRADING_HALTED
 *
 */
#define QED_FIELD_OFLOW 0x01      /** One or more fields have overflows */
#define QED_FEED_ERROR  0x02      /** An error occurred in the Encoding Server */
#define QED_BOOK_INCONISTENT 0x04  /** DEPRECATED: The book is inconsistent */
#define QED_BOOK_INCONSISTENT 0x04 /** The book is inconsistent */
#define QED_SLOW_SOURCE 0x08      /** The packet is provided by slow source like fiber*/
#define QED_BOOK_UNCHANGED 0x80   /** Book unchanged since last update and not
                                      present in the current message */
#define QED_ERROR_FLAGS (QED_FIELD_OFLOW | QED_FEED_ERROR | QED_BOOK_INCONSISTENT)

/**
 * Trading status.
 *
 * The CME trading status are mapped as following to the QED trading status:
 * -  2 Trading Halt:                QED_TRADING_HALTED
 * - 17 Ready to Trade:              QED_TRADING_TRADING
 * - 18 Not Available for Trading:   QED_TRADING_NOT_TRADING
 * - 21 Pre-open:                    QED_TRADING_NOT_TRADING
 * - 24 Pre-Cross:                   QED_TRADING_UNKNOWN
 * - 25 Cross:                       QED_TRADING_UNKNOWN
 */
enum QED_TradingStatus {
  QED_TRADING_UNKNOWN = 0,
  QED_TRADING_NOT_TRADING = 1,
  QED_TRADING_TRADING = 2,
  QED_TRADING_HALTED = 3
};

/**
 * True if there are no errors: the overflow, feed handler error, and
 * inconsistent book bits are all 0.
 */
#define QED_statusOk(x) (!((x) & QED_ERROR_FLAGS))
#define QED_statusOverFlow(x) ((x) & QED_FIELD_OFLOW)
#define QED_statusInconsistentBook(x) ((x) & QED_BOOK_INCONSISTENT)
#define QED_isSlowSource(x) ((x) & QED_SLOW_SOURCE)
#define QED_statusBookUnchanged(x) ((x) & QED_BOOK_UNCHANGED)
#define QED_tradingStatus(x) ((x) & ~0x80) >> 5

/**
 * Trade Flag Macros
 */
#define QED_AGGRESSOR_TRADE_FLAGS 0x03

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif

#endif /* QED_MSG_H__ */
