/**
 * Copyright (c) 2013 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_BITSET_H__
#define QED_BITSET_H__

#include "qed_compat.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

 /**
 * QED_BitSet: This is a very simple bitset implementation used to efficiently manage
 * subscriptions and filter messages from incoming packets.
 */

/*
 * For internal use only
 */
typedef struct {
  unsigned char *bitSet;
  unsigned int sizeInBits;
  unsigned int sizeInBytes;
} QED_BitSet;

/**
 * Initialize the bit set.
 *
 * This must be called before any other QED_BitSet operations.
 *
 * \param bitSet A pointer to the QED_BitSet struct to initialize.
 * \param sizeInBits The length of the bit set.
 * \return none.
 */
INLINE void QED_initializeBitSet(QED_BitSet *bitSet, unsigned int sizeInBits);

/**
 * Free the bit set.
 *
 * Free any resources  allocated in QED_initializeBitSet.
 *
 * \see QED_initializeBitSet
 *
 * \param bitSet Pointer to the bitSet to free.
 * \return none
 */
INLINE void QED_freeBitSet(QED_BitSet *bitSet);

/**
 * Test a bit.
 *
 * Test the value of the bit at the specified 0 based index.
 *
 * \param bitSet pointer.
 * \param bitIndex 0 based offset of bit to test.
 * \return true if bit is set otherwise false.
 */
INLINE bool QED_bitSetIsSet(QED_BitSet *bitSet, unsigned int bitIndex);

/**
 * Set a bit.
 *
 * Set the specified bit to true.
 *
 * \param bitSet pointer.
 * \param bitIndex 0 based offset of bit to set.
 * \return none.
 */
INLINE void QED_setBitSet(QED_BitSet *bitSet, unsigned int bitIndex);

/**
 * Reset a bit.
 *
 * Reset the specified bit to false.
 *
 * \param bitSet pointer.
 * \param bitIndex 0 based offset of bit to reset.
 * \return none.
 */
INLINE void QED_resetBitSet(QED_BitSet *bitSet, unsigned int bitIndex);

/**
 * Reset all bits
 *
 * Resets all bits in the bit set to false.
 *
 * \param bitSet pointer.
 * \return none.
 */
INLINE void QED_resetAllBitSet(QED_BitSet *bitSet);

/**
 * Inline  QED_BitSet implementation
 */

INLINE void QED_initializeBitSet(QED_BitSet *bitSet, unsigned int sizeInBits) {
  bitSet->sizeInBits = sizeInBits;
  bitSet->sizeInBytes = sizeInBits / 8 + 1;
  bitSet->bitSet = (unsigned char*)malloc(bitSet->sizeInBytes);
  bzero(bitSet->bitSet, bitSet->sizeInBytes);
}

INLINE void QED_freeBitSet(QED_BitSet *bitSet) {
  free(bitSet->bitSet);
  bitSet->bitSet = NULL;
}

INLINE bool QED_bitSetIsSet(QED_BitSet *bitSet, unsigned int bitIndex) {
  if (bitIndex < bitSet->sizeInBits) {
    unsigned char mask = 1 << (bitIndex % 8);
    return (bitSet->bitSet[bitIndex / 8] & mask) != 0;
  } else {
    return false;
  }
}

INLINE void QED_setBitSet(QED_BitSet *bitSet, unsigned int bitIndex) {
  if (bitIndex < bitSet->sizeInBits) {
    unsigned char mask = 1 << (bitIndex % 8);
    bitSet->bitSet[bitIndex / 8] |= mask;
  }
}
    
INLINE void QED_resetBitSet(QED_BitSet *bitSet, unsigned int bitIndex) {
  if (bitIndex < bitSet->sizeInBits) {
    unsigned char mask = 1 << (bitIndex % 8);
    bitSet->bitSet[bitIndex / 8] &= ~mask;
  }
}

INLINE void QED_resetAllBitSet(QED_BitSet *bitSet) {
  bzero(bitSet->bitSet, bitSet->sizeInBytes);
}

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif

#endif /* QED_BITSET_H__ */
