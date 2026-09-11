/*******************************************************************************
 *09-Oct-2023
 *  FILE NAME: ETILayoutsNS_Derivatives.h
 *
 *  INTERFACE VERSION:   9.0
 *
 *  SUBVERSION:          D0003
 *
 *  BUILD NUMBER:        90.370.0.ga-90003070-74
 *
 *  DESCRIPTION:
 *
 *    This header file documents the binary message format of ETI.
 *    - All integers are in little endian byte order.
 *    - Padding bytes in following structures (char PadX[...]) are not required to be initialized.
 *
 *    DISCLAIMER:
 *
 *      Supported on Linux/x64 platforms with GNU C/C++ version 4.1 and 4.4.
 *
 *      This header file is meant to be compatible (but not supported) with any C/C++
 *      compiler/architecture that defines C99 compliant integer types in stdint.h and
 *      corresponds with the following alignment and padding requirements:
 *
 *    Padding:
 *      The compiler does not add implicit padding bytes between any of the following
 *      structure members. All padding bytes required for the alignment rules below are
 *      already explicitly contained in the structures.
 *
 *    Alignment rules:
 *      1 byte alignment for  int8_t and  uint8_t types
 *      2 byte alignment for int16_t and uint16_t types
 *      4 byte alignment for int32_t and uint32_t types
 *      8 byte alignment for int64_t and uint64_t types
 *
 *******************************************************************************/

#ifndef __ETI_DERIVATIVES_LAYOUTS_WITH_NAMESPACE__
#define __ETI_DERIVATIVES_LAYOUTS_WITH_NAMESPACE__

#include <stdint.h>

