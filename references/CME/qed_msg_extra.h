/** 
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

/** 
 * \brief Extra Message Accessors
 *
 * The methods in this file access optional QED message fields. These are not
 * supported for all exchanges or QED feeds. They were added in order to  meet
 * ICE's conformance requirements, and are not published by the production feed.
 * They remain in the API to facilitate future conformance tests.
 * 
 */

#ifndef QED_MSG_EXTRA_H__
#define QED_MSG_EXTRA_H__

#include "qed_msg.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif 

/** 
 * \brief Get the Extra Field Count
 *
 * This method allows clients to determine if there are extra fields present and
 * how many.
 *
 * @param msg
 * @return the number of extra fields. If the message is invalid the method
 * returns UINT8_MAX and qed_last_error is set to QED_EINVAL.
 *
 * ERRORS:
 *  - QED_EOK: Success.
 *  - QED_EINVAL: Message was invalid.
 */
extern uint8_t QED_msg_get_extra_count(QED_msg msg);

/**
 * \brief Get the open interest
 *
 * @return The open interest if available. If the field is not present the
 * return value is QED_INVALID_QTY and qed_last_error is QED_EINVAL. If there is an
 * overflow, the return value is QED_INVALID_QTY and qed_last_error is QED_EOK.
 *
 * ERRORS:
 *  - QED_EOK: Success. Overflow if return value is QED_INVALID_QTY.
 *  - QED_EINVAL: Field not present.
 */
extern int32_t QED_msg_get_open_interest(QED_msg msg);

/**
 * \brief Get the total volume
 *
 * @return The total volume if available. If the field is not present the
 * return value is QED_INVALID_QTY and qed_last_error is QED_EINVAL. If there is an
 * overflow, the return value is QED_INVALID_QTY and qed_last_error is QED_EOK.
 *
 * ERRORS:
 *  - QED_EOK: Success. Overflow if return value is QED_INVALID_QTY.
 *  - QED_EINVAL: Field not present.
 */
extern int32_t QED_msg_get_total_volume(QED_msg msg);

/**
 * \brief Get the High Price
 *
 * @return The high price if available. If the field is not present the return
 * value is invalid according to QED_price_is_valid() and qed_last_error is
 * QED_EINVAL. If there is an overflow, the return value is invalid  and
 * qed_last_error is QED_EOK.
 *
 * ERRORS:
 *  - QED_EOK: Success. Overflow if QED_price_is_valid() is false.
 *  - QED_EINVAL: Field not present.
 */
extern QED_Price QED_msg_get_high_price(QED_msg msg);

/**
 * \brief Get the Low Price
 *
 * @return The low price if available. If the field is not present the return
 * value is invalid according to QED_price_is_valid() and qed_last_error is
 * QED_EINVAL. If there is an overflow, the return value is invalid  and
 * qed_last_error is QED_EOK.
 *
 * ERRORS:
 *  - QED_EOK: Success. Overflow if QED_price_is_valid() is false.
 *  - QED_EINVAL: Field not present.
 */
extern QED_Price QED_msg_get_low_price(QED_msg msg);

/**
 * \brief Get the Last Trade Price
 *
 * @return The last trade price if available. If the field is not present the
 * return value is invalid according to QED_price_is_valid() and qed_last_error
 * is QED_EINVAL. If there is an overflow trade, the return value is invalid
 * and qed_last_error is QED_EOK.
 *
 * ERRORS:
 *  - QED_EOK: Success. Overflow if QED_price_is_valid() is false.
 *  - QED_EINVAL: Field not present.
 */
extern QED_Price QED_msg_get_last_trade_price(QED_msg msg);

/**
 * \brief Get the Settlement Price
 *
 * @return The settlement price if available. If the field is not present the
 * return value is invalid according to QED_price_is_valid() and qed_last_error
 * is QED_EINVAL. If there is an overflow, the return value is invalid  and
 * qed_last_error is QED_EOK.
 *
 * ERRORS:
 *  - QED_EOK: Success. Overflow if QED_price_is_valid() is false.
 *  - QED_EINVAL: Field not present.
 */
extern QED_Price QED_msg_get_settlement_price(QED_msg msg);

/**
 * \brief Get the completion indicator.
 *
 * @return 0 on an incomplete transaction. non-0 in all other cases.
 *
 * ERRORS:
 *  - QED_EOK: Success. 
 *  - QED_EINVAL: Field or message corrupt.
 */
extern int QED_msg_get_completion_indicator(QED_msg msg);

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif 

#endif /* QED_MSG_EXTRA_H__ */
