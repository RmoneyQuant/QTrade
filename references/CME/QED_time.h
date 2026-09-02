/* Copyright (c) 2013 Quincy Data LLC -- All Rights Reserved */

#ifndef QED_TIME_H
#define QED_TIME_H

#ifndef INLINE
#define INLINE static inline
#endif

#include "qed_compat.h"

#ifdef __cplusplus
namespace QED
{
  extern "C" {
#endif

typedef struct {
  unsigned short year; // since 0
  unsigned char month; // 1 - 12
  unsigned char day; // 1 - 31
  unsigned char hour;
  unsigned char minute;
  unsigned char second;
  unsigned short millisecond;
} QED_DateTime;

/**
 * \brief DateTime API
 *
 * Simple data time methods for logging.
 */

/**
 * Get local date time using localtime_r().
 *
 * \param date pointer to a QED_DateTime struct
 */
INLINE void QED_getLocalDateTime(QED_DateTime *date);

INLINE void QED_getLocalDateTime(QED_DateTime *date) {
  time_t currentTime = time(NULL);
  struct tm currentLocalTimeB;
  struct tm *currentLocalTime = localtime_r(&currentTime, &currentLocalTimeB);
  date->year = currentLocalTime->tm_year + 1900;
  date->month = currentLocalTime->tm_mon + 1;
  date->day = currentLocalTime->tm_mday;
  date->hour = currentLocalTime->tm_hour;
  date->minute = currentLocalTime->tm_min;
  date->second = currentLocalTime->tm_sec;
  struct timeval tv;
  struct timezone tz;
  gettimeofday(&tv, &tz);
  date->millisecond = tv.tv_usec / 1000;
}

#ifdef __cplusplus
} /* extern "C" */
} /* namespace QED */
#endif
#endif