#if defined(__cplusplus) || defined(c_plusplus)
namespace Derivatives
{
extern "C"
{
#endif

#define ETI_INTERFACE_VERSION "9.0"
#define ETI_BUILD_NUMBER      "90.370.0.ga-90003070-74"

/*
 * No Value defines
 */
#define NO_VALUE_SLONG                                   ((int64_t) 0x8000000000000000L)
#define NO_VALUE_ULONG                                   ((uint64_t) 0xffffffffffffffffUL)
#define NO_VALUE_SINT                                    ((int32_t) 0x80000000)
#define NO_VALUE_UINT                                    ((uint32_t) 0xffffffff)
#define NO_VALUE_SSHORT                                  ((int16_t) 0x8000)
#define NO_VALUE_USHORT                                  ((uint16_t) 0xffff)
#define NO_VALUE_SCHAR                                   ((int8_t) 0x80)
#define NO_VALUE_UCHAR                                   ((uint8_t) 0xff)
#define NO_VALUE_STR                                     0
#define NO_VALUE_DATA_16                                 {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}

/*
 * Template IDs defines
 */
#define	TID_AUCTION_NOTIFICATION                         11028		// < AuctionNotification (Auction Notification)
#define	TID_BROADCAST_ERROR_NOTIFICATION                 10032		// < BroadcastErrorNotification (Gap Fill)
#define	TID_CONNECTION_GATEWAY_REQUEST                   10020		// < ConnectionGatewayRequest (Connection Gateway Request)
#define	TID_CONNECTION_GATEWAY_RESPONSE                  10021		// < ConnectionGatewayResponse (Connection Gateway Response)
#define	TID_DELETE_ALL_ORDER_BROADCAST                   10122		// < DeleteAllOrderBroadcast (Order Mass Cancellation Notification)
#define	TID_DELETE_ALL_ORDER_NR_RESPONSE                 10124		// < DeleteAllOrderNRResponse (Order Mass Cancellation Response No Hits)
#define	TID_DELETE_ALL_ORDER_QUOTE_EVENT_BROADCAST       10308		// < DeleteAllOrderQuoteEventBroadcast (Mass Cancellation Event)
#define	TID_DELETE_ALL_ORDER_REQUEST                     10120		// < DeleteAllOrderRequest (Order Mass Cancellation Request)
#define	TID_DELETE_ALL_ORDER_RESPONSE                    10121		// < DeleteAllOrderResponse (Order Mass Cancellation Response)
#define	TID_DELETE_ORDER_BROADCAST                       10112		// < DeleteOrderBroadcast (Cancel Order Notification)
#define	TID_DELETE_ORDER_COMPLEX_REQUEST                 10123		// < DeleteOrderComplexRequest (Cancel Order complex)
#define	TID_DELETE_ORDER_NR_RESPONSE                     10111		// < DeleteOrderNRResponse (Cancel Order Response (Lean Order))
#define	TID_DELETE_ORDER_RESPONSE                        10110		// < DeleteOrderResponse (Cancel Order Response (Standard Order))
#define	TID_DELETE_ORDER_SINGLE_REQUEST                  10109		// < DeleteOrderSingleRequest (Cancel Order Single)
#define	TID_EXD_EX_CANCELLATION_CONFIRMATION             8037		// < ExDExCancellationConfirmation ( Ex/DEx Cancellation Confirmation )
#define	TID_EXD_EX_CANCELLATION_REQUEST                  8036		// < ExDExCancellationRequest ( Ex/DEx Cancellation Request )
#define	TID_EXD_EX_ENTRY_CONFIRMATION                    8031		// < ExDExEntryConfirmation ( Ex/DEx Entry Confirmation )
#define	TID_EXD_EX_ENTRY_REQUEST                         8030		// < ExDExEntryRequest ( Ex/DEx Entry Request )
#define	TID_EXD_EX_MODIFICATION_CONFIRMATION             8034		// < ExDExModificationConfirmation ( Ex/DEx Modification Confirmation )
#define	TID_EXD_EX_MODIFICATION_REQUEST                  8033		// < ExDExModificationRequest (Ex/DEx Modification Request )
#define	TID_EXD_EX_NOTIFICATION                          8040		// < ExDExNotification ( Ex/DEx Notification )
#define	TID_EXTENDED_ORDER_INFORMATION                   10994		// < ExtendedOrderInformation (Extended Order Information)
#define	TID_FORCED_LOGOUT_NOTIFICATION                   10012		// < ForcedLogoutNotification (Session Logout Notification)
#define	TID_FORCED_USER_LOGOUT_NOTIFICATION              10043		// < ForcedUserLogoutNotification (User Logout Notification)
#define	TID_HEARTBEAT                                    10011		// < Heartbeat (Heartbeat)
#define	TID_HEARTBEAT_NOTIFICATION                       10023		// < HeartbeatNotification (Heartbeat Notification)
#define	TID_IMMEDIATE_EXECUTION_RESPONSE                 10993		// < ImmediateExecutionResponse (Immediate Execution Response)
#define	TID_INQUIRE_PRE_TRADE_RISK_LIMITS_REQUEST        10311		// < InquirePreTradeRiskLimitsRequest (Inquire Pre-Trade Risk Limits Request)
#define	TID_LOGON_REQUEST                                10000		// < LogonRequest (Session Logon)
#define	TID_LOGON_RESPONSE                               10001		// < LogonResponse (Session Logon Response)
#define	TID_LOGOUT_REQUEST                               10002		// < LogoutRequest (Session Logout)
#define	TID_LOGOUT_RESPONSE                              10003		// < LogoutResponse (Session Logout Response)
#define	TID_MARGIN_CHANGE_NOTIFICATION                   4025		// < MarginChangeNotification (Margin Change Notification)
#define	TID_MARKET_STATUS_NOTIFICATION                   4125		// < MarketStatusNotification ( Market Status Notification )
#define	TID_MARKET_WIDE_OI_ALERT                         8101		// < MarketWideOIAlert (Market Wide OI Alert)
#define	TID_MARKET_WIDE_OI_VIOLATION                     8100		// < MarketWideOIViolation (Market Wide OI Violation)
#define	TID_MODIFY_ORDER_COMPLEX_REQUEST                 10114		// < ModifyOrderComplexRequest (Replace Order complex)
#define	TID_MODIFY_ORDER_NR_RESPONSE                     10108		// < ModifyOrderNRResponse (Replace Order Response (Lean Order))
#define	TID_MODIFY_ORDER_RESPONSE                        10107		// < ModifyOrderResponse (Replace Order Response (Standard Order))
#define	TID_MODIFY_ORDER_SINGLE_REQUEST                  10106		// < ModifyOrderSingleRequest (Replace Order Single)
#define	TID_MODIFY_ORDER_SINGLE_SHORT_REQUEST            10126		// < ModifyOrderSingleShortRequest (Replace Order Single (short layout))
#define	TID_NEW_ORDER_COMPLEX_REQUEST                    10113		// < NewOrderComplexRequest (New Order Multi Leg)
#define	TID_NEW_ORDER_MULTI_LEG                          10991		// < NewOrderMultiLeg (New Order MultiLeg)
#define	TID_NEW_ORDER_NR_RESPONSE                        10102		// < NewOrderNRResponse (New Order Response (Lean Order))
#define	TID_NEW_ORDER_RESPONSE                           10101		// < NewOrderResponse (New Order Response (Standard Order))
#define	TID_NEW_ORDER_SINGLE_REQUEST                     10100		// < NewOrderSingleRequest (New Order Single)
#define	TID_NEW_ORDER_SINGLE_SHORT_REQUEST               10125		// < NewOrderSingleShortRequest (New Order Single (short layout))
#define	TID_NEWS_BROADCAST                               10031		// < NewsBroadcast (News)
#define	TID_ORDER_EXEC_NOTIFICATION                      10104		// < OrderExecNotification (Book Order Execution)
#define	TID_ORDER_EXEC_REPORT_BROADCAST                  10117		// < OrderExecReportBroadcast (Extended Order Information)
#define	TID_ORDER_EXEC_RESPONSE                          10103		// < OrderExecResponse (Immediate Execution Response)
#define	TID_PRE_TRADE_RISK_LIMIT_RESPONSE                10313		// < PreTradeRiskLimitResponse (Pre-Trade Risk Limit Response)
#define	TID_REJECT                                       10010		// < Reject (Reject)
#define	TID_REJECT_MULTI_LEG                             10992		// < RejectMultiLeg (Reject MultiLeg)
#define	TID_RESUBMIT_APPROVAL_CONFIRMATION               8510		// < ResubmitApprovalConfirmation (ResubmitForApproval)
#define	TID_RESUBMIT_APPROVAL_REQUEST                    8500		// < ResubmitApprovalRequest (ApprovalRequest)
#define	TID_RETRANSMIT_ME_MESSAGE_REQUEST                10026		// < RetransmitMEMessageRequest (Retransmit (Order/Quote Event))
#define	TID_RETRANSMIT_ME_MESSAGE_RESPONSE               10027		// < RetransmitMEMessageResponse (Retransmit Response (Order/Quote Event))
#define	TID_RETRANSMIT_REQUEST                           10008		// < RetransmitRequest (Retransmit)
#define	TID_RETRANSMIT_RESPONSE                          10009		// < RetransmitResponse (Retransmit Response)
#define	TID_SERVICE_AVAILABILITY_BROADCAST               10030		// < ServiceAvailabilityBroadcast (Service Availability)
#define	TID_SESSION_PASSWORD_CHANGE                      10997		// < SessionPasswordChange (user password change)
#define	TID_SESSION_PASSWORD_CHANGE_RESPONSE             10995		// < SessionPasswordChangeResponse (user password change)
#define	TID_SUBSCRIBE_REQUEST                            10025		// < SubscribeRequest (Subscribe)
#define	TID_SUBSCRIBE_RESPONSE                           10005		// < SubscribeResponse (Subscribe Response)
#define	TID_SYSTEM_INFORMATION_DOWNLOAD                  4100		// < SystemInformationDownload (System Information Download)
#define	TID_TM_TRADING_SESSION_STATUS_BROADCAST          10501		// < TMTradingSessionStatusBroadcast (Trade Notification Status)
#define	TID_THROTTLE_UPDATE_NOTIFICATION                 10028		// < ThrottleUpdateNotification (Throttle Update Notification)
#define	TID_TRADE_BROADCAST                              10500		// < TradeBroadcast (Trade Notification)
#define	TID_TRADE_ENHANCEMENT_NOTIFICATION               10989		// < TradeEnhancementNotification (TradeEnhancementNotification)
#define	TID_TRADE_MODIFICATION_NOTIFICATION              8020		// < TradeModificationNotification (TradeModificationNotification)
#define	TID_TRADE_MODIFICATION_REQUEST                   8005		// < TradeModificationRequest (TradeModificationRequest)
#define	TID_TRADE_MODIFICATION_RESPONSE                  8010		// < TradeModificationResponse (TradeModificationResponse)
#define	TID_TRADING_SESSION_STATUS_BROADCAST             10307		// < TradingSessionStatusBroadcast (Trading Session Event)
#define	TID_UNSUBSCRIBE_REQUEST                          10006		// < UnsubscribeRequest (Unsubscribe)
#define	TID_UNSUBSCRIBE_RESPONSE                         10007		// < UnsubscribeResponse (Unsubscribe Response)
#define	TID_USER_LOGIN_REQUEST                           10018		// < UserLoginRequest (User Logon)
#define	TID_USER_LOGIN_RESPONSE                          10019		// < UserLoginResponse (User Logon Response)
#define	TID_USER_LOGOUT_REQUEST                          10029		// < UserLogoutRequest (User Logout)
#define	TID_USER_LOGOUT_RESPONSE                         10024		// < UserLogoutResponse (User Logout Response)
#define	TID_USER_PASSWORD_CHANGE                         10996		// < UserPasswordChange (user password change)
#define	TID_USER_PASSWORD_CHANGE_RESPONSE                10990		// < UserPasswordChangeResponse (user password change)

const int ETI_DERIVATIVES_TID_MIN = 4025;  // lowest assigned template ID
const int ETI_DERIVATIVES_TID_MAX = 11028;  // highest assigned template ID
/*
 * Max defines for sequences defines
 */
#define MAX_DELETE_ALL_ORDER_BROADCAST_NOT_AFFECTED_ORDERS_GRP	500
#define MAX_DELETE_ALL_ORDER_RESPONSE_NOT_AFFECTED_ORDERS_GRP	500
#define MAX_DELETE_ALL_ORDER_RESPONSE_AFFECTED_ORDERS_GRP	500
#define MAX_EXTENDED_ORDER_INFORMATION_MULTI_LEG_GRP    	254
#define MAX_EXTENDED_ORDER_INFORMATION_MULTI_LEG_FILL_GRP	254
#define MAX_EXTENDED_ORDER_INFORMATION_INSTRMNT_LEG_EXEC_GRP	600
#define MAX_IMMEDIATE_EXECUTION_RESPONSE_MULTI_LEG_EXEC_GRP	254
#define MAX_IMMEDIATE_EXECUTION_RESPONSE_INSTRMNT_LEG_EXEC_GRP	600
#define MAX_MODIFY_ORDER_COMPLEX_REQUEST_LEG_ORD_GRP    	20
#define MAX_NEW_ORDER_COMPLEX_REQUEST_LEG_ORD_GRP       	20
#define MAX_NEW_ORDER_MULTI_LEG_MULTI_LEG_ORD_GRP       	254
#define MAX_ORDER_EXEC_NOTIFICATION_FILLS_GRP           	100
#define MAX_ORDER_EXEC_NOTIFICATION_INSTRMNT_LEG_EXEC_GRP	600
#define MAX_ORDER_EXEC_REPORT_BROADCAST_LEG_ORD_GRP     	20
#define MAX_ORDER_EXEC_REPORT_BROADCAST_FILLS_GRP       	100
#define MAX_ORDER_EXEC_REPORT_BROADCAST_INSTRMNT_LEG_EXEC_GRP	600
#define MAX_ORDER_EXEC_RESPONSE_FILLS_GRP               	100
#define MAX_ORDER_EXEC_RESPONSE_INSTRMNT_LEG_EXEC_GRP   	600
#define MAX_PRE_TRADE_RISK_LIMIT_RESPONSE_RISK_LIMITS_RPT_GRP	64
#define MAX_MARGIN_CHANGE_NOTIFICATION_MARGIN_CHANGE_RPT_GRP	50

/*
 * Data Type defines
 */

// DataType Account
#define LEN_ACCOUNT                                      2

// DataType AccountType
#define ENUM_ACCOUNT_TYPE_OWN                            1
#define ENUM_ACCOUNT_TYPE_CLIENT                         3
#define ENUM_ACCOUNT_TYPE_INSTITUTION                    5
#define ENUM_ACCOUNT_TYPE_NO_VALUE                       ((uint8_t) 0xff)

// DataType AggressorIndicator
#define ENUM_AGGRESSOR_INDICATOR_PASSIVE                 0
#define ENUM_AGGRESSOR_INDICATOR_AGGRESSIVE              1
#define ENUM_AGGRESSOR_INDICATOR_NO_VALUE                ((uint8_t) 0xff)

// DataType AlertLevel
#define ENUM_ALERT_LEVEL_ALERT1_SET_OI                   1
#define ENUM_ALERT_LEVEL_ALERT2_SET_OI                   2
#define ENUM_ALERT_LEVEL_ALERT3_SET_OI                   3
#define ENUM_ALERT_LEVEL_ALERT4_SET_OI                   4
#define ENUM_ALERT_LEVEL_NO_VALUE                        ((uint8_t) 0xff)

// DataType AllOrNoneFlag
#define LEN_ALL_OR_NONE_FLAG                             1
#define ENUM_ALL_OR_NONE_FLAG_PARTIAL_EXECUTION          "N"
#define ENUM_ALL_OR_NONE_FLAG_PARTIAL_EXECUTION_CHAR     'N'
#define ENUM_ALL_OR_NONE_FLAG_NO_VALUE                   0

// DataType ApplBegMsgID
#define LEN_APPL_BEG_MSGID                               16

// DataType ApplEndMsgID
#define LEN_APPL_END_MSGID                               16

// DataType ApplID
#define ENUM_APPLID_TRADE_ENHANCEMENT                    0
#define ENUM_APPLID_TRADE                                1
#define ENUM_APPLID_NEWS                                 2
#define ENUM_APPLID_SERVICE_AVAILABILITY                 3
#define ENUM_APPLID_SESSION_DATA                         4
#define ENUM_APPLID_LISTENER_DATA                        5
#define ENUM_APPLID_NO_VALUE                             ((uint8_t) 0xff)

// DataType ApplIDStatus
#define ENUM_APPL_ID_STATUS_OUTBOUND_CONVERSION_ERROR    105
#define ENUM_APPL_ID_STATUS_MORE_VALUES                  106
#define ENUM_APPL_ID_STATUS_NO_VALUE                     ((uint32_t) 0xffffffff)

// DataType ApplMsgID
#define LEN_APPL_MSGID                                   16

// DataType ApplResendFlag
#define ENUM_APPL_RESEND_FLAG_FALSE                      0
#define ENUM_APPL_RESEND_FLAG_TRUE                       1
#define ENUM_APPL_RESEND_FLAG_NO_VALUE                   ((uint8_t) 0xff)

// DataType ApplSeqIndicator
#define ENUM_APPL_SEQ_INDICATOR_NO_RECOVERY_REQUIRED     0
#define ENUM_APPL_SEQ_INDICATOR_STANDARD_ORDER           1
#define ENUM_APPL_SEQ_INDICATOR_NO_VALUE                 ((uint8_t) 0xff)

// DataType ApplSeqStatus
#define ENUM_APPL_SEQ_STATUS_UNAVAILABLE                 0
#define ENUM_APPL_SEQ_STATUS_AVAILABLE                   1
#define ENUM_APPL_SEQ_STATUS_NO_VALUE                    ((uint8_t) 0xff)

// DataType ApplUsageOrders
#define LEN_APPL_USAGE_ORDERS                            1
#define ENUM_APPL_USAGE_ORDERS_AUTOMATED                 "A"
#define ENUM_APPL_USAGE_ORDERS_AUTOMATED_CHAR            'A'
#define ENUM_APPL_USAGE_ORDERS_MANUAL                    "M"
#define ENUM_APPL_USAGE_ORDERS_MANUAL_CHAR               'M'
#define ENUM_APPL_USAGE_ORDERS_AUTO_SELECT               "B"
#define ENUM_APPL_USAGE_ORDERS_AUTO_SELECT_CHAR          'B'
#define ENUM_APPL_USAGE_ORDERS_NONE                      "N"
#define ENUM_APPL_USAGE_ORDERS_NONE_CHAR                 'N'
#define ENUM_APPL_USAGE_ORDERS_NO_VALUE                  0

// DataType ApplUsageQuotes
#define LEN_APPL_USAGE_QUOTES                            1
#define ENUM_APPL_USAGE_QUOTES_AUTOMATED                 "A"
#define ENUM_APPL_USAGE_QUOTES_AUTOMATED_CHAR            'A'
#define ENUM_APPL_USAGE_QUOTES_MANUAL                    "M"
#define ENUM_APPL_USAGE_QUOTES_MANUAL_CHAR               'M'
#define ENUM_APPL_USAGE_QUOTES_AUTO_SELECT               "B"
#define ENUM_APPL_USAGE_QUOTES_AUTO_SELECT_CHAR          'B'
#define ENUM_APPL_USAGE_QUOTES_NONE                      "N"
#define ENUM_APPL_USAGE_QUOTES_NONE_CHAR                 'N'
#define ENUM_APPL_USAGE_QUOTES_NO_VALUE                  0

// DataType ApplicationSystemName
#define LEN_APPLICATION_SYSTEM_NAME                      30

// DataType ApplicationSystemVendor
#define LEN_APPLICATION_SYSTEM_VENDOR                    30

// DataType ApplicationSystemVersion
#define LEN_APPLICATION_SYSTEM_VERSION                   30

// DataType AssetType
#define ENUM_ASSET_TYPE_ASSET_CLASS                      0
#define ENUM_ASSET_TYPE_ASSET                            1
#define ENUM_ASSET_TYPE_UNDERLYING                       2
#define ENUM_ASSET_TYPE_PRODUCT                          3
#define ENUM_ASSET_TYPE_NO_VALUE                         ((uint8_t) 0xff)

// DataType AuctionEnquiryTradingStatus
#define ENUM_AUCTION_ENQUIRY_TRADING_STATUS_NOT_DEFINED  0
#define ENUM_AUCTION_ENQUIRY_TRADING_STATUS_OPEN         1
#define ENUM_AUCTION_ENQUIRY_TRADING_STATUS_CLOSED       2
#define ENUM_AUCTION_ENQUIRY_TRADING_STATUS_NO_VALUE     ((uint8_t) 0xff)

// DataType CPCode
#define LEN_CP_CODE                                      12

// DataType CustOrderHandlingInst
#define LEN_CUST_ORDER_HANDLING_INST                     1

// DataType DefaultCstmApplVerID
#define LEN_DEFAULT_CSTM_APPL_VERID                      30

// DataType ExDExFlag
#define ENUM_EXD_EX_FLAG_EXERCISE_INSTRUCTION            1
#define ENUM_EXD_EX_FLAG_DONT_EXERCISE_INSTRUCTION       2
#define ENUM_EXD_EX_FLAG_NO_VALUE                        ((uint16_t) 0xffff)

// DataType ExDExstatus
#define ENUM_EXD_EXSTATUS_ACCEPTED                       1
#define ENUM_EXD_EXSTATUS_CANCELLED                      2
#define ENUM_EXD_EXSTATUS_NO_VALUE                       ((uint16_t) 0xffff)

// DataType ExecInst
#define ENUM_EXEC_INST_H                                 1
#define ENUM_EXEC_INST_Q                                 2
#define ENUM_EXEC_INST_NO_VALUE                          ((uint8_t) 0xff)

// DataType ExecRestatementReason
#define ENUM_EXEC_RESTATEMENT_REASON_ORDER_BOOK_RESTATEMENT 1
#define ENUM_EXEC_RESTATEMENT_REASON_ORDER_ADDED         101
#define ENUM_EXEC_RESTATEMENT_REASON_ORDER_MODIFIED      102
#define ENUM_EXEC_RESTATEMENT_REASON_ORDER_CANCELLED     103
#define ENUM_EXEC_RESTATEMENT_REASON_IOC_ORDER_CANCELLED 105
#define ENUM_EXEC_RESTATEMENT_REASON_BOOK_ORDER_EXECUTED 108
#define ENUM_EXEC_RESTATEMENT_REASON_CHANGED_TO_IOC      114
#define ENUM_EXEC_RESTATEMENT_REASON_INSTRUMENT_STATE_CHANGE 122
#define ENUM_EXEC_RESTATEMENT_REASON_MARKET_ORDER_TRIGGERED 135
#define ENUM_EXEC_RESTATEMENT_REASON_CAO_ORDER_ACTIVATED 149
#define ENUM_EXEC_RESTATEMENT_REASON_CAO_ORDER_INACTIVATED 150
#define ENUM_EXEC_RESTATEMENT_REASON_OCO_ORDER_TRIGGERED 164
#define ENUM_EXEC_RESTATEMENT_REASON_STOP_ORDER_TRIGGERED 172
#define ENUM_EXEC_RESTATEMENT_REASON_OWNERSHIP_CHANGED   181
#define ENUM_EXEC_RESTATEMENT_REASON_ORDER_CANCELLATION_PENDING 197
#define ENUM_EXEC_RESTATEMENT_REASON_PENDING_CANCELLATION_EXECUTED 199
#define ENUM_EXEC_RESTATEMENT_REASON_ORDER_ADD_SENT_RISK_VALIDATION 200
#define ENUM_EXEC_RESTATEMENT_REASON_SELF_TRADE_ORDER_DELETED 201
#define ENUM_EXEC_RESTATEMENT_REASON_ORDER_CANCELLED_SESSION_EXPIRY 202
#define ENUM_EXEC_RESTATEMENT_REASON_BOC_ORDER_CANCELLED 212
#define ENUM_EXEC_RESTATEMENT_REASON_RISK_REDUCTION_TIMER_EXPIRED 215
#define ENUM_EXEC_RESTATEMENT_REASON_PROVISIONAL_ORDER_ADDED 221
#define ENUM_EXEC_RESTATEMENT_REASON_ACTIVE_DELETE_DUE_TO_SMPF 248
#define ENUM_EXEC_RESTATEMENT_REASON_ACTIVE_MODIFY_DUE_TO_SMPF 250
#define ENUM_EXEC_RESTATEMENT_REASON_PASSIVE_DELETE_DUE_TO_SMPF 252
#define ENUM_EXEC_RESTATEMENT_REASON_PASSIVE_MODIFY_DUE_TO_SMPF 254
#define ENUM_EXEC_RESTATEMENT_REASON_PANIC_CANCEL        261
#define ENUM_EXEC_RESTATEMENT_REASON_EX_DEX_REQUEST_ADDED 270
#define ENUM_EXEC_RESTATEMENT_REASON_EX_DEX_REQUEST_MODIFIED 271
#define ENUM_EXEC_RESTATEMENT_REASON_EX_DEX_REQUEST_CANCELLED 272
#define ENUM_EXEC_RESTATEMENT_REASON_NO_VALUE            ((uint16_t) 0xffff)

// DataType ExecType
#define LEN_EXEC_TYPE                                    1
#define ENUM_EXEC_TYPE_NEW                               "0"
#define ENUM_EXEC_TYPE_NEW_CHAR                          '0'
#define ENUM_EXEC_TYPE_CANCELED                          "4"
#define ENUM_EXEC_TYPE_CANCELED_CHAR                     '4'
#define ENUM_EXEC_TYPE_REPLACED                          "5"
#define ENUM_EXEC_TYPE_REPLACED_CHAR                     '5'
#define ENUM_EXEC_TYPE_SUSPENDED                         "9"
#define ENUM_EXEC_TYPE_SUSPENDED_CHAR                    '9'
#define ENUM_EXEC_TYPE_RESTATED                          "D"
#define ENUM_EXEC_TYPE_RESTATED_CHAR                     'D'
#define ENUM_EXEC_TYPE_TRIGGERED                         "L"
#define ENUM_EXEC_TYPE_TRIGGERED_CHAR                    'L'
#define ENUM_EXEC_TYPE_TRADE                             "F"
#define ENUM_EXEC_TYPE_TRADE_CHAR                        'F'
#define ENUM_EXEC_TYPE_NO_VALUE                          0

// DataType FIXEngineName
#define LEN_FIX_ENGINE_NAME                              30

// DataType FIXEngineVendor
#define LEN_FIX_ENGINE_VENDOR                            30

// DataType FIXEngineVersion
#define LEN_FIX_ENGINE_VERSION                           30

// DataType FillLiquidityInd
#define ENUM_FILL_LIQUIDITY_IND_ADDED_LIQUIDITY          1
#define ENUM_FILL_LIQUIDITY_IND_REMOVED_LIQUIDITY        2
#define ENUM_FILL_LIQUIDITY_IND_AUCTION                  4
#define ENUM_FILL_LIQUIDITY_IND_TRIGGERED_STOP_ORDER     5
#define ENUM_FILL_LIQUIDITY_IND_TRIGGERED_OCO_ORDER      6
#define ENUM_FILL_LIQUIDITY_IND_TRIGGERED_MARKET_ORDER   7
#define ENUM_FILL_LIQUIDITY_IND_NO_VALUE                 ((uint8_t) 0xff)

// DataType Filler6
#define LEN_FILLER6                                      1

// DataType Filler7
#define LEN_FILLER7                                      32

// DataType FreeText1
#define LEN_FREE_TEXT1                                   12

// DataType FreeText2
#define LEN_FREE_TEXT2                                   12

// DataType FreeText3
#define LEN_FREE_TEXT3                                   12

// DataType GiveupStatus
#define ENUM_GIVEUP_STATUS_ACCEPTED                      1
#define ENUM_GIVEUP_STATUS_REJECTED                      2
#define ENUM_GIVEUP_STATUS_PENDING                       3
#define ENUM_GIVEUP_STATUS_NO_VALUE                      ((uint8_t) 0xff)

// DataType GroupID
#define ENUM_GROUPID_APPLICABLE_FOR_ALL_GROUPS           0
#define ENUM_GROUPID_NO_VALUE                            ((uint16_t) 0xffff)

// DataType Headline
#define LEN_HEADLINE                                     256

// DataType IVVector
#define LEN_IV_VECTOR                                    13

// DataType Key
#define LEN_KEY                                          33

// DataType LastFragment
#define ENUM_LAST_FRAGMENT_NOT_LAST_MESSAGE              0
#define ENUM_LAST_FRAGMENT_LAST_MESSAGE                  1
#define ENUM_LAST_FRAGMENT_NO_VALUE                      ((uint8_t) 0xff)

// DataType LegAccount
#define LEN_LEG_ACCOUNT                                  2

// DataType LegPositionEffect
#define LEN_LEG_POSITION_EFFECT                          1
#define ENUM_LEG_POSITION_EFFECT_CLOSE                   "C"
#define ENUM_LEG_POSITION_EFFECT_CLOSE_CHAR              'C'
#define ENUM_LEG_POSITION_EFFECT_NO_VALUE                0

// DataType LegSide
#define ENUM_LEG_SIDE_BUY                                1
#define ENUM_LEG_SIDE_SELL                               2
#define ENUM_LEG_SIDE_NO_VALUE                           ((uint8_t) 0xff)

// DataType MassActionReason
#define ENUM_MASS_ACTION_REASON_NO_SPECIAL_REASON        0
#define ENUM_MASS_ACTION_REASON_STOP_TRADING             1
#define ENUM_MASS_ACTION_REASON_EMERGENCY                2
#define ENUM_MASS_ACTION_REASON_MARKET_MAKER_PROTECTION  3
#define ENUM_MASS_ACTION_REASON_STOP_BUTTON_ACTIVATED    4
#define ENUM_MASS_ACTION_REASON_BUSINESS_UNIT_SUSPENDED  5
#define ENUM_MASS_ACTION_REASON_SESSION_LOSS             6
#define ENUM_MASS_ACTION_REASON_DUPLICATE_SESSION_LOGIN  7
#define ENUM_MASS_ACTION_REASON_CLEARING_RISK_CONTROL    8
#define ENUM_MASS_ACTION_REASON_INTERNAL_CONNECTION_LOSS 100
#define ENUM_MASS_ACTION_REASON_PRODUCT_STATE_HALT       105
#define ENUM_MASS_ACTION_REASON_PRODUCT_STATE_HOLIDAY    106
#define ENUM_MASS_ACTION_REASON_INSTRUMENT_SUSPENDED     107
#define ENUM_MASS_ACTION_REASON_PRICE_BAND_SHRINK        108
#define ENUM_MASS_ACTION_REASON_COMPLEX_INSTRUMENT_DELETION 109
#define ENUM_MASS_ACTION_REASON_VOLATILITY_INTERRUPTION  110
#define ENUM_MASS_ACTION_REASON_PRODUCT_TEMPORARILY_NOT_TRADEABLE 111
#define ENUM_MASS_ACTION_REASON_MEMBER_DISABLE           117
#define ENUM_MASS_ACTION_REASON_RRM_MARKET_WIDE_POSITION_LIMIT 119
#define ENUM_MASS_ACTION_REASON_PRODUCT_STATE_EOD        120
#define ENUM_MASS_ACTION_REASON_MEMBER_STATUS_CHANGE     179
#define ENUM_MASS_ACTION_REASON_RRM_CANCELLATION         202
#define ENUM_MASS_ACTION_REASON_SQUARE_OFF               203
#define ENUM_MASS_ACTION_REASON_USER_SUSPENDED           204
#define ENUM_MASS_ACTION_REASON_DEBARRED                 205
#define ENUM_MASS_ACTION_REASON_DISALLOWED               206
#define ENUM_MASS_ACTION_REASON_USER_DISABLED            207
#define ENUM_MASS_ACTION_REASON_ENTITY_SQUARE_OFF        208
#define ENUM_MASS_ACTION_REASON_SUSPENDED_SQUARE_OFF     209
#define ENUM_MASS_ACTION_REASON_SUSPENDED                217
#define ENUM_MASS_ACTION_REASON_NO_VALUE                 ((uint8_t) 0xff)

// DataType MatchSubType
#define ENUM_MATCH_SUB_TYPE_OPENING_AUCTION              1
#define ENUM_MATCH_SUB_TYPE_INTRADAY_AUCTION             3
#define ENUM_MATCH_SUB_TYPE_NO_VALUE                     ((uint8_t) 0xff)

// DataType MatchType
#define ENUM_MATCH_TYPE_CONFIRMED_TRADE_REPORT           3
#define ENUM_MATCH_TYPE_AUTO_MATCH_INCOMING              4
#define ENUM_MATCH_TYPE_AUTO_MATCH_RESTING               11
#define ENUM_MATCH_TYPE_NO_VALUE                         ((uint8_t) 0xff)

// DataType MatchingEngineStatus
#define ENUM_MATCHING_ENGINE_STATUS_UNAVAILABLE          0
#define ENUM_MATCHING_ENGINE_STATUS_AVAILABLE            1
#define ENUM_MATCHING_ENGINE_STATUS_NO_VALUE             ((uint8_t) 0xff)

// DataType MsgType
#define LEN_MSG_TYPE                                     3
#define ENUM_MSG_TYPE_HEARTBEAT                          "0  "
#define ENUM_MSG_TYPE_TEST_REQUEST                       "1  "
#define ENUM_MSG_TYPE_REJECT                             "3  "
#define ENUM_MSG_TYPE_LOGOUT                             "5  "
#define ENUM_MSG_TYPE_EXECUTION_REPORT                   "8  "
#define ENUM_MSG_TYPE_LOGON                              "A  "
#define ENUM_MSG_TYPE_NEW_ORDER_MULTILEG                 "AB "
#define ENUM_MSG_TYPE_MULTILEG_ORDER_CANCEL_REPLACE      "AC "
#define ENUM_MSG_TYPE_TRADE_CAPTURE_REPORT               "AE "
#define ENUM_MSG_TYPE_TRADE_CAPTURE_REPORT_ACK           "AR "
#define ENUM_MSG_TYPE_NEWS                               "B  "
#define ENUM_MSG_TYPE_LOGONR                              "B1  "
#define ENUM_MSG_TYPE_USER_REQUEST                       "BE "
#define ENUM_MSG_TYPE_USER_RESPONSE                      "BF "
#define ENUM_MSG_TYPE_APPLICATION_MESSAGE_REQUEST        "BW "
#define ENUM_MSG_TYPE_APPLICATION_MESSAGE_REQUEST_ACK    "BX "
#define ENUM_MSG_TYPE_APPLICATION_MESSAGE_REPORT         "BY "
#define ENUM_MSG_TYPE_ORDER_MASS_ACTION_REPORT           "BZ "
#define ENUM_MSG_TYPE_ORDER_MASS_ACTION_REQUEST          "CA "
#define ENUM_MSG_TYPE_USER_NOTIFICATION                  "CB "
#define ENUM_MSG_TYPE_NEW_ORDER_SINGLE                   "D  "
#define ENUM_MSG_TYPE_ORDER_CANCEL_REQUEST               "F  "
#define ENUM_MSG_TYPE_ORDER_CANCEL_REPLACE_REQUEST       "G  "
#define ENUM_MSG_TYPE_PARTY_RISK_LIMITS_REQUEST          "CL "
#define ENUM_MSG_TYPE_PARTY_RISK_LIMITS_REPORT           "CM "
#define ENUM_MSG_TYPE_TRADING_SESSION_STATUS             "h  "
#define ENUM_MSG_TYPE_SECURITY_STATUS_DEFINITION_REQUEST "U27"
#define ENUM_MSG_TYPE_MULTI_LEG_ORDER                    "U28"
#define ENUM_MSG_TYPE_MULTI_LEG_ORDER_ACKNOWLEDGEMENT    "U29"
#define ENUM_MSG_TYPE_MULTI_LEG_EXECUTION_REPORT         "U30"
#define ENUM_MSG_TYPE_TRADE_ENHANCEMENT_STATUS           "U31"
#define ENUM_MSG_TYPE_NOTIFICATION_AUCTION_REQUEST       "U44"
#define ENUM_MSG_TYPE_EXDEX_ENTRY_REQUEST                "U74"
#define ENUM_MSG_TYPE_EXDEX_ENTRY_CONFIRMATION           "U75"
#define ENUM_MSG_TYPE_EXDEX_CANCELLATION_REQUEST         "U76"
#define ENUM_MSG_TYPE_EXDEX_MODIFICATION_REQUEST         "U77"
#define ENUM_MSG_TYPE_EXDEX_NOTIFICATION                 "U78"
#define ENUM_MSG_TYPE_SYSTEM_INFORMATION                 "U79"
#define ENUM_MSG_TYPE_EXDEX_MODIFICATION_CONFIRMATION    "U80"
#define ENUM_MSG_TYPE_EXDEX_CANCELLATION_CONFIRMATION    "U81"
#define ENUM_MSG_TYPE_SESSION_PASSWORD_CHANGE_RESPONSE   "U82"
#define ENUM_MSG_TYPE_USER_PASSWORD_CHANGE_REQUEST       "U83"
#define ENUM_MSG_TYPE_SESSION_PASSWORD_CHANGE            "U84"
#define ENUM_MSG_TYPE_USER_PASSWORD_CHANGE_RESPONSE      "U85"
#define ENUM_MSG_TYPE_RESUBMIT_APPROVAL_REQUEST          "U86"
#define ENUM_MSG_TYPE_RESUBMIT_APPROVAL_CONFIRMATION     "U87"
#define ENUM_MSG_TYPE_MARKET_WIDE_OI_VIOLATION           "U88"
#define ENUM_MSG_TYPE_MARKET_WIDE_OI_ALERT               "U89"
#define ENUM_MSG_TYPE_MARKET_STATUS_NOTIFICATION         "U90"
#define ENUM_MSG_TYPE_MARGIN_CHANGE_NOTIFICATION         "U91"
#define ENUM_MSG_TYPE_TRADE_MODIFICATION_REQUEST         "U92"
#define ENUM_MSG_TYPE_TRADE_MODIFICATION_RESPONSE        "U93"
#define ENUM_MSG_TYPE_TRADE_MODIFICATION_NOTIFICATIONS   "U94"
#define ENUM_MSG_TYPE_NO_VALUE                             0

// DataType MultiLegReportingType
#define ENUM_MULTI_LEG_REPORTING_TYPE_SINGLE_SECURITY    1
#define ENUM_MULTI_LEG_REPORTING_TYPE_INDIVIDUAL_LEG_COMPLEX_ORDER 2
#define ENUM_MULTI_LEG_REPORTING_TYPE_NO_VALUE           ((uint8_t) 0xff)

// DataType NetworkMsgID
#define LEN_NETWORK_MSGID                                8

// DataType NewAccount
#define LEN_NEW_ACCOUNT                                  10

// DataType NewAccountType
#define LEN_NEW_ACCOUNT_TYPE                             1
#define ENUM_NEW_ACCOUNT_TYPE_OWN                        "1"
#define ENUM_NEW_ACCOUNT_TYPE_OWN_CHAR                   '1'
#define ENUM_NEW_ACCOUNT_TYPE_CLIENT                     "3"
#define ENUM_NEW_ACCOUNT_TYPE_CLIENT_CHAR                '3'
#define ENUM_NEW_ACCOUNT_TYPE_INSTITUTION                "5"
#define ENUM_NEW_ACCOUNT_TYPE_INSTITUTION_CHAR           '5'
#define ENUM_NEW_ACCOUNT_TYPE_NO_VALUE                   0

// DataType NewCPCode
#define LEN_NEW_CP_CODE                                  12

// DataType NewClearingAccount
#define LEN_NEW_CLEARING_ACCOUNT                         12

// DataType NewCustomerorFirm
#define LEN_NEW_CUSTOMEROR_FIRM                          1
#define ENUM_NEW_CUSTOMEROR_FIRM_OWN                     "1"
#define ENUM_NEW_CUSTOMEROR_FIRM_OWN_CHAR                '1'
#define ENUM_NEW_CUSTOMEROR_FIRM_CLIENT                  "3"
#define ENUM_NEW_CUSTOMEROR_FIRM_CLIENT_CHAR             '3'
#define ENUM_NEW_CUSTOMEROR_FIRM_INSTITUTION             "5"
#define ENUM_NEW_CUSTOMEROR_FIRM_INSTITUTION_CHAR        '5'
#define ENUM_NEW_CUSTOMEROR_FIRM_NO_VALUE                0

// DataType NewPassword
#define LEN_NEW_PASSWORD                                 344

// DataType NewSpecialMarginFlatPerc
#define ENUM_NEW_SPECIAL_MARGIN_FLAT_PERC_IM_PERCENTAGE  1
#define ENUM_NEW_SPECIAL_MARGIN_FLAT_PERC_IM_ABSOLUTE    2
#define ENUM_NEW_SPECIAL_MARGIN_FLAT_PERC_NO_VALUE       ((uint16_t) 0xffff)

// DataType NewUCCCode
#define LEN_NEW_UCC_CODE                                 10

// DataType OldAccount
#define LEN_OLD_ACCOUNT                                  10

// DataType OldAccountType
#define LEN_OLD_ACCOUNT_TYPE                             1
#define ENUM_OLD_ACCOUNT_TYPE_OWN                        "1"
#define ENUM_OLD_ACCOUNT_TYPE_OWN_CHAR                   '1'
#define ENUM_OLD_ACCOUNT_TYPE_CLIENT                     "3"
#define ENUM_OLD_ACCOUNT_TYPE_CLIENT_CHAR                '3'
#define ENUM_OLD_ACCOUNT_TYPE_INSTITUTION                "5"
#define ENUM_OLD_ACCOUNT_TYPE_INSTITUTION_CHAR           '5'
#define ENUM_OLD_ACCOUNT_TYPE_NO_VALUE                   0

// DataType OldCPCode
#define LEN_OLD_CP_CODE                                  12

// DataType OldClearingAccount
#define LEN_OLD_CLEARING_ACCOUNT                         12

// DataType OldCustomerorFirm
#define LEN_OLD_CUSTOMEROR_FIRM                          1
#define ENUM_OLD_CUSTOMEROR_FIRM_OWN                     "1"
#define ENUM_OLD_CUSTOMEROR_FIRM_OWN_CHAR                '1'
#define ENUM_OLD_CUSTOMEROR_FIRM_CLIENT                  "3"
#define ENUM_OLD_CUSTOMEROR_FIRM_CLIENT_CHAR             '3'
#define ENUM_OLD_CUSTOMEROR_FIRM_INSTITUTION             "5"
#define ENUM_OLD_CUSTOMEROR_FIRM_INSTITUTION_CHAR        '5'
#define ENUM_OLD_CUSTOMEROR_FIRM_NO_VALUE                0

// DataType OldInitMarginFlatPerc
#define ENUM_OLD_INIT_MARGIN_FLAT_PERC_IM_PERCENTAGE     1
#define ENUM_OLD_INIT_MARGIN_FLAT_PERC_IM_ABSOLUTE       2
#define ENUM_OLD_INIT_MARGIN_FLAT_PERC_NO_VALUE          ((uint16_t) 0xffff)

// DataType OldInitialMarginSpreadBenefitonorOffFlag
#define ENUM_OLD_INITIAL_MARGIN_SPREAD_BENEFITONOR_OFF_FLAG_OFF 0
#define ENUM_OLD_INITIAL_MARGIN_SPREAD_BENEFITONOR_OFF_FLAG_ON 1
#define ENUM_OLD_INITIAL_MARGIN_SPREAD_BENEFITONOR_OFF_FLAG_NO_VALUE ((uint16_t) 0xffff)

// DataType OldSpecialMarginFlatPerc
#define ENUM_OLD_SPECIAL_MARGIN_FLAT_PERC_IM_PERCENTAGE  1
#define ENUM_OLD_SPECIAL_MARGIN_FLAT_PERC_IM_ABSOLUTE    2
#define ENUM_OLD_SPECIAL_MARGIN_FLAT_PERC_NO_VALUE       ((uint16_t) 0xffff)

// DataType OldUCCCode
#define LEN_OLD_UCC_CODE                                 10

// DataType OrdStatus
#define LEN_ORD_STATUS                                   1
#define ENUM_ORD_STATUS_NEW                              "0"
#define ENUM_ORD_STATUS_NEW_CHAR                         '0'
#define ENUM_ORD_STATUS_PARTIALLY_FILLED                 "1"
#define ENUM_ORD_STATUS_PARTIALLY_FILLED_CHAR            '1'
#define ENUM_ORD_STATUS_FILLED                           "2"
#define ENUM_ORD_STATUS_FILLED_CHAR                      '2'
#define ENUM_ORD_STATUS_CANCELED                         "4"
#define ENUM_ORD_STATUS_CANCELED_CHAR                    '4'
#define ENUM_ORD_STATUS_PENDING_CANCEL                   "6"
#define ENUM_ORD_STATUS_PENDING_CANCEL_CHAR              '6'
#define ENUM_ORD_STATUS_RRM_SUSPENDED                    "7"
#define ENUM_ORD_STATUS_RRM_SUSPENDED_CHAR               '7'
#define ENUM_ORD_STATUS_SQUARE_OFF_SUSPENDED             "8"
#define ENUM_ORD_STATUS_SQUARE_OFF_SUSPENDED_CHAR        '8'
#define ENUM_ORD_STATUS_SUSPENDED                        "9"
#define ENUM_ORD_STATUS_SUSPENDED_CHAR                   '9'
#define ENUM_ORD_STATUS_NO_VALUE                         0

// DataType OrdType
#define ENUM_ORD_TYPE_MARKET                             1
#define ENUM_ORD_TYPE_LIMIT                              2
#define ENUM_ORD_TYPE_STOP                               3
#define ENUM_ORD_TYPE_STOP_LIMIT                         4
#define ENUM_ORD_TYPE_MARKET_TO_LIMIT                    5
#define ENUM_ORD_TYPE_AUCTION_BUY_IN                     6
#define ENUM_ORD_TYPE_AUCTION_SELL_OUT                   7
#define ENUM_ORD_TYPE_NO_VALUE                           ((uint8_t) 0xff)

// DataType OrderCategory
#define LEN_ORDER_CATEGORY                               1
#define ENUM_ORDER_CATEGORY_ORDER                        "1"
#define ENUM_ORDER_CATEGORY_ORDER_CHAR                   '1'
#define ENUM_ORDER_CATEGORY_MULTILEG_ORDER               "3"
#define ENUM_ORDER_CATEGORY_MULTILEG_ORDER_CHAR          '3'
#define ENUM_ORDER_CATEGORY_NO_VALUE                     0

// DataType OrderRoutingIndicator
#define LEN_ORDER_ROUTING_INDICATOR                      1
#define ENUM_ORDER_ROUTING_INDICATOR_YES                 "Y"
#define ENUM_ORDER_ROUTING_INDICATOR_YES_CHAR            'Y'
#define ENUM_ORDER_ROUTING_INDICATOR_NO                  "N"
#define ENUM_ORDER_ROUTING_INDICATOR_NO_CHAR             'N'
#define ENUM_ORDER_ROUTING_INDICATOR_NO_VALUE            0

// DataType OrderSide
#define ENUM_ORDER_SIDE_BUY                              1
#define ENUM_ORDER_SIDE_SELL                             2
#define ENUM_ORDER_SIDE_NO_VALUE                         ((uint8_t) 0xff)

// DataType Pad1
#define LEN_PAD1                                         1

// DataType Pad2
#define LEN_PAD2                                         2

// DataType Pad3
#define LEN_PAD3                                         3

// DataType Pad4
#define LEN_PAD4                                         4

// DataType Pad4_1
#define LEN_PAD4_1                                       4

// DataType Pad5
#define LEN_PAD5                                         5

// DataType Pad6
#define LEN_PAD6                                         6

// DataType Pad7
#define LEN_PAD7                                         7

// DataType PartyDetailExecutingUnit
#define LEN_PARTY_DETAIL_EXECUTING_UNIT                  5

// DataType PartyDetailStatus
#define ENUM_PARTY_DETAIL_STATUS_ACTIVE                  0
#define ENUM_PARTY_DETAIL_STATUS_SUSPEND                 1
#define ENUM_PARTY_DETAIL_STATUS_REJECTED                2
#define ENUM_PARTY_DETAIL_STATUS_NO_VALUE                ((uint8_t) 0xff)

// DataType PartyExecutingUnit
#define LEN_PARTY_EXECUTING_UNIT                         5

// DataType PartyIDBeneficiary
#define LEN_PARTY_ID_BENEFICIARY                         9

// DataType PartyIDEnteringFirm
#define ENUM_PARTY_ID_ENTERING_FIRM_PARTICIPANT          1
#define ENUM_PARTY_ID_ENTERING_FIRM_MARKET_SUPERVISION   2
#define ENUM_PARTY_ID_ENTERING_FIRM_NO_VALUE             ((uint8_t) 0xff)

// DataType PartyIDLocationID
#define LEN_PARTY_ID_LOCATIONID                          2

// DataType PartyIDOrderOriginationFirm
#define LEN_PARTY_ID_ORDER_ORIGINATION_FIRM              7

// DataType PartyIDTakeUpTradingFirm
#define LEN_PARTY_ID_TAKE_UP_TRADING_FIRM                5

// DataType Password
#define LEN_PASSWORD                                     344

// DataType PositionEffect
#define LEN_POSITION_EFFECT                              1
#define ENUM_POSITION_EFFECT_CLOSE                       "C"
#define ENUM_POSITION_EFFECT_CLOSE_CHAR                  'C'
#define ENUM_POSITION_EFFECT_NO_VALUE                    0

// DataType PriceValidityCheckType
#define ENUM_PRICE_VALIDITY_CHECK_TYPE_NONE              0
#define ENUM_PRICE_VALIDITY_CHECK_TYPE_NO_VALUE          ((uint8_t) 0xff)

// DataType ProductComplex
#define ENUM_PRODUCT_COMPLEX_SIMPLE_INSTRUMENT           1
#define ENUM_PRODUCT_COMPLEX_FUTURES_SPREAD              5
#define ENUM_PRODUCT_COMPLEX_NO_VALUE                    ((uint8_t) 0xff)

// DataType ProductLine
#define LEN_PRODUCT_LINE                                 1
#define ENUM_PRODUCT_LINE_FUTURE                         "F"
#define ENUM_PRODUCT_LINE_FUTURE_CHAR                    'F'
#define ENUM_PRODUCT_LINE_OPTION                         "O"
#define ENUM_PRODUCT_LINE_OPTION_CHAR                    'O'
#define ENUM_PRODUCT_LINE_NO_VALUE                       0

// DataType RefApplID
#define ENUM_REF_APPLID_TRADE                            1
#define ENUM_REF_APPLID_NEWS                             2
#define ENUM_REF_APPLID_SERVICE_AVAILABILITY             3
#define ENUM_REF_APPLID_SESSION_DATA                     4
#define ENUM_REF_APPLID_LISTENER_DATA                    5
#define ENUM_REF_APPLID_SERVICE_AVAILABILITY_MARKET      10
#define ENUM_REF_APPLID_EXD_EX                           12
#define ENUM_REF_APPLID_TRADE_ENHANCEMENT                13
#define ENUM_REF_APPLID_NO_VALUE                         ((uint8_t) 0xff)

// DataType RefApplLastMsgID
#define LEN_REF_APPL_LAST_MSGID                          16

// DataType RegulatoryID
#define LEN_REGULATORY_ID		                         4

// DataType RelatedProductComplex
#define ENUM_RELATED_PRODUCT_COMPLEX_FUTURES_SPREAD      5
#define ENUM_RELATED_PRODUCT_COMPLEX_NO_VALUE            ((uint8_t) 0xff)

// DataType RequestingPartyEnteringFirm
#define LEN_REQUESTING_PARTY_ENTERING_FIRM               9

// DataType RiskLimitGroup
#define LEN_RISK_LIMIT_GROUP                             3

// DataType RiskLimitPlatform
#define ENUM_RISK_LIMIT_PLATFORM_ON_BOOK                 0
#define ENUM_RISK_LIMIT_PLATFORM_OFF_BOOK                1
#define ENUM_RISK_LIMIT_PLATFORM_NO_VALUE                ((uint8_t) 0xff)

// DataType RiskLimitType
#define ENUM_RISK_LIMIT_TYPE_LONG_LIMIT                  4
#define ENUM_RISK_LIMIT_TYPE_SHORT_LIMIT                 5
#define ENUM_RISK_LIMIT_TYPE_NO_VALUE                    ((uint8_t) 0xff)

// DataType RootPartyClearingFirm
#define LEN_ROOT_PARTY_CLEARING_FIRM                     5

// DataType RootPartyClearingOrganization
#define LEN_ROOT_PARTY_CLEARING_ORGANIZATION             4

// DataType RootPartyExecutingFirm
#define LEN_ROOT_PARTY_EXECUTING_FIRM                    5

// DataType RootPartyExecutingTrader
#define LEN_ROOT_PARTY_EXECUTING_TRADER                  6

// DataType RootPartyIDBeneficiary
#define LEN_ROOT_PARTY_ID_BENEFICIARY                    9

// DataType RootPartyIDOrderOriginationFirm
#define LEN_ROOT_PARTY_ID_ORDER_ORIGINATION_FIRM         7

// DataType RootPartyIDTakeUpTradingFirm
#define LEN_ROOT_PARTY_ID_TAKE_UP_TRADING_FIRM           5

// DataType SMPFOrderIdentifier
#define ENUM_SMPF_ORDER_IDENTIFIER_PASSIVE               0
#define ENUM_SMPF_ORDER_IDENTIFIER_ACTIVE                1
#define ENUM_SMPF_ORDER_IDENTIFIER_NO_VALUE              ((uint8_t) 0xff)

// DataType STPCFlag
#define ENUM_STPC_FLAG_PASSIVE                           0
#define ENUM_STPC_FLAG_ACTIVE                            1
#define ENUM_STPC_FLAG_NO_VALUE                          ((uint8_t) 0xff)

// DataType SecurityType
#define ENUM_SECURITY_TYPE_CUMMULATIVE                   0
#define ENUM_SECURITY_TYPE_UNDERLYING_FUTURES            1
#define ENUM_SECURITY_TYPE_OPTIONS                       3
#define ENUM_SECURITY_TYPE_FUTURES                       4
#define ENUM_SECURITY_TYPE_NO_VALUE                      ((uint16_t) 0xffff)

// DataType SessionMode
#define ENUM_SESSION_MODE_LF                             2
#define ENUM_SESSION_MODE_NO_VALUE                       ((uint8_t) 0xff)

// DataType SessionRejectReason
#define ENUM_SESSION_REJECT_REASON_REQUIRED_TAG_MISSING  1
#define ENUM_SESSION_REJECT_REASON_VALUE_IS_INCORRECT    5
#define ENUM_SESSION_REJECT_REASON_DECRYPTION_PROBLEM    7
#define ENUM_SESSION_REJECT_REASON_INVALID_MSGID         11
#define ENUM_SESSION_REJECT_REASON_INCORRECT_NUM_IN_GROUP_COUNT 16
#define ENUM_SESSION_REJECT_REASON_OTHER                 99
#define ENUM_SESSION_REJECT_REASON_THROTTLE_LIMIT_EXCEEDED 100
#define ENUM_SESSION_REJECT_REASON_EXPOSURE_LIMIT_EXCEEDED 101
#define ENUM_SESSION_REJECT_REASON_SERVICE_TEMPORARILY_NOT_AVAILABLE 102
#define ENUM_SESSION_REJECT_REASON_SERVICE_NOT_AVAILABLE 103
#define ENUM_SESSION_REJECT_REASON_RESULT_TRANSACTION_UNKNOWN 104
#define ENUM_SESSION_REJECT_REASON_OUTBOUND_CONVERSION_ERROR 105
#define ENUM_SESSION_REJECT_REASON_HEARTBEAT_VIOLATION   152
#define ENUM_SESSION_REJECT_REASON_INTERNAL_TECHNICAL_ERROR 200
#define ENUM_SESSION_REJECT_REASON_VALIDATION_ERROR      210
#define ENUM_SESSION_REJECT_REASON_USER_ALREADY_LOGGED_IN 211
#define ENUM_SESSION_REJECT_REASON_ORDER_NOT_FOUND       10000
#define ENUM_SESSION_REJECT_REASON_PRICE_NOT_REASONABLE  10001
#define ENUM_SESSION_REJECT_REASON_BU_BOOK_ORDER_LIMIT_EX_CEEDED 10004
#define ENUM_SESSION_REJECT_REASON_SESSION_BOOK_ORDER_LIMIT_EXCEEDED 10005
#define ENUM_SESSION_REJECT_REASON_LST_UPDATE_TIMESTAMP_NOT_MATCHED 10006
#define ENUM_SESSION_REJECT_REASON_NO_VALUE              ((uint32_t) 0xffffffff)

// DataType SessionStatus
#define ENUM_SESSION_STATUS_ACTIVE                       0
#define ENUM_SESSION_STATUS_LOGOUT                       4
#define ENUM_SESSION_STATUS_NO_VALUE                     ((uint8_t) 0xff)

// DataType Side
#define ENUM_SIDE_BUY                                    1
#define ENUM_SIDE_SELL                                   2
#define ENUM_SIDE_NO_VALUE                               ((uint8_t) 0xff)

// DataType Status
#define ENUM_STATUS_OPEN                                 1
#define ENUM_STATUS_CLOSED                               2
#define ENUM_STATUS_HALT                                 3
#define ENUM_STATUS_NO_VALUE                             ((uint16_t) 0xffff)

// DataType Text
#define LEN_TEXT                                         12

// DataType TimeInForce
#define ENUM_TIME_IN_FORCE_DAY                           0
#define ENUM_TIME_IN_FORCE_GTC                           1
#define ENUM_TIME_IN_FORCE_IOC                           3
#define ENUM_TIME_IN_FORCE_GTD                           6
#define ENUM_TIME_IN_FORCE_SESSION                       7
#define ENUM_TIME_IN_FORCE_NO_VALUE                      ((uint8_t) 0xff)

// DataType TradSesEvent
#define ENUM_TRAD_SES_EVENT_START_OF_SERVICE             101
#define ENUM_TRAD_SES_EVENT_MARKET_RESET                 102
#define ENUM_TRAD_SES_EVENT_END_OF_RESTATEMENT           103
#define ENUM_TRAD_SES_EVENT_END_OF_DAY_SERVICE           104
#define ENUM_TRAD_SES_EVENT_SERVICE_RESUMED              105
#define ENUM_TRAD_SES_EVENT_NO_VALUE                     ((uint8_t) 0xff)

// DataType TradSesMode
#define ENUM_TRAD_SES_MODE_SIMULATED                     2
#define ENUM_TRAD_SES_MODE_PRODUCTION                    3
#define ENUM_TRAD_SES_MODE_NO_VALUE                      ((uint8_t) 0xff)


// DataType TradeManagerStatus
#define ENUM_TRADE_MANAGER_STATUS_UNAVAILABLE            0
#define ENUM_TRADE_MANAGER_STATUS_AVAILABLE              1
#define ENUM_TRADE_MANAGER_STATUS_NO_VALUE               ((uint8_t) 0xff)

// DataType TradeReportType
#define ENUM_TRADE_REPORT_TYPE_SUBMIT                    0
#define ENUM_TRADE_REPORT_TYPE_ALLEGED                   1
#define ENUM_TRADE_REPORT_TYPE_ACCEPT                    2
#define ENUM_TRADE_REPORT_TYPE_DECLINE                   3
#define ENUM_TRADE_REPORT_TYPE_NO_WAS_REPLACED           5
#define ENUM_TRADE_REPORT_TYPE_TRADE_REPORT_CANCEL       6
#define ENUM_TRADE_REPORT_TYPE_TRADE_BREAK               7
#define ENUM_TRADE_REPORT_TYPE_ALLEGED_NEW               11
#define ENUM_TRADE_REPORT_TYPE_ALLEGED_NO_WAS            13
#define ENUM_TRADE_REPORT_TYPE_NO_VALUE                  ((uint8_t) 0xff)

// DataType TradingCapacity
#define ENUM_TRADING_CAPACITY_CUSTOMER                   1
#define ENUM_TRADING_CAPACITY_NO_VALUE                   ((uint8_t) 0xff)

// DataType TradingSessionSubID
#define ENUM_TRADING_SESSION_SUBID_START_OF_DAY          1
#define ENUM_TRADING_SESSION_SUBID_PRE_TRADING           2
#define ENUM_TRADING_SESSION_SUBID_TRADING               3
#define ENUM_TRADING_SESSION_SUBID_CLOSING_AUCTION       4
#define ENUM_TRADING_SESSION_SUBID_POST_TRADING          5
#define ENUM_TRADING_SESSION_SUBID_END_OF_DAY            6
#define ENUM_TRADING_SESSION_SUBID_POST_END_OF_DAY       7
#define ENUM_TRADING_SESSION_SUBID_HALT                  8
#define ENUM_TRADING_SESSION_SUBID_CLOSING_WITH_TAC      10
#define ENUM_TRADING_SESSION_SUBID_SPECIAL_AUCTION       105
#define ENUM_TRADING_SESSION_SUBID_NO_VALUE              ((uint8_t) 0xff)

// DataType TransferReason
#define LEN_TRANSFER_REASON			                      1

// DataType Triggered
#define ENUM_TRIGGERED_NOT_TRIGGERED                     0
#define ENUM_TRIGGERED_TRIGGERED_STOP                    1
#define ENUM_TRIGGERED_NO_VALUE                          ((uint8_t) 0xff)

// DataType UserReferenceText
#define LEN_USER_REFERENCE_TEXT                          20

// DataType VarText
#define LEN_VAR_TEXT                                     2000

// DataType ViolationStatus
#define ENUM_VIOLATION_STATUS_ACTIVE                     0
#define ENUM_VIOLATION_STATUS_INACTIVE                   1
#define ENUM_VIOLATION_STATUS_NO_VALUE                   ((uint8_t) 0xff)

/*
 * Structure defines for components and sequences
 */


// Structure: AffectedOrdersGrp
typedef struct
{
    uint64_t AffectedOrderRequestID;
} AffectedOrdersGrpSeqT;

// Structure: FillsGrp
typedef struct
{
    int64_t FillPx;
    int64_t FillQty;
    uint32_t FillMatchID;
    int32_t FillExecID;
    uint8_t FillLiquidityInd;
    char Pad7[LEN_PAD7];
} FillsGrpSeqT;

// Structure: InstrmntLegExecGrp
typedef struct
{
    int64_t LegSecurityID;
    int64_t LegLastPx;
    int64_t LegLastQty;
    int32_t LegExecID;
    uint8_t LegSide;
    uint8_t FillRefID;
    char Pad2[LEN_PAD2];
} InstrmntLegExecGrpSeqT;

// Structure: LegOrdGrp
typedef struct
{
    char LegAccount[LEN_LEG_ACCOUNT];
    char LegPositionEffect[LEN_LEG_POSITION_EFFECT];
    char Pad5[LEN_PAD5];
} LegOrdGrpSeqT;

// Structure: MarginChangeRptGrp
typedef struct
{
    uint64_t InstrumentIdentifier;
    int64_t OldInitialBuyMarginRate;
    int64_t OldInitialSellMarginRate;
    int64_t OldSpecialBuyMarginRate;
    int64_t OldSpecialSellMarginRate;
    int64_t OldExtremeLossBuyMarginRate;
    int64_t OldExtremeLossSellMarginRate;
    int64_t NewInitialBuyMarginRate;
    int64_t NewInitialSellMarginRate;
    int64_t NewSpecialBuyMarginRate;
    int64_t NewSpecialSellMarginRate;
    int64_t NewExtremeLossBuyMarginRate;
    int64_t NewExtremeLossSellMarginRate;
    uint16_t OldInitMarginFlatPerc;
    uint16_t OldInitialMarginSpreadBenefitonorOffFlag;
    uint16_t OldSpecialMarginFlatPerc;
    uint16_t OldConfigurationforExtremeLossMargin;
    uint16_t NewInitMarginFlatPerc;
    uint16_t NewInitialMarginSpreadBenefitOnorOffFlag;
    uint16_t NewSpecialMarginFlatPerc;
    uint16_t NewConfigurationforExtremeLossMargin;
} MarginChangeRptGrpSeqT;

// Structure: MessageHeaderIn
typedef struct
{
    uint32_t BodyLen;
    uint16_t TemplateID;
    char NetworkMsgID[LEN_NETWORK_MSGID];
    char Pad2[LEN_PAD2];
} MessageHeaderInCompT;

// Structure: MessageHeaderOut
typedef struct
{
    uint32_t BodyLen;
    uint16_t TemplateID;
    char Pad2[LEN_PAD2];
} MessageHeaderOutCompT;

// Structure: MultiLegExecGrp
typedef struct
{
    uint64_t OrderID;
    uint64_t InstrumentIdentifier;
    int64_t CumQty;
    int64_t CxlQty;
    uint16_t ExecRestatementReason;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    char Pad4[LEN_PAD4];
} MultiLegExecGrpSeqT;

// Structure: MultiLegFillGrp
typedef struct
{
    int64_t FillPx;
    int64_t SecurityID;
    int64_t FillQty;
    uint32_t FillMatchID;
    int32_t FillExecID;
} MultiLegFillGrpSeqT;

// Structure: MultiLegGrp
typedef struct
{
    uint64_t InstrumentIdentifier;
    uint64_t OrderID;
    int64_t Price;
    int64_t MaxPricePercentage;
    int32_t Echo;
    char Pad4[LEN_PAD4];
    int64_t OrderQty;
    int64_t CumQty;
    int64_t CxlQty;
    uint16_t ExecRestatementReason;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint8_t Side;
    uint8_t OrdType;
    char Pad6[LEN_PAD6];
	char Pad4_1[LEN_PAD4_1];
} MultiLegGrpSeqT;

// Structure: MultiLegOrdGrp
typedef struct
{
    uint64_t InstrumentIdentifier;
    int64_t Price;
    int64_t MaxPricePercentage;
    int32_t Echo;
    int32_t MarketSegmentID;
    int64_t OrderQty;
    uint8_t ProductComplex;
    uint8_t Side;
    uint8_t OrdType;
    char Pad5[LEN_PAD5];
} MultiLegOrdGrpSeqT;

// Structure: NRBCHeader
typedef struct
{
    uint64_t SendingTime;
    uint32_t ApplSubID;
    uint8_t ApplID;
    uint8_t LastFragment;
    char Pad2[LEN_PAD2];
} NRBCHeaderCompT;

// Structure: NRResponseHeaderME
typedef struct
{
    uint64_t RequestTime;
    uint64_t Reserve0;
    uint64_t Reserve1;
    uint64_t Reserve2;
    uint64_t Reserve3;
    uint64_t SendingTime;
    uint32_t MsgSeqNum;
    uint8_t LastFragment;
    char Pad3[LEN_PAD3];
} NRResponseHeaderMECompT;

// Structure: NotAffectedOrdersGrp
typedef struct
{
    uint64_t NotAffectedOrderID;
    uint64_t NotAffOrigClOrdID;
} NotAffectedOrdersGrpSeqT;

// Structure: NotifHeader
typedef struct
{
    uint64_t SendingTime;
} NotifHeaderCompT;

// Structure: RBCHeader
typedef struct
{
    uint64_t SendingTime;
    uint64_t ApplSeqNum;
    uint32_t ApplSubID;
    uint16_t PartitionID;
    uint8_t ApplResendFlag;
    uint8_t ApplID;
    uint8_t LastFragment;
    char Pad7[LEN_PAD7];
} RBCHeaderCompT;

// Structure: RBCHeaderME
typedef struct
{
    uint64_t Reserve2;
    uint64_t SendingTime;
    uint32_t ApplSubID;
    uint16_t PartitionID;
    char ApplMsgID[LEN_APPL_MSGID];
    uint8_t ApplID;
    uint8_t ApplResendFlag;
    uint8_t LastFragment;
    char Pad7[LEN_PAD7];
} RBCHeaderMECompT;

// Structure: RequestHeader
typedef struct
{
    uint32_t MsgSeqNum;
    uint32_t SenderSubID;
} RequestHeaderCompT;

// Structure: ResponseHeader
typedef struct
{
    uint64_t RequestTime;
    uint64_t SendingTime;
    uint32_t MsgSeqNum;
    char Pad4[LEN_PAD4];
} ResponseHeaderCompT;

// Structure: ResponseHeaderME
typedef struct
{
    uint64_t RequestTime;
    uint64_t Reserve0;
    uint64_t Reserve1;
    uint64_t Reserve2;
    uint64_t Reserve3;
    uint64_t SendingTime;
    uint32_t MsgSeqNum;
    uint16_t PartitionID;
    uint8_t ApplID;
    char ApplMsgID[LEN_APPL_MSGID];
    uint8_t LastFragment;
} ResponseHeaderMECompT;

// Structure: RiskLimitQtyGrp
typedef struct
{
    int64_t RiskLimitQty;
    uint8_t RiskLimitType;
    char ProductLine[LEN_PRODUCT_LINE];
    char Pad6[LEN_PAD6];
} RiskLimitQtyGrpDerivativesSeqT;

// Structure: RiskLimitsRptGrp
typedef struct
{
    int64_t RiskLimitQty;
    int32_t MarketSegmentID;
    uint8_t RiskLimitType;
    char ProductLine[LEN_PRODUCT_LINE];
    char Pad2[LEN_PAD2];
} RiskLimitsRptGrpSeqT;

/*
 * Structure defines for messages
 */

// Message:	    AuctionNotification
// TemplateID:  11028
// Alias:       Auction Notification
// FIX MsgType: NotificationAuctionRequest = "U44"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint64_t LstUpdtTime;
    int64_t OrderQty;
    int64_t Price;
    int64_t CumQty;
    int32_t MarketSegmentID;
    uint32_t SimpleSecurityID;
    uint16_t TradingSessionID;
    uint8_t AuctionEnquiryTradingStatus;
    char Pad5[LEN_PAD5];
} AuctionNotificationT;

// Message:	    BroadcastErrorNotification
// TemplateID:  10032
// Alias:       Gap Fill
// FIX MsgType: ApplicationMessageReport = "BY"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NotifHeaderCompT NotifHeader;
    uint32_t ApplIDStatus;
    uint32_t RefApplSubID;
    uint16_t VarTextLen;
    uint8_t RefApplID;
    uint8_t SessionStatus;
    char Pad4[LEN_PAD4];
    char VarText[LEN_VAR_TEXT];
} BroadcastErrorNotificationT;

