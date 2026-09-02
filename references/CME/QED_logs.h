/*
 * Copyright (c) 2013 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_LOGS_H
#define QED_LOGS_H


#ifndef INLINE
#define INLINE static inline
#endif

#include "qed_compat.h"
#include "QED_time.h"

#ifdef __cplusplus
namespace QED {
  extern "C" {
#endif

/*
 * Internal API
 *
 * NOTE: Some of the logging code, including the array of log buffers, exists
 * for asynchronous logging. Currently the QED API does not implement
 * asynchronous logging, but it may in future release.
 *
 */
#define QED_LOG_STRING_BUFFER_SIZE 1000
#define QED_LOG_HEADER_MAX_SIZE 250
// QED_LOG_HEADER_MAX_SIZE must be smaller than QED_LOG_STRING_BUFFER_SIZE to
// leave space for the log text
#define QED_N_LOG_STRING_BUFFERS 10

#define QED_LOG_COMMON_HEADER_FORMAT "%.200s:%d: %04d%02d%02d-%02d:%02d:%02d.%03d (%x)"
#define QED_LOG_COMMON_HEADER_LOCATION_ARGS __FILE__, __LINE__, __func__
#define QED_LOG_COMMON_HEADER_TIME_ARGS(dateTime)\
  (dateTime).year, (dateTime).month, (dateTime).day, \
  (dateTime).hour, (dateTime).minute, (dateTime).second,\
  (dateTime).millisecond


extern void QED_internalAppendLog(const char *s);
INLINE char *QED_internalGetStringBuffer();
INLINE void QED_internalGet2StringBuffers(char **s1, char **s2);
INLINE void QED_internalGet3StringBuffers(char **s1, char **s2, char **s3);
extern char **QED_logStringBufferArray;
extern unsigned int QED_currentlogStringBuffer;

/*
 * Public API
 */

/**
 * Initialize logging
 *
 * This should be called as earlier as possible before generating any log
 * output.
 */
extern void QED_initializeLogger();

/**
 * Flush the log files.
 *
 * Call this method on exit to ensure that log files are written completely to
 * disk.
 *
 * \param  isSignal set to true if calling from a signal handler.
 */
extern void QED_flushLogger(bool isSignal);

/**
 * Shutdown the logging.
 */
extern void QED_shutdownLogger();

/**
 * Sets the file where to log the output
 *
 * \param logFile output log file. Must be properly opened and writable.
 */
extern void QED_setLoggerFile(FILE *logFile);

INLINE void QED_internalLog(const char *format, const char *fileName, int line, const char *functionName, const char *message, ...) {
  va_list args ;
  va_start (args, message) ;
  QED_DateTime currentTime; QED_getLocalDateTime(&currentTime);
  char *s = QED_internalGetStringBuffer();
  char *ps = s;
  ps += snprintf(ps, QED_LOG_HEADER_MAX_SIZE, format, fileName, line, QED_LOG_COMMON_HEADER_TIME_ARGS(currentTime), pthread_self(), functionName);
  ps += vsnprintf(ps, QED_LOG_STRING_BUFFER_SIZE - QED_LOG_HEADER_MAX_SIZE - 2, message, args);
  *ps++ = '\n';
  *ps++ = 0;
  QED_internalAppendLog(s);
  va_end(args);
}

/**
 * Log an error message
 *
 * \param message printf style format string.
 * \param args printf style arguments.
 */
