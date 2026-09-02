/**
 * Copyright (c) 2013 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_QEDMAP_H
#define QED_QEDMAP_H

#include "qed_compat.h"

#ifndef INLINE
#define INLINE static inline
#endif

#ifdef __cplusplus
namespace QED {
  extern "C" {
#endif

/**
 * QED_map: C HashMap used for Symbol to ID mapping
 */

/*
 * For internal use only: QED_MapStringKey
 */
typedef struct {
  const char *key;
  uint32_t keyLength;
  char value[8];
} QED_StringKeyValue;

struct QED_StringBucketStruct;

struct QED_StringBucketStruct {
  QED_StringKeyValue *keyValues;
  unsigned int keyValuesSize;
  struct QED_StringBucketStruct *nextBucket;
};
typedef struct QED_StringBucketStruct QED_StringBucket;

typedef struct {
  QED_StringBucket *bucketArray;
  unsigned int bucketArraySize;
  unsigned int defaultListSize;
  size_t valueSize;
  size_t keyValueSize;
} QED_InternalMapStringKey;

typedef struct {
  QED_InternalMapStringKey *map;
  QED_StringKeyValue *keyValue;
  QED_StringBucket *bucket;
  unsigned int bucketI;
  unsigned int arrayI;
} QED_InternalMapStringKeyIterator;

typedef QED_InternalMapStringKey QED_MapStringKey;
typedef QED_InternalMapStringKeyIterator QED_MapStringKeyIterator;

/*
 * For internal use only: QED_MapIntKey
 */
typedef struct {
  uint32_t key;
  char value[8];
} QEDI_IntKeyValue;

struct QEDI_IntBucketStruct;

struct QEDI_IntBucketStruct {
  void *keyValues;
  unsigned int keyValuesSize;
  struct QEDI_IntBucketStruct *nextBucket;
};
typedef struct QEDI_IntBucketStruct QEDI_IntBucket;

typedef struct {
  QEDI_IntBucket *bucketArray;
  unsigned int bucketArraySize;
  unsigned int defaultListSize;
  size_t valueSize;
  size_t keyValueSize;
} QEDI_InternalMapIntKey;

typedef struct {
  QEDI_InternalMapIntKey *map;
  QEDI_IntKeyValue *keyValue;
  QEDI_IntBucket *bucket;
  unsigned int bucketI;
  unsigned int arrayI;
} QEDI_InternalMapIntKeyIterator;

static const uint32_t QEDI_unusedIntKey = 0xffffffff;

typedef QEDI_InternalMapIntKey QEDI_MapIntKey;
typedef QEDI_InternalMapIntKeyIterator QEDI_MapIntKeyIterator;

/**
 * Initialize the map
 *
 * This method initializes a hash map with values of the specified size.
 *
 * \param map pointer to QED_MapStringKey to initialize.
 * \param valueSize Size of values to store in the map
 * \param bucketArraySize The number of buckets in the hash table. This should be
 *        significantly (10x) larger than number of items expected in the map.
 * \param defaultListSize The starting size for the lists.
 */
extern void QED_initializeMapStringKey(QED_MapStringKey *map,
    size_t valueSize,
    unsigned int bucketArraySize,
    unsigned int defaultListSize);

/**
 * Initialize the map
 *
 * This method initializes a hash map with pointer values.
 *
 * \param map pointer to QED_MapStringKey to initialize.
 * \param bucketArraySize The number of buckets in the hash table. This should be
 *        significantly (10x) larger than number of items expected in the map.
 * \param defaultListSize The starting size for the lists.
 */
extern void QED_initializeMapStringKeyPtrData(QED_MapStringKey *map,
    unsigned int bucketArraySize,
    unsigned int defaultListSize);

/*
 * Free a map.
 *
 * \param map pointer to the map to free.
 */
extern void QED_freeMapStringKey(QED_MapStringKey *map);

/**
 * Adds a new key,value.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param QED_MapStringKey The HashMap you want to write to
 * \param key The Key as a null-terminated char*
 * \param value The value
 *
 */
INLINE void QED_putInMapStringKeyC(QED_MapStringKey *map, const char *key, const
                                   void *value);
/**
 * Adds a new key, pointer value.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param QED_MapStringKey The HashMap you want to write to
 * \param key The Key as a null-terminated char*
 * \param value The value which is a pointer type
 *
 */
