/** 
 * Copyright (c) 2012 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_H__
#define QED_H__

#include "qed_compat.h"
#include "qed_error.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

/**
 * Return the version string for the API.
 */
extern const char* QED_getVersion();

/**
 * Initialize the Quincy Data API logs.
 * This is mandatory to call this once, before any other call to the Quincy API.
 */
extern void QED_initializeLogs();

/**
 * Set the log file. This method optionally sets the file for QED logging
 * output. It should be called after \p QED_initializeLogs() but prior to other
 * API calls. By default QED logs to stderr.
 *
 * \p file The log file
 */
extern void QED_setLogFile(FILE* file);

/**
 * Connection context for connection to a Distribution Server
 *
 * Applications must create a connection context with the \p QED_init() function
 * for each distribution server from which they receive market data.
 */
typedef struct QED_ctx_* QED_ctx;

/**
 * Initialize the Quincy Data API.
 *
 * \p QED_init() initializes the connection context, \p QED_ctx and receives
 * symbol reference data from the distribution server. The distribution server,
 * sends the reference data at regular, sub-second, intervals.
 *
 * \p QED_init() requires a configuration file, <em>qed.cfg</em>. By default,
 * <tt>QED_init()</tt> searches the current directory (.), followed by /etc for
 * <em>qed.cfg</em>. \p QED_initWithPath() allows application to alter the
 * search path.
 *
 * \param config the configuration/feed name. There is a separate distribution
 *               server for each market, each requiring its own context and
 *               configuration.
 *
 * \return the connection context. 
 *  If an error occurs the return value will be NULL and \p QED_errno() and
 *  \p QED_error_string().
 *
 * ERRORS:
 *  - QDQ_EINVAL: Invalid argument.
 *  - QED_ENOMEM: Not enough memory to allocate the context
 *  - QED_EIO:    An IO error occurred receiving reference data from the
 *                distribution server.
 *  - QED_ECFG    Error loading the configuration file
 *  - QED_EBADCFG Corrupt configuration file
 *
 *
 */
extern QED_ctx QED_init(const char* config);

/**
 * Initialize the context and specify the search path to locate the
 * <em>qed.cfg</em> configuration file.
 *
 * \param config the configuration/feed name. There is a separate distribution
 *               server for each market, each requiring its own context and
 *               configuration.
 *
 * \param path Colon delimited list of directories to search for
 *             <em>qed.cfg</em>.
 *
 * \return the connection context. 
 *  If an error occurs the return value will be NULL and \p QED_errno() and
 *  \p QED_error_string().
 *
 * ERRORS:
 *  - QDQ_EINVAL: Invalid argument.
 *  - QED_ENOMEM: Not enough memory to allocate the context
 *  - QED_EIO:    An IO error occurred receiving reference data from the
 *                distribution server.
 *  - QED_ECFG    Error loading the configuration file
 *  - QED_EBADCFG Corrupt configuration file
 *
 *
 *
 * \see QED_init()
 */
extern QED_ctx QED_initWithPath(const char* config, const char* path);

/**
 * Initialize the context. Use the supplied data to initialize the symbol
 * information rather than receiving a multicast packet. This method is useful
 * for testing with saved data and where live market data is not available.
 *
 * \param config the configuration/feed name. There is a separate distribution
 *               server for each market, each requiring its own context and
 *               configuration. Can be NULL to use default settings.
 *
 * \param path Colon delimited list of directories to search for
 *             <em>qed.cfg</em>. Must be NULL if config is NULL.
 *
 * \return the connection context.
 *  If an error occurs the return value will be NULL and \p QED_errno() and
 *  \p QED_error_string().
 *
 * ERRORS:
 *  - QDQ_EINVAL: Invalid argument.
 *  - QED_ENOMEM: Not enough memory to allocate the context
 *  - QED_EIO:    An IO error occurred receiving reference data from the
 *                distribution server.
 *  - QED_ECFG    Error loading the configuration file
 *  - QED_EBADCFG Corrupt configuration file
 *
 *
 *
 * \see QED_init()
 * \see QED_initWithPath()
 * \see QED_getSymbolListPacket()
 */
extern QED_ctx QED_initWithPacket(const char* config, const char* path,
    const char* data, size_t len);

/**
 * \brief Initialize the context.
 *
 * Users who wish to process the symbol list stream with their own code should
 * use this function instead of the other QED_init functions.  After calling
 * this method, the application should then process the symbol list packets by
 * passing the QED_ctx to * QED_processSymbolPacket().
 *
 * The context returned by this function is not usable until the user has
 * successfully passed all the required symbol list packets to the client
 * library via QED_processSymbolPacket(): no other function should be used on
 * this context before processing the symbol list packets.
 *
 * \param config the configuration/feed name. There is a separate distribution
 *               server for each market, each requiring its own context and
 *               configuration. Can be NULL to use default settings.
 *
 * \param path Colon delimited list of directories to search for
 *             <em>qed.cfg</em>. Must be NULL if config is NULL.
 *
 * \return the connection context.
 *  If an error occurs the return value will be NULL and \p QED_errno() and
 *  \p QED_error_string().
 *
 * ERRORS:
 *  - QDQ_EINVAL: Invalid argument.
 *  - QED_ENOMEM: Not enough memory to allocate the context
 *  - QED_ECFG    Error loading the configuration file
 *  - QED_EBADCFG Corrupt configuration file
 *
 *
 *
 * \see QED_init()
 * \see QED_initWithPath()
 * \see QED_processSymbolPacket()
 */
extern QED_ctx QED_initWithUserSymbolPackets(const char* config, const char* path);

/**
 * Close the Quincy Data API
 *
 * \p QED_close() frees any resources associated with the connection context.
 * Applications should invoke this method after freeing any other resources and
 * should not process messages for the context after calling \p QED_close()
 *
 * \param ctx The connection context created by \p QED_init()
 *
 * \return QED_EOK on success
 *
 * ERRORS:
 *  - QED_EINVAL: Invalid (\p NULL) context passed to \p QED_close().
 */
extern QED_Status QED_close(QED_ctx ctx);

/**
 * Close the Quincy Data logger 
 *
 * \p QED_closeLogs() flushes and frees any resources associated with the logger.
 * Applications should invoke this method after calling QED_close()
 */
extern void QED_closeLogs();

#if defined(__cplusplus)
} // extern "C" 
} // namespace QED
#endif

#endif /* QED_H__ */

/**
 * \mainpage
 * Quincy Extreme Data API.
 * \copyright 2012 Quincy Data LLC -- All Rights Reserved
 * \image html  Logo_QD_200_55.png
 * \latexonly 
 * \begin{center}
 * \endlatexonly 
 * \image latex Logo_QD.eps "" width=6cm
 * \latexonly 
 * \end{center}
 * \endlatexonly 
 */