#define QED_logError(message, args...)\
  QED_internalLog(QED_LOG_COMMON_HEADER_FORMAT " ERROR:  (%.200s) \
  ", QED_LOG_COMMON_HEADER_LOCATION_ARGS, message, ##args)

/**
 * Log an info message
 *
 * \param message printf style format string.
 * \param args printf style arguments.
 */
#define QED_logInfo(message, args...)\
  QED_internalLog(QED_LOG_COMMON_HEADER_FORMAT " INFO:  (%.200s) \
  ", QED_LOG_COMMON_HEADER_LOCATION_ARGS, message, ##args)

#ifdef QED_DEBUG

/**
 * Log an debug message
 *
 * These messages will only appear if the library is builtwith QED_DEBUG
 * defined.
 *
 * \param message printf style format string.
 * \param args printf style arguments.
 */
#define QED_logDebug(message, args...)\
  QED_internalLog(QED_LOG_COMMON_HEADER_FORMAT \
  " DEBUG:  (%.200s) ", \
  QED_LOG_COMMON_HEADER_LOCATION_ARGS, message, ##args)

#else

#define QED_logDebug(message, args...) QED_internalDoNothing()

#endif

/**
 * Log an message with an error number and description.
 *
 * Message must be static and contain %d and %s (in this order) 
 *
 * \param message printf style format string.
 * \param args printf style arguments.
 */
#define QED_logErrorErrno(message, value) QED_internalLogErrorErrno(QED_LOG_COMMON_HEADER_FORMAT " ERROR:  (%.200s) %.200s\n", message, value, QED_LOG_COMMON_HEADER_LOCATION_ARGS)


/**
 * Tell the logging to write to stdout rather than a log file.
 */
extern void QED_logToStdOut();

/*
 * Inline implementation
 */
INLINE void QED_internalDoNothing() {
}

INLINE char *QED_internalGetStringBuffer() {
  unsigned int bufferIndex =
  __sync_fetch_and_add(&QED_currentlogStringBuffer, 1) % QED_N_LOG_STRING_BUFFERS;
  return QED_logStringBufferArray[bufferIndex];
}

INLINE void QED_internalGet2StringBuffers(char **s1, char **s2) {
  unsigned int bufferIndex = __sync_fetch_and_add(&QED_currentlogStringBuffer, 2);
  *s1 = QED_logStringBufferArray[bufferIndex % QED_N_LOG_STRING_BUFFERS];
  *s2 = QED_logStringBufferArray[(bufferIndex + 1) % QED_N_LOG_STRING_BUFFERS];
}

INLINE void QED_internalGet3StringBuffers(char **s1, char **s2, char **s3) {
  unsigned int bufferIndex = __sync_fetch_and_add(&QED_currentlogStringBuffer, 3);
  *s1 = QED_logStringBufferArray[bufferIndex % QED_N_LOG_STRING_BUFFERS];
  *s2 = QED_logStringBufferArray[(bufferIndex + 1) % QED_N_LOG_STRING_BUFFERS];
  *s3 = QED_logStringBufferArray[(bufferIndex + 2) % QED_N_LOG_STRING_BUFFERS];
}

INLINE void QED_internalLogError(const char *format,
                                 const char *fileName,
                                 int line,
                                 const char *functionName,
                                 const char *message) {
  QED_DateTime currentTime; QED_getLocalDateTime(&currentTime);
  char *s = QED_internalGetStringBuffer();
  sprintf(s, format, fileName, line, QED_LOG_COMMON_HEADER_TIME_ARGS(currentTime),
    pthread_self(), functionName, message);
  QED_internalAppendLog(s);
}

INLINE void QED_internalLogInfo(const char *format,
                                const char *fileName,
                                int line,
                                const char *functionName,
                                const char *message) {
  QED_DateTime currentTime; QED_getLocalDateTime(&currentTime);
  char *s = QED_internalGetStringBuffer();
  sprintf(s, format, fileName, line, QED_LOG_COMMON_HEADER_TIME_ARGS(currentTime),
    pthread_self(), functionName, message);
  QED_internalAppendLog(s);
}

INLINE void QED_internalLogErrorErrno(const char *messageFormat,
                                      const char *message,
                                      int value,
                                      const char *fileName,
                                      int line,
                                      const char *functionName) {
  QED_DateTime currentTime; QED_getLocalDateTime(&currentTime);
  char *s1, *s2;
  QED_internalGet2StringBuffers(&s1, &s2);
  sprintf(s2, message, value, strerror(value));
  sprintf(s1, messageFormat, fileName, line,
    QED_LOG_COMMON_HEADER_TIME_ARGS(currentTime), pthread_self(), functionName, s2);
  QED_internalAppendLog(s1);
}

#ifdef __cplusplus
} /* extern "C" */
} /* namespace QED */
# endif
#endif