// Message:	    ConnectionGatewayRequest
// TemplateID:  10020
// Alias:       Connection Gateway Request
// FIX MsgType: Logon = "A"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t PartyIDSessionID;
    char DefaultCstmApplVerID[LEN_DEFAULT_CSTM_APPL_VERID];
    char Pad2[LEN_PAD2];
    uint32_t EncryptedDataMessageSize;
	char Filler7[LEN_FILLER7];
    char Password[LEN_PASSWORD];
} ConnectionGatewayRequestT;

// Message:	    ConnectionGatewayResponse
// TemplateID:  10021
// Alias:       Connection Gateway Response
// FIX MsgType: LogonR = "B1"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
    uint32_t GatewayID;
    uint32_t GatewaySubID;
    uint32_t SecondaryGatewayID;
    uint32_t SecondaryGatewaySubID;
    uint8_t SessionMode;
    uint8_t TradSesMode;
    char Pad6[LEN_PAD6];
	char Key[LEN_KEY];
    char IVVector[LEN_IV_VECTOR];
    char Pad2[LEN_PAD2];
} ConnectionGatewayResponseT;

// Message:	    DeleteAllOrderBroadcast
// TemplateID:  10122
// Alias:       Order Mass Cancellation Notification
// FIX MsgType: OrderMassActionReport = "BZ"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    uint64_t MassActionReportID;
    int64_t SecurityID;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint64_t OrderID;
    int32_t MarketSegmentID;
    uint32_t TargetPartyIDSessionID;
    uint32_t TargetPartyIDExecutingTrader;
    uint32_t PartyIDEnteringTrader;
    uint32_t RegulatoryID;
    int32_t Echo;
    uint16_t NoNotAffectedOrders;
    uint8_t PartyIDEnteringFirm;
    uint8_t MassActionReason;
    uint8_t ExecInst;
    uint8_t Side;
    char Account[LEN_ACCOUNT];
    uint8_t TimeInForce;
    uint8_t OrdType;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char Pad3[LEN_PAD3];
    NotAffectedOrdersGrpSeqT NotAffectedOrdersGrp[MAX_DELETE_ALL_ORDER_BROADCAST_NOT_AFFECTED_ORDERS_GRP];
} DeleteAllOrderBroadcastT;