INLINE void QED_putInMapStringKeyPtrDataC(QED_MapStringKey *map,
                                          const char *key, const void *value);

/**
 * Gets a value given a key.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param QED_MapStringKey The HashMap you want to get the data from
 * \param key The Key as a char*
 * \param keyLength The length of the Key
 *
 * \return Pointer to value if found, NULL otherwise.
 */
INLINE void* QED_getInMapStringKey(QED_MapStringKey *map, const char *key,
                                   uint32_t keyLength);

/**
 * Gets a pointer value given a key.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param QED_MapStringKey The HashMap you want to get the data from
 * \param key The Key as a char*
 * 
 * \return Pointer to value if found, NULL otherwise.
 */
INLINE void* QED_getInMapStringKeyPtrDataC(QED_MapStringKey *map,
                                           const char *key);
/**
 * Gets a value given a key.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param QED_MapStringKey The HashMap.
 * \param key The Key as a null-terminated char*
 * 
 * \return Pointer to value if found, NULL otherwise.
 */
INLINE void* QED_getInMapStringKeyC(QED_MapStringKey *map, const char *key);

/**
 * Remove a value from the map
 *
 * \param map Pointer to the map.
 * \param key The corresponding to the value to remove.
 *
 * \return a pointer to the removed value.
 */
INLINE void *QED_deleteInMapStringKeyC(QED_MapStringKey *map, const char *key);

/**
 * Remove a pointer value from the map
 *
 * \param map Pointer to the map.
 * \param key The corresponding to the value to remove.
 *
 * \return the removed pointer.
 */
INLINE void *QED_deleteInMapStringKeyPtrDataC(QED_MapStringKey *map,
                                              const char *key);

/**
 * Clear and free all the keys
 *
 * \param QED_MapStringKey The HashMap.
 */
extern void QED_clearMapStringKeyPtrData(QED_MapStringKey *map);

/*
 * String hash function
 *
 * by Paul Hsieh http://www.azillionmonkeys.com/qed/hash.html
 */
extern uint32_t QED_hashStringKey(const char *key, int keyLength);

/**
 * Initialize a map iterator.
 *
 * \param map The map
 * \param iterator Pointer to iterator structure to initialize
 */
INLINE void QED_getIteratorMapStringKey(QED_MapStringKey *map,
                                        QED_MapStringKeyIterator *iterator);
/**
 * Increment the iterator
 *
 * \param QED_MapStringKeyIterator The iterator
 *
 * \return true if there is more data
 */
INLINE bool QED_nextIteratorMapStringKey(QED_MapStringKeyIterator *iterator);

/**
 * Test iterator value
 *
 * Test if there is a value associated with the current key in the iteration.
 *
 * \param iterator The iterator.
 */
INLINE bool QED_hasValueIteratorMapStringKey(QED_MapStringKeyIterator *iterator);

/**
 * Get the iterator value
 *
 * Return a pointer to the current value of the iterator.
 *
 * \param iterator The iterator.
 * \return Pointer to the current value.
 */
INLINE void* QED_getValueIteratorMapStringKey(QED_MapStringKeyIterator *iterator);

/**
 * Get the iterator pointer value
 *
 * Return the value.
 *
 * \param iterator The iterator.
 * \return The current value.
 */
INLINE void* QED_getValueIteratorMapStringKeyPtrData(QED_MapStringKeyIterator *iterator);

/**
 * Get the key for the current iterator position
 *
 * \param iterator The iterator.
 * \return they key
 */
INLINE const char* QED_getKeyIteratorMapStringKey(QED_MapStringKeyIterator *iterator);

/**
 * Return the length of the key at the current iterator position.
 *
 * \param iterator The iterator.
 * \return The key length.
 */
INLINE uint32_t QED_getKeyLengthIteratorMapStringKey(QED_MapStringKeyIterator *iterator);

/**
 * Delete the item at the current iterator position.
 *
 * \param iterator The iterator 
 */
INLINE bool QED_deleteIteratorMapStringKey(QED_MapStringKeyIterator *iterator);

/**
 * Get the value associated with the key
 *
 * \param QED_MapStringKey The HashMap
 * \param QED_StringBucket The bucket associated with the given hash
 * \param i location in the bucket for a given key
 *
 * \return QED_StringKeyValue
 */
