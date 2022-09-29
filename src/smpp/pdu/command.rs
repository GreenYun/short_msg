// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

//! Defines the various Operation PDUs that make up the SMPP protocol. The
//! Operations are described in 6 categories: Session Management ([`session`]),
//! Message Submission ([`submit`]), Message Delivery ([`delivery`]), Message
//! Broadcast, Anciliary Submission and Anciliary Broadcast operations.

#[derive(Clone, Debug)]
pub enum Id {
    GenericNack,         /* = 0x80000000 */
    BindReceiver,        /* = 0x00000001 */
    BindReceiverResp,    /* = 0x80000001 */
    BindTransmitter,     /* = 0x00000002 */
    BindTransmitterResp, /* = 0x80000002 */
    QuerySm,             /* = 0x00000003 */
    QuerySmResp,         /* = 0x80000003 */
    SubmitSm,            /* = 0x00000004 */
    SubmitSmResp,        /* = 0x80000004 */
    DeliverSm,           /* = 0x00000005 */
    DeliverSmResp,       /* = 0x80000005 */
    Unbind,              /* = 0x00000006 */
    UnbindResp,          /* = 0x80000006 */
    ReplaceSm,           /* = 0x00000007 */
    ReplaceSmResp,       /* = 0x80000007 */
    CancelSm,            /* = 0x00000008 */
    CancelSmResp,        /* = 0x80000008 */
    BindTransceiver,     /* = 0x00000009 */
    BindTransceiverResp, /* = 0x80000009 */
    // Reserved             = 0x0000000A, 0x8000000A
    Outbind, /* = 0x0000000B */
    // Reserved             = 0x0000000C-0x00000014,
    //                        0x8000000C-0x80000014
    EnquireLink,     /* = 0x00000015 */
    EnquireLinkResp, /* = 0x80000015 */
    // Reserved             = 0x00000016-0x00000020,
    //                        0x80000016-0x80000020
    SubmitMulti,     /* = 0x00000021 */
    SubmitMultiResp, /* = 0x80000021 */
    // Reserved             = 0x00000022-0x000000FF,
    //                        0x80000022-0x800000FF
    // Reserved             = 0x00000100, 0x80000100
    // Reserved             = 0x00000101, 0x80000101
    AlertNotification, /* = 0x00000102 */
    DataSm,            /* = 0x00000103 */
    DataSmResp,        /* = 0x80000103 */
    #[cfg(feature = "v5")]
    BroadcastSm, /* = 0x00000111 */
    #[cfg(feature = "v5")]
    BroadcastSmResp, /* = 0x80000111 */
    #[cfg(feature = "v5")]
    QueryBroadcastSm, /* = 0x00000112 */
    #[cfg(feature = "v5")]
    QueryBroadcastSmResp, /* = 0x80000112 */
    #[cfg(feature = "v5")]
    CancelBroadcastSm, /* = 0x00000113 */
    #[cfg(feature = "v5")]
    CancelBroadcastSmResp, /* = 0x80000113 */
    // Reseved for SMPP extension (v3)
    //                    = 0x00000104-0x0000FFFF,
    //                      0x80000104-0x8000FFFF
    // Reserved           = 0x00010000-0x000101FF,
    //                      0x80010000-0x800101FF
    // Reseved for SMSC (v5: MC) Vendor
    //                    = 0x00010200-0x000102FF,
    //                      0x80010200-0x800102FF
    // Reserved           = 0x00010300-0x7FFFFFFF,
    //                      0x80010300-0x8FFFFFFF
    Reserved(u32),
}

impl bincode::Decode for Id {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u32::decode(decoder)?;