// Message:	    DeleteAllOrderNRResponse
// TemplateID:  10124
// Alias:       Order Mass Cancellation Response No Hits
// FIX MsgType: OrderMassActionReport = "BZ"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t MassActionReportID;
} DeleteAllOrderNRResponseT;

// Message:	    DeleteAllOrderQuoteEventBroadcast
// TemplateID:  10308
// Alias:       Mass Cancellation Event
// FIX MsgType: OrderMassActionReport = "BZ"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    uint64_t MassActionReportID;
    int64_t SecurityID;
    int32_t MarketSegmentID;
    uint8_t MassActionReason;
    uint8_t ExecInst;
    char Pad2[LEN_PAD2];
} DeleteAllOrderQuoteEventBroadcastT;

// Message:	    DeleteAllOrderRequest
// TemplateID:  10120
// Alias:       Order Mass Cancellation Request
// FIX MsgType: OrderMassActionRequest = "CA"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    int64_t SecurityID;
    int32_t MarketSegmentID;
    uint32_t RegulatoryID;
    uint32_t TargetPartyIDSessionID;
    uint32_t TargetPartyIDExecutingTrader;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint64_t OrderID;
    int32_t Echo;
    uint8_t Side;
    char Account[LEN_ACCOUNT];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    uint8_t TimeInForce;
    uint8_t OrdType;
    char Pad7[LEN_PAD7];
} DeleteAllOrderRequestT;

// Message:	    DeleteAllOrderResponse
// TemplateID:  10121
// Alias:       Order Mass Cancellation Response
// FIX MsgType: OrderMassActionReport = "BZ"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderMECompT ResponseHeaderME;
    uint64_t MassActionReportID;
    uint16_t NoNotAffectedOrders;
    uint16_t NoAffectedOrderRequests;
    char Pad4[LEN_PAD4];
    NotAffectedOrdersGrpSeqT NotAffectedOrdersGrp[MAX_DELETE_ALL_ORDER_RESPONSE_NOT_AFFECTED_ORDERS_GRP];
    AffectedOrdersGrpSeqT AffectedOrdersGrp[MAX_DELETE_ALL_ORDER_RESPONSE_AFFECTED_ORDERS_GRP];
} DeleteAllOrderResponseT;

// Message:	    DeleteOrderBroadcast
// TemplateID:  10112
// Alias:       Cancel Order Notification
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    uint64_t ExecID;
    int64_t CumQty;
    int64_t CxlQty;
    int32_t Echo;
    int32_t MarketSegmentID;
    uint32_t PartyIDEnteringTrader;
    uint16_t ExecRestatementReason;
    uint8_t PartyIDEnteringFirm;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint8_t ProductComplex;
    uint8_t Side;
    char Pad5[LEN_PAD5];
} DeleteOrderBroadcastT;

// Message:	    DeleteOrderComplexRequest
// TemplateID:  10123
// Alias:       Cancel Order complex
// FIX MsgType: OrderCancelRequest = "F"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    uint64_t LstUpdtTime;
    int32_t Echo;
    int32_t MarketSegmentID;
    uint32_t TargetPartyIDSessionID;
    uint32_t RegulatoryID;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
} DeleteOrderComplexRequestT;

// Message:	    DeleteOrderNRResponse
// TemplateID:  10111
// Alias:       Cancel Order Response (Lean Order)
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    uint64_t ExecID;
    int64_t CumQty;
    int64_t CxlQty;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint16_t ExecRestatementReason;
    uint8_t ProductComplex;
    uint8_t TransactionDelayIndicator;
	char Pad2[LEN_PAD2];
} DeleteOrderNRResponseT;

// Message:	    DeleteOrderResponse
// TemplateID:  10110
// Alias:       Cancel Order Response (Standard Order)
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderMECompT ResponseHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    uint64_t ExecID;
    int64_t CumQty;
    int64_t CxlQty;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint16_t ExecRestatementReason;
    uint8_t ProductComplex;
    char Pad3[LEN_PAD3];
} DeleteOrderResponseT;

// Message:	    DeleteOrderSingleRequest
// TemplateID:  10109
// Alias:       Cancel Order Single
// FIX MsgType: OrderCancelRequest = "F"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    uint64_t LstUpdtTime;
    uint64_t StrategyID;
    uint64_t StrategyTriggerSeqNo;
    int32_t Echo;
    int32_t MarketSegmentID;
    uint32_t SimpleSecurityID;
    uint32_t TargetPartyIDSessionID;
    uint32_t RegulatoryID;
    char Pad4[LEN_PAD4];
} DeleteOrderSingleRequestT;

// Message:	    ExDExCancellationConfirmation
// TemplateID:  8037
// Alias:        Ex/DEx Cancellation Confirmation
// FIX MsgType:  ExDExCancellationConfirmation = "U81"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t Quantity;
    uint64_t ExDExOrderNumber;
    uint64_t LastUpdatedTime;
    uint32_t SimpleSecurityID;
    int32_t Echo;
    uint32_t TargetPartyIDSessionID;
    uint16_t ExDExFlag;
    uint8_t AccountType;
    uint8_t Side;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad4[LEN_PAD4];
} ExDExCancellationConfirmationT;

// Message:	    ExDExCancellationRequest
// TemplateID:  8036
// Alias:        Ex/DEx Cancellation Request
// FIX MsgType:  ExDExCancellationRequest = "U76"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t ExDExOrderNumber;
    uint64_t LastUpdatedTime;
    uint64_t Quantity;
    uint32_t SimpleSecurityID;
    int32_t Echo;
    uint32_t TargetPartyIDSessionID;
    uint16_t ExDExFlag;
    uint8_t Side;
    uint8_t AccountType;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad4[LEN_PAD4];
} ExDExCancellationRequestT;

// Message:	    ExDExEntryConfirmation
// TemplateID:  8031
// Alias:        Ex/DEx Entry Confirmation
// FIX MsgType:  ExDExEntryConfirmation = "U75"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t ExDExOrderNumber;
    uint64_t TrdRegTSEntryTime;
    uint64_t LastUpdatedTime;
    uint64_t Quantity;
    uint32_t SimpleSecurityID;
    int32_t Echo;
    uint32_t TargetPartyIDSessionID;
    uint16_t ExDExFlag;
    uint8_t AccountType;
    uint8_t Side;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad4[LEN_PAD4];
} ExDExEntryConfirmationT;

// Message:	    ExDExEntryRequest
// TemplateID:  8030
// Alias:        Ex/DEx Entry Request
// FIX MsgType: ExDExEntryRequest = "U74"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t Quantity;
    uint32_t SimpleSecurityID;
    int32_t Echo;
    uint32_t TargetPartyIDSessionID;
    uint16_t ExDExFlag;
    uint8_t AccountType;
    uint8_t Side;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad4[LEN_PAD4];
} ExDExEntryRequestT;

// Message:	    ExDExModificationConfirmation
// TemplateID:  8034
// Alias:        Ex/DEx Modification Confirmation
// FIX MsgType: ExDExModificationConfirmation = "U80"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t Quantity;
    uint64_t ExDExOrderNumber;
    uint64_t LastUpdatedTime;
    int32_t Echo;
    uint32_t TargetPartyIDSessionID;
    uint32_t SimpleSecurityID;
    uint16_t ExDExFlag;
    uint8_t AccountType;
    uint8_t Side;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad4[LEN_PAD4];
} ExDExModificationConfirmationT;

// Message:	    ExDExModificationRequest
// TemplateID:  8033
// Alias:       Ex/DEx Modification Request
// FIX MsgType: ExDExModificationRequest = "U77"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t Quantity;
    uint64_t ExDExOrderNumber;
    uint64_t LastUpdatedTime;
    uint32_t SimpleSecurityID;
    int32_t Echo;
    uint32_t TargetPartyIDSessionID;
    uint16_t ExDExFlag;
    uint8_t AccountType;
    uint8_t Side;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad4[LEN_PAD4];
} ExDExModificationRequestT;

// Message:	    ExDExNotification
// TemplateID:  8040
// Alias:        Ex/DEx Notification
// FIX MsgType: ExDExNotification = "U78"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint64_t Quantity;
    uint64_t ExDExOrderNumber;
    uint64_t LastUpdatedTime;
    uint32_t SimpleSecurityID;
    int32_t Echo;
    uint32_t TargetPartyIDSessionID;
    uint16_t ExecRestatementReason;
    uint8_t AccountType;
    uint8_t Side;
    uint16_t ExDExstatus;
    uint16_t ExDExFlag;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
} ExDExNotificationT;

// Message:	    ExtendedOrderInformation
// TemplateID:  10994
// Alias:       Extended Order Information
// FIX MsgType: MultiLegExecutionReport = "U30"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    uint64_t TerminalInfo;
    uint64_t ClOrdID;
    uint64_t ExecID;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint16_t NoLegExecs;
    uint8_t AccountType;
    uint8_t NoOfMultiLeg;
    uint8_t NoOfMultiLegExecs;
    char FreeText1[LEN_FREE_TEXT1];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char CPCode[LEN_CP_CODE];
    uint8_t MultiLegType;
    char FreeText3[LEN_FREE_TEXT3];
    char Pad2[LEN_PAD2];
	MultiLegGrpSeqT MultiLegGrp[MAX_EXTENDED_ORDER_INFORMATION_MULTI_LEG_GRP];
    MultiLegFillGrpSeqT MultiLegFillGrp[MAX_EXTENDED_ORDER_INFORMATION_MULTI_LEG_FILL_GRP];
    InstrmntLegExecGrpSeqT InstrmntLegExecGrp[MAX_EXTENDED_ORDER_INFORMATION_INSTRMNT_LEG_EXEC_GRP];
} ExtendedOrderInformationT;