INLINE QED_StringKeyValue *QED_getStringKeyValue(QED_MapStringKey *map,
                                                 QED_StringBucket *bucket,
                                                 unsigned int i);

/**
 * Initialize the map
 *
 * This method initializes a hash map with values of the specified size.
 *
 * \param map pointer to map to initialize.
 * \param valueSize Size of values to store in the map
 * \param bucketArraySize The number of buckets in the hash table. This should be
 *        significantly (10x) larger than number of items expected in the map.
 * \param defaultListSize The starting size for the lists.
 */
extern void QEDI_initializeMapIntKey(QEDI_MapIntKey *map,
                                    size_t valueSize,
                                    unsigned int bucketArraySize,
                                    unsigned int defaultListSize);

/**
 * Initialize the map
 *
 * This method initializes a hash map with pointer values.
 *
 * \param map pointer to map to initialize.
 * \param bucketArraySize The number of buckets in the hash table. This should be
 *        significantly (10x) larger than number of items expected in the map.
 * \param defaultListSize The starting size for the lists.
 */
extern void QEDI_initializeMapIntKeyPtrData(QEDI_MapIntKey *map,
                                           unsigned int bucketArraySize,
                                           unsigned int defaultListSize);

/*
 * Free a map.
 *
 * \param map pointer to the map to free.
 */
extern void QEDI_freeMapIntKey(QEDI_MapIntKey *map);

/**
 * Adds a new key,value.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param map The HashMap you want to write to
 * \param key The Key
 * \param value The value
 *
 */
INLINE void QEDI_putInMapIntKey(QEDI_MapIntKey *map, uint32_t key, const void *value);

/**
 * Adds a new key, pointer value.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param map The HashMap you want to write to
 * \param key The Key
 * \param value The value which is a pointer type
 *
 */
INLINE void QEDI_putInMapIntKeyPtrData(QEDI_MapIntKey *map, uint32_t key, const void *value);

/**
 * Gets a value given a key.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param map The HashMap you want to get the data from
 * \param key The Key
 *
 * \return Pointer to value if found, NULL otherwise.
 */
INLINE void* QEDI_getInMapIntKey(QEDI_MapIntKey *map, uint32_t key);

/**
 * Gets a pointer value given a key.
 *
 * Caution: doesn't replace the existing value if the key was already present.
 * When returning void*, returns a pointer to the stored data
 *
 * \param map The HashMap you want to get the data from
 * \param key The Key
 *
 * \return Pointer to value if found, NULL otherwise.
 */
INLINE void* QEDI_getInMapIntKeyPtrData(QEDI_MapIntKey *map, uint32_t key);

/**
 * Remove a pointer value from the map
 *
 * \param map Pointer to the map.
 * \param key The corresponding to the value to remove.
 *
 * \return the removed pointer.
 */
INLINE void* QEDI_deleteInMapIntKey(QEDI_MapIntKey *map, uint32_t key);

/**
 * Clear and free all the keys
 *
 * \param map The HashMap.
 */
extern void QEDI_clearMapIntKey(QEDI_MapIntKey *map);

/*
 * Int hash function
 *
 * by Donald KNUTH (Knuth multiplicative)
 */
INLINE unsigned int QEDI_hashIntKey(uint32_t key);

/**
 * Initialize a map iterator.
 *
 * \param map The map
 * \param iterator Pointer to iterator structure to initialize
 */
INLINE void QEDI_getIteratorMapIntKey(QEDI_MapIntKey *map, QEDI_MapIntKeyIterator *iterator);

/**
 * Increment the iterator
 *
 * \param iterator The iterator
 *
 * \return true if there is more data
 */
INLINE bool QEDI_nextIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator);

/**
 * Test iterator value
 *
 * Test if there is a value associated with the current key in the iteration.
 *
 * \param iterator The iterator.
 */
INLINE bool QEDI_hasValueIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator);

/**
 * Get the iterator value
 *
 * Return a pointer to the current value of the iterator.
 *
 * \param iterator The iterator.
 * \return Pointer to the current value.
 */
INLINE void* QEDI_getValueIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator);

/**
 * Get the iterator pointer value
 *
 * Return the value.
 *
 * \param iterator The iterator.
 * \return The current value.
 */
INLINE void* QEDI_getValueIteratorMapIntKeyPtrData(QEDI_MapIntKeyIterator *iterator);

