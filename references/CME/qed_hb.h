/**
 * Copyright (c) 2014 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_HB_H__
#define QED_HB_H__

#include "qed_compat.h"
#include "qed.h"
#include "qed_error.h"
#include "qed_parser.h"
#include "qed_types.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

/**
 * HB Values
 */

#define QED_INVALID_HB_ID 0
#define QED_INVALID_HB_TIME 0

/**
 * QED heartbeat type. Since the message structure refers to and accesses data in the
 * heartbeat packet, the packet memory must remain in scope while using the message.
 */
typedef struct QED_hb_* QED_heartbeat;


/**
 * Parses a heartbeat message and returns a QED_heartbeat object
 *
 * \param parser The parser
 * \param packet The packet data
 * \param len The packet length
 *
 * \return The QED_heartbeat on success.
 *
 * ERRORS:
 *  - QED_EOK: Success.
 *  - QED_EINVAL: Invalid parameter or bad message type.
 *  - QED_ECODEC: Bad message encoding
 */
extern QED_heartbeat QED_heartbeat_parse(QED_parser parser, const char* packet,
    size_t len);

/**
 * Returns an array of QED_closure_info which applies to the current heartbeat.
 * QED_closure_info is of length len and contains the closures for the
 * instruments associated the current heartbeat id. The list only includes closures for
 * instruments for which the application passed true to
 * QED_parser_register_symbol() for the subscribe_hb parameter 
 *
 * \param heartbeat The QED_heartbeat returned from QED_heartbeat_parse()
 * \param len Pointer to a size_t to receive the number of closure_info structs
 *        returned.
 *
 * \return Pointer to array of 'len' QED_closure_info struct pointers or NULL.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL: Invalid parameter.
 *
 * \see QED_parser_register_symbol
 * \see QED_heartbeat_parse
 */
extern QED_closure_info** QED_heartbeat_get_closure_infos(QED_heartbeat heartbeat, size_t* len);

/**
 * Returns the Heartbeat ID
 *
 * \param heartbeat The QED_heartbeat returned from QED_heartbeat_parse()
 *
 * \return the id on success or QED_INVALID_HB_ID.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL: Invalid parameter.
 */
extern uint16_t QED_heartbeat_get_id(QED_heartbeat heartbeat);

/**
 * Returns the Encoding Server send timestamp in nanoseconds from epoch
 *
 * \param heartbeat The QED_heartbeat returned from QED_heartbeat_parse()
 *
 * \return nanoseconds since the epoch on success or QED_INVALID_HB_TIME.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL: Invalid parameter.
 */
extern uint64_t QED_heartbeat_get_es_timestamp(QED_heartbeat heartbeat);

/**
 * Returns the Decoding Server timestamp in nanoseconds from epoch
 *
 * \param heartbeat The QED_heartbeat returned from QED_heartbeat_parse()
 *
 * \return nanoseconds since the epoch on success or QED_INVALID_HB_TIME.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL: Invalid parameter.
 */
extern uint64_t QED_heartbeat_get_ds_timestamp(QED_heartbeat heartbeat);

/**
 * \brief Get Heartbeat Sequence Number
 *
 * The heartbeat sequence number is a 2 byte sequence number that gives an
 * indication of packet loss when market data is not flowing. It wraps to 0
 * after 65535.
 *
 * QED maintains separate sequence numbers for each heartbeat id. Furthermore,
 * the same heartbeat packets are sent over multiple channels, because QED
 * splits data from a single source (exchange) across multiple channels. To
 * properly interpret heartbeat sequence numbers, applications must process them
 * per channel and per heartbeat id.
 *
 * In the rare event of a fail over from our primary encoding server to the
 * backup, applications may see a gap in the sequence numbers.
 *
 * \param heartbeat The QED_heartbeat returned from QED_heartbeat_parse()
 *
 * \return The sequence number. Zero if the heartbeat parameter is NULL.
 *
 * ERRORS:
 *    QED_EOK    - On success
 *    QED_EINVAL - Null heartbeat parameter.
 */
extern uint16_t QED_heartbeat_get_seq_no(QED_heartbeat heartbeat);

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif

#endif /* QED_HB_H__ */