// Message:	    ForcedLogoutNotification
// TemplateID:  10012
// Alias:       Session Logout Notification
// FIX MsgType: Logout = "5"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NotifHeaderCompT NotifHeader;
    uint16_t VarTextLen;
    char Pad6[LEN_PAD6];
    char VarText[LEN_VAR_TEXT];
} ForcedLogoutNotificationT;

// Message:	    ForcedUserLogoutNotification
// TemplateID:  10043
// Alias:       User Logout Notification
// FIX MsgType: Logout = "5"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
} ForcedUserLogoutNotificationT;

// Message:	    Heartbeat
// TemplateID:  10011
// Alias:       Heartbeat
// FIX MsgType: Heartbeat = "0"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
} HeartbeatT;

// Message:	    HeartbeatNotification
// TemplateID:  10023
// Alias:       Heartbeat Notification
// FIX MsgType: Heartbeat = "0"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NotifHeaderCompT NotifHeader;
} HeartbeatNotificationT;

// Message:	    ImmediateExecutionResponse
// TemplateID:  10993
// Alias:       Immediate Execution Response
// FIX MsgType: MultiLegExecutionReport = "U30"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderMECompT ResponseHeaderME;
    uint64_t ClOrdID;
    uint64_t ExecID;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint16_t NoLegExecs;
    uint8_t NoOfMultiLeg;
    uint8_t NoOfMultiLegExecs;
    uint8_t MultiLegType;
    char Pad3[LEN_PAD3];
    MultiLegExecGrpSeqT MultiLegExecGrp[MAX_IMMEDIATE_EXECUTION_RESPONSE_MULTI_LEG_EXEC_GRP];
    InstrmntLegExecGrpSeqT InstrmntLegExecGrp[MAX_IMMEDIATE_EXECUTION_RESPONSE_INSTRMNT_LEG_EXEC_GRP];
} ImmediateExecutionResponseT;

// Message:	    InquirePreTradeRiskLimitsRequest
// TemplateID:  10311
// Alias:       Inquire Pre-Trade Risk Limits Request
// FIX MsgType: PartyRiskLimitsRequest = "CL"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    int32_t MarketSegmentID;
    uint32_t RootPartyIDExecutingTrader;
    uint8_t RiskLimitPlatform;
    char PartyExecutingUnit[LEN_PARTY_EXECUTING_UNIT];
    char RiskLimitGroup[LEN_RISK_LIMIT_GROUP];
    char Pad7[LEN_PAD7];
} InquirePreTradeRiskLimitsRequestT;

// Message:	    LogonRequest
// TemplateID:  10000
// Alias:       Session Logon
// FIX MsgType: Logon = "A"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t HeartBtInt;
    uint32_t PartyIDSessionID;
    uint32_t EncryptedDataMessageSize;
    char DefaultCstmApplVerID[LEN_DEFAULT_CSTM_APPL_VERID];
	char Filler7[LEN_FILLER7];
    char ApplUsageOrders[LEN_APPL_USAGE_ORDERS];
    char ApplUsageQuotes[LEN_APPL_USAGE_QUOTES];
    char OrderRoutingIndicator[LEN_ORDER_ROUTING_INDICATOR];
    char FIXEngineName[LEN_FIX_ENGINE_NAME];
    char FIXEngineVersion[LEN_FIX_ENGINE_VERSION];
    char FIXEngineVendor[LEN_FIX_ENGINE_VENDOR];
    char ApplicationSystemName[LEN_APPLICATION_SYSTEM_NAME];
    char ApplicationSystemVersion[LEN_APPLICATION_SYSTEM_VERSION];
    char ApplicationSystemVendor[LEN_APPLICATION_SYSTEM_VENDOR];
    char Password[LEN_PASSWORD];
    char Pad7[LEN_PAD7];
} LogonRequestT;

// Message:	    LogonResponse
// TemplateID:  10001
// Alias:       Session Logon Response
// FIX MsgType: Logon = "A"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
    int64_t ThrottleTimeInterval;
    uint64_t LastLoginTime;
    uint32_t LastLoginIP;
    uint32_t ThrottleNoMsgs;
    uint32_t ThrottleDisconnectLimit;
    uint32_t HeartBtInt;
    uint32_t SessionInstanceID;
    uint8_t TradSesMode;
    uint8_t NoOfPartition;
    uint8_t DaysLeftForPasswordExpiry;
    uint8_t GraceLoginsLeft;
    char DefaultCstmApplVerID[LEN_DEFAULT_CSTM_APPL_VERID];
    char Pad2[LEN_PAD2];
} LogonResponseT;

// Message:	    LogoutRequest
// TemplateID:  10002
// Alias:       Session Logout
// FIX MsgType: Logout = "5"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
} LogoutRequestT;

// Message:	    LogoutResponse
// TemplateID:  10003
// Alias:       Session Logout Response
// FIX MsgType: Logout = "5"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
} LogoutResponseT;

// Message:	    MarketStatusNotification
// TemplateID:  4125
// Alias:        Market Status Notification
// FIX MsgType:  MarketStatusNotification = "U90"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    int32_t MarketSegmentID;
    uint16_t GroupID;
    uint16_t Status;
    uint16_t TradingSessionID;
    uint8_t TradingSessionSubID;
    char Pad5[LEN_PAD5];
} MarketStatusNotificationT;

// Message:	    MarketWideOIAlert
// TemplateID:  8101
// Alias:       Market Wide OI Alert
// FIX MsgType: MarketWideOIAlert = "U89"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint64_t OrigTime;
    char Headline[LEN_HEADLINE];
    uint16_t VarTextLen;
    uint8_t AssetType;
    uint8_t AlertLevel;
    char Pad4[LEN_PAD4];
    int64_t SecurityID;
    int64_t AlertPercentage;
    int64_t UtilizedAlertPercentage;
    uint64_t LastModifyDate;
    uint32_t MaturityDate;
    uint16_t SecurityType;
    char VarText[LEN_VAR_TEXT];
    char Pad2[LEN_PAD2];
} MarketWideOIAlertT;

// Message:	    MarketWideOIViolation
// TemplateID:  8100
// Alias:       Market Wide OI Violation
// FIX MsgType: MarketWideOIViolation = "U88"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint64_t OrigTime;
    uint64_t LastUpdateDate;
    int64_t OIPercentageLimit;
    uint32_t MaturityDate;
    uint32_t SimpleSecurityID;
    uint32_t OIQuantityLimit;
    uint32_t EffectiveOILimit;
    uint32_t ActualOIQuantity;
    uint16_t VarTextLen;
    uint16_t SecurityType;
    char Headline[LEN_HEADLINE];
    uint8_t AssetType;
    uint8_t ViolationStatus;
    char VarText[LEN_VAR_TEXT];
    char Pad6[LEN_PAD6];
} MarketWideOIViolationT;

// Message:	    MarginChangeNotification
// TemplateID:  4025
// Alias:       Margin Change Notification
// FIX MsgType: MarginChangeNotification = "U91"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint8_t NoMarginChange;
    char Pad7[LEN_PAD7];
    MarginChangeRptGrpSeqT MarginChangeRptGrp[MAX_MARGIN_CHANGE_NOTIFICATION_MARGIN_CHANGE_RPT_GRP];
} MarginChangeNotificationT;

// Message:	    ModifyOrderComplexRequest
// TemplateID:  10114
// Alias:       Replace Order complex
// FIX MsgType: MultilegOrderCancelReplace = "AC"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    int64_t Price;
    int64_t MaxPricePercentage;
    uint64_t TerminalInfo;
    uint64_t LstUpdtTime;
    uint64_t Filler1;
    uint32_t Filler2;
    int32_t MarketSegmentID;
    int64_t OrderQty;
    int64_t DisclosedQty;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    int32_t Echo;
    uint32_t ExpireDate;
    uint32_t TargetPartyIDSessionID;
    uint32_t RegulatoryID;
    uint16_t Filler4;
    char PartyIDTakeUpTradingFirm[LEN_PARTY_ID_TAKE_UP_TRADING_FIRM];
    char PartyIDOrderOriginationFirm[LEN_PARTY_ID_ORDER_ORIGINATION_FIRM];
    char PartyIDBeneficiary[LEN_PARTY_ID_BENEFICIARY];
    uint8_t AccountType;
    uint8_t ApplSeqIndicator;
    uint8_t ProductComplex;
    uint8_t Side;
    uint8_t OrdType;
    uint8_t PriceValidityCheckType;
    uint8_t ExecInst;
    uint8_t TimeInForce;
    uint8_t Filler5;
    uint8_t TradingCapacity;
    char Filler6[LEN_FILLER6];
    char PartyIDLocationID[LEN_PARTY_ID_LOCATIONID];
    char CustOrderHandlingInst[LEN_CUST_ORDER_HANDLING_INST];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char FreeText3[LEN_FREE_TEXT3];
    uint8_t NoLegs;
    char Pad2[LEN_PAD2];
    LegOrdGrpSeqT LegOrdGrp[MAX_MODIFY_ORDER_COMPLEX_REQUEST_LEG_ORD_GRP];
} ModifyOrderComplexRequestT;

// Message:	    ModifyOrderNRResponse
// TemplateID:  10108
// Alias:       Replace Order Response (Lean Order)
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t PriceMkToLimitPx;
    int64_t SecurityID;
    uint64_t ExecID;
    int64_t LeavesQty;
    int64_t CumQty;
    int64_t CxlQty;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint16_t ExecRestatementReason;
    uint8_t CrossedIndicator;
    uint8_t ProductComplex;
    uint8_t Triggered;
    uint8_t TransactionDelayIndicator;
} ModifyOrderNRResponseT;

// Message:	    ModifyOrderResponse
// TemplateID:  10107
// Alias:       Replace Order Response (Standard Order)
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderMECompT ResponseHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    uint64_t ExecID;
    int64_t PriceMkToLimitPx;
    int64_t Filler10;
    int64_t Filler11;
    uint64_t Reserve14;
    uint64_t LstUpdtTime;
    uint64_t Filler1;
    int64_t LeavesQty;
    int64_t CumQty;
    int64_t CxlQty;
    uint32_t Filler2;
    uint16_t Filler4;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint16_t ExecRestatementReason;
    uint8_t ProductComplex;
    uint8_t Filler5;
    char Pad4[LEN_PAD4];
} ModifyOrderResponseT;

// Message:	    ModifyOrderSingleRequest
// TemplateID:  10106
// Alias:       Replace Order Single
// FIX MsgType: OrderCancelReplaceRequest = "G"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t Price;
    int64_t StopPx;
    int64_t MaxPricePercentage;
    uint64_t TerminalInfo;
    uint64_t LstUpdtTime;
    uint64_t Filler1;
    uint32_t Filler2;
    int32_t Echo;
    int64_t OrderQty;
    int64_t DisclosedQty;
    uint64_t StrategyID;
    uint64_t StrategyTriggerSeqNo;
    uint32_t ExpireDate;
    int32_t MarketSegmentID;
    uint32_t SimpleSecurityID;
    uint32_t TargetPartyIDSessionID;
    uint32_t RegulatoryID;
    uint16_t Filler4;
    char PartyIDTakeUpTradingFirm[LEN_PARTY_ID_TAKE_UP_TRADING_FIRM];
    char PartyIDOrderOriginationFirm[LEN_PARTY_ID_ORDER_ORIGINATION_FIRM];
    char PartyIDBeneficiary[LEN_PARTY_ID_BENEFICIARY];
    uint8_t AccountType;
    uint8_t ApplSeqIndicator;
    uint8_t Side;
    uint8_t OrdType;
    uint8_t PriceValidityCheckType;
    uint8_t TimeInForce;
    uint8_t ExecInst;
    uint8_t Filler5;
    uint8_t TradingSessionSubID;
    uint8_t TradingCapacity;
    char Filler6[LEN_FILLER6];
    char Account[LEN_ACCOUNT];
    char PositionEffect[LEN_POSITION_EFFECT];
    char PartyIDLocationID[LEN_PARTY_ID_LOCATIONID];
    char CustOrderHandlingInst[LEN_CUST_ORDER_HANDLING_INST];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char FreeText3[LEN_FREE_TEXT3];
    char Pad4[LEN_PAD4];
} ModifyOrderSingleRequestT;

// Message:	    ModifyOrderSingleShortRequest
// TemplateID:  10126
// Alias:       Replace Order Single (short layout)
// FIX MsgType: OrderCancelReplaceRequest = "G"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t Price;
    uint64_t TerminalInfo;
    uint64_t LstUpdtTime;
    int64_t OrderQty;
    int64_t DisclosedQty;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint32_t SimpleSecurityID;
    uint32_t Filler2;
    uint16_t Filler4;
    uint8_t AccountType;
    uint8_t Side;
    uint8_t PriceValidityCheckType;
    uint8_t TimeInForce;
    uint8_t ExecInst;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char Pad1[LEN_PAD1];
} ModifyOrderSingleShortRequestT;

// Message:	    NewOrderComplexRequest
// TemplateID:  10113
// Alias:       New Order Multi Leg
// FIX MsgType: NewOrderMultileg = "AB"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t ClOrdID;
    int64_t SecurityID;
    int64_t MaxPricePercentage;
    uint64_t TerminalInfo;
    int64_t Price;
    uint64_t Filler1;
    uint32_t Filler2;
    int32_t Echo;
    int64_t OrderQty;
    int64_t DisclosedQty;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    int32_t MarketSegmentID;
    uint32_t ExpireDate;
    uint32_t RegulatoryID;
    uint16_t Filler4;
    char PartyIDTakeUpTradingFirm[LEN_PARTY_ID_TAKE_UP_TRADING_FIRM];
    char PartyIDOrderOriginationFirm[LEN_PARTY_ID_ORDER_ORIGINATION_FIRM];
    char PartyIDBeneficiary[LEN_PARTY_ID_BENEFICIARY];
    uint8_t AccountType;
    uint8_t ApplSeqIndicator;
    uint8_t ProductComplex;
    uint8_t Side;
    uint8_t OrdType;
    uint8_t PriceValidityCheckType;
    uint8_t ExecInst;
    uint8_t TimeInForce;
    uint8_t SMPFOrderIdentifier;
    uint8_t Filler5;
    uint8_t TradingCapacity;
    char PartyIDLocationID[LEN_PARTY_ID_LOCATIONID];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char CustOrderHandlingInst[LEN_CUST_ORDER_HANDLING_INST];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char FreeText3[LEN_FREE_TEXT3];
    uint8_t NoLegs;
    char Pad6[LEN_PAD6];
    LegOrdGrpSeqT LegOrdGrp[MAX_NEW_ORDER_COMPLEX_REQUEST_LEG_ORD_GRP];
} NewOrderComplexRequestT;

// Message:	    NewOrderMultiLeg
// TemplateID:  10991
// Alias:       New Order MultiLeg
// FIX MsgType: MultiLegOrder = "U28"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t TerminalInfo;
    uint64_t ClOrdID;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint8_t AccountType;
    uint8_t NoOfMultiLeg;
    uint8_t SMPFOrderIdentifier;
    uint8_t MultiLegType;
    char AllOrNoneFlag[LEN_ALL_OR_NONE_FLAG];
    uint8_t ApplSeqIndicator;
    uint8_t ExecInst;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char FreeText3[LEN_FREE_TEXT3];
    char Pad1[LEN_PAD1];
    MultiLegOrdGrpSeqT MultiLegOrdGrp[MAX_NEW_ORDER_MULTI_LEG_MULTI_LEG_ORD_GRP];
} NewOrderMultiLegT;

// Message:	    NewOrderNRResponse
// TemplateID:  10102
// Alias:       New Order Response (Lean Order)
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    int64_t PriceMkToLimitPx;
    int64_t SecurityID;
    uint64_t ExecID;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint16_t ExecRestatementReason;
    uint8_t CrossedIndicator;
    uint8_t ProductComplex;
    uint8_t Triggered;
    uint8_t TransactionDelayIndicator;
} NewOrderNRResponseT;

// Message:	    NewOrderResponse
// TemplateID:  10101
// Alias:       New Order Response (Standard Order)
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderMECompT ResponseHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    int64_t PriceMkToLimitPx;
    uint64_t Reserved1;
    uint64_t Reserved2;
    uint64_t ExecID;
    uint64_t TrdRegTSEntryTime;
    uint64_t Reserve14;
    uint64_t LstUpdtTime;
    uint64_t Filler1;
    uint32_t SimpleSecurityID;
    uint32_t Filler2;
    uint16_t Filler4;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint16_t ExecRestatementReason;
    uint8_t ProductComplex;
    uint8_t Filler5;
} NewOrderResponseT;

// Message:	    NewOrderSingleRequest
// TemplateID:  10100
// Alias:       New Order Single
// FIX MsgType: NewOrderSingle = "D"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    int64_t Price;
    int64_t StopPx;
    int64_t MarketProtectionPercentage;
    uint64_t TerminalInfo;
    uint64_t ClOrdID;
    uint64_t StrategyID;
    uint64_t StrategyTriggerSeqNo;
    uint64_t Filler1;
    uint32_t TargetPartyIDSessionID;
    int32_t Echo;
    int64_t OrderQty;
    int64_t DisclosedQty;
    uint32_t ExpireDate;
    int32_t MarketSegmentID;
    uint32_t SimpleSecurityID;
    uint32_t RegulatoryID;
    uint16_t Filler4;
    char PartyIDTakeUpTradingFirm[LEN_PARTY_ID_TAKE_UP_TRADING_FIRM];
    char PartyIDOrderOriginationFirm[LEN_PARTY_ID_ORDER_ORIGINATION_FIRM];
    char PartyIDBeneficiary[LEN_PARTY_ID_BENEFICIARY];
    uint8_t AccountType;
    uint8_t ApplSeqIndicator;
    uint8_t Side;
    uint8_t OrdType;
    uint8_t PriceValidityCheckType;
    uint8_t TimeInForce;
    uint8_t ExecInst;
    uint8_t SMPFOrderIdentifier;
    uint8_t Filler5;
    uint8_t TradingSessionSubID;
    uint8_t TradingCapacity;
    char Account[LEN_ACCOUNT];
    char PositionEffect[LEN_POSITION_EFFECT];
    char PartyIDLocationID[LEN_PARTY_ID_LOCATIONID];
    char CustOrderHandlingInst[LEN_CUST_ORDER_HANDLING_INST];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char FreeText3[LEN_FREE_TEXT3];
} NewOrderSingleRequestT;

// Message:	    NewOrderSingleShortRequest
// TemplateID:  10125
// Alias:       New Order Single (short layout)
// FIX MsgType: NewOrderSingle = "D"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    int64_t Price;
    uint64_t TerminalInfo;
    uint64_t ClOrdID;
    int64_t OrderQty;
    int64_t DisclosedQty;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint32_t SimpleSecurityID;
    uint32_t Filler2;
    uint16_t Filler4;
    uint8_t AccountType;
    uint8_t Side;
    uint8_t PriceValidityCheckType;
    uint8_t TimeInForce;
    uint8_t SMPFOrderIdentifier;
    uint8_t ExecInst;
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
} NewOrderSingleShortRequestT;

// Message:	    NewsBroadcast
// TemplateID:  10031
// Alias:       News
// FIX MsgType: News = "B"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint64_t OrigTime;
    uint16_t VarTextLen;
    char Headline[LEN_HEADLINE];
    char Pad6[LEN_PAD6];
    char VarText[LEN_VAR_TEXT];
} NewsBroadcastT;

// Message:	    OrderExecNotification
// TemplateID:  10104
// Alias:       Book Order Execution
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    uint64_t OrderID;
    uint64_t TerminalInfo;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    uint64_t ExecID;
    uint64_t LstUpdtTime;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint64_t Reserve1;
    uint32_t Reserve2;
    int32_t Echo;
    int64_t LeavesQty;
    int64_t CumQty;
    int64_t CxlQty;
    int32_t MarketSegmentID;
    uint16_t NoLegExecs;
    uint16_t Filler4;
    uint16_t ExecRestatementReason;
    uint8_t AccountType;
    uint8_t ProductComplex;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint8_t Triggered;
    uint8_t NoFills;
    uint8_t Side;
    uint8_t Filler5;
    char Account[LEN_ACCOUNT];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char FreeText3[LEN_FREE_TEXT3];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad4[LEN_PAD4];
    FillsGrpSeqT FillsGrp[MAX_ORDER_EXEC_NOTIFICATION_FILLS_GRP];
    InstrmntLegExecGrpSeqT InstrmntLegExecGrp[MAX_ORDER_EXEC_NOTIFICATION_INSTRMNT_LEG_EXEC_GRP];
} OrderExecNotificationT;