/**
 * Get the key for the current iterator position
 *
 * \param iterator The iterator.
 * \return they key
 */
INLINE uint32_t QEDI_getKeyIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator);

/*
 * Inline MapStringKey implementation
 */

INLINE QED_StringKeyValue *QED_getStringKeyValue(QED_MapStringKey *map,
                                                 QED_StringBucket *bucket,
                                                 unsigned int i) {
  char *keyValueAddress = (char*) bucket->keyValues + (i * map->keyValueSize);
  return (QED_StringKeyValue *) keyValueAddress;
}

INLINE void QED_initializeStringKeyValue(QED_StringKeyValue *keyValue) {
  keyValue->key = NULL;
  keyValue->keyLength = QEDI_unusedIntKey;
}

INLINE bool QED_equalsStringKeyValue(QED_StringKeyValue *keyValue,
                                     const char *key2, uint32_t key2Length) {
  return (keyValue->keyLength == key2Length)
      && ((keyValue->key == key2)
      || (memcmp(keyValue->key, key2, key2Length) == 0));
}

INLINE QED_StringKeyValue *QED_findKeyValueInMapStringKey(QED_MapStringKey *map,
                                                          const char *key,
                                                          uint32_t keyLength) {
  const uint32_t hash = QED_hashStringKey(key, keyLength);
  const unsigned int bucketI = hash % map->bucketArraySize;
  QED_StringBucket *bucket = &(map->bucketArray[bucketI]);
  while (bucket != NULL) {
    for (unsigned int keyValueI = 0; keyValueI < bucket->keyValuesSize; keyValueI++) {
      QED_StringKeyValue *keyValue = QED_getStringKeyValue(map, bucket, keyValueI);
      if (QED_equalsStringKeyValue(keyValue, key, keyLength)) {
        return keyValue;
      }
    }
    bucket = bucket->nextBucket;
  }
  return NULL;
}

INLINE void* QED_addInMapStringKey(QED_MapStringKey *map, const char *key,
                                   uint32_t keyLength) {
  const uint32_t hash = QED_hashStringKey(key, keyLength);
  const unsigned int bucketI = hash % map->bucketArraySize;
  unsigned int keyValueI = 0;
  QED_StringBucket *bucket = &(map->bucketArray[bucketI]);
  while (true) {
    if (keyValueI >= bucket->keyValuesSize) {
      keyValueI = 0;
      if (bucket->nextBucket == NULL) {
        unsigned int newListSize = bucket->keyValuesSize + map->defaultListSize;
        bucket->nextBucket = (QED_StringBucket*) malloc(sizeof (QED_StringBucket));
        bucket->nextBucket->keyValues =
          (QED_StringKeyValue *) malloc(newListSize * map->keyValueSize);
        bucket->nextBucket->keyValuesSize = newListSize;
        bucket->nextBucket->nextBucket = NULL;
        for (unsigned int keyValueI2 = 0; keyValueI2 < newListSize; keyValueI2++) {
          QED_initializeStringKeyValue(QED_getStringKeyValue(map,
            bucket->nextBucket, keyValueI2));
        }
      }
      bucket = bucket->nextBucket;
    }
    {
      QED_StringKeyValue *keyValue = QED_getStringKeyValue(map, bucket,
        keyValueI);

      if (keyValue->keyLength == QEDI_unusedIntKey) {
        keyValue->key = (const char *) malloc(keyLength);
        memcpy((char *) keyValue->key, key, keyLength);
        keyValue->keyLength = keyLength;
        return &(keyValue->value[0]);
      }
    }
    keyValueI++;
  }
}

INLINE void QED_putInMapStringKey(QED_MapStringKey *map, const char *key,
                                  uint32_t keyLength, const void *value) {
  void *dest = QED_addInMapStringKey(map, key, keyLength);
  memcpy(dest, value, map->valueSize);
}

INLINE void QED_putInMapStringKeyC(QED_MapStringKey *map, const char *key,
                                   const void *value) {
  QED_putInMapStringKey(map, key, strlen(key), value);
}

INLINE void QED_putInMapStringKeyPtrData(QED_MapStringKey *map, const char *key,
                                         uint32_t keyLength, const void *value){
  void **dest = (void**) QED_addInMapStringKey(map, key, keyLength);
  *dest = (void *) value;
}