        Ok(match u {
            0x80000000 => Self::GenericNack,
            0x00000001 => Self::BindReceiver,
            0x80000001 => Self::BindReceiverResp,
            0x00000002 => Self::BindTransmitter,
            0x80000002 => Self::BindTransmitterResp,
            0x00000003 => Self::QuerySm,
            0x80000003 => Self::QuerySmResp,
            0x00000004 => Self::SubmitSm,
            0x80000004 => Self::SubmitSmResp,
            0x00000005 => Self::DeliverSm,
            0x80000005 => Self::DeliverSmResp,
            0x00000006 => Self::Unbind,
            0x80000006 => Self::UnbindResp,
            0x00000007 => Self::ReplaceSm,
            0x80000007 => Self::ReplaceSmResp,
            0x00000008 => Self::CancelSm,
            0x80000008 => Self::CancelSmResp,
            0x00000009 => Self::BindTransceiver,
            0x80000009 => Self::BindTransceiverResp,
            0x0000000B => Self::Outbind,
            0x00000015 => Self::EnquireLink,
            0x80000015 => Self::EnquireLinkResp,
            0x00000021 => Self::SubmitMulti,
            0x80000021 => Self::SubmitMultiResp,
            0x00000102 => Self::AlertNotification,
            0x00000103 => Self::DataSm,
            0x80000103 => Self::DataSmResp,
            #[cfg(feature = "v5")]
            0x00000111 => Self::BroadcastSm,
            #[cfg(feature = "v5")]
            0x80000111 => Self::BroadcastSmResp,
            #[cfg(feature = "v5")]
            0x00000112 => Self::QueryBroadcastSm,
            #[cfg(feature = "v5")]
            0x80000112 => Self::QueryBroadcastSmResp,
            #[cfg(feature = "v5")]
            0x00000113 => Self::CancelBroadcastSm,
            #[cfg(feature = "v5")]
            0x80000113 => Self::CancelBroadcastSmResp,
            x => Self::Reserved(x),
        })
    }
}

impl bincode::Encode for Id {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u = match self {
            Self::GenericNack => 0x80000000u32,
            Self::BindReceiver => 0x00000001,
            Self::BindReceiverResp => 0x80000001,
            Self::BindTransmitter => 0x00000002,
            Self::BindTransmitterResp => 0x80000002,
            Self::QuerySm => 0x00000003,
            Self::QuerySmResp => 0x80000003,
            Self::SubmitSm => 0x00000004,
            Self::SubmitSmResp => 0x80000004,
            Self::DeliverSm => 0x00000005,
            Self::DeliverSmResp => 0x80000005,
            Self::Unbind => 0x00000006,
            Self::UnbindResp => 0x80000006,
            Self::ReplaceSm => 0x00000007,
            Self::ReplaceSmResp => 0x80000007,
            Self::CancelSm => 0x00000008,
            Self::CancelSmResp => 0x80000008,
            Self::BindTransceiver => 0x00000009,
            Self::BindTransceiverResp => 0x80000009,
            Self::Outbind => 0x0000000B,
            Self::EnquireLink => 0x00000015,
            Self::EnquireLinkResp => 0x80000015,
            Self::SubmitMulti => 0x00000021,
            Self::SubmitMultiResp => 0x80000021,
            Self::AlertNotification => 0x00000102,
            Self::DataSm => 0x00000103,
            Self::DataSmResp => 0x80000103,
            #[cfg(feature = "v5")]
            Self::BroadcastSm => 0x00000111,
            #[cfg(feature = "v5")]
            Self::BroadcastSmResp => 0x80000111,
            #[cfg(feature = "v5")]
            Self::QueryBroadcastSm => 0x00000112,
            #[cfg(feature = "v5")]
            Self::QueryBroadcastSmResp => 0x80000112,
            #[cfg(feature = "v5")]
            Self::CancelBroadcastSm => 0x00000113,
            #[cfg(feature = "v5")]
            Self::CancelBroadcastSmResp => 0x80000113,
            Self::Reserved(x) => *x,
        };

        u.encode(encoder)
    }
}

