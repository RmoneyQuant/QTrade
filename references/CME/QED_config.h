/* Copyright (c) 2013 Quincy Data LLC -- All Rights Reserved */

#ifndef QED_CONFIG_H
#define QED_CONFIG_H

#include "qed_compat.h"
#include <libconfig.h>

#include "QED_logs.h"

#ifndef INLINE
#define INLINE static inline
#endif

#ifdef __cplusplus
namespace QED {
  extern "C" {
#endif

/**
 * \brief Config API: Thin libconfig wrapper.
 */


/*
 * For internal use only
 */

struct QED_InternalConfig {
  config_t config;
};

/** Configuration context */
typedef struct QED_InternalConfig QED_Config;

/**
 * Initialize the configuration context
 *
 * \param config Pointer to context to initialize.
 */
extern void QED_initializeConfig(QED_Config *config);

/**
 * Free configuration context
 *
 * \param config The context to free.
 */
extern void QED_freeConfig(QED_Config *config);

/**
 * Set the include path
 *
 * Libconfig searches this path for included configuration files.
 *
 * \param config The context.
 * \param includePath The patch to search.
 */
extern void QED_setConfigIncludePath(QED_Config *config, const char *includePath);

/**
 * Parse a configuration file
 *
 * \param config The context.
 * \param filePath Path to file including file name and extension.
 * \return true if parse is successful otherwise false.
 */
extern bool QED_parseConfigFile(QED_Config *config, const char *filePath);

/**
 * For testing: Parse String rather than file.
 *
 * \param config The context.
 * \param str \0 terminated string with configuration data.
 * \return true if parse is successful otherwise false.
 */
extern bool QED_parseConfigString(QED_Config *config, const char *str);

/*
 * Lookup configuration string value
 *
 * Looks up the string value with the specified configuration name. See
 * libconfig documentation for details.
 *
 * \param config The context
 * \param name The configuration name/key
 * \value contains a pointer to the value string on success
 * \return true if the lookup succeeds.
 */
extern bool QED_getConfigString(const QED_Config *config, const char *name, const char **value);

/*
 * Lookup configuration string value
 *
 * Looks up the int value with the specified configuration name. See libconfig
 * documentation for details.
 *
 * Warning: the full UInt range is not supported, only up to the max signed int
 * value
 *
 * \param config The context
 * \param name The configuration name/key
 * \value pointer to a uint32_t which will get set to the value on success.
 * \return true if the lookup succeeds.
 */
extern bool QED_getConfigUInt32(const QED_Config *config, const char *name, uint32_t *value);

#ifdef __cplusplus
} /* extern "C" */
} /* namespace QED */
#endif
#endif