INLINE void QED_putInMapStringKeyPtrDataC(QED_MapStringKey *map,
                                          const char *key, const void *value) {
  QED_putInMapStringKeyPtrData(map, key, strlen(key), value);
}

INLINE void* QED_getInMapStringKey(QED_MapStringKey *map, const char *key, uint32_t keyLength) {
  QED_StringKeyValue *keyValue = QED_findKeyValueInMapStringKey(map, key, keyLength);
  if (keyValue == NULL) {
    return NULL;
  }
  return (void*) (&(keyValue->value[0]));
}

INLINE void* QED_getInMapStringKeyC(QED_MapStringKey *map, const char *key) {
  return QED_getInMapStringKey(map, key, strlen(key));
}

INLINE void* QED_getInMapStringKeyPtrData(QED_MapStringKey *map,
                                          const char *key, uint32_t keyLength) {
  void **res = (void **) QED_getInMapStringKey(map, key, keyLength);
  return (res == NULL) ? NULL : *res;
}

INLINE void* QED_getInMapStringKeyPtrDataC(QED_MapStringKey *map,
                                           const char *key) {
  return QED_getInMapStringKeyPtrData(map, key, strlen(key));
}

INLINE void *QED_deleteInMapStringKey(QED_MapStringKey *map, const char *key,
                                      uint32_t keyLength) {
  QED_StringKeyValue *keyValue = QED_findKeyValueInMapStringKey(map, key,
    keyLength);

  if (keyValue == NULL) {
    return NULL;
  }
  if (keyValue->key != NULL) {
    free((void*)keyValue->key);
    keyValue->key = NULL;
  }
  QED_initializeStringKeyValue(keyValue);
  return (void*) (&(keyValue->value[0]));
}

INLINE void *QED_deleteInMapStringKeyC(QED_MapStringKey *map, const char *key) {
  return QED_deleteInMapStringKey(map, key, strlen(key));
}

INLINE void *QED_deleteInMapStringKeyPtrData(QED_MapStringKey *map,
                                             const char *key,
                                             uint32_t keyLength) {
  void **res = (void **) QED_deleteInMapStringKey(map, key, keyLength);
  return (res == NULL) ? NULL : *res;
}

INLINE void *QED_deleteInMapStringKeyPtrDataC(QED_MapStringKey *map,
                                              const char *key) {
  return QED_deleteInMapStringKeyPtrData(map, key, strlen(key));
}

INLINE void QED_getIteratorMapStringKey(QED_MapStringKey *map,
                                        QED_MapStringKeyIterator *iterator) {
  iterator->map = map;
  iterator->bucketI = 0;
  iterator->arrayI = 0;
  iterator->bucket = &(map->bucketArray[0]);
  QED_nextIteratorMapStringKey(iterator);
}

INLINE bool QED_internalNextIteratorMapStringKey(QED_MapStringKeyIterator *iterator) {
  QED_MapStringKey *map = iterator->map;
  iterator->keyValue = NULL;
  while (true) {
    if (iterator->bucket != NULL) {
      for (unsigned int arrayI = iterator->arrayI; arrayI < iterator->bucket->keyValuesSize; arrayI++) {
        QED_StringKeyValue *keyValue = QED_getStringKeyValue(map, iterator->bucket, arrayI);
        if (keyValue->keyLength != QEDI_unusedIntKey) {
          iterator->keyValue = keyValue;
          // Set the next position
          iterator->arrayI = arrayI + 1;
          if (iterator->arrayI >= iterator->bucket->keyValuesSize) {
            iterator->arrayI = 0;
            iterator->bucket = iterator->bucket->nextBucket;
          }
          return true;
        }
      }
      iterator->bucket = iterator->bucket->nextBucket;
      iterator->arrayI = 0;
    } else {
      iterator->bucketI++;
      if (iterator->bucketI >= map->bucketArraySize) {
        return false;
      }
      iterator->bucket = &(map->bucketArray[iterator->bucketI]);
      iterator->arrayI = 0;
    }
  }
}

INLINE bool QED_nextIteratorMapStringKey(QED_MapStringKeyIterator *iterator) {
  return QED_internalNextIteratorMapStringKey(iterator);
}

