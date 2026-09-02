#ifndef QED_CRYPT_CYPHER_H
#define QED_CRYPT_CYPHER_H

#include "qedc_typedef.h"
#include "qed_error.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif //__cplusplus

////////////////////////////////////////////////////////////////////////////
// cipher structure
////////////////////////////////////////////////////////////////////////////

///@brief struct used to map AES key file
typedef struct  {
  uint8_t  version;
  uint32_t mcastChannel;
  uint8_t  key[128];
} __attribute__((packed)) QEDC_AES_file_st;

///@brief cipher struct used to decrypt one Multicast stream
typedef struct {
  EVP_CIPHER_CTX* aes_ctx;
  bool            initialized;
  uint8_t*        pass_phrase;
  size_t          pass_phrase_len;
  uint64_t        salt;
  uint64_t        ttl;
  uint64_t        counters;
  QEDC_KEY_TYPE   aes_key;
  QEDC_IV_TYPE    iv;
} QEDC_cipher_st;

///@brief Plaintext header struct
typedef union {
    struct{
      uint8_t  ds_extra_header[AES_BLOCK_SIZE];
      uint8_t  version;
      uint8_t  unused[3];
      uint32_t mcastChannel;
      uint64_t saltCnt : 64 - QED_CRYPT_AES_LIFE_TIME_BITS;
      uint64_t aesCnt  : QED_CRYPT_AES_LIFE_TIME_BITS;
    } header __attribute__((packed));

    uint64_t all[QEDC_CRYPT_PLAINTEXT_HEADER_SIZE / sizeof(uint64_t)];
} __attribute__((packed)) QEDC_header_st;

/**
 * @brief initalize a cipher with a pass phrase
 * @param cipher cipher to initialize (must be allocated)
 * @param phrase pass phrase use to generate AES cipher
 * @param phraseLen pass phrase length
 *
 * @return QED_OK if succeed else see ERRORS below:
 *   - QED_EINVAL: one of the input parameters is NULL or already intialized
 *   - QED_EIO   : failure on AES key generation
 */
extern QED_Status QEDC_init_cipher(QEDC_cipher_st* cipher, const uint8_t* phrase, size_t phraseLen);

/**
 * @brief decypt a buffer, it is possible to use the same in/out buffer
 *
 * @param cipher cipher to use to decrypt
 * @param in input buffer (plaintext header + crypted payload)
 * @param out output buffer (decrypted payload only)
 * @param lenBytes length of the input buffer, on succeed it will contain
 * length of the output buffer
 *
 * @return QED_OK if succeed else see ERRORS below:
 *   - QED_EINVAL: one of the input parameters is NULL or not correctly intialized
 *   - QED_ECODEC: library version and input packet header version mismatch
 *   - QED_EIO   : failure on AES key generation
 *                 or failure while applying AES cipher blocks
 */
extern QED_Status QEDC_decrypt(QEDC_cipher_st* cipher, const uint8_t* in, uint8_t* out, size_t* lenBytes);

/**
 * @brief destroy internal data of cipher (do not deallocate cipher itself)
 * @param cipher cipher to clean up
 */
extern void QEDC_destroy_cipher(QEDC_cipher_st* cipher);

#if defined(__cplusplus)
} // extern C
} // namespace QED
#endif //__cplusplus

#endif // QED_CRYPT_CYPHER_H

