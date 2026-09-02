/**
 * Copyright (c) 2014 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_SYMBOL_LIST_H
#define	QED_SYMBOL_LIST_H

#include "Quincy/qed_error.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

typedef struct {
  unsigned int n_packets_received;
  unsigned int n_legacy_packets_received;
  unsigned int index_to_receive;
  unsigned int total_n_messages;
  bool has_only_legacy_symbol_list;
  char *symbol_buffer;
  size_t symbol_buffer_len;
} QEDI_SymbolListContext;

/* The QEDI_SymbolListContext structure handles the state of reception of
 * v3 symbol list packet.
 * The reception is in 2 steps:
 * 1. reception and concatenation of the packets: this is performed
 * by QEDI_processReceivedSymbolListPacket.
 * 2. decoding: this is done with QEDI_decodeSymbolListPacket.
 */

QED_Status QEDI_createSymbolListSocket(QED_Config* cfg,
                                       const char* cfg_name, int *sock);

QED_Status QEDI_decodeSymbolListPacket(QED_ctx ctx, const char* data,
                                       size_t len);

QED_Status QEDI_receiveSymbolListPacket(int sock, char* buff,
                                        size_t buff_len, size_t* len, uint32_t timeout);

typedef QED_Status (*QEDI_SymbolListPacketReceiver)
  (int , char* , size_t , size_t* , uint32_t);

QED_Status QEDI_receiveSymbolList(int sock,
                                  char** buff,
                                  size_t* len,
                                  uint32_t timeout);

QED_Status QEDI_receiveV3SymbolList(int sock,
                                    char** buff,
                                    size_t* len,
                                    uint32_t timeout,
                                    bool *has_only_legacy_symbol_list);


/**
 * Initializes the QEDI_SymbolListContext
 * @param receiveCtx QEDI_SymbolListContext to be initialized
 * @return a QED_Status depending on the success or failure
 */
extern QED_Status QEDI_initSymbolListContext(QEDI_SymbolListContext *receiveCtx);

/**
 * Frees the QEDI_SymbolListContext after use
 * @param receiveCtx EDI_SymbolListContext to be freed
 * @return a QED_Status depending on the success or failure
 */
extern QED_Status QEDI_freeSymbolListContext(QEDI_SymbolListContext *receiveCtx);

/**
 * Process or ignore symbol packet according to its version
 * @param receiveCtx
 * @param receiveBuffer buffer with the symbol list packet
 * @param receivedLen symbol list packet  len
 * @return EOK on success, EAGAIN if more packets are needed
 *
 * On sucess the symbol_buffer and symbol_buffer_len are filled and the user just
 * needs to parse the symbol info from the *symbol_buffer with
 * QEDI_decodeSymbolListPacket
 */
extern QED_Status QEDI_processReceivedSymbolListPacket(QEDI_SymbolListContext *receiveCtx,
                                                       const char *receiveBuffer,
                                                       size_t receivedLen);

/**
 * Returns the buffer with the symbol list.
 * Should only be used after QEDI_processReceivedSymbolListPacket has returned EOK
 * The caller gets the ownership of the buffer and should free it after use.
 * @param receiveCtx
 * @param symbol_buffer ptr where to return the buffer where the symbol list
 * packets have been appended
 * @param symbol_buffer_len ptr where to indicate the length of the symbol list
 * buffer
 */
extern void QEDI_getSymbolListBuffer(QEDI_SymbolListContext *receiveCtx,
                                     char **symbolBuffer,
                                     size_t *symbolBufferLen);


#if defined(__cplusplus)
} // extern "C"
} // namespace QED
#endif

#endif	/* QED_SYMBOL_LIST_H */