INLINE bool QED_deleteIteratorMapStringKey(QED_MapStringKeyIterator *iterator) {
  if (iterator->keyValue != NULL) {
    iterator->keyValue->keyLength = QEDI_unusedIntKey;
    if (iterator->keyValue->key != NULL) {
      free((void*)iterator->keyValue->key);
      iterator->keyValue->key = NULL;
    }
  }
  return QED_internalNextIteratorMapStringKey(iterator);
}

INLINE bool QED_hasValueIteratorMapStringKey(QED_MapStringKeyIterator *iterator) {
  return iterator->keyValue != NULL;
}

INLINE void* QED_getValueIteratorMapStringKey(QED_MapStringKeyIterator *iterator) {
  return (void*) (&(iterator->keyValue->value[0]));
}

INLINE void* QED_getValueIteratorMapStringKeyPtrData(QED_MapStringKeyIterator *iterator) {
  void **res = (void **) QED_getValueIteratorMapStringKey(iterator);
  return (res == NULL) ? NULL : *res;
}

INLINE const char* QED_getKeyIteratorMapStringKey(QED_MapStringKeyIterator *iterator) {
  return iterator->keyValue->key;
}

INLINE uint32_t QED_getKeyLengthIteratorMapStringKey(QED_MapStringKeyIterator *iterator) {
  return iterator->keyValue->keyLength;
}

/*
 * Inline MapIntKey implementation
 */

INLINE uint32_t QEDI_hashIntKey(uint32_t key) {
  /* Knuth multiplicative */
  return key * 2654435761;
}

INLINE void QEDI_initializeIntKeyValue(QEDI_IntKeyValue *keyValue) {
  keyValue->key = QEDI_unusedIntKey;
}

INLINE QEDI_IntKeyValue *QEDI_getIntKeyValue(QEDI_MapIntKey *map, QEDI_IntBucket *bucket, unsigned int i) {
  char *keyValueAddress = (char*)bucket->keyValues + (i * map->keyValueSize);
  return (QEDI_IntKeyValue *)keyValueAddress;
}

INLINE QEDI_IntKeyValue *QEDI_findKeyValueInMapIntKey(QEDI_MapIntKey *map, uint32_t key) {
  const uint32_t hash = QEDI_hashIntKey(key);
  const unsigned int bucketI = hash % map->bucketArraySize;
  QEDI_IntBucket *bucket = &(map->bucketArray[bucketI]);
  while (bucket != NULL) {
    for (unsigned int keyValueI = 0; keyValueI < bucket->keyValuesSize; keyValueI++) {
      QEDI_IntKeyValue *keyValue = QEDI_getIntKeyValue(map, bucket, keyValueI);
      if (keyValue->key == key) {
        return keyValue;
      }
    }
    bucket = bucket->nextBucket;
  }
  return NULL;
}

INLINE void* QEDI_addInMapIntKey(QEDI_MapIntKey *map, uint32_t key) {
  const uint32_t hash = QEDI_hashIntKey(key);
  const unsigned int bucketI = hash % map->bucketArraySize;
  unsigned int keyValueI = 0;
  QEDI_IntBucket *bucket = &(map->bucketArray[bucketI]);
  while (true) {
    if (keyValueI >= bucket->keyValuesSize) {
      keyValueI = 0;
      if (bucket->nextBucket == NULL) {
        unsigned int newListSize = bucket->keyValuesSize + map->defaultListSize;
        bucket->nextBucket = (QEDI_IntBucket*)malloc(sizeof (QEDI_IntBucket));
        bucket->nextBucket->keyValues = malloc(newListSize * map->keyValueSize);
        bucket->nextBucket->keyValuesSize = newListSize;
        bucket->nextBucket->nextBucket = NULL;
        for (unsigned int keyValueI2 = 0; keyValueI2 < newListSize; keyValueI2++) {
          QEDI_initializeIntKeyValue(QEDI_getIntKeyValue(map, bucket->nextBucket, keyValueI2));
        }
      }
      bucket = bucket->nextBucket;
    }
    {
      QEDI_IntKeyValue *keyValue = QEDI_getIntKeyValue(map, bucket, keyValueI);
      if (keyValue->key == QEDI_unusedIntKey) {
        keyValue->key = key;
        return &(keyValue->value[0]);
      }
    }
    keyValueI++;
  }
}

INLINE void QEDI_putInMapIntKey(QEDI_MapIntKey *map, uint32_t key, const void *value) {
  void *dest = QEDI_addInMapIntKey(map, key);
  memcpy(dest, value, map->valueSize);
}

