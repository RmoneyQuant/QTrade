/**
 * Copyright (c) 2013 Quincy Data LLC -- All Rights Reserved
 */

#ifndef QED_QEDSOCK_H__
#define QED_QEDSOCK_H__

#include "qed_compat.h"

#if defined(__cplusplus)
namespace QED {
extern "C" {
#endif

/** Socket Wrapper Functions */

/**
 *  Close the socket.
 *
 *  If there is an error this method returns silently. Currently it is only used
 *  to close the symbol list socket.
 *
 *  \param socket The socket descriptor.
 *  \return none
 */
INLINE void QED_closeSocket(int socket);

/**
 * Create a UDP Server Socket
 *
 * Creates a UDP socket bound to the specified port, but does not bind
 * the socket to an interface nor does it join any multicast groups.
 *
 * \param socket Pointer to the socket descriptor to be created.
 * \param port The port.
 * \return True on success otherwise false.
 */
extern bool QED_createUDPServerSocket(int* socket, unsigned int port);

/**
 * Create a UDP Server Socket and bind address if provided
 *
 * Creates a UDP socket bound to the specified port and address if specified,
 * but does not bind the socket to an interface nor does it join any
 * multicast groups.
 *
 * \param socket Pointer to the socket descriptor to be created.
 * \param port The port.
 * \param address_optional The multicast address to bind if provided. INADDR_ANY if NULL.
 * \return True on success otherwise false.
 */
extern bool QED_createUDPServerSocketOnAddr(int* socket, unsigned int port, const char * address_optional);

/**
 * Join a multicast group.
 *
 * This method adds the multicast group membership and listens on all interfaces.
 *
 * \see QED_addMulticastInterfaceMembership
 *
 * \param socket The socket descriptor.
 * \address The multicast address.
 * \return True on success.
 */
extern bool QED_addMulticastMembership(int socket, const char *address);

/**
 * Join a multicast group on an specific interface.
 *
 * Similar to QED_addMulticastMembership; however this method allows callers to
 * specify the interface on which to listen to traffic rather than INADDR_ANY.
 *
 * \see QED_addMulticastMembership
 *
 * \param socket The socket descriptor.
 * \address The multicast address.
 * \return True on success.
 */
extern bool QED_addMulticastInterfaceMembership(int socket, const char *address,
                                                const char *interfaceAddress);

/**
 * Receive data from a multicast socket
 *
 * Used to retrieve the symbol list, this method receives a UDP packet from the
 * socket and logs any errors.
 *
 * \param socket The socket
 * \param buffer a pointer to a buffer for the udp packet.
 * \param receivedSize On success contains the size of the packet received
 * \param bufferSize The size of buffer in bytes.
 * \return True on success.
 */
bool QED_receiveFromUDPSocket(int socket, char *buffer,
    unsigned int *receivedSize, unsigned int bufferSize);

/*
 * Inline  QED_Socket implementation
 */

void QED_closeSocket(int socket) {
  shutdown(socket, SHUT_RDWR);
  close(socket);
  socket = -1;
}

#if defined(__cplusplus)
} /* extern "C" */
} /* namespace QED */
#endif

#endif /* QED_QEDSOCK_H__ */
