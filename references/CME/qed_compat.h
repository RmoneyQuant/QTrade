/**
 * Copyright (c) 2016 Quincy Data LLC -- All Rights Reserved
 */
#ifndef QED_COMPAT_H__
#define QED_COMPAT_H__

#ifdef __GNUC__
 #define THREAD_LOCAL __thread
#else
 #define THREAD_LOCAL thread_local
#endif

#if defined(__cplusplus)
// new style header includes
// was only introduced in C++11
#if __cplusplus < 201103L
 #include <stdio.h>
 #include <stdlib.h>
 #include <stdbool.h>
 #include <stdint.h>
 #include <inttypes.h>
#else
 #include <cstdio>
 #include <cstdlib>
 #include <cstdbool>
 #include <cstdint>
 #include <cinttypes>
#endif
 #include <cstdarg>
 #include <cstring>
 #include <climits>
 #include <cmath>
 #include <ctime>
#else
 #include <stdio.h>
 #include <stdlib.h>
 #include <stdbool.h>
 #include <stdint.h>
 #include <stdarg.h>
 #include <string.h>
 #include <inttypes.h>
 #include <limits.h>
 #include <math.h>
 #include <time.h>
#endif

#ifdef __clang__
  // it turns out that pow(10, x) is about 15 quicker than __builtin_powl(10, x)
  // when compiling in release mode with clang
  #define pow_base_10(x) pow(10, x)
#elif  __ICC
  #define pow_base_10 pow10
#elif  __CYGWIN__
  #define pow_base_10 exp10
#else
  #define pow_base_10 __builtin_exp10l
#endif

#ifdef _WIN32
 #include <io.h>
#else
 #include <unistd.h>
#endif

#include <sys/epoll.h>
#include <pthread.h>

#include <signal.h>
#include <sys/types.h>	       /* See NOTES */
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <sys/stat.h>
#include <sys/mman.h>
#include <sys/time.h>
#include <net/if.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <arpa/inet.h>
#include <signal.h>
#include <errno.h>
#include <dirent.h>
#include <fcntl.h>

#endif /* QED_COMPAT_H__ */