// Message:	    OrderExecReportBroadcast
// TemplateID:  10117
// Alias:       Extended Order Information
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
	int64_t MaxPricePercentage;
    uint64_t TerminalInfo;
    uint64_t ExecID;
    uint64_t TrdRegTSEntryTime;
    uint64_t Reserve14;
    int64_t Price;
    int64_t StopPx;
    uint64_t Reserved2;
    uint64_t Reserved1;
    uint64_t LstUpdTime;
	uint64_t mLReferenceNumber;
    int64_t LeavesQty;
    int64_t DisclosedQty;
    int64_t CumQty;
    int64_t CxlQty;
    int64_t OrderQty;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
	int64_t SpreadPrice;
    int32_t MarketSegmentID;
    int32_t Echo;
    uint32_t ExpireDate;
    uint32_t PartyIDExecutingUnit;
    uint32_t PartyIDSessionID;
    uint32_t PartyIDExecutingTrader;
    uint32_t PartyIDEnteringTrader;
    uint16_t NoLegExecs;
    uint16_t ExecRestatementReason;
    uint8_t AccountType;
    uint8_t PartyIDEnteringFirm;
    uint8_t ProductComplex;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint8_t Side;
    uint8_t OrdType;
    uint8_t TradingCapacity;
    uint8_t TimeInForce;
    uint8_t ExecInst;
    uint8_t TradingSessionSubID;
    uint8_t ApplSeqIndicator;
    uint8_t STPCFlag;
    uint8_t MultiLegType;
    char Account[LEN_ACCOUNT];
    char PositionEffect[LEN_POSITION_EFFECT];
    char PartyIDTakeUpTradingFirm[LEN_PARTY_ID_TAKE_UP_TRADING_FIRM];
    char PartyIDOrderOriginationFirm[LEN_PARTY_ID_ORDER_ORIGINATION_FIRM];
    char PartyIDBeneficiary[LEN_PARTY_ID_BENEFICIARY];
    char PartyIDLocationID[LEN_PARTY_ID_LOCATIONID];
    char CustOrderHandlingInst[LEN_CUST_ORDER_HANDLING_INST];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char FreeText3[LEN_FREE_TEXT3];
    uint8_t NoFills;
    uint8_t NoLegs;
    uint8_t Triggered;
    char Pad2[LEN_PAD2];
    LegOrdGrpSeqT LegOrdGrp[MAX_ORDER_EXEC_REPORT_BROADCAST_LEG_ORD_GRP];
    FillsGrpSeqT FillsGrp[MAX_ORDER_EXEC_REPORT_BROADCAST_FILLS_GRP];
    InstrmntLegExecGrpSeqT InstrmntLegExecGrp[MAX_ORDER_EXEC_REPORT_BROADCAST_INSTRMNT_LEG_EXEC_GRP];
} OrderExecReportBroadcastT;

// Message:	    OrderExecResponse
// TemplateID:  10103
// Alias:       Immediate Execution Response
// FIX MsgType: ExecutionReport = "8"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderMECompT ResponseHeaderME;
    uint64_t OrderID;
    uint64_t ClOrdID;
    uint64_t OrigClOrdID;
    int64_t SecurityID;
    uint64_t ExecID;
    uint64_t TrdRegTSEntryTime;
    uint64_t Reserve4;
    uint64_t LstUpdtTime;
    int64_t DisclosedQty;
    uint64_t Reserve1;
    uint32_t Filler2;
    int32_t MarketSegmentID;
    int64_t LeavesQty;
    int64_t CumQty;
    int64_t CxlQty;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    uint16_t Filler4;
    uint16_t NoLegExecs;
    uint16_t ExecRestatementReason;
    uint8_t ProductComplex;
    uint8_t Side;
    char OrdStatus[LEN_ORD_STATUS];
    char ExecType[LEN_EXEC_TYPE];
    uint8_t Triggered;
    uint8_t Filler5;
    uint8_t NoFills;
    char Pad3[LEN_PAD3];
    FillsGrpSeqT FillsGrp[MAX_ORDER_EXEC_RESPONSE_FILLS_GRP];
    InstrmntLegExecGrpSeqT InstrmntLegExecGrp[MAX_ORDER_EXEC_RESPONSE_INSTRMNT_LEG_EXEC_GRP];
} OrderExecResponseT;

// Message:	    PreTradeRiskLimitResponse
// TemplateID:  10313
// Alias:       Pre-Trade Risk Limit Response
// FIX MsgType: PartyRiskLimitsReport = "CM"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
    uint64_t RiskLimitReportID;
    uint32_t RootPartyIDExecutingTrader;
    uint32_t Filler2;
    uint8_t NoRiskLimits;
    uint8_t PartyDetailStatus;
    uint8_t RiskLimitPlatform;
    char PartyDetailExecutingUnit[LEN_PARTY_DETAIL_EXECUTING_UNIT];
    RiskLimitsRptGrpSeqT RiskLimitsRptGrp[MAX_PRE_TRADE_RISK_LIMIT_RESPONSE_RISK_LIMITS_RPT_GRP];
} PreTradeRiskLimitResponseT;

// Message:	    Reject
// TemplateID:  10010
// Alias:       Reject
// FIX MsgType: Reject = "3"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint32_t SessionRejectReason;
    uint16_t VarTextLen;
    uint8_t SessionStatus;
    char Pad1[LEN_PAD1];
    char VarText[LEN_VAR_TEXT];
} RejectT;

// Message:	    RejectMultiLeg
// TemplateID:  10992
// Alias:       Reject MultiLeg
// FIX MsgType: MultiLegOrderAcknowledgement = "U29"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    int64_t SecurityID;
    uint32_t SessionRejectReason;
    uint16_t VarTextLen;
    uint8_t SessionStatus;
    char Pad1[LEN_PAD1];
    char VarText[LEN_VAR_TEXT];
} RejectMultiLegT;

// Message:	    ResubmitApprovalConfirmation
// TemplateID:  8510
// Alias:       ResubmitForApproval
// FIX MsgType: ResubmitApprovalConfirmation = "U87"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRResponseHeaderMECompT NRResponseHeaderME;
    uint64_t LastUpdateDate;
    uint64_t OrderID;
    uint32_t SideTradeID;
    uint32_t SimpleSecurityID;
    uint8_t Side;
    uint8_t GiveUpStatus;
    char Pad6[LEN_PAD6];
} ResubmitApprovalConfirmationT;

// Message:	    ResubmitApprovalRequest
// TemplateID:  8500
// Alias:       ApprovalRequest
// FIX MsgType: ResubmitApprovalRequest = "U86"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t OrderID;
    uint32_t SideTradeID;
    uint32_t SimpleSecurityID;
    uint8_t Side;
    char Pad7[LEN_PAD7];
} ResubmitApprovalRequestT;

// Message:	    RetransmitMEMessageRequest
// TemplateID:  10026
// Alias:       Retransmit (Order/Quote Event)
// FIX MsgType: ApplicationMessageRequest = "BW"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t SubscriptionScope;
    uint16_t PartitionID;
    uint8_t RefApplID;
    char ApplBegMsgID[LEN_APPL_BEG_MSGID];
    char ApplEndMsgID[LEN_APPL_END_MSGID];
    char Pad1[LEN_PAD1];
} RetransmitMEMessageRequestT;

// Message:	    RetransmitMEMessageResponse
// TemplateID:  10027
// Alias:       Retransmit Response (Order/Quote Event)
// FIX MsgType: ApplicationMessageRequestAck = "BX"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
    uint16_t ApplTotalMessageCount;
    char ApplEndMsgID[LEN_APPL_END_MSGID];
    char RefApplLastMsgID[LEN_REF_APPL_LAST_MSGID];
    char Pad6[LEN_PAD6];
} RetransmitMEMessageResponseT;

// Message:	    RetransmitRequest
// TemplateID:  10008
// Alias:       Retransmit
// FIX MsgType: ApplicationMessageRequest = "BW"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t ApplBegSeqNum;
    uint64_t ApplEndSeqNum;
    uint32_t SubscriptionScope;
    uint16_t PartitionID;
    uint8_t RefApplID;
    char Pad1[LEN_PAD1];
} RetransmitRequestT;

// Message:	    RetransmitResponse
// TemplateID:  10009
// Alias:       Retransmit Response
// FIX MsgType: ApplicationMessageRequestAck = "BX"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
    uint64_t ApplEndSeqNum;
    uint64_t RefApplLastSeqNum;
    uint16_t ApplTotalMessageCount;
    char Pad6[LEN_PAD6];
} RetransmitResponseT;

// Message:	    ServiceAvailabilityBroadcast
// TemplateID:  10030
// Alias:       Service Availability
// FIX MsgType: UserNotification = "CB"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NRBCHeaderCompT NRBCHeader;
    uint32_t MatchingEngineTradeDate;
    uint32_t TradeManagerTradeDate;
    uint32_t ApplSeqTradeDate;
    uint16_t PartitionID;
    uint8_t MatchingEngineStatus;
    uint8_t TradeManagerStatus;
    uint8_t ApplSeqStatus;
    char Pad7[LEN_PAD7];
} ServiceAvailabilityBroadcastT;

// Message:	    SessionPasswordChange
// TemplateID:  10997
// Alias:       user password change
// FIX MsgType: SessionPasswordChange = "U84"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t PartyIDSessionID;
    uint32_t Filler2;
    char Password[LEN_PASSWORD];
    char NewPassword[LEN_NEW_PASSWORD];
} SessionPasswordChangeT;

// Message:	    SessionPasswordChangeResponse
// TemplateID:  10995
// Alias:       user password change
// FIX MsgType: SessionPasswordChangeResponse = "U82"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
} SessionPasswordChangeResponseT;

// Message:	    SubscribeRequest
// TemplateID:  10025
// Alias:       Subscribe
// FIX MsgType: ApplicationMessageRequest = "BW"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t SubscriptionScope;
    uint8_t RefApplID;
    char Pad3[LEN_PAD3];
} SubscribeRequestT;

// Message:	    SubscribeResponse
// TemplateID:  10005
// Alias:       Subscribe Response
// FIX MsgType: ApplicationMessageRequestAck = "BX"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
    uint32_t ApplSubID;
    char Pad4[LEN_PAD4];
} SubscribeResponseT;

// Message:	    SystemInformationDownload
// TemplateID:  4100
// Alias:       System Information Download
// FIX MsgType: SystemInformation = "U79"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    int64_t DisclosedQuantityPercentAllowed;
    uint16_t TradeModificationTimeIndicator;
    uint16_t DeliveryInstructionModificationTimeIndicator;
    uint16_t Status;
    char Pad2[LEN_PAD2];
} SystemInformationDownloadT;

// Message:	    TMTradingSessionStatusBroadcast
// TemplateID:  10501
// Alias:       Trade Notification Status
// FIX MsgType: TradingSessionStatus = "h"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint8_t TradSesEvent;
    char Pad7[LEN_PAD7];
} TMTradingSessionStatusBroadcastT;

// Message:	    ThrottleUpdateNotification
// TemplateID:  10028
// Alias:       Throttle Update Notification
// FIX MsgType: UserNotification = "CB"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    NotifHeaderCompT NotifHeader;
    int64_t ThrottleTimeInterval;
    uint32_t ThrottleNoMsgs;
    uint32_t ThrottleDisconnectLimit;
} ThrottleUpdateNotificationT;

// Message:	    TradeBroadcast
// TemplateID:  10500
// Alias:       Trade Notification
// FIX MsgType: TradeCaptureReport = "AE"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    int64_t RelatedSecurityID;
    int64_t Price;
    int64_t LastPx;
    int64_t SideLastPx;
    int64_t ClearingTradePrice;
    int64_t Filler10;
    int64_t Filler11;
    uint64_t TransactTime;
    uint64_t OrderID;
    uint64_t TerminalInfo;
    uint64_t ClOrdID;
    uint64_t LstUpdtTime;
    uint64_t StrategyID;
    uint64_t StrategySequenceNumber;
    int64_t LastQty;
    int64_t SideLastQty;
    uint64_t Filler1;
    int64_t CumQty;
    int64_t LeavesQty;
    uint32_t SimpleSecurityID;
    uint32_t Filler2;
    int32_t Echo;
    uint32_t TradeID;
    uint32_t OrigTradeID;
    uint32_t RootPartyIDExecutingUnit;
    uint32_t RootPartyIDSessionID;
    uint32_t RootPartyIDExecutingTrader;
    uint32_t RootPartyIDClearingUnit;
    int32_t MarketSegmentID;
    uint32_t SideTradeID;
    uint32_t MatchDate;
    uint32_t TrdMatchID;
    uint32_t StrategyLinkID;
    int32_t TotNumTradeReports;
    char Account[LEN_ACCOUNT];
    uint16_t Filler4;
    uint8_t MultiLegReportingType;
    uint8_t TradeReportType;
    uint8_t TransferReason;
    uint8_t Filler5;
    char RootPartyIDBeneficiary[LEN_ROOT_PARTY_ID_BENEFICIARY];
    char RootPartyIDTakeUpTradingFirm[LEN_ROOT_PARTY_ID_TAKE_UP_TRADING_FIRM];
    char RootPartyIDOrderOriginationFirm[LEN_ROOT_PARTY_ID_ORDER_ORIGINATION_FIRM];
    uint8_t AccountType;
    uint8_t MatchType;
    uint8_t MatchSubType;
    uint8_t Side;
    uint8_t AggressorIndicator;
    uint8_t TradingCapacity;
    char PositionEffect[LEN_POSITION_EFFECT];
    char CustOrderHandlingInst[LEN_CUST_ORDER_HANDLING_INST];
    char FreeText1[LEN_FREE_TEXT1];
    char CPCode[LEN_CP_CODE];
    char FreeText3[LEN_FREE_TEXT3];
    char OrderCategory[LEN_ORDER_CATEGORY];
    uint8_t OrdType;
    uint8_t RelatedProductComplex;
    uint8_t OrderSide;
    char RootPartyClearingOrganization[LEN_ROOT_PARTY_CLEARING_ORGANIZATION];
    char RootPartyExecutingFirm[LEN_ROOT_PARTY_EXECUTING_FIRM];
    char RootPartyExecutingTrader[LEN_ROOT_PARTY_EXECUTING_TRADER];
    char RootPartyClearingFirm[LEN_ROOT_PARTY_CLEARING_FIRM];
    char UserReferenceText[LEN_USER_REFERENCE_TEXT];
    char Pad7[LEN_PAD7];
} TradeBroadcastT;

// Message:	    TradeEnhancementNotification
// TemplateID:  10989
// Alias:       TradeEnhancementNotification
// FIX MsgType: TradeEnhancementStatus = "U31"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    int64_t ClearingTradePrice;
    uint64_t TransactionTime;
    uint64_t OrderID;
    int64_t ClearingTradeQty;
    uint32_t SimpleSecurityID;
    uint32_t TradeID;
    uint32_t RootPartyIDSessionID;
    uint32_t SideTradeID;
    int32_t MarketSegmentID;
    uint32_t MatchDate;
    uint8_t AccountType;
    uint8_t Side;
    uint8_t GiveupStatus;
    char CPCode[LEN_CP_CODE];
    char FreeText1[LEN_FREE_TEXT1];
    char Pad5[LEN_PAD5];
} TradeEnhancementNotificationT;

// Message:	    TradeModificationNotification
// TemplateID:  8020
// Alias:       TradeModificationNotification
// FIX MsgType: TradeModificationNotifications = "U94"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderCompT RBCHeader;
    uint64_t OrderID;
    uint32_t SimpleSecurityID;
    uint32_t RootPartyIDExecutingTrader;
    uint32_t SideTradeID;
    uint32_t TradeID;
    uint32_t RootPartyIDSubmitter;
    int32_t Echo;
    char OldCustomerorFirm[LEN_OLD_CUSTOMEROR_FIRM];
    char OldAccount[LEN_OLD_ACCOUNT];
    char OldClearingAccount[LEN_OLD_CLEARING_ACCOUNT];
    char NewCustomerorFirm[LEN_NEW_CUSTOMEROR_FIRM];
    char NewAccount[LEN_NEW_ACCOUNT];
    char NewClearingAccount[LEN_NEW_CLEARING_ACCOUNT];
    char Text[LEN_TEXT];
    char Pad6[LEN_PAD6];
} TradeModificationNotificationT;

// Message:	    TradeModificationRequest
// TemplateID:  8005
// Alias:       TradeModificationRequest
// FIX MsgType: TradeModificationRequest = "U92"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint64_t OrderID;
    uint32_t SimpleSecurityID;
    uint32_t RootPartyIDExecutingTrader;
    uint32_t SideTradeID;
    uint32_t TradeID;
    int32_t MarketSegmentID;
    int32_t Echo;
    char OldAccountType[LEN_OLD_ACCOUNT_TYPE];
    char OldUCCCode[LEN_OLD_UCC_CODE];
    char OldCPCode[LEN_OLD_CP_CODE];
    char NewAccountType[LEN_NEW_ACCOUNT_TYPE];
    char NewUCCCode[LEN_NEW_UCC_CODE];
    char NewCPCode[LEN_NEW_CP_CODE];
    char Text[LEN_TEXT];
    char Pad6[LEN_PAD6];
} TradeModificationRequestT;

// Message:	    TradeModificationResponse
// TemplateID:  8010
// Alias:       TradeModificationResponse
// FIX MsgType: TradeModificationResponse = "U93"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderMECompT ResponseHeaderME;
    uint64_t OrderID;
    uint64_t LastUpdateTime;
    uint32_t SimpleSecurityID;
    int32_t Echo;
    uint32_t SideTradeID;
    uint32_t TradeID;
    uint32_t RootPartyIDExecutingTrader;
    char OldAccountType[LEN_OLD_ACCOUNT_TYPE];
    char OldUCCCode[LEN_OLD_UCC_CODE];
    char OldCPCode[LEN_OLD_CP_CODE];
    char NewAccountType[LEN_NEW_ACCOUNT_TYPE];
    char NewUCCCode[LEN_NEW_UCC_CODE];
    char NewCPCode[LEN_NEW_CP_CODE];
    char Text[LEN_TEXT];
    char Pad2[LEN_PAD2];
} TradeModificationResponseT;

// Message:	    TradingSessionStatusBroadcast
// TemplateID:  10307
// Alias:       Trading Session Event
// FIX MsgType: TradingSessionStatus = "h"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    RBCHeaderMECompT RBCHeaderME;
    int32_t MarketSegmentID;
    uint32_t TradeDate;
    uint8_t TradSesEvent;
    char RefApplLastMsgID[LEN_REF_APPL_LAST_MSGID];
    char Pad7[LEN_PAD7];
} TradingSessionStatusBroadcastT;

// Message:	    UnsubscribeRequest
// TemplateID:  10006
// Alias:       Unsubscribe
// FIX MsgType: ApplicationMessageRequest = "BW"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t RefApplSubID;
    char Pad4[LEN_PAD4];
} UnsubscribeRequestT;

// Message:	    UnsubscribeResponse
// TemplateID:  10007
// Alias:       Unsubscribe Response
// FIX MsgType: ApplicationMessageRequestAck = "BX"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
} UnsubscribeResponseT;

// Message:	    UserLoginRequest
// TemplateID:  10018
// Alias:       User Logon
// FIX MsgType: UserRequest = "BE"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t Username;
    uint32_t EncryptedDataMessageSize;
	char Filler7[LEN_FILLER7];
    char Password[LEN_PASSWORD];
} UserLoginRequestT;

// Message:	    UserLoginResponse
// TemplateID:  10019
// Alias:       User Logon Response
// FIX MsgType: UserResponse = "BF"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
    uint64_t LastLoginTime;
    uint8_t DaysLeftForPasswordExpiry;
    uint8_t GraceLoginsLeft;
    char Pad6[LEN_PAD6];
} UserLoginResponseT;

// Message:	    UserLogoutRequest
// TemplateID:  10029
// Alias:       User Logout
// FIX MsgType: UserRequest = "BE"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t Username;
    char Pad4[LEN_PAD4];
} UserLogoutRequestT;

// Message:	    UserLogoutResponse
// TemplateID:  10024
// Alias:       User Logout Response
// FIX MsgType: UserResponse = "BF"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
} UserLogoutResponseT;

// Message:	    UserPasswordChange
// TemplateID:  10996
// Alias:       user password change
// FIX MsgType: UserPasswordChangeRequest = "U83"
typedef struct
{
    MessageHeaderInCompT MessageHeaderIn;
    RequestHeaderCompT RequestHeader;
    uint32_t Username;
    uint32_t Filler2;
    char Password[LEN_PASSWORD];
    char NewPassword[LEN_NEW_PASSWORD];
} UserPasswordChangeT;

// Message:	    UserPasswordChangeResponse
// TemplateID:  10990
// Alias:       user password change
// FIX MsgType: UserPasswordChangeResponse = "U85"
typedef struct
{
    MessageHeaderOutCompT MessageHeaderOut;
    ResponseHeaderCompT ResponseHeader;
} UserPasswordChangeResponseT;

