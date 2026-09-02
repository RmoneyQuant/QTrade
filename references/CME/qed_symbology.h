/** 
 * Copyright (c) 2014 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_SYMBOLOGY_H__
#define QED_SYMBOLOGY_H__

#include "qed_error.h"
#include "qed.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif 

/**
 * \file QED Symbology
 *
 * Provides functions pertaining to symbol information.
 *
 * The QED 1.0 client API uses a single packet to transmit the complete
 * symbology data.
 *
 * - The symbol list packet is received by multicast when using QED_init.
 * - The user can retrieve the symbol list packet for persistence with
 *   QED_getSymbolListPacket().
 * - the API can be initialized with a persisted packet with QED_initWithPacket().
 * - the user can also choose to receive the symbol list packet using his own
 *   code and initialize the API with QED_initWithPacket().
 *
 * The QED 2.0 client API and later versions  use several packets to transmit
 * the symbology data.  The client API receives the packets, reassembles them in
 * a single buffer and parses them. 
 *
 * - The packets are received over multicast by calling QED_init().
 * - The user can retrieve the reassembled symbol list buffer for persistence with
 *   QED_getSymbolListPacket()
 * - the API can be initialized with a persisted buffer with QED_initWithPacket().
 * - the user can also choose to receive the symbol list packets using his own
 *   code by initializing the API with QED_initWithUserSymbolPackets() and
 *   processing the symbol list packets with QED_processSymbolPacket.
 *
 * In order to be compatible with 1.0 and 2.0 distribution servers, the QED 2.0
 * client API by default attempts to process the now symbol list packets, but
 * will fall back to the legacy 1.0 symbol list if no newer format can be
 * received.
 *
 * \see QED_initWithPacket()
 * \see QED_initWithUserSymbolPackets()
 */
  

/**
 * \brief Get the list of symbols for this context.
 *
 * Get the list of symbols for this context. The caller does not own the memory
 * returned by this method and should not free or modify the symbols.
 *
 *
 * \param ctx The connection context created by \p QED_init()
 * \param symbols pointer to to an array of c strings (char***). This array
 *                will contain the symbols.
 *
 * \return the number of symbols in the list.
 *
 * USAGE:
 * \code{.c}
 * const char** symbols;
 * uint16_t count;
 * int i;
 *
 * count = QED_getSymbolList(ctx, &symbols);
 * for(i = 0; i < count; i++)
 *    printf("Found symbol %s\n", symbols[i];
 * \endcode
 *
 * \see QED_init()
 */
extern uint16_t QED_getSymbolList(QED_ctx ctx, const char*** symbols);

/**
 * \brief Get the channel id associated with the given symbol.
 *
 *
 * \param ctx The connection context created by \p QED_init()
 * \param symbol The symbol
 *
 * \return the channel id for the given symbol
 */
extern uint8_t QED_getSymbolChannelID(QED_ctx ctx, const char* symbol);

/**
 * \brief Get a the symbol list packet for serialization or persistence.
 *
 * Since the QED API receives the symbol list message "under the hood",
 * applications that cannot use the API to work off line with saved or logged
 * data. This method returns a pointer to the symbol list packet and its length
 * to allow applications to persist this data. When running off-line,
 * applications can call <code>QED_initWithPacket()</code> to initialize the API
 * using the serialized packet data.
 *
 * The data returned by this method should not be altered or freed.
 *
 * This function must be called after \p QED_initXXX()
 *
 * \param ctx The QED_ctx for the symbol list.
 * \param data Pointer to a char* that after successful return will point
 * to the symbol list packet.
 * \param len  Pointer to an integer to hold the length of the packet.
 *
 * \return QED_EOK on success
 *
 * ERRORS:
 *  - QED_EINVAL: Invalid argument.
 *
 * USAGE:
 * \code
 * const char *symbolPacket;
 * size_t len;
 *
 * QED_Status status = QED_getSymbolListPacket(ctx, &symbolPacket, &len);
 * \endcode
 *
 * \see QED_initWithPacket
 */
extern QED_Status QED_getSymbolListPacket(QED_ctx ctx,
                                          const char** data,
                                          size_t* len);

/**
 * \brief Provides the context with the data from the symbol list stream.
 * This function should only be called on a context returned by
 * QED_initWithUserSymbolPackets.
 *
 * \param ctx The QED_ctx
 * \param data the content of the UDP packet received on the symbol list stream
 * \param len the length of the UDP packet payload
 * \return QED_EOK on success. The context is now fully initialized.
 * If the return value is QED_EAGAIN, there was not enough data to fully
 * initialize the context. In that case QED_processSymbolPacket needs
 * to be called again with the next packet received from the symbol list stream.
 *
 * ERRORS:
 *  - QED_EAGAIN: More packets needed, see above. This is not a permanent failure.
 *  - QED_EINVAL: Invalid argument.
 *
 */
extern QED_Status QED_processSymbolPacket(QED_ctx ctx,
                                          const char* data,
                                          size_t len);


#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif 

#endif /* QED_SYMBOLOGY_H__ */