/// The SMPP Error status codes are returned by the SMSC (v5: MC) in the
/// *command_status* field of the SMPP message header and in the
/// *error_status_code* field of a ***submit_multi_resp*** message.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
#[repr(u32)]
pub enum Status {
    /// No Error.
    ///
    /// Specified in a response PDU to indicate the success of the corresponding
    /// request PDU.
    ESME_ROK, /* = 0x00000000 */
    /// Message Length is invalid.
    ///
    /// short_message field or message_payload TLV has an invalid length
    /// (usually too long for the given MC or underlying network
    /// technology).
    ESME_RINVMSGLEN, /* = 0x00000001 */
    /// Command Length is invalid.
    ///
    /// PDU length is considered invalid, either because the value is too short
    /// or too large for the given PDU.
    ESME_RINVCMDLEN, /* = 0x00000002 */
    /// Invalid Command ID.
    ///
    /// Command ID is not recognised, either because the operation is not
    /// supported or unknown.
    ESME_RINVCMDID, /* = 0x00000003 */
    /// Incorrect BIND Status for given command.
    ///
    /// PDU has been sent in the wrong session state. E.g. sending a
    /// ***submit_sm*** without first establishing a Bound_TX session state.
    ESME_RINVBNDSTS, /* = 0x00000004 */
    /// ESME Already in Bound State.
    ///
    /// A bind request has been issued within a session that is already bound.
    ESME_RALYBND, /* = 0x00000005 */
    /// Invalid Priority Flag.
    ///
    /// Priority flag contains an illegal or unsupported value.
    ESME_RINVPRTFLG, /* = 0x00000006 */
    /// Invalid Registered Delivery Flag.
    ///
    /// Registered field contains an invalid setting.
    ESME_RINVREGDLVFLG, /* = 0x00000007 */
    /// System Error.
    ///
    /// MC system error indicating that all or part of the MC is currently
    /// unavailable. This can be returned in any response PDU.
    ESME_RSYSERR, /* = 0x00000008 */
    // Reserved              = 0x00000009
    /// Invalid Source Address.
    ///
    /// Source address of message is considered invalid. Usually this is
    /// because the field is either too long or contains invalid characters.
    ESME_RINVSRCADR, /* = 0x0000000A */
    /// Invalid Destination Address.
    ///
    /// Destination address of message is considered invalid. Usually this is
    /// because the field is either zero length, too long or contains invalid
    /// characters.
    ESME_RINVDSTADR, /* = 0x0000000B */
    /// Message ID is invalid.
    ///
    /// Message ID specified in ***cancel_sm***, ***query_sm*** or other
    /// operations is invalid.
    ESME_RINVMSGID, /* = 0x0000000C */
    /// Bind Failed.
    ///
    /// A generic failure scenario for a bind attempt. This may be due to a
    /// provisioning error, incorrect password or other reason. A SMSC (v5: MC)
    /// will typically return this error for an invalid *system_id*,
    /// *system_type*, *password* or other attribute that may cause a bind
    /// failure.
    ESME_RBINDFAIL, /* = 0x0000000D */
    /// Invalid Password.
    ///
    /// Password field in bind PDU is invalid. This is usually returned when the
    /// length is too short or too long. It is not supposed to be returned when
    /// the ESME has specified the incorrect password.
    ESME_RINVPASWD, /* = 0x0000000E */
    /// Invalid System ID.
    ///
    /// The System ID field in bind PDU is invalid. This is usually returned
    /// when the length is too short or too long. It is not supposed to be
    /// returned when the ESME has specified the incorrect system id.
    ESME_RINVSYSID, /* = 0x0000000F */
    // Reserved              = 0x00000010
    /// Cancel SM Failed.
    ///
    /// Generic failure error for ***cancel_sm*** operation.
    ESME_RCANCELFAIL, /* = 0x00000011 */
    // Reserved              = 0x00000012
    /// Replace SM Failed.
    ///
    /// Generic failure for ***replace_sm*** operation.
    ESME_RREPLACEFAIL, /* = 0x00000013 */
    /// Message Queue Full.
    ///
    /// Used to indicate a resource error within the MC. This may be interpreted
    /// as the maximum number of messages addressed to a single destination
    /// or a global maximum of undelivered messages within the MC.
    ESME_RMSGQFUL, /* = 0x00000014 */
    /// Invalid Service Type.
    ///
    /// Service type is rejected either because it is not recognised by the MC
    /// or because its length is not within the defined range.
    ESME_RINVSERTYP, /* = 0x00000015 */
    // Reserved              = 0x00000016-0x00000032
    /// Invalid number of destinations.
    ///
    /// The number_of_dests field in the ***submit_multi*** PDU is invalid.
    ESME_RINVNUMDESTS, /* = 0x00000033 */
    /// Invalid Distribution List name.
    ///
    /// The *dl_name* field specified in the ***submit_multi*** PDU is either
    /// invalid, or non-existent.
    ESME_RINVDLNAME, /* = 0x00000034 */
    // Reserved              = 0x00000035-0x0000003F
    /// Destination flag is invalid (submit_multi).
    ///
    /// The *dest_flag* field in the ***submit_multi*** PDU has been encoded
    /// with an invalid setting.
    ESME_RINVDESTFLAG, /* = 0x00000040 */
    // Reserved              = 0x00000041,
    /// Submit w/replace functionality has been requested where it is either
    /// unsupported or inappropriate for the particular SMSC (v5: MC). This can
    /// typically occur with ***submit_multi*** where the context of "replace if
    /// present" is often a best effort operation and SMSCs (v5: MCs) may not
    /// support the feature in ***submit_multi***.
    ///
    /// Another reason for returning this error would be where the feature has
    /// been denied to an ESME.
    ESME_RINVSUBREP, /* = 0x00000042 */
    /// Invalid *esm_class* field data.
    ///
    /// The *esm_class* field has an unsupported setting.
    ESME_RINVESMCLASS, /* = 0x00000043 */
    /// Cannot Submit to Distribution List.
    ///
    /// Distribution lists are not supported, are denied or unavailable.
    ESME_RCNTSUBDL, /* = 0x00000044 */
    /// ***submit_sm***, ***data_sm*** or ***submit_multi*** failed.
    ///
    /// Generic failure error for submission operations.
    ESME_RSUBMITFAIL, /* = 0x00000045 */
    // Reserved              = 0x00000046-0x00000047
    /// Invalid Source address TON.
    ///
    /// The source TON of the message is either invalid or unsupported.
    ESME_RINVSRCTON, /* = 0x00000048 */
    /// Invalid Source address NPI.
    ///
    /// The source NPI of the message is either invalid or unsupported.
    ESME_RINVSRCNPI, /* = 0x00000049 */
    /// Invalid Destination address TON.
    ///
    /// The destination TON of the message is either invalid or unsupported.
    ESME_RINVDSTTON, /* = 0x00000050 */
    /// Invalid Destination address NPI.
    ///
    /// The destination NPI of the message is either invalid or unsupported.
    ESME_RINVDSTNPI, /* = 0x00000051 */
    // Reserved              = 0x00000052
    /// Invalid *system_type* field.
    ///
    /// The System type of bind PDU has an incorrect length or contains illegal
    /// characters.
    ESME_RINVSYSTYP, /* = 0x00000053 */
    /// Invalid *replace_if_present* flag.
    ///
    /// The *replace_if_present* flag has been encoded with an invalid or
    /// unsupported setting.
    ESME_RINVREPFLAG, /* = 0x00000054 */
    /// Invalid number of messages.
    ESME_RINVNUMMSGS, /* = 0x00000055 */
    // Reserved              = 0x00000056-0x00000057,
    /// Throttling error (ESME has exceeded allowed message limits).
    ///
    /// This type of error is usually returned where an ESME has exceeded a
    /// predefined messaging rate restriction applied by the operator.
    ESME_RTHROTTLED, /* = 0x00000058 */
    // Reserved              = 0x00000059-0x00000060,
    /// Invalid Scheduled Delivery Time.
    ///
    /// Scheduled delivery time is either the incorrect length or is invalid.
    ESME_RINVSCHED, /* = 0x00000061 */
    /// Invalid message validity period (Expiry time).
    ///
    /// Expiry time is either the incorrect length or is invalid.
    ESME_RINVEXPIRY, /* = 0x00000062 */
    /// Predefined Message ID is Invalid or specified predefined message was not
    /// found.
    ///
    /// The default (pre-defined) message id is either invalid or refers to a
    /// non-existent pre-defined message.
    ESME_RINVDFTMSGID, /* = 0x00000063 */
    /// ESME Receiver Temporary App Error Code.
    ///
    /// Rx or Trx ESME is unable to process a delivery due to a temporary
    /// problem and is requesting that the message be retried at some future
    /// point.
    ESME_RX_T_APPN, /* = 0x00000064 */
    /// ESME Receiver Permanent App Error Code.
    ///
    /// Rx or Trx ESME is unable to process a delivery due to a permanent
    /// problem relating to the given destination address and is requesting that
    /// the message and all other messages queued to the same destination
    /// should NOT be retried any further.
    ESME_RX_P_APPN, /* = 0x00000065 */
    /// ESME Receiver Reject Message Error Code.
    ///
    /// Rx or Trx ESME is unable to process a delivery due to a problem relating
    /// to the given message and is requesting that the message is rejected and
    /// not retried. This does not affect other messages queued for the same
    /// ESME or destination address.
    ESME_RX_R_APPN, /* = 0x00000066 */
    /// ***query_sm*** request failed.
    ///
    /// Generic failure scenario for a query request.
    ESME_RQUERYFAIL, /* = 0x00000067 */
    // Reserved              = 0x00000068-0x000000BF
    /// Error in the optional part of the PDU Body.
    ///
    /// Decoding of TLVs (Optional Parameters) has resulted in one of the
    /// following scenarios:
    /// - PDU decoding completed with 1-3 octets of data remaining, indicating a
    ///   corrupt PDU.
    /// - A TLV indicated a length that was not present in the remaining PDU
    ///   data (e.g. a TLV specifying a length of 10 where only 6 octets of PDU
    ///   data remain).
    ESME_RINVOPTPARSTREAM, /* = 0x000000C0 */
    /// TLV not allowed.
    ///
    /// A TLV has been used in an invalid context, either inappropriate or
    /// deliberately rejected by the operator.
    ESME_ROPTPARNOTALLWD, /* = 0x000000C1 */
    /// Invalid Parameter Length.
    ///
    /// A TLV has specified a length that is considered invalid.
    ESME_RINVPARLEN, /* = 0x000000C2 */
    /// Expected TLV missing.
    ///
    /// A mandatory TLV such as the message_payload TLV within a ***data_sm***
    /// PDU is missing.
    ESME_RMISSINGOPTPARAM, /* = 0x000000C3 */
    /// Invalid TLV Value.
    ///
    /// The data content of a TLV is invalid and cannot be decoded.
    ESME_RINVOPTPARAMVAL, /* = 0x000000C4 */
    // Reserved              = 0x000000C5-0x000000FD
    /// Transaction Delivery Failure.
    ///
    /// A data_sm or submit_sm operation issued in transaction mode has resulted
    /// in a failed delivery.
    #[cfg(feature = "v5")]
    ESME_RDELIVERYFAILURE, /* = 0x000000FE */
    /// Unknown Error.
    ///
    /// Some unexpected error has occurred.
    #[cfg(feature = "v5")]
    ESME_RUNKNOWNERR, /* = 0x000000FF */
    /// ESME Not authorised to use specified *service_type*.
    ///
    /// Specific service_type has been denied for use by the given ESME.
    #[cfg(feature = "v5")]
    ESME_RSERTYPUNAUTH, /* = 0x00000100 */
    /// ESME Prohibited from using specified operation.
    ///
    /// The PDU request was recognised but is denied to the ESME.
    #[cfg(feature = "v5")]
    ESME_RPROHIBITED, /* = 0x00000101 */
    /// Specified *service_type* is unavailable.
    ///
    /// Due to a service outage within the MC, a service is unavailable.
    #[cfg(feature = "v5")]
    ESME_RSERTYPUNAVAIL, /* = 0x00000102 */
    /// Specified *service_typ* is denied.
    ///
    /// Due to inappropriate message content wrt. the selected *service_type*.
    #[cfg(feature = "v5")]
    ESME_RSERTYPDENIED, /* = 0x00000103 */
    /// Invalid Data Coding Scheme.
    ///
    /// Specified DCS is invalid or MC does not support it.
    #[cfg(feature = "v5")]
    ESME_RINVDCS, /* = 0x00000104 */
    /// Source Address Sub unit is Invalid.
    #[cfg(feature = "v5")]
    ESME_RINVSRCADDRSUBUNIT, /* = 0x00000105 */
    /// Destination Address Sub unit is Invalid.
    #[cfg(feature = "v5")]
    ESME_RINVDSTADDRSUBUNIT, /* = 0x00000106 */
    /// Broadcast Frequency Interval is invalid.
    ///
    /// Specified value is either invalid or not supported.
    #[cfg(feature = "v5")]
    ESME_RINVBCASTFREQINT, /* = 0x00000107 */
    /// Broadcast Alias Name is invalid.
    ///
    /// Specified value has an incorrect length or contains invalid/unsupported
    /// characters.
    #[cfg(feature = "v5")]
    ESME_RINVBCASTALIAS_NAME, /* = 0x00000108 */
    /// Broadcast Area Format is invalid.
    ///
    /// Specified value violates protocol or is unsupported.
    #[cfg(feature = "v5")]
    ESME_RINVBCASTAREAFMT, /* = 0x00000109 */
    /// Number of Broadcast Areas is invalid.
    ///
    /// Specified value violates protocol or is unsupported.
    #[cfg(feature = "v5")]
    ESME_RINVNUMBCAST_AREAS, /* = 0x0000010A */
    /// Broadcast Content Type is invalid.
    ///
    /// Specified value violates protocol or is unsupported.
    #[cfg(feature = "v5")]
    ESME_RINVBCASTCNTTYPE, /* = 0x0000010B */
    /// Broadcast Message Class is invalid.
    ///
    /// Specified value violates protocol or is unsupported.
    #[cfg(feature = "v5")]
    ESME_RINVBCASTMSGCLASS, /* = 0x0000010C */
    /// ***broadcast_sm*** operation failed.
    #[cfg(feature = "v5")]
    ESME_RBCASTFAIL, /* = 0x0000010D */
    /// ***query_broadcast_sm*** operation failed.
    #[cfg(feature = "v5")]
    ESME_RBCASTQUERYFAIL, /* = 0x0000010E */
    /// ***cancel_broadcast_sm*** operation failed.
    #[cfg(feature = "v5")]
    ESME_RBCASTCANCELFAIL, /* = 0x0000010F */
    /// Number of Repeated Broadcasts is invalid.
    ///
    /// Specified value violates protocol or is unsupported.
    #[cfg(feature = "v5")]
    ESME_RINVBCAST_REP, /* = 0x00000110 */
    /// Broadcast Service Group is invalid.
    ///
    /// Specified value violates protocol or is unsupported.
    #[cfg(feature = "v5")]
    ESME_RINVBCASTSRVGRP, /* = 0x00000111 */
    /// Broadcast Channel Indicator is invalid.
    ///
    /// Specified value violates protocol or is unsupported.
    #[cfg(feature = "v5")]
    ESME_RINVBCASTCHANIND, /* = 0x00000112 */
    // Reserved for SMPP extension
    //                       = 0x00000100-0x000003FF
    // Reserved for SMSC (v5: MC) vendor specific errors
    //                       = 0x00000400-0x000004FF
    // Reserved              = 0x00000500-0xFFFFFFFF,
    Reserved(u32),
}

