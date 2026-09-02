/** 
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

/** 
 * \brief Extra Message Accessors for HF service
 *
 * The methods in this file access optional QED message fields. These are only
 * supported for QHF feeds.
 * Availability of each firld can depend on the instrument and version of teh QHF feed. 
 * Please check the QHF documentation addendum.
 * 
 */

#ifndef QED_MSG_EXTRA_HF_H__
#define QED_MSG_EXTRA_HF_H__

#include "qed_msg.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif 

typedef enum {
  QED_Down = 0,
  QED_Up = 1
} QED_Direction;

/**
 * \brief Get the HF protocol id and version.
 *
 * If the message contains a QHF signal, *id and *version are set to
 * the QHF protocol id and version found in the message.
 * 
 * ERRORS:
 *  - QED_EOK: Success. 
 *  - QED_EINVAL: Field not present (not a QHF signal) or message corrupt.
 */
extern void QED_msg_get_hf_id_and_version(QED_msg msg, uint32_t *id, uint32_t *version);

/**
 * \brief Get the HF move indicator side and magnitude.
 * If the message contains a QHF signal, *direction and *threshold are set to
 * the QHF direction and threshold.
 * 
 * ERRORS:
 *  - QED_EOK: Success. 
 *  - QED_EINVAL: Field not present (not a QHF signal) or message corrupt.
 */
extern void QED_msg_get_hf_direction_and_threshold(QED_msg msg, QED_Direction *direction, uint64_t *threshold);

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif 

#endif /* QED_MSG_EXTRA_HF_H__ */
