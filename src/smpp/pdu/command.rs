//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

//! Defines the various Operation PDUs that make up the SMPP protocol. The
//! Operations are described in 6 categories: Session Management ([`session`]),
//! Message Submission ([`submit`]), Message Delivery ([`delivery`]), Message
//! Broadcast, Anciliary Submission and Anciliary Broadcast operations.

#[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
#[repr(u32)]
pub enum Id {
    GenericNack           = 0x80000000,
    BindReceiver          = 0x00000001,
    BindReceiverResp      = 0x80000001,
    BindTransmitter       = 0x00000002,
    BindTransmitterResp   = 0x80000002,
    QuerySm               = 0x00000003,
    QuerySmResp           = 0x80000003,
    SubmitSm              = 0x00000004,
    SubmitSmResp          = 0x80000004,
    DeliverSm             = 0x00000005,
    DeliverSmResp         = 0x80000005,
    Unbind                = 0x00000006,
    UnbindResp            = 0x80000006,
    ReplaceSm             = 0x00000007,
    ReplaceSmResp         = 0x80000007,
    CancelSm              = 0x00000008,
    CancelSmResp          = 0x80000008,
    BindTransceiver       = 0x00000009,
    BindTransceiverResp   = 0x80000009,
    // Reserved           = 0x0000000A, 0x8000000A
    Outbind               = 0x0000000B,
    // Reserved           = 0x0000000C-0x00000014,
    //                      0x8000000C-0x80000014
    EnquireLink           = 0x00000015,
    EnquireLinkResp       = 0x80000015,
    // Reserved           = 0x00000016-0x00000020,
    //                      0x80000016-0x80000020
    SubmitMulti           = 0x00000021,
    SubmitMultiResp       = 0x80000021,
    // Reserved           = 0x00000022-0x000000FF,
    //                      0x80000022-0x800000FF
    // Reserved           = 0x00000100, 0x80000100
    // Reserved           = 0x00000101, 0x80000101
    AlertNotification     = 0x00000102,
    DataSm                = 0x00000103,
    DataSmResp            = 0x80000103,
    #[cfg(feature = "v5")]
    BroadcastSm           = 0x00000111,
    #[cfg(feature = "v5")]
    BroadcastSmResp       = 0x80000111,
    #[cfg(feature = "v5")]
    QueryBroadcastSm      = 0x00000112,
    #[cfg(feature = "v5")]
    QueryBroadcastSmResp  = 0x80000112,
    #[cfg(feature = "v5")]
    CancelBroadcastSm     = 0x00000113,
    #[cfg(feature = "v5")]
    CancelBroadcastSmResp = 0x80000113,
    // Reseved for SMPP extension (v3)
    //                    = 0x00000104-0x0000FFFF,
    //                      0x80000104-0x8000FFFF
    // Reserved           = 0x00010000-0x000101FF,
    //                      0x80010000-0x800101FF
    // Reseved for SMCC (v5: MC) Vendor
    //                    = 0x00010200-0x000102FF,
    //                      0x80010200-0x800102FF
    // Reserved           = 0x00010300-0x7FFFFFFF,
    //                      0x80010300-0x8FFFFFFF
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
#[repr(u32)]
pub enum Status {
    ESME_ROK                 = 0x00000000,
    ESME_RINVMSGLEN          = 0x00000001,
    ESME_RINVCMDLEN          = 0x00000002,
    ESME_RINVCMDID           = 0x00000003,
    ESME_RINVBNDSTS          = 0x00000004,
    ESME_RALYBND             = 0x00000005,
    ESME_RINVPRTFLG          = 0x00000006,
    ESME_RINVREGDLVFLG       = 0x00000007,
    ESME_RSYSERR             = 0x00000008,
    // Reserved              = 0x00000009
    ESME_RINVSRCADR          = 0x0000000A,
    ESME_RINVDSTADR          = 0x0000000B,
    ESME_RINVMSGID           = 0x0000000C,
    ESME_RBINDFAIL           = 0x0000000D,
    ESME_RINVPASWD           = 0x0000000E,
    ESME_RINVSYSID           = 0x0000000F,
    // Reserved              = 0x00000010
    ESME_RCANCELFAIL         = 0x00000011,
    // Reserved              = 0x00000012
    ESME_RREPLACEFAIL        = 0x00000013,
    ESME_RMSGQFUL            = 0x00000014,
    ESME_RINVSERTYP          = 0x00000015,
    // Reserved              = 0x00000016-0x00000032
    ESME_RINVNUMDESTS        = 0x00000033,
    ESME_RINVDLNAME          = 0x00000034,
    // Reserved              = 0x00000035-0x0000003F
    ESME_RINVDESTFLAG        = 0x00000040,
    // Reserved              = 0x00000041,
    ESME_RINVSUBREP          = 0x00000042,
    ESME_RINVESMCLASS        = 0x00000043,
    ESME_RCNTSUBDL           = 0x00000044,
    ESME_RSUBMITFAIL         = 0x00000045,
    // Reserved              = 0x00000046-0x00000047
    ESME_RINVSRCTON          = 0x00000048,
    ESME_RINVSRCNPI          = 0x00000049,
    ESME_RINVDSTTON          = 0x00000050,
    ESME_RINVDSTNPI          = 0x00000051,
    // Reserved              = 0x00000052
    ESME_RINVSYSTYP          = 0x00000053,
    ESME_RINVREPFLAG         = 0x00000054,
    ESME_RINVNUMMSGS         = 0x00000055,
    // Reserved              = 0x00000056-0x00000057,
    ESME_RTHROTTLED          = 0x00000058,
    // Reserved              = 0x00000059-0x00000060,
    ESME_RINVSCHED           = 0x00000061,
    ESME_RINVEXPIRY          = 0x00000062,
    ESME_RINVDFTMSGID        = 0x00000063,
    ESME_RX_T_APPN           = 0x00000064,
    ESME_RX_P_APPN           = 0x00000065,
    ESME_RX_R_APPN           = 0x00000066,
    ESME_RQUERYFAIL          = 0x00000067,
    // Reserved              = 0x00000068-0x000000BF
    ESME_RINVOPTPARSTREAM    = 0x000000C0,
    ESME_ROPTPARNOTALLWD     = 0x000000C1,
    ESME_RINVPARLEN          = 0x000000C2,
    ESME_RMISSINGOPTPARAM    = 0x000000C3,
    ESME_RINVOPTPARAMVAL     = 0x000000C4,
    // Reserved              = 0x000000C5-0x000000FD
    #[cfg(feature = "v5")]
    ESME_RDELIVERYFAILURE    = 0x000000FE,
    #[cfg(feature = "v5")]
    ESME_RUNKNOWNERR         = 0x000000FF,
    #[cfg(feature = "v5")]
    ESME_RSERTYPUNAUTH       = 0x00000100,
    #[cfg(feature = "v5")]
    ESME_RPROHIBITED         = 0x00000101,
    #[cfg(feature = "v5")]
    ESME_RSERTYPUNAVAIL      = 0x00000102,
    #[cfg(feature = "v5")]
    ESME_RSERTYPDENIED       = 0x00000103,
    #[cfg(feature = "v5")]
    ESME_RINVDCS             = 0x00000104,
    #[cfg(feature = "v5")]
    ESME_RINVSRCADDRSUBUNIT  = 0x00000105,
    #[cfg(feature = "v5")]
    ESME_RINVDSTADDRSUBUNIT  = 0x00000106,
    #[cfg(feature = "v5")]
    ESME_RINVBCASTFREQINT    = 0x00000107,
    #[cfg(feature = "v5")]
    ESME_RINVBCASTALIAS_NAME = 0x00000108,
    #[cfg(feature = "v5")]
    ESME_RINVBCASTAREAFMT    = 0x00000109,
    #[cfg(feature = "v5")]
    ESME_RINVNUMBCAST_AREAS  = 0x0000010A,
    #[cfg(feature = "v5")]
    ESME_RINVBCASTCNTTYPE    = 0x0000010B,
    #[cfg(feature = "v5")]
    ESME_RINVBCASTMSGCLASS   = 0x0000010C,
    #[cfg(feature = "v5")]
    ESME_RBCASTFAIL          = 0x0000010D,
    #[cfg(feature = "v5")]
    ESME_RBCASTQUERYFAIL     = 0x0000010E,
    #[cfg(feature = "v5")]
    ESME_RBCASTCANCELFAIL    = 0x0000010F,
    #[cfg(feature = "v5")]
    ESME_RINVBCAST_REP       = 0x00000110,
    #[cfg(feature = "v5")]
    ESME_RINVBCASTSRVGRP     = 0x00000111,
    #[cfg(feature = "v5")]
    ESME_RINVBCASTCHANIND    = 0x00000112,
    // Reserved for SMPP extension
    //                       = 0x00000100-0x000003FF
    // Reserved for SMSC (v5: MC) vendor specific errors
    //                       = 0x00000400-0x000004FF
    // Reserved              = 0x00000500-0xFFFFFFFF,
}

pub use delivery::*;
pub use session::*;
pub use submit::*;

pub mod delivery;
pub mod session;
pub mod submit;
