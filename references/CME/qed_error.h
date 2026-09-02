/**
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_ERROR_H__
#define QED_ERROR_H__

#include "qed_compat.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

/**
 * Error values
 */
typedef enum {
  QED_EOK = 0,
  QED_EINVAL,
  QED_ENOCFG,
  QED_EBADCFG,
  QED_ESOCK,
  QED_ECODEC,
  QED_EBADMSG,
  QED_ETIMEOUT,
  QED_ENOMEM,
  QED_EIO,
  QED_EIDX,
  QED_EHB,
  QED_EAGAIN,
  QED_MAX_STATUS // This should always be the last value of the enum
} QED_Status;

extern THREAD_LOCAL QED_Status qed_last_error;

#define QED_errno() qed_last_error

extern const char* QED_ERROR_STRING[QED_MAX_STATUS];

#if defined(__cplusplus)
#define QED_error_string(x) (((x) >= 0 && (x) < QED::QED_MAX_STATUS) ? QED::QED_ERROR_STRING[x] : "UNKNOWN")
#else
#define QED_error_string(x) (((x) >= 0 && (x) < QED_MAX_STATUS) ? QED_ERROR_STRING[x] : "UNKNOWN")
#endif


#if defined(__cplusplus)
} // extern "C"
} // namespace QED
#endif

#endif /* QED_ERROR_H__ */
