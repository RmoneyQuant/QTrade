#ifndef QED_CRYPT_TYPEDEF_H
#define QED_CRYPT_TYPEDEF_H

#include <openssl/aes.h>
#include <openssl/evp.h>
#include "qed_compat.h"

#ifdef __GNUC__
#define likely(x) __builtin_expect((x),1)
#define unlikely(x) __builtin_expect((x),0)
#else
#define likely(x) (x)
#define unlikely(x) (x)
#endif

// We want to align on 128 bits
// for SSE3 or AVX vector instructions
#undef  ALIGNED
#define ALIGNED aligned(16)

#define QEDC_CRYPT_PROTOCOL_VERSION 1
#define QED_CRYPT_AES_LIFE_TIME_BITS 20
#define QEDC_CRYPT_AES_LIFE_TIME (1 << QED_CRYPT_AES_LIFE_TIME_BITS)
#define QED_CRYPT_PLAINTEXT_EXTRA_DS_HEADER_AES_SIZE 1
#define QEDC_CRYPT_PLAINTEXT_HEADER_SIZE \
  ((QED_CRYPT_PLAINTEXT_EXTRA_DS_HEADER_AES_SIZE * AES_BLOCK_SIZE) + AES_BLOCK_SIZE)

#define QEDC_KEY_LEN_BITS 128
#define QEDC_KEY_LEN_BYTES (QEDC_KEY_LEN_BITS >> 3)

typedef const EVP_MD* (*QED_HASH_FUNC_TYPE)();
typedef const EVP_CIPHER* (*QED_CIPHER_FUNC_TYPE)();
typedef uint8_t QEDC_KEY_TYPE[QEDC_KEY_LEN_BYTES] __attribute__((ALIGNED));
typedef uint8_t QEDC_IV_TYPE[QEDC_KEY_LEN_BYTES] __attribute__((ALIGNED));

#endif // QED_CRYPT_TYPEDEF_H