impl bincode::Decode for Status {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u32::decode(decoder)?;

        Ok(match u {
            0x00000000 => Self::ESME_ROK,
            0x00000001 => Self::ESME_RINVMSGLEN,
            0x00000002 => Self::ESME_RINVCMDLEN,
            0x00000003 => Self::ESME_RINVCMDID,
            0x00000004 => Self::ESME_RINVBNDSTS,
            0x00000005 => Self::ESME_RALYBND,
            0x00000006 => Self::ESME_RINVPRTFLG,
            0x00000007 => Self::ESME_RINVREGDLVFLG,
            0x00000008 => Self::ESME_RSYSERR,
            0x0000000A => Self::ESME_RINVSRCADR,
            0x0000000B => Self::ESME_RINVDSTADR,
            0x0000000C => Self::ESME_RINVMSGID,
            0x0000000D => Self::ESME_RBINDFAIL,
            0x0000000E => Self::ESME_RINVPASWD,
            0x0000000F => Self::ESME_RINVSYSID,
            0x00000011 => Self::ESME_RCANCELFAIL,
            0x00000013 => Self::ESME_RREPLACEFAIL,
            0x00000014 => Self::ESME_RMSGQFUL,
            0x00000015 => Self::ESME_RINVSERTYP,
            0x00000033 => Self::ESME_RINVNUMDESTS,
            0x00000034 => Self::ESME_RINVDLNAME,
            0x00000040 => Self::ESME_RINVDESTFLAG,
            0x00000042 => Self::ESME_RINVSUBREP,
            0x00000043 => Self::ESME_RINVESMCLASS,
            0x00000044 => Self::ESME_RCNTSUBDL,
            0x00000045 => Self::ESME_RSUBMITFAIL,
            0x00000048 => Self::ESME_RINVSRCTON,
            0x00000049 => Self::ESME_RINVSRCNPI,
            0x00000050 => Self::ESME_RINVDSTTON,
            0x00000051 => Self::ESME_RINVDSTNPI,
            0x00000053 => Self::ESME_RINVSYSTYP,
            0x00000054 => Self::ESME_RINVREPFLAG,
            0x00000055 => Self::ESME_RINVNUMMSGS,
            0x00000058 => Self::ESME_RTHROTTLED,
            0x00000061 => Self::ESME_RINVSCHED,
            0x00000062 => Self::ESME_RINVEXPIRY,
            0x00000063 => Self::ESME_RINVDFTMSGID,
            0x00000064 => Self::ESME_RX_T_APPN,
            0x00000065 => Self::ESME_RX_P_APPN,
            0x00000066 => Self::ESME_RX_R_APPN,
            0x00000067 => Self::ESME_RQUERYFAIL,
            0x000000C0 => Self::ESME_RINVOPTPARSTREAM,
            0x000000C1 => Self::ESME_ROPTPARNOTALLWD,
            0x000000C2 => Self::ESME_RINVPARLEN,
            0x000000C3 => Self::ESME_RMISSINGOPTPARAM,
            0x000000C4 => Self::ESME_RINVOPTPARAMVAL,
            #[cfg(feature = "v5")]
            0x000000FE => Self::ESME_RDELIVERYFAILURE,
            #[cfg(feature = "v5")]
            0x000000FF => Self::ESME_RUNKNOWNERR,
            #[cfg(feature = "v5")]
            0x00000100 => Self::ESME_RSERTYPUNAUTH,
            #[cfg(feature = "v5")]
            0x00000101 => Self::ESME_RPROHIBITED,
            #[cfg(feature = "v5")]
            0x00000102 => Self::ESME_RSERTYPUNAVAIL,
            #[cfg(feature = "v5")]
            0x00000103 => Self::ESME_RSERTYPDENIED,
            #[cfg(feature = "v5")]
            0x00000104 => Self::ESME_RINVDCS,
            #[cfg(feature = "v5")]
            0x00000105 => Self::ESME_RINVSRCADDRSUBUNIT,
            #[cfg(feature = "v5")]
            0x00000106 => Self::ESME_RINVDSTADDRSUBUNIT,
            #[cfg(feature = "v5")]
            0x00000107 => Self::ESME_RINVBCASTFREQINT,
            #[cfg(feature = "v5")]
            0x00000108 => Self::ESME_RINVBCASTALIAS_NAME,
            #[cfg(feature = "v5")]
            0x00000109 => Self::ESME_RINVBCASTAREAFMT,
            #[cfg(feature = "v5")]
            0x0000010A => Self::ESME_RINVNUMBCAST_AREAS,
            #[cfg(feature = "v5")]
            0x0000010B => Self::ESME_RINVBCASTCNTTYPE,
            #[cfg(feature = "v5")]
            0x0000010C => Self::ESME_RINVBCASTMSGCLASS,
            #[cfg(feature = "v5")]
            0x0000010D => Self::ESME_RBCASTFAIL,
            #[cfg(feature = "v5")]
            0x0000010E => Self::ESME_RBCASTQUERYFAIL,
            #[cfg(feature = "v5")]
            0x0000010F => Self::ESME_RBCASTCANCELFAIL,
            #[cfg(feature = "v5")]
            0x00000110 => Self::ESME_RINVBCAST_REP,
            #[cfg(feature = "v5")]
            0x00000111 => Self::ESME_RINVBCASTSRVGRP,
            #[cfg(feature = "v5")]
            0x00000112 => Self::ESME_RINVBCASTCHANIND,
            x => Self::Reserved(x),
        })
    }
}