/*
 * Begin of DEPRECATED defines
 */

#define BYTE_ARRAY_OF_0_16 {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}

#define	TID_AUCTIONNOTIFICATION                          11028		// < AuctionNotification (Auction Notification)
#define	TID_BROADCAST_ERROR_NOTIFICATION                 10032		// < BroadcastErrorNotification (Gap Fill)
#define	TID_CONNECTION_GATEWAY_REQUEST                   10020		// < ConnectionGatewayRequest (Connection Gateway Request)
#define	TID_CONNECTION_GATEWAY_RESPONSE                  10021		// < ConnectionGatewayResponse (Connection Gateway Response)
#define	TID_DELETE_ALL_ORDER_BROADCAST                   10122		// < DeleteAllOrderBroadcast (Order Mass Cancellation Notification)
#define	TID_DELETE_ALL_ORDER_NR_RESPONSE                 10124		// < DeleteAllOrderNRResponse (Order Mass Cancellation Response No Hits)
#define	TID_DELETE_ALL_ORDER_QUOTE_EVENT_BROADCAST       10308		// < DeleteAllOrderQuoteEventBroadcast (Mass Cancellation Event)
#define	TID_DELETE_ALL_ORDER_REQUEST                     10120		// < DeleteAllOrderRequest (Order Mass Cancellation Request)
#define	TID_DELETE_ALL_ORDER_RESPONSE                    10121		// < DeleteAllOrderResponse (Order Mass Cancellation Response)
#define	TID_DELETE_ORDER_BROADCAST                       10112		// < DeleteOrderBroadcast (Cancel Order Notification)
#define	TID_DELETE_ORDER_COMPLEX_REQUEST                 10123		// < DeleteOrderComplexRequest (Cancel Order complex)
#define	TID_DELETEORDERNRRESPONSE                        10111		// < DeleteOrderNRResponse (Cancel Order Response (Lean Order))
#define	TID_DELETE_ORDER_RESPONSE                        10110		// < DeleteOrderResponse (Cancel Order Response (Standard Order))
#define	TID_DELETE_ORDER_SINGLE_REQUEST                  10109		// < DeleteOrderSingleRequest (Cancel Order Single)
#define	TID_EXD_EX_CANCELLATION_CONFIRMATION             8037		// < ExDExCancellationConfirmation ( Ex/DEx Cancellation Confirmation )
#define	TID_EXD_EX_CANCELLATION_REQUEST                  8036		// < ExDExCancellationRequest ( Ex/DEx Cancellation Request )
#define	TID_EXD_EX_ENTRY_CONFIRMATION                    8031		// < ExDExEntryConfirmation ( Ex/DEx Entry Confirmation )
#define	TID_EXD_EX_ENTRY_REQUEST                         8030		// < ExDExEntryRequest ( Ex/DEx Entry Request )
#define	TID_EXD_EX_MODIFICATION_CONFIRMATION             8034		// < ExDExModificationConfirmation ( Ex/DEx Modification Confirmation )
#define	TID_EXD_EX_MODIFICATION_REQUEST                  8033		// < ExDExModificationRequest (Ex/DEx Modification Request )
#define	TID_EXD_EX_NOTIFICATION                          8040		// < ExDExNotification ( Ex/DEx Notification )
#define	TID_EXTENDED_ORDER_INFORMATION                   10994		// < ExtendedOrderInformation (Extended Order Information)
#define	TID_FORCED_LOGOUT_NOTIFICATION                   10012		// < ForcedLogoutNotification (Session Logout Notification)
#define	TID_FORCED_USER_LOGOUT_NOTIFICATION              10043		// < ForcedUserLogoutNotification (User Logout Notification)
#define	TID_HEARTBEAT                                    10011		// < Heartbeat (Heartbeat)
#define	TID_HEARTBEAT_NOTIFICATION                       10023		// < HeartbeatNotification (Heartbeat Notification)
#define	TID_IMMEDIATE_EXECUTION_RESPONSE                 10993		// < ImmediateExecutionResponse (Immediate Execution Response)
#define	TID_INQUIREPRETRADERISKLIMITSREQUEST             10311		// < InquirePreTradeRiskLimitsRequest (Inquire Pre-Trade Risk Limits Request)
#define	TID_LOGON_REQUEST                                10000		// < LogonRequest (Session Logon)
#define	TID_LOGON_RESPONSE                               10001		// < LogonResponse (Session Logon Response)
#define	TID_LOGOUT_REQUEST                               10002		// < LogoutRequest (Session Logout)
#define	TID_LOGOUT_RESPONSE                              10003		// < LogoutResponse (Session Logout Response)
#define	TID_MARGIN_CHANGE_NOTIFICATION                   4025		// < MarginChangeNotification (Margin Change Notification)
#define	TID_MARKET_STATUS_NOTIFICATION                   4125		// < MarketStatusNotification ( Market Status Notification )
#define	TID_MARKET_WIDE_OI_ALERT                         8101		// < MarketWideOIAlert (Market Wide OI Alert)
#define	TID_MARKET_WIDE_OI_VIOLATION                     8100		// < MarketWideOIViolation (Market Wide OI Violation)
#define	TID_MODIFY_ORDER_COMPLEX_REQUEST                 10114		// < ModifyOrderComplexRequest (Replace Order complex)
#define	TID_MODIFYORDERNRRESPONSE                        10108		// < ModifyOrderNRResponse (Replace Order Response (Lean Order))
#define	TID_MODIFY_ORDER_RESPONSE                        10107		// < ModifyOrderResponse (Replace Order Response (Standard Order))
#define	TID_MODIFY_ORDER_SINGLE_REQUEST                  10106		// < ModifyOrderSingleRequest (Replace Order Single)
#define	TID_MODIFY_ORDER_SINGLE_SHORT_REQUEST            10126		// < ModifyOrderSingleShortRequest (Replace Order Single (short layout))
#define	TID_NEW_ORDER_COMPLEX_REQUEST                    10113		// < NewOrderComplexRequest (New Order Multi Leg)
#define	TID_NEW_ORDER_MULTI_LEG                          10991		// < NewOrderMultiLeg (New Order MultiLeg)
#define	TID_NEWORDERNRRESPONSE                           10102		// < NewOrderNRResponse (New Order Response (Lean Order))
#define	TID_NEW_ORDER_RESPONSE                           10101		// < NewOrderResponse (New Order Response (Standard Order))
#define	TID_NEW_ORDER_SINGLE_REQUEST                     10100		// < NewOrderSingleRequest (New Order Single)
#define	TID_NEW_ORDER_SINGLE_SHORT_REQUEST               10125		// < NewOrderSingleShortRequest (New Order Single (short layout))
#define	TID_NEWS_BROADCAST                               10031		// < NewsBroadcast (News)
#define	TID_ORDER_EXEC_NOTIFICATION                      10104		// < OrderExecNotification (Book Order Execution)
#define	TID_ORDER_EXEC_REPORT_BROADCAST                  10117		// < OrderExecReportBroadcast (Extended Order Information)
#define	TID_ORDER_EXEC_RESPONSE                          10103		// < OrderExecResponse (Immediate Execution Response)
#define	TID_PRETRADERISKLIMITRESPONSE                    10313		// < PreTradeRiskLimitResponse (Pre-Trade Risk Limit Response)
#define	TID_REJECT                                       10010		// < Reject (Reject)
#define	TID_REJECTMULTILEG                               10992		// < RejectMultiLeg (Reject MultiLeg)
#define	TID_RESUBMIT_APPROVAL_CONFIRMATION               8510		// < ResubmitApprovalConfirmation (ResubmitForApproval)
#define	TID_RESUBMIT_APPROVAL_REQUEST                    8500		// < ResubmitApprovalRequest (ApprovalRequest)
#define	TID_RETRANSMIT_ME_MESSAGE_REQUEST                10026		// < RetransmitMEMessageRequest (Retransmit (Order/Quote Event))
#define	TID_RETRANSMIT_ME_MESSAGE_RESPONSE               10027		// < RetransmitMEMessageResponse (Retransmit Response (Order/Quote Event))
#define	TID_RETRANSMIT_REQUEST                           10008		// < RetransmitRequest (Retransmit)
#define	TID_RETRANSMIT_RESPONSE                          10009		// < RetransmitResponse (Retransmit Response)
#define	TID_SERVICE_AVAILABILITY_BROADCAST               10030		// < ServiceAvailabilityBroadcast (Service Availability)
#define	TID_SESSION_PASSWORD_CHANGE                      10997		// < SessionPasswordChange (user password change)
#define	TID_SESSION_PASSWORD_CHANGE_RESPONSE             10995		// < SessionPasswordChangeResponse (user password change)
#define	TID_SUBSCRIBE_REQUEST                            10025		// < SubscribeRequest (Subscribe)
#define	TID_SUBSCRIBE_RESPONSE                           10005		// < SubscribeResponse (Subscribe Response)
#define	TID_SYSTEMINFORMATIONDOWNLOAD                    4100		// < SystemInformationDownload (System Information Download)
#define	TID_TM_TRADING_SESSION_STATUS_BROADCAST          10501		// < TMTradingSessionStatusBroadcast (Trade Notification Status)
#define	TID_THROTTLE_UPDATE_NOTIFICATION                 10028		// < ThrottleUpdateNotification (Throttle Update Notification)
#define	TID_TRADE_BROADCAST                              10500		// < TradeBroadcast (Trade Notification)
#define	TID_TRADE_ENHANCEMENT_NOTIFICATION               10989		// < TradeEnhancementNotification (TradeEnhancementNotification)
#define	TID_TRADE_MODIFICATION_NOTIFICATION              8020		// < TradeModificationNotification (TradeModificationNotification)
#define	TID_TRADE_MODIFICATION_REQUEST                   8005		// < TradeModificationRequest (TradeModificationRequest)
#define	TID_TRADE_MODIFICATION_RESPONSE                  8010		// < TradeModificationResponse (TradeModificationResponse)
#define	TID_TRADING_SESSION_STATUS_BROADCAST             10307		// < TradingSessionStatusBroadcast (Trading Session Event)
#define	TID_UNSUBSCRIBE_REQUEST                          10006		// < UnsubscribeRequest (Unsubscribe)
#define	TID_UNSUBSCRIBE_RESPONSE                         10007		// < UnsubscribeResponse (Unsubscribe Response)
#define	TID_USER_LOGIN_REQUEST                           10018		// < UserLoginRequest (User Logon)
#define	TID_USER_LOGIN_RESPONSE                          10019		// < UserLoginResponse (User Logon Response)
#define	TID_USER_LOGOUT_REQUEST                          10029		// < UserLogoutRequest (User Logout)
#define	TID_USER_LOGOUT_RESPONSE                         10024		// < UserLogoutResponse (User Logout Response)
#define	TID_USER_PASSWORD_CHANGE                         10996		// < UserPasswordChange (user password change)
#define	TID_USERPASSWORDCHANGERESPONSE                   10990		// < UserPasswordChangeResponse (user password change)

#define MAX_IMMEDIATEEXECUTIONRESPONSE_MULTILEGEXECGRP  	254
#define MAX_IMMEDIATEEXECUTIONRESPONSE_INSTRMNTLEGEXECGRP	600
#define MAX_MARGINCHANGENOTIFICATION_MARGINCHANGERPTGRP 	50
#define MAX_MODIFYORDERCOMPLEXREQUEST_LEGORDGRP         	20
#define MAX_NEWORDERCOMPLEXREQUEST_LEGORDGRP            	20
#define MAX_NEWORDERMULTILEG_MULTILEGORDGRP             	254
#define MAX_ORDEREXECNOTIFICATION_FILLSGRP              	100
#define MAX_ORDEREXECNOTIFICATION_INSTRMNTLEGEXECGRP    	600
#define MAX_ORDEREXECREPORTBROADCAST_LEGORDGRP          	20
#define MAX_ORDEREXECREPORTBROADCAST_FILLSGRP           	100
#define MAX_ORDEREXECREPORTBROADCAST_INSTRMNTLEGEXECGRP 	600
#define MAX_ORDEREXECRESPONSE_FILLSGRP                  	100
#define MAX_ORDEREXECRESPONSE_INSTRMNTLEGEXECGRP        	600
#define MAX_DELETEALLORDERRESPONSE_AFFECTEDORDERSGRP    	500
#define MAX_DELETEALLORDERBROADCAST_NOTAFFECTEDORDERSGRP	500
#define MAX_DELETEALLORDERRESPONSE_NOTAFFECTEDORDERSGRP 	500
#define MAX_EXTENDEDORDERINFORMATION_MULTILEGGRP        	254
#define MAX_EXTENDEDORDERINFORMATION_MULTILEGFILLGRP    	254
#define MAX_EXTENDEDORDERINFORMATION_INSTRMNTLEGEXECGRP 	600
#define MAX_IMMEDIATEEXECUTIONRESPONSE_MULTILEGEXECGRP  	254
#define MAX_IMMEDIATEEXECUTIONRESPONSE_INSTRMNTLEGEXECGRP	600
#define MAX_NEWORDERCOMPLEXREQUEST_LEGORDGRP            	20
#define MAX_NEWORDERMULTILEG_MULTILEGORDGRP             	254
#define MAX_PRETRADERISKUTILIZEDLIMITSRESPONSE_RISKLIMITSRPTGRP	64

