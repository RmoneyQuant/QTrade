/**
 * Copyright (c) 2015 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_CRYPT_CONTEXT_H
#define QED_CRYPT_CONTEXT_H

#include "qed_compat.h"
#include "qed_error.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif //__cplusplus

#define QEDC_MAGIC_SIZE 4
#define QEDC_PAYLOAD_OFFSET QEDC_MAGIC_SIZE
typedef struct QEDC_context_st* QEDC_context;

/**
 * @brief clean up internal context
 * @param ctx context to clean up
 */
extern void QEDC_destroy_context(QEDC_context* ctx);

/**
 * @brief initialize a context
 * @param ctx context to initialize
 * @param keys_folder keys Folder containing AES keys files. All files with
 * ".bin" suffix will be loaded.
 * @return QED_OK if succeed else see ERRORS below:
 *   - QED_EINVAL: one of the input parameters is NULL or not correctly intialized
 *   - QED_EIO   : failure on AES key generation
 *   - QED_ENOMEM: ran out of memory
 *   - QED_ECODEC : AES file mismatch version with library one
 *   - QED_EBADCFG: failure while accessing to folder pointed by @r keys_folder
 *                  or failure while accessing to AES file in folder
 *   - QED_EIO    : unable to mmap a file found in keys_folder
 */
extern QED_Status QEDC_init_context(QEDC_context* ctx,
                                    const char* keys_folder);

/**
 * @brief decypt a buffer, it is possible to use the same in/out buffer
 *
 * you must apply QEDC_PAYLOAD_OFFSET as offset to the output buffer before calling
 * QED_parser_begin
 *
 * ex: QED_parser_begin(parser, out + QEDC_PAYLOAD_OFFSET, *lenBytes - QEDC_PAYLOAD_OFFSET)
 *
 * @param ctx context containing the ciphers to use to decrypt
 * @param in input buffer (plaintext header + crypted payload)
 * @param out output buffer (decrypted payload only) with allocated length
 * greater or equal to the one of the input buffer. Must be distinct from
 * the input buffer.
 * @param lenBytes length of the input buffer, on success it will contain
 * length of the output buffer
 *
 * @return QED_OK if succeeded else see ERRORS below:
 *   - QED_EINVAL: one of the input parameters is NULL or not correctly intialized
 *   - QED_ECODEC: library version and input packet header version mismatch
 *   - QED_EIO   : failure on AES key generation
 *                 or failure while applying AES cipher blocks
 */
extern QED_Status QEDC_decrypt_packet(QEDC_context ctx,
                                      const uint8_t* in,
                                      uint8_t* out,
                                      size_t* lenBytes);

/**
 * @brief check if packet is an encrypted one
 * @return true if packet must be decrypted, false otherwise
 */
extern bool QEDC_is_encrypted_packet(const uint8_t* in, size_t lenBytes);

/**
 * @brief check if context has been initialized
 * @param ctx context
 * @return true if ctx is correctly initialized, false otherwise
 */
extern bool QEDC_context_get_init_state(const QEDC_context ctx);

/**
 * @brief returns numbers of AES key files found in AES folder while initializing
 * @param ctx context
 * @return -1 if context is invalid, number of files otherwise
 */
extern int32_t QEDC_context_get_nb_files_found(const QEDC_context ctx);

/**
 * @brief returns the AES folder path used to initialize the context
 * @param ctx context
 * @return NULL if context is invalid, the path otherwise
 */
extern const char* QEDC_context_get_init_folder_path(const QEDC_context ctx);

#if defined(__cplusplus)
} // extern C
} // namespace QED
#endif //__cplusplus

#endif // QED_CRYPT_CONTEXT_H

