/**
 * File:   DsMacro.h
 *
 * Copyright (c) 2013 Quincy Data LLC -- All Rights Reserved
 */

#ifndef DS_DSMACRO_H
#define DS_DSMACRO_H

#include <inttypes.h>
#include <stdint.h>
#include <sys/time.h>

/******************************************************************************
                       This is Necessary for C Client API

We want to be able to include this file in the C Client API so we don't have to
maintain the macros in more than one place.
******************************************************************************/

/* This is the newest codec version supported by the client */
#define QED_CLIENT_MAX_VERSION 0x05
/* This is the oldest codec version supported by the client */
#define QED_CLIENT_MIN_VERSION 0x05

/* DS_CODEC_VERSION and DS_COMPAT_CODEC are legacy definitons */
#define DS_CODEC_VERSION  0x05
/* This is the oldest decoder that is compatible with this codec version */
#define DS_COMPAT_CODEC  0x05

/* Packet Types
 * Available in V3+ packets and Symbol List Packets
 */
#define DS_PKT_SYMBOL_LIST      0x00
#define DS_PKT_SNAPSHOT         0x01
#define DS_PKT_HEARTBEAT        0x02
#define DS_PKT_ENCRYPTED        0x03

/* Message Types */
#define DS_MSG_SYMBOL_SNAPSHOT  0x01
#define DS_MSG_HEARTBEAT        0x02

/* Error codes */
#define QED_FIELD_OFLOW 0x01      /** One or more fields have overflows */

/*****************************************************************************/

#endif	// DS_DSMACRO_H

