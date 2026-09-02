/**
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_PRICE_H__
#define QED_PRICE_H__

#include "qed_compat.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

/**
 * The \p QED_Price type is a packed structure containing scaled decimal. The
 * value is <code>price.man * exp10(price.exp)</code>
 */
typedef struct {
  int64_t    man;
  int8_t     exp;
} __attribute__((packed)) QED_Price;

static inline int64_t QED_price_mantissa(QED_Price price){
  return price.man;
}

static inline int8_t QED_price_exponent(QED_Price price){
  return price.exp;
}

static inline int QED_price_is_valid(QED_Price price){
  return (!(INT8_MIN == price.exp && INT64_MIN == price.man));
}

static inline double QED_price_as_double(QED_Price price){
  return (double)(price.man) * pow_base_10((double)(price.exp));
}

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif

#endif /* QED_PRICE_H__ */