#define LEN_ACCOUNT                                      2
#define ENUM_ACCOUNTTYPE_OWN                             1
#define ENUM_ACCOUNTTYPE_CLIENT                          3
#define ENUM_ACCOUNTTYPE_INSTITUTION                     5
#define ENUM_AGGRESSORINDICATOR_PASSIVE                  0
#define ENUM_AGGRESSORINDICATOR_AGGRESSIVE               1
#define LEN_ALLORNONEFLAG                                1
#define ENUM_ALLORNONEFLAG_PARTIALEXECUTION              "N"
#define LEN_APPLBEGMSGID                                 16
#define LEN_APPLENDMSGID                                 16
#define ENUM_APPLID_TRADE_ENHANCEMENT                    0
#define ENUM_APPLID_TRADE                                1
#define ENUM_APPLID_NEWS                                 2
#define ENUM_APPLID_SERVICE_AVAILABILITY                 3
#define ENUM_APPLID_SESSION_DATA                         4
#define ENUM_APPLID_LISTENER_DATA                        5
#define ENUM_APPLID_RISKCONTROL                          6
#define ENUM_APPLID_SPECIALIST_DATA                      11
#define ENUM_APPLIDSTATUS_OUTBOUND_CONVERSION_ERROR      105
#define ENUM_APPLIDSTATUS_MORE_VALUES                    106
#define LEN_APPLMSGID                                    16
#define ENUM_APPLRESENDFLAG_FALSE                        0
#define ENUM_APPLRESENDFLAG_TRUE                         1
#define ENUM_APPLSEQINDICATOR_STANDARDORDER              1
#define ENUM_APPLSEQSTATUS_UNAVAILABLE                   0
#define ENUM_APPLSEQSTATUS_AVAILABLE                     1
#define LEN_APPLUSAGEORDERS                              1
#define ENUM_APPLUSAGEORDERS_AUTOMATED                   "A"
#define ENUM_APPLUSAGEORDERS_MANUAL                      "M"
#define ENUM_APPLUSAGEORDERS_AUTOSELECT                  "B"
#define ENUM_APPLUSAGEORDERS_NONE                        "N"
#define LEN_APPLUSAGEQUOTES                              1
#define ENUM_APPLUSAGEQUOTES_AUTOMATED                   "A"
#define ENUM_APPLUSAGEQUOTES_MANUAL                      "M"
#define ENUM_APPLUSAGEQUOTES_AUTOSELECT                  "B"
#define ENUM_APPLUSAGEQUOTES_NONE                        "N"
#define LEN_APPLICATIONSYSTEMVENDOR                      30
#define LEN_APPLICATIONSYSTEMVERSION                     30
#define ENUM_ASSETTYPE_ASSET_CLASS                       0
#define ENUM_ASSETTYPE_ASSET                             1
#define ENUM_ASSETTYPE_UNDERLYING                        2
#define ENUM_ASSETTYPE_PRODUCT                           3
#define LEN_CPCODE                                       12
#define LEN_COMPLIANCETEXT                               20
#define LEN_CUSTORDERHANDLINGINST                        1
#define ENUM_CUSTORDERHANDLINGINST_W                     "W"
#define ENUM_CUSTORDERHANDLINGINST_Y                     "Y"
#define ENUM_CUSTORDERHANDLINGINST_C                     "C"
#define ENUM_CUSTORDERHANDLINGINST_G                     "G"
#define ENUM_CUSTORDERHANDLINGINST_H                     "H"
#define ENUM_CUSTORDERHANDLINGINST_D                     "D"
#define LEN_DEFAULTCSTMAPPLVERID                         30
#define ENUM_EXDEXFLAG_EXERCISE_INSTRUCTION              1
#define ENUM_EXDEXFLAG_DONT_EXERCISE_INSTRUCTION         2
#define ENUM_EXDEXSTATUS_ACCEPTED                        1
#define ENUM_EXDEXSTATUS_CANCELLED                       2
#define ENUM_EXECINST_H                                  1
#define ENUM_EXECRESTATEMENTREASON_CORPORATE_ACTION      0
#define ENUM_EXECRESTATEMENTREASON_ORDER_BOOK_RESTATEMENT 1
#define ENUM_EXECRESTATEMENTREASON_EXCHANGE_OPTION       8
#define ENUM_EXECRESTATEMENTREASON_ORDER_ADDED           101
#define ENUM_EXECRESTATEMENTREASON_ORDER_MODIFIED        102
#define ENUM_EXECRESTATEMENTREASON_ORDER_CANCELLED       103
#define ENUM_EXECRESTATEMENTREASON_IOC_ORDER_CANCELLED   105
#define ENUM_EXECRESTATEMENTREASON_BOOK_ORDER_EXECUTED   108
#define ENUM_EXECRESTATEMENTREASON_CHANGED_TO_IOC        114
#define ENUM_EXECRESTATEMENTREASON_CHANGE_OF_SPECIALIST  119
#define ENUM_EXECRESTATEMENTREASON_INSTRUMENT_STATE_CHANGE 122
#define ENUM_EXECRESTATEMENTREASON_MARKET_ORDER_TRIGGERED 135
#define ENUM_EXECRESTATEMENTREASON_OCO_ORDER_TRIGGERED   164
#define ENUM_EXECRESTATEMENTREASON_STOP_ORDER_TRIGGERED  172
#define ENUM_EXECRESTATEMENTREASON_OWNERSHIP_CHANGED     181
#define ENUM_EXECRESTATEMENTREASON_ORDER_CANCELLATION_PENDING 197
#define ENUM_EXECRESTATEMENTREASON_PENDING_CANCELLATION_EXECUTED 199
#define ENUM_EXECRESTATEMENTREASON_ORDER_ADD_SENT_RISK_VALIDATION 200
#define ENUM_EXECRESTATEMENTREASON_SELF_TRADE_ORDER_DELETED 201
#define ENUM_EXECRESTATEMENTREASON_ORDERCANCELLED_SESSION_EXPIRY 202
#define ENUM_EXECRESTATEMENTREASON_BOC_ORDER_CANCELLED   212
#define ENUM_EXECRESTATEMENTREASON_ORDER_RISK_VALIDATION 215
#define ENUM_EXECRESTATEMENTREASON_PROVISIONAL_ORDER_ADDED 221
#define ENUM_EXECRESTATEMENTREASON_ACTIVE_DELETE_DUE_TO_SMPF 248
#define ENUM_EXECRESTATEMENTREASON_ACTIVE_MODIFY_DUE_TO_SMPF 250
#define ENUM_EXECRESTATEMENTREASON_PASSIVE_DELETE_DUE_TO_SMPF 252
#define ENUM_EXECRESTATEMENTREASON_PASSIVE_MODIFY_DUE_TO_SMPF 254
#define ENUM_EXECRESTATEMENTREASON_EXDEX_REQUEST_ADDED   270
#define ENUM_EXECRESTATEMENTREASON_EXDEX_REQUEST_MODIFIED 271
#define ENUM_EXECRESTATEMENTREASON_EXDEX_REQUEST_CANCELLED 272
#define LEN_EXECTYPE                                     1
#define ENUM_EXECTYPE_NEW                                "0"
#define ENUM_EXECTYPE_CANCELED                           "4"
#define ENUM_EXECTYPE_REPLACED                           "5"
#define ENUM_EXECTYPE_SUSPENDED                          "9"
#define ENUM_EXECTYPE_RESTATED                           "D"
#define ENUM_EXECTYPE_TRIGGERED                          "L"
#define ENUM_EXECTYPE_TRADE                              "F"
#define LEN_FIXENGINENAME                                30
#define LEN_FIXENGINEVENDOR                              30
#define LEN_FIXENGINEVERSION                             30
#define LEN_FILLER6                                      1
#define LEN_FREETEXT1                                    12
#define LEN_FREETEXT2                                    12
#define LEN_FREETEXT3                                    12
#define ENUM_GIVEUPSTATUS_ACCEPTED                       1
#define ENUM_GIVEUPSTATUS_REJECTED                       2
#define ENUM_GIVEUPSTATUS_PENDING                        3
#define ENUM_GROUPID_APPLICABLE_FOR_ALL_GROUPS           0
#define LEN_HEADLINE                                     256
#define LEN_IVVECTOR                                     13
#define LEN_KEY                                          33
#define ENUM_LASTFRAGMENT_NOT_LAST_MESSAGE               0
#define ENUM_LASTFRAGMENT_LAST_MESSAGE                   1
#define LEN_LEGACCOUNT                                   2
#define LEN_LEGPOSITIONEFFECT                            1
#define ENUM_LEGPOSITIONEFFECT_CLOSE                     "C"
#define ENUM_LEGSIDE_BUY                                 1
#define ENUM_LEGSIDE_SELL                                2
#define ENUM_MASSACTIONREASON_NO_SPECIAL_REASON          0
#define ENUM_MASSACTIONREASON_STOP_TRADING               1
#define ENUM_MASSACTIONREASON_EMERGENCY                  2
#define ENUM_MASSACTIONREASON_MARKET_MAKER_PROTECTION    3
#define ENUM_MASSACTIONREASON_STOP_BUTTON_ACTIVATED      4
#define ENUM_MASSACTIONREASON_BUSINESS_UNIT_SUSPENDED    5
#define ENUM_MASSACTIONREASON_SESSION_LOSS               6
#define ENUM_MASSACTIONREASON_DUPLICATE_SESSION_LOGIN    7
#define ENUM_MASSACTIONREASON_CLEARING_RISK_CONTROL      8
#define ENUM_MASSACTIONREASON_INTERNAL_CONNECTION_LOSS   100
#define ENUM_MASSACTIONREASON_PRODUCT_STATE_HALT         105
#define ENUM_MASSACTIONREASON_PRODUCT_STATE_HOLIDAY      106
#define ENUM_MASSACTIONREASON_INSTRUMENT_SUSPENDED       107
#define ENUM_MASSACTIONREASON_PRICE_BAND_SHRINK          108
#define ENUM_MASSACTIONREASON_COMPLEX_INSTRUMENT_DELETION 109
#define ENUM_MASSACTIONREASON_VOLATILITY_INTERRUPTION    110
#define ENUM_MASSACTIONREASON_PRODUCT_TEMPORARILY_NOT_TRADEABLE 111
#define ENUM_MASSACTIONREASON_INSTRUMENT_STOPPED         113
#define ENUM_MASSACTIONREASON_PMATCHINGENGINESTATUSRODUCT_STATE_CLOSING 114
#define ENUM_MASSACTIONREASON_INSTRUMENT_KNOCK_OUT       115
#define ENUM_MASSACTIONREASON_INSTRUMENT_SOLD_OUT        116
#define ENUM_MASSACTIONREASON_MEMBER_DISABLE             117
#define ENUM_MASSACTIONREASON_RRM_MARKET_WIDE_POSITION_LIMIT 119
#define ENUM_MASSACTIONREASON_INSTRUMENT_KNOCK_OUT_REVERTED 118
#define ENUM_MASSACTIONREASON_PRODUCT_STATE_EOD          120
#define ENUM_MASS_ACTION_REASON_MEMBER_STATUS_CHANGE     179
#define ENUM_MASS_ACTION_REASON_RRM_CANCELLATION         202
#define ENUM_MASS_ACTION_REASON_SQAREOFF                 203
#define ENUM_MASS_ACTION_REASON_USER_SUSPENDED           204
#define ENUM_MASS_ACTION_REASON_DEBARRED                 205
#define ENUM_MASS_ACTION_REASON_DISALLLOWED              206
#define ENUM_MATCHSUBTYPE_OPENING_AUCTION                1
#define ENUM_MATCHSUBTYPE_INTRADAY_AUCTION               3
#define ENUM_MATCHSUBTYPE_TRADE_AT_CLOSE                 5
#define ENUM_MATCHTYPE_CONFIRMED_TRADE_REPORT            3
#define ENUM_MATCHTYPE_AUTO_MATCH_INCOMING               4
#define ENUM_MATCHTYPE_AUTO_MATCH_RESTING                11
#define ENUM_MATCHTYPE_AUTO_MATCH_AT_MID_POINT           12
#define ENUM_MATCHINGENGINESTATUS_UNAVAILABLE            0
#define ENUM_MATCHINGENGINESTATUS_AVAILABLE              1
#define LEN_MSGTYPE                                      3
#define ENUM_MSGTYPE_HEARTBEAT                           "0  "
#define ENUM_MSGTYPE_TESTREQUEST                         "1  "
#define ENUM_MSGTYPE_REJECT                              "3  "
#define ENUM_MSGTYPE_LOGOUT                              "5  "
#define ENUM_MSGTYPE_EXECUTIONREPORT                     "8  "
#define ENUM_MSGTYPE_LOGON                               "A  "
#define ENUM_MSGTYPE_NEWORDERMULTILEG                    "AB "
#define ENUM_MSGTYPE_MULTILEGORDERCANCELREPLACE          "AC "
#define ENUM_MSGTYPE_TRADECAPTUREREPORT                  "AE "
#define ENUM_MSGTYPE_TRADECAPTUREREPORTACK               "AR "
#define ENUM_MSGTYPE_NEWS                                "B  "
#define ENUM_MSGTYPE_USERREQUEST                         "BE "
#define ENUM_MSGTYPE_USERRESPONSE                        "BF "
#define ENUM_MSGTYPE_APPLICATIONMESSAGEREQUEST           "BW "
#define ENUM_MSGTYPE_APPLICATIONMESSAGEREQUESTACK        "BX "
#define ENUM_MSGTYPE_APPLICATIONMESSAGEREPORT            "BY "
#define ENUM_MSGTYPE_ORDERMASSACTIONREPORT               "BZ "
#define ENUM_MSGTYPE_ORDERMASSACTIONREQUEST              "CA "
#define ENUM_MSGTYPE_USERNOTIFICATION                    "CB "
#define ENUM_MSGTYPE_NEWORDERSINGLE                      "D  "
#define ENUM_MSGTYPE_ORDERCANCELREQUEST                  "F  "
#define ENUM_MSGTYPE_ORDERCANCELREPLACEREQUEST           "G  "
#define ENUM_MSGTYPE_TRADINGSESSIONSTATUS                "h  "
#define ENUM_MSGTYPE_SECURITYSTATUSDEFINITIONREQUEST     "U27"
#define ENUM_MSGTYPE_MULTILEGORDER                       "U28"
#define ENUM_MSG_TYPE_MULTI_LEG_ORDER_ACKNOWLEDGEMENT    "U29"
#define ENUM_MSGTYPE_MULTILEGEXECUTIONREPORT             "U30"
#define ENUM_MSGTYPE_TRADEENHANCEMENTSTATUS              "U31"
#define ENUM_MSGTYPE_SYSTEMINFORMATION                   "U79"
#define ENUM_MSGTYPE_MARKETSTATUSNOTIFICATION            "U90"
#define ENUM_MSGTYPE_TRADEMODIFICATIONREQUEST            "U92"
#define ENUM_MSGTYPE_TRADEMODIFICATIONRESPONSE           "U93"
#define ENUM_MSGTYPE_TRADEMODIFICATIONNOTIFICATIONS      "U94"
#define ENUM_MSGTYPE_EXDEXENTRYREQUEST                   "U74"
#define ENUM_MSGTYPE_EXDEXENTRYCONFIRMATION              "U75"
#define ENUM_MSGTYPE_EXDEXMODIFICATIONREQUEST            "U77"
#define ENUM_MSGTYPE_EXDEXMODIFICATIONCONFIRMATION       "U80"
#define ENUM_MSGTYPE_EXDEXCANCELLATIONREQUEST            "U76"
#define ENUM_MSGTYPE_EXDEXCANCELLATIONCONFIRMATION       "U81"
#define ENUM_MSGTYPE_EXDEXNOTIFICATION                   "U78"
#define ENUM_MSGTYPE_MARKETWIDEOIVIOLATION               "U88"
#define ENUM_MSGTYPE_MARKETWIDEOIALERT                   "U89"
#define ENUM_MSGTYPE_RESUBMITAPPROVALREQUEST             "U86"
#define ENUM_MSGTYPE_RESUBMITAPPROVALCONFIRMATION        "U87"
#define ENUM_MSGTYPE_LOGON                               "A  "
#define ENUM_MSGTYPE_LOGONR                              "B1 "
#define ENUM_MSGTYPE_USERPASSWORDCHANGERESPONSE          "U85"
#define ENUM_MSGTYPE_SESSIONPASSWORDCHANGERESPONSE       "U82"
#define ENUM_MSGTYPE_USERPASSWORDCHANGEREQUEST           "U83"
#define ENUM_MSGTYPE_SESSIONPASSWORDCHANGE               "U84"
#define ENUM_MULTILEGREPORTINGTYPE_SINGLE_SECURITY       1
#define ENUM_MULTILEGREPORTINGTYPE_INDIVIDUAL_LEG_COMPLEX_ORDER 2
#define LEN_NETWORKMSGID                                 8
#define LEN_NEWACCOUNTTYPE                               1
#define ENUM_NEWACCOUNTTYPE_OWN                          "1"
#define ENUM_NEWACCOUNTTYPE_CLIENT                       "2"
#define ENUM_NEWACCOUNTTYPE_INSTITUTION                  "3"
#define LEN_NEWCPCODE                                    12
#define LEN_NEWCLEARINGACCOUNT                           12
#define LEN_NEWPASSWORD                                  344
#define ENUM_NEWSPECIALMARGINFLATPERC_IM_PERCENTAGE      1
#define ENUM_NEWSPECIALMARGINFLATPERC_IM_ABSOLUTE        2
#define LEN_NEWUCCCODE                                   10
#define LEN_OLDACCOUNTTYPE                               1
#define ENUM_OLDACCOUNTTYPE_OWN                          "1"
#define ENUM_OLDACCOUNTTYPE_CLIENT                       "2"
#define ENUM_OLDACCOUNTTYPE_INSTITUTION                  "3"
#define LEN_OLDCPCODE                                    12
#define ENUM_OLDINITMARGINFLATPERC_IM_PERCENTAGE         1
#define ENUM_OLDINITMARGINFLATPERC_IM_ABSOLUTE           2
#define ENUM_OLDINITIALMARGINSPREADBENEFITONOROFFFLAG_OFF 0
#define ENUM_OLDINITIALMARGINSPREADBENEFITONOROFFFLAG_ON 1
#define ENUM_OLDSPECIALMARGINFLATPERC_IM_PERCENTAGE      1
#define ENUM_OLDSPECIALMARGINFLATPERC_IM_ABSOLUTE        2
#define LEN_OLDUCCCODE                                   10
#define LEN_ORDSTATUS                                    1
#define ENUM_ORDSTATUS_NEW                               "0"
#define ENUM_ORDSTATUS_PARTIALLY_FILLED                  "1"
#define ENUM_ORDSTATUS_FILLED                            "2"
#define ENUM_ORDSTATUS_CANCELED                          "4"
#define ENUM_ORDSTATUS_SUSPENDED                         "9"
#define ENUM_ORDTYPE_LIMIT                               2
#define ENUM_ORDTYPE_STOP                                3
#define ENUM_ORDTYPE_STOP_LIMIT                          4
#define ENUM_ORDTYPE_MARKET_TO_LIMIT                     5
#define ENUM_ORDTYPE_AUCTION_BUY_IN                      6
#define ENUM_ORDTYPE_AUCTION_SELL_OUT                    7
#define LEN_ORDERCATEGORY                                1
#define ENUM_ORDERCATEGORY_ORDER                         "1"
#define ENUM_ORDERCATEGORY_MULTILEG_ORDER                "3"
#define LEN_ORDERROUTINGINDICATOR                        1
#define ENUM_ORDERROUTINGINDICATOR_YES                   "Y"
#define ENUM_ORDERROUTINGINDICATOR_NO                    "N"
#define ENUM_ORDERSIDE_BUY                               1
#define ENUM_ORDERSIDE_SELL                              2
#define LEN_PAD1                                         1
#define LEN_PAD2                                         2
#define LEN_PAD3                                         3
#define LEN_PAD4                                         4
#define LEN_PAD45                                        54
#define LEN_PAD4_1                                       4
#define LEN_PAD5                                         5
#define LEN_PAD6                                         6
#define LEN_PAD7                                         7
#define LEN_PARTYIDBENEFICIARY                           9
#define ENUM_PARTYIDENTERINGFIRM_PARTICIPANT             1
#define ENUM_PARTYIDENTERINGFIRM_MARKETSUPERVISION       2
#define LEN_PARTYIDLOCATIONID                            2
#define LEN_PARTYIDTAKEUPTRADINGFIRM                     5
#define LEN_POSITIONEFFECT                               1
#define ENUM_POSITIONEFFECT_CLOSE                        "C"
#define ENUM_PRICEVALIDITYCHECKTYPE_NONE                 0
#define ENUM_PRODUCTCOMPLEX_SIMPLE_INSTRUMENT            1
#define ENUM_PRODUCTCOMPLEX_FUTURES_SPREAD               5
#define ENUM_REFAPPLID_TRADE                             1
#define ENUM_REFAPPLID_NEWS                              2
#define ENUM_REFAPPLID_SERVICE_AVAILABILITY              3
#define ENUM_REFAPPLID_SESSION_DATA                      4
#define ENUM_REFAPPLID_LISTENER_DATA                     5
#define ENUM_REFAPPLID_RISKCONTROL                       6
#define ENUM_REFAPPLID_EXDEX                             12
#define ENUM_REFAPPLID_TRADE_ENHANCEMENT                 13
#define ENUM_REFAPPLID_RISK_ADMIN                        14
#define LEN_REFAPPLLASTMSGID                             16
#define ENUM_RELATEDPRODUCTCOMPLEX_FUTURES_SPREAD        5
#define LEN_REQUESTINGPARTYCLEARINGFIRM                  9
#define LEN_ROOTPARTYCLEARINGFIRM                        5
#define LEN_ROOTPARTYCLEARINGORGANIZATION                4
#define LEN_ROOTPARTYEXECUTINGFIRM                       5
#define LEN_ROOTPARTYEXECUTINGTRADER                     6
#define LEN_ROOTPARTYIDBENEFICIARY                       9
#define LEN_ROOTPARTYIDORDERORIGINATIONFIRM              7
#define LEN_ROOTPARTYIDTAKEUPTRADINGFIRM                 5
#define ENUM_SMPFORDERIDENTIFIER_PASSIVE                 0
#define ENUM_SMPFORDERIDENTIFIER_ACTIVE                  1
#define ENUM_STPCFLAG_PASSIVE                            0
#define ENUM_STPCFLAG_ACTIVE                             1
#define ENUM_SECURITYTYPE_CUMMULATIVE                    0
#define ENUM_SECURITYTYPE_UNDERLYING_FUTURES             1
#define ENUM_SECURITYTYPE_OPTIONS                        3
#define ENUM_SECURITYTYPE_FUTURES                        4
#define ENUM_SESSIONMODE_LF                              2
#define ENUM_SESSIONREJECTREASON_REQUIRED_TAG_MISSING    1
#define ENUM_SESSIONREJECTREASON_VALUE_IS_INCORRECT      5
#define ENUM_SESSIONREJECTREASON_DECRYPTION_PROBLEM      7
#define ENUM_SESSIONREJECTREASON_INVALID_MSGID           11
#define ENUM_SESSIONREJECTREASON_INCORRECT_NUMINGROUP_COUNT 16
#define ENUM_SESSIONREJECTREASON_OTHER                   99
#define ENUM_SESSIONREJECTREASON_THROTTLE_LIMIT_EXCEEDED 100
#define ENUM_SESSIONREJECTREASON_EXPOSURE_LIMIT_EXCEEDED 101
#define ENUM_SESSIONREJECTREASON_SERVICE_TEMPORARILY_NOT_AVAILABLE 102
#define ENUM_SESSIONREJECTREASON_SERVICE_NOT_AVAILABLE   103
#define ENUM_SESSIONREJECTREASON_RESULT_TRANSACTION_UNKNOWN 104
#define ENUM_SESSIONREJECTREASON_OUTBOUND_CONVERSION_ERROR 105
#define ENUM_SESSIONREJECTREASON_HEARTBEAT_VIOLATION     152
#define ENUM_SESSIONREJECTREASON_INTERNAL_TECHNICAL_ERROR 200
#define ENUM_SESSIONREJECTREASON_VALIDATION_ERROR        210
#define ENUM_SESSIONREJECTREASON_USER_ALREADY_LOGGED_IN  211
#define ENUM_SESSIONREJECTREASON_ORDER_NOT_FOUND         10000
#define ENUM_SESSIONREJECTREASON_PRICE_NOT_REASONABLE    10001
#define ENUM_SESSIONREJECTREASON_BU_BOOK_ORDER_LIMIT_EX_CEEDED 10004
#define ENUM_SESSIONREJECTREASON_SESSION_BOOK_ORDER_LIMIT_EXCEEDED 10005
#define ENUM_SESSIONREJECTREASON_LSTUPDATE_TIMESTAMP_NOT_MATCHED 10006
#define ENUM_SESSIONSTATUS_ACTIVE                        0
#define ENUM_SESSIONSTATUS_LOGOUT                        4
#define ENUM_SIDE_BUY                                    1
#define ENUM_SIDE_SELL                                   2
#define LEN_TEXT                                         12
#define ENUM_TIMEINFORCE_DAY                             0
#define ENUM_TIMEINFORCE_GTC                             1
#define ENUM_TIMEINFORCE_IOC                             3
#define ENUM_TIMEINFORCE_GTD                             6
#define ENUM_TIMEINFORCE_SESSION                         7
#define ENUM_TRADSESEVENT_START_OF_SERVICE               101
#define ENUM_TRADSESEVENT_MARKET_RESET                   102
#define ENUM_TRADSESEVENT_END_OF_RESTATEMENT             103
#define ENUM_TRADSESEVENT_END_OF_DAY_SERVICE             104
#define ENUM_TRADSESEVENT_SERVICE_RESUMED                105
#define ENUM_TRADSESMODE_SIMULATED                       2
#define ENUM_TRADSESMODE_PRODUCTION                      3
#define ENUM_TRADEMANAGERSTATUS_UNAVAILABLE              0
#define ENUM_TRADEMANAGERSTATUS_AVAILABLE                1
#define ENUM_TRADEREPORTTYPE_SUBMIT                      0
#define ENUM_TRADEREPORTTYPE_ALLEGED                     1
#define ENUM_TRADEREPORTTYPE_ACCEPT                      2
#define ENUM_TRADEREPORTTYPE_DECLINE                     3
#define ENUM_TRADEREPORTTYPE_NO_WAS_REPLACED             5
#define ENUM_TRADEREPORTTYPE_TRADE_REPORT_CANCEL         6
#define ENUM_TRADEREPORTTYPE_TRADE_BREAK                 7
#define ENUM_TRADEREPORTTYPE_ALLEGED_NEW                 11
#define ENUM_TRADEREPORTTYPE_ALLEGED_NO_WAS              13
#define ENUM_TRADINGCAPACITY_CUSTOMER                    1
#define ENUM_TRADINGCAPACITY_BROKER_DEALER               3
#define ENUM_TRADINGCAPACITY_PRINCIPAL                   5
#define ENUM_TRADINGCAPACITY_MARKET_MAKER                6
#define ENUM_TRADINGCAPACITY_RISKLESS_PRINCIPAL          9
#define ENUM_TRADINGSESSIONSUBID_PRE_TRADING             1
#define ENUM_TRADINGSESSIONSUBID_OPENING_AUCTION         2
#define ENUM_TRADINGSESSIONSUBID_CLOSING_AUCTION         4
#define ENUM_TRADINGSESSIONSUBID_POST_TRADING            5
#define ENUM_TRADINGSESSIONSUBID_INTRADAY_AUCTION        6
#define ENUM_TRADINGSESSIONSUBID_ANY_AUCTION             8
#define ENUM_TRADINGSESSIONSUBID_SPECIAL_AUCTION         105
#define ENUM_TRANSFERREASON_OWNER                        1
#define ENUM_TRANSFERREASON_CLEARER                      2
#define ENUM_TRIGGERED_NOT_TRIGGERED                     0
#define ENUM_TRIGGERED_TRIGGERED_STOP                    1
#define LEN_USERREFERENCETEXT                            20
#define LEN_VARTEXT                                      2000
#define ENUM_VIOLATIONSTATUS_ACTIVE                      0
#define ENUM_VIOLATIONSTATUS_INACTIVE                    1

/*
 * End of DEPRECATED defines
 */

#if defined(__cplusplus) || defined(c_plusplus)
} /* close scope of 'extern "C"' declaration. */
} /* close namespace Derivatives */
#endif

#endif