impl bincode::Encode for Status {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u = match self {
            Self::ESME_ROK => 0x00000000,
            Self::ESME_RINVMSGLEN => 0x00000001,
            Self::ESME_RINVCMDLEN => 0x00000002,
            Self::ESME_RINVCMDID => 0x00000003,
            Self::ESME_RINVBNDSTS => 0x00000004,
            Self::ESME_RALYBND => 0x00000005,
            Self::ESME_RINVPRTFLG => 0x00000006,
            Self::ESME_RINVREGDLVFLG => 0x00000007,
            Self::ESME_RSYSERR => 0x00000008,
            Self::ESME_RINVSRCADR => 0x0000000A,
            Self::ESME_RINVDSTADR => 0x0000000B,
            Self::ESME_RINVMSGID => 0x0000000C,
            Self::ESME_RBINDFAIL => 0x0000000D,
            Self::ESME_RINVPASWD => 0x0000000E,
            Self::ESME_RINVSYSID => 0x0000000F,
            Self::ESME_RCANCELFAIL => 0x00000011,
            Self::ESME_RREPLACEFAIL => 0x00000013,
            Self::ESME_RMSGQFUL => 0x00000014,
            Self::ESME_RINVSERTYP => 0x00000015,
            Self::ESME_RINVNUMDESTS => 0x00000033,
            Self::ESME_RINVDLNAME => 0x00000034,
            Self::ESME_RINVDESTFLAG => 0x00000040,
            Self::ESME_RINVSUBREP => 0x00000042,
            Self::ESME_RINVESMCLASS => 0x00000043,
            Self::ESME_RCNTSUBDL => 0x00000044,
            Self::ESME_RSUBMITFAIL => 0x00000045,
            Self::ESME_RINVSRCTON => 0x00000048,
            Self::ESME_RINVSRCNPI => 0x00000049,
            Self::ESME_RINVDSTTON => 0x00000050,
            Self::ESME_RINVDSTNPI => 0x00000051,
            Self::ESME_RINVSYSTYP => 0x00000053,
            Self::ESME_RINVREPFLAG => 0x00000054,
            Self::ESME_RINVNUMMSGS => 0x00000055,
            Self::ESME_RTHROTTLED => 0x00000058,
            Self::ESME_RINVSCHED => 0x00000061,
            Self::ESME_RINVEXPIRY => 0x00000062,
            Self::ESME_RINVDFTMSGID => 0x00000063,
            Self::ESME_RX_T_APPN => 0x00000064,
            Self::ESME_RX_P_APPN => 0x00000065,
            Self::ESME_RX_R_APPN => 0x00000066,
            Self::ESME_RQUERYFAIL => 0x00000067,
            Self::ESME_RINVOPTPARSTREAM => 0x000000C0,
            Self::ESME_ROPTPARNOTALLWD => 0x000000C1,
            Self::ESME_RINVPARLEN => 0x000000C2,
            Self::ESME_RMISSINGOPTPARAM => 0x000000C3,
            Self::ESME_RINVOPTPARAMVAL => 0x000000C4,
            #[cfg(feature = "v5")]
            Self::ESME_RDELIVERYFAILURE => 0x000000FE,
            #[cfg(feature = "v5")]
            Self::ESME_RUNKNOWNERR => 0x000000FF,
            #[cfg(feature = "v5")]
            Self::ESME_RSERTYPUNAUTH => 0x00000100,
            #[cfg(feature = "v5")]
            Self::ESME_RPROHIBITED => 0x00000101,
            #[cfg(feature = "v5")]
            Self::ESME_RSERTYPUNAVAIL => 0x00000102,
            #[cfg(feature = "v5")]
            Self::ESME_RSERTYPDENIED => 0x00000103,
            #[cfg(feature = "v5")]
            Self::ESME_RINVDCS => 0x00000104,
            #[cfg(feature = "v5")]
            Self::ESME_RINVSRCADDRSUBUNIT => 0x00000105,
            #[cfg(feature = "v5")]
            Self::ESME_RINVDSTADDRSUBUNIT => 0x00000106,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCASTFREQINT => 0x00000107,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCASTALIAS_NAME => 0x00000108,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCASTAREAFMT => 0x00000109,
            #[cfg(feature = "v5")]
            Self::ESME_RINVNUMBCAST_AREAS => 0x0000010A,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCASTCNTTYPE => 0x0000010B,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCASTMSGCLASS => 0x0000010C,
            #[cfg(feature = "v5")]
            Self::ESME_RBCASTFAIL => 0x0000010D,
            #[cfg(feature = "v5")]
            Self::ESME_RBCASTQUERYFAIL => 0x0000010E,
            #[cfg(feature = "v5")]
            Self::ESME_RBCASTCANCELFAIL => 0x0000010F,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCAST_REP => 0x00000110,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCASTSRVGRP => 0x00000111,
            #[cfg(feature = "v5")]
            Self::ESME_RINVBCASTCHANIND => 0x00000112,
            Self::Reserved(x) => *x,
        };

        u.encode(encoder)
    }
}

pub use delivery::*;
pub use session::*;
pub use submit::*;

pub mod delivery;
pub mod session;
pub mod submit;