INLINE void QEDI_putInMapIntKeyPtrData(QEDI_MapIntKey *map, uint32_t key, const void *value) {
  void **dest = (void **)QEDI_addInMapIntKey(map, key);
  *dest = (void *)value;
}

INLINE void* QEDI_getInMapIntKey(QEDI_MapIntKey *map, uint32_t key) {
  QEDI_IntKeyValue *keyValue = QEDI_findKeyValueInMapIntKey(map, key);
  if (keyValue == NULL) {
    return NULL;
  }
  return (void*)(&(keyValue->value[0]));
}

INLINE void* QEDI_getInMapIntKeyPtrData(QEDI_MapIntKey *map, uint32_t key) {
  void **res = (void**)QEDI_getInMapIntKey(map, key);
  return (res == NULL) ? NULL : *res;
}

INLINE void* QEDI_internalDeleteInMapIntKey(QEDI_MapIntKey *map, uint32_t key) {
  QEDI_IntKeyValue *keyValue = QEDI_findKeyValueInMapIntKey(map, key);
  if (keyValue == NULL) {
    return NULL;
  }
  QEDI_initializeIntKeyValue(keyValue);
  return (void*)(&(keyValue->value[0]));
}

INLINE void* QEDI_deleteInMapIntKey(QEDI_MapIntKey *map, uint32_t key) {
  return QEDI_internalDeleteInMapIntKey(map, key);
}

INLINE void* QEDI_deleteInMapIntKeyPtrData(QEDI_MapIntKey *map, uint32_t key) {
  void **res = (void**)QEDI_internalDeleteInMapIntKey(map, key);
  return (res == NULL) ? NULL : *res;
}

INLINE void QEDI_getIteratorMapIntKey(QEDI_MapIntKey *map, QEDI_MapIntKeyIterator *iterator) {
  iterator->map = map;
  iterator->bucketI = 0;
  iterator->arrayI = 0;
  iterator->bucket = &(map->bucketArray[0]);
  QEDI_nextIteratorMapIntKey(iterator);
}

INLINE bool QEDI_internalNextIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator) {
  QEDI_MapIntKey *map = iterator->map;
  iterator->keyValue = NULL;
  while (true) {
    if (iterator->bucket != NULL) {
      for (unsigned int arrayI = iterator->arrayI; arrayI < iterator->bucket->keyValuesSize; arrayI++) {
        QEDI_IntKeyValue *keyValue = QEDI_getIntKeyValue(map, iterator->bucket, arrayI);
        if (keyValue->key != QEDI_unusedIntKey) {
          iterator->keyValue = keyValue;
          // Set the next position
          iterator->arrayI = arrayI + 1;
          if (iterator->arrayI >= iterator->bucket->keyValuesSize) {
            iterator->arrayI = 0;
            iterator->bucket = iterator->bucket->nextBucket;
          }
          return true;
        }
      }
      iterator->bucket = iterator->bucket->nextBucket;
      iterator->arrayI = 0;
    } else {
      iterator->bucketI++;
      if (iterator->bucketI >= map->bucketArraySize) {
        return false;
      }
      iterator->bucket = &(map->bucketArray[iterator->bucketI]);
      iterator->arrayI = 0;
    }
  }
}

INLINE bool QEDI_nextIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator) {
  return QEDI_internalNextIteratorMapIntKey(iterator);
}

INLINE bool QEDI_hasValueIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator) {
  return iterator->keyValue != NULL;
}

INLINE void* QEDI_getValueIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator) {
  return (void*)&(iterator->keyValue->value[0]);
}

INLINE void* QEDI_getValueIteratorMapIntKeyPtrData(QEDI_MapIntKeyIterator *iterator) {
  void **res = (void**)(&(iterator->keyValue->value[0]));
  return *res;
}

INLINE uint32_t QEDI_getKeyIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator) {
  return iterator->keyValue->key;
}

INLINE bool QEDI_deleteIteratorMapIntKey(QEDI_MapIntKeyIterator *iterator) {
  if (iterator->keyValue != NULL) {
    iterator->keyValue->key = QEDI_unusedIntKey;
  }
  return QEDI_internalNextIteratorMapIntKey(iterator);
}

#ifdef __cplusplus
  }
}
#endif

#endif /* QED_QEDMAP_H__ */
