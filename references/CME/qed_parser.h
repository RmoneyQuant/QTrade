/** 
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_PARSER_H__
#define QED_PARSER_H__

#include "qed_compat.h"
#include "qed.h"
#include "qed_msg.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

#define QED_INVALID_CHANNEL UINT8_MAX
#define QED_INVALID_PACKET_TYPE UINT8_MAX

/**
 * Parser type for extracting messages from packets.
 *
 */
typedef struct QED_parser_* QED_parser;

/**
 * Create a packet parser.
 *
 * \p QED_parser breaks incoming messages into packets and filters the messages
 * by symbol. The \p QED_parser_register_symbol() method determines the messages
 * which are processed.
 *
 * \param ctx The initialized QED_ctx.
 *
 * \return A handle to the parser or \p NULL on error. If an error occurs it
 *         \p QED_errno() and \p QED_error_string() describe the failure.
 *
 * ERRORS:
 *   - QED_EOK: Success
 *   - QED_ENOMEM: Not enough memory to create the parser.
 *   - QED_EINVAL: The \p QED_ctx was not valid.
 */
extern QED_parser QED_create_parser(QED_ctx ctx);

/**
 * Regsister a symbol with the parser.
 * 
 * \p QED_parser_register_symbol() configures the parser to process message for
 *    the specified symbol.
 *
 * \param parser The parser.
 * \param symbol The symbol.
 * \param subscribe_hb Subscribe to HB information for this symbol if available.
 *        Heartbeat information is only available for symbols published in v3
 *        encoding or later.
 * \param closure User supplied pointer to context data for the symbol. The
 *                parser will make this value available through
 *                QED_msg_get_closure() for all messages for \e symbol.
 *
 * \return QED_EOK on success or the appropriate error code.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL The parser or symbol were not valid.
 */
extern QED_Status QED_parser_register_symbol(QED_parser parser, 
    const char* symbol, bool subscribe_hb, void* closure);

/** 
 * Start parsing a packet
 *
 * \p QED_parser_begin() initializes the parser with a new packet.
 *
 * \param parser The parser
 * \param packet A pointer to the packet data.
 * \param length The length of the packet.
 *
 * @return The QED_msg on success. A NULL return value indicates
 * either a failure or that there is no message for the subscribed
 * instruments in the packet. QED_errno() will contain the error in
 * the event of a failure.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EHB: Packet is a heartbeat packet. Parse with QED_heartbeat_parse().
 *  - QED_EINVAL Invalid parser
 *  - QED_EBADMSG Unsupported message type
 *  - QED_ECODEC Incompatible encoding or corrupt packet
 *
 * Note that if the packet only contains messages related to
 * instruments that have not been subscribed, the return value is NULL
 * and QED_errno() contains QED_EOK.
 */
extern QED_msg QED_parser_begin(QED_parser parser, const char* packet,
    size_t len);

/**
 * Get the next message in the packet.
 *
 * \p QED_parser_next() returns the next message in the packet. 
 *
 * \param parser The parser
 *
 * \return The next message or NULL. A return value of NULL indicates the end of
 *         the packet or an error. \p QED_errno() will be \p QED_EOK for the
 *         end of packet.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EBADMSG: Error parsing packet.
 *  - QED_EINVAL: Invalid argument.
 */
extern QED_msg QED_parser_next(QED_parser parser);

/**
 * Free the parser
 *
 * Frees resources associated with the \p QED_parser.
 *
 * \param parser The parser
 *
 * \return QED_EOK on success.
 *
 * ERRORS:
 *  - QED_EOK: Success
 *  - QED_EINVAL: Invalid argument.
 */
extern QED_Status QED_parser_free(QED_parser parser);

/**
 * Return the packet type for the current packet. The QED Distribution Server
 * sets this field in V3+ messages to distinguish between snapshot, heartbeat,
 * mixed, and other packet types.
 *
 * For packets prior to V3, the return value is always a snapshot packet.
 * QED_parser_begin will still filter out invalid v1 or v2 packet.
 *
 * This method can be called directly on the packet read from the socket (prior
 * to invoking QED_parser_begin().
 *
 * \param packet The packet
 * \param len The length of the packet.
 *
 * \return The sequence number or QED_INVALID_PACKET_TYPE if the packet is invalid. For
 *         invalid packets the \p QED_errno() will be set.
 *
 * ERROR:
 *  - QED_EOK: No error
 *  - QED_EINVAL: The packet is not valid.
 */
extern uint8_t QED_parser_get_pckt_type(const char* packet, size_t len);

/**
 * Return the sequence number for the current packet. The QED platform maintains
 * packet sequence number streams for each multicast channel/instrument group.
 * Gaps in the sequence numbers indicate a lost packet somewhere between the
 * encoding server and the client. Clients may use these sequence numbers for
 * line arbitrage.
 *
 * This method can be called directly on the packet read from the socket (prior
 * to invoking QED_parser_begin().
 *
 * \param packet The packet
 * \param len The length of the packet.
 *
 * \return The sequence number or QED_INVALID_SEQNO if the packet is invalid. For
 *         invalid packets the \p QED_errno() will be set.
 *
 * ERROR:
 *  - QED_EOK: No error
 *  - QED_EINVAL: The packet is not valid.
 */
extern uint64_t QED_parser_get_pckt_seq_no(const char* packet, size_t len);

/**
 * Return the channel id for this packet. The QED Distribution Server assigns
 * each multicast channel a unique id. This id allows applications to join
 * multiple multicast channels on a single socket, and identify the channel on
 * which a given packet arrived to assist in validating the sequence numbers on
 * a per-channel basis.
 *
 * This method can be called directly on the packet read from the socket (prior
 * to invoking QED_parser_begin().
 *
 * \param packet The packet
 * \param len The length of the packet.
 *
 * \return The channel id or QED_INVALID_CHANNEL if the packet is invalid. For
 *         invalid channels the \p QED_errno() will be set.
 *
 * ERROR:
 *  - QED_EOK: No error
 *  - QED_EINVAL: The packet is not valid.
 */
extern uint8_t QED_parser_get_pckt_channel(const char* packet, size_t len);

#if defined(__cplusplus)
} // extern "C" 
} // namespace QED
#endif

#endif /* QED_PARSER_H__ */
