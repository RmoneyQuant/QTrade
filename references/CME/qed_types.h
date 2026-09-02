/**
 * Copyright (c) 2014 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_TYPES_H__
#define QED_TYPES_H__

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

/**
 * Closure information
 * Carries a pointer to the user defined closure as well as the
 * QED symbol name as a null terminated c style string.
 */
typedef struct {
  const char *symbol;
  void *closure;
} QED_closure_info;

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif

#endif /* QED_TYPES_H__ */
