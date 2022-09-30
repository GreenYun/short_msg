// Copyright (c) 2022 GreenYun Organization
// SPDX-License-Identifier: MIT

/// TLV fields may be optionally included in a SMPP message. TLVs must always
/// appear at the end of a SMPP PDU. However, they may be included in any
/// convenient order and need not be encoded in the order presented in this
/// document.
///
/// For a particular SMPP PDU, the ESME or SMSC (v5: MC) may include some, all
/// or none of the defined TLVs as required for the particular application
/// context. For example a paging system may in a SMPP ***submit_sm***
/// operation, include only the “call-back number” related TLVs.
#[derive(Clone, Debug)]
pub struct TLV {
    pub tag: Tag,
    pub len: u16,
    pub val: Vec<u8>,
}

impl bincode::Decode for TLV {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        use bincode::de::read::Reader;

        let tag = Tag::decode(decoder)?;
        let len = u16::decode(decoder)?;

        let mut val = vec![Default::default(); len.into()];
        decoder.reader().read(&mut val)?;

        Ok(Self { tag, len, val })
    }
}

impl bincode::Encode for TLV {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        use bincode::enc::write::Writer;

        self.tag.encode(encoder)?;
        self.len.encode(encoder)?;
        encoder.writer().write(&self.val)
    }
}

/// SMPP Optional Parameter Tag
#[derive(Clone, Debug)]
#[repr(u16)]
pub enum Tag {
    DestAddrSubunit,          /* = 0x0005 */
    DestNetworkType,          /* = 0x0006 */
    DestBearerType,           /* = 0x0007 */
    DestTelematicsId,         /* = 0x0008 */
    SourceAddrSubunit,        /* = 0x000D */
    SourceNetworkType,        /* = 0x000E */
    SourceBearerType,         /* = 0x000F */
    SourceTelematicsId,       /* = 0x0010 */
    QosTimeToLive,            /* = 0x0017 */
    PayloadType,              /* = 0x0019 */
    AdditionalStatusInfoText, /* = 0x001D */
    ReceiptedMessageId,       /* = 0x001E */
    MsMsgWaitFacilities,      /* = 0x0030 */
    PrivacyIndicator,         /* = 0x0201 */
    SourceSubaddress,         /* = 0x0202 */
    DestSubaddress,           /* = 0x0203 */
    UserMessageReference,     /* = 0x0204 */
    UserResponseCode,         /* = 0x0205 */
    SourcePort,               /* = 0x020A */
    DestPort,                 /* = 0x020B */
    SarMsgRefNum,             /* = 0x020C */
    LanguageIndicator,        /* = 0x020D */
    SarTotalSegments,         /* = 0x020E */
    SarSegmentSeqnum,         /* = 0x020F */
    ScInterfaceVersion,       /* = 0x0210 */
    CallbackNumPresInd,       /* = 0x0302 */
    CallbackNumAtag,          /* = 0x0303 */
    NumberOfMessages,         /* = 0x0304 */
    CallbackNum,              /* = 0x0381 */
    DpfResult,                /* = 0x0420 */
    SetDpf,                   /* = 0x0421 */
    MsAvailabilityStatus,     /* = 0x0422 */
    NetworkErrorCode,         /* = 0x0423 */
    MessagePayload,           /* = 0x0424 */
    DeliveryFailureReason,    /* = 0x0425 */
    MoreMessagesToSend,       /* = 0x0426 */
    MessageState,             /* = 0x0427 */
    #[cfg(feature = "v5")]
    CongestionState, /* = 0x0428 */
    UssdServiceOp,            /* = 0x0501 */
    #[cfg(feature = "v5")]
    BroadcastChannelIndicator, /* = 0x0600 */
    #[cfg(feature = "v5")]
    BroadcastContentType, /* = 0x0601 */
    #[cfg(feature = "v5")]
    BroadcastContentTypeInfo, /* = 0x0602 */
    #[cfg(feature = "v5")]
    BroadcastMessageClass, /* = 0x0603 */
    #[cfg(feature = "v5")]
    BroadcastRepNum, /* = 0x0604 */
    #[cfg(feature = "v5")]
    BroadcastFrequencyInterval, /* = 0x0605 */
    #[cfg(feature = "v5")]
    BroadcastAreaIdentifier, /* = 0x0606 */
    #[cfg(feature = "v5")]
    BroadcastErrorStatus, /* = 0x0607 */
    #[cfg(feature = "v5")]
    BroadcastAreaSuccess, /* = 0x0608 */
    #[cfg(feature = "v5")]
    BroadcastEndTime, /* = 0x0609 */
    #[cfg(feature = "v5")]
    BroadcastServiceGroup, /* = 0x060A */
    #[cfg(feature = "v5")]
    BillingIdentification, /* = 0x060B */
    #[cfg(feature = "v5")]
    SourceNetworkId, /* = 0x060D */
    #[cfg(feature = "v5")]
    DestNetworkId, /* = 0x060E */
    #[cfg(feature = "v5")]
    SourceNodeId, /* = 0x060F */
    #[cfg(feature = "v5")]
    DestNodeId, /* = 0x0610 */
    #[cfg(feature = "v5")]
    DestAddrNpResolution, /* = 0x0611 */
    #[cfg(feature = "v5")]
    DestAddrNpInformation, /* = 0x0612 */
    #[cfg(feature = "v5")]
    DestAddrNpCountry, /* = 0x0613 */
    DisplayTime,              /* = 0x1201 */
    SmsSignal,                /* = 0x1203 */
    MsValidity,               /* = 0x1204 */
    AlertOnMessageDelivery,   /* = 0x130C */
    ItsReplyType,             /* = 0x1380 */
    ItsSessionInfo,           /* = 0x1383 */
    UnknownTag(u16),
}

impl bincode::Decode for Tag {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u16::decode(decoder)?;

        Ok(match u {
            0x0005 => Self::DestAddrSubunit,
            0x0006 => Self::DestNetworkType,
            0x0007 => Self::DestBearerType,
            0x0008 => Self::DestTelematicsId,
            0x000D => Self::SourceAddrSubunit,
            0x000E => Self::SourceNetworkType,
            0x000F => Self::SourceBearerType,
            0x0010 => Self::SourceTelematicsId,
            0x0017 => Self::QosTimeToLive,
            0x0019 => Self::PayloadType,
            0x001D => Self::AdditionalStatusInfoText,
            0x001E => Self::ReceiptedMessageId,
            0x0030 => Self::MsMsgWaitFacilities,
            0x0201 => Self::PrivacyIndicator,
            0x0202 => Self::SourceSubaddress,
            0x0203 => Self::DestSubaddress,
            0x0204 => Self::UserMessageReference,
            0x0205 => Self::UserResponseCode,
            0x020A => Self::SourcePort,
            0x020B => Self::DestPort,
            0x020C => Self::SarMsgRefNum,
            0x020D => Self::LanguageIndicator,
            0x020E => Self::SarTotalSegments,
            0x020F => Self::SarSegmentSeqnum,
            0x0210 => Self::ScInterfaceVersion,
            0x0302 => Self::CallbackNumPresInd,
            0x0303 => Self::CallbackNumAtag,
            0x0304 => Self::NumberOfMessages,
            0x0381 => Self::CallbackNum,
            0x0420 => Self::DpfResult,
            0x0421 => Self::SetDpf,
            0x0422 => Self::MsAvailabilityStatus,
            0x0423 => Self::NetworkErrorCode,
            0x0424 => Self::MessagePayload,
            0x0425 => Self::DeliveryFailureReason,
            0x0426 => Self::MoreMessagesToSend,
            0x0427 => Self::MessageState,
            #[cfg(feature = "v5")]
            0x0428 => Self::CongestionState,
            0x0501 => Self::UssdServiceOp,
            #[cfg(feature = "v5")]
            0x0600 => Self::BroadcastChannelIndicator,
            #[cfg(feature = "v5")]
            0x0601 => Self::BroadcastContentType,
            #[cfg(feature = "v5")]
            0x0602 => Self::BroadcastContentTypeInfo,
            #[cfg(feature = "v5")]
            0x0603 => Self::BroadcastMessageClass,
            #[cfg(feature = "v5")]
            0x0604 => Self::BroadcastRepNum,
            #[cfg(feature = "v5")]
            0x0605 => Self::BroadcastFrequencyInterval,
            #[cfg(feature = "v5")]
            0x0606 => Self::BroadcastAreaIdentifier,
            #[cfg(feature = "v5")]
            0x0607 => Self::BroadcastErrorStatus,
            #[cfg(feature = "v5")]
            0x0608 => Self::BroadcastAreaSuccess,
            #[cfg(feature = "v5")]
            0x0609 => Self::BroadcastEndTime,
            #[cfg(feature = "v5")]
            0x060A => Self::BroadcastServiceGroup,
            #[cfg(feature = "v5")]
            0x060B => Self::BillingIdentification,
            #[cfg(feature = "v5")]
            0x060D => Self::SourceNetworkId,
            #[cfg(feature = "v5")]
            0x060E => Self::DestNetworkId,
            #[cfg(feature = "v5")]
            0x060F => Self::SourceNodeId,
            #[cfg(feature = "v5")]
            0x0610 => Self::DestNodeId,
            #[cfg(feature = "v5")]
            0x0611 => Self::DestAddrNpResolution,
            #[cfg(feature = "v5")]
            0x0612 => Self::DestAddrNpInformation,
            #[cfg(feature = "v5")]
            0x0613 => Self::DestAddrNpCountry,
            0x1201 => Self::DisplayTime,
            0x1203 => Self::SmsSignal,
            0x1204 => Self::MsValidity,
            0x130C => Self::AlertOnMessageDelivery,
            0x1380 => Self::ItsReplyType,
            0x1383 => Self::ItsSessionInfo,
            x => Self::UnknownTag(x),
        })
    }
}

impl bincode::Encode for Tag {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u = match self {
            Self::DestAddrSubunit => 0x0005,
            Self::DestNetworkType => 0x0006,
            Self::DestBearerType => 0x0007,
            Self::DestTelematicsId => 0x0008,
            Self::SourceAddrSubunit => 0x000D,
            Self::SourceNetworkType => 0x000E,
            Self::SourceBearerType => 0x000F,
            Self::SourceTelematicsId => 0x0010,
            Self::QosTimeToLive => 0x0017,
            Self::PayloadType => 0x0019,
            Self::AdditionalStatusInfoText => 0x001D,
            Self::ReceiptedMessageId => 0x001E,
            Self::MsMsgWaitFacilities => 0x0030,
            Self::PrivacyIndicator => 0x0201,
            Self::SourceSubaddress => 0x0202,
            Self::DestSubaddress => 0x0203,
            Self::UserMessageReference => 0x0204,
            Self::UserResponseCode => 0x0205,
            Self::SourcePort => 0x020A,
            Self::DestPort => 0x020B,
            Self::SarMsgRefNum => 0x020C,
            Self::LanguageIndicator => 0x020D,
            Self::SarTotalSegments => 0x020E,
            Self::SarSegmentSeqnum => 0x020F,
            Self::ScInterfaceVersion => 0x0210,
            Self::CallbackNumPresInd => 0x0302,
            Self::CallbackNumAtag => 0x0303,
            Self::NumberOfMessages => 0x0304,
            Self::CallbackNum => 0x0381,
            Self::DpfResult => 0x0420,
            Self::SetDpf => 0x0421,
            Self::MsAvailabilityStatus => 0x0422,
            Self::NetworkErrorCode => 0x0423,
            Self::MessagePayload => 0x0424,
            Self::DeliveryFailureReason => 0x0425,
            Self::MoreMessagesToSend => 0x0426,
            Self::MessageState => 0x0427,
            #[cfg(feature = "v5")]
            Self::CongestionState => 0x0428,
            Self::UssdServiceOp => 0x0501,
            #[cfg(feature = "v5")]
            Self::BroadcastChannelIndicator => 0x0600,
            #[cfg(feature = "v5")]
            Self::BroadcastContentType => 0x0601,
            #[cfg(feature = "v5")]
            Self::BroadcastContentTypeInfo => 0x0602,
            #[cfg(feature = "v5")]
            Self::BroadcastMessageClass => 0x0603,
            #[cfg(feature = "v5")]
            Self::BroadcastRepNum => 0x0604,
            #[cfg(feature = "v5")]
            Self::BroadcastFrequencyInterval => 0x0605,
            #[cfg(feature = "v5")]
            Self::BroadcastAreaIdentifier => 0x0606,
            #[cfg(feature = "v5")]
            Self::BroadcastErrorStatus => 0x0607,
            #[cfg(feature = "v5")]
            Self::BroadcastAreaSuccess => 0x0608,
            #[cfg(feature = "v5")]
            Self::BroadcastEndTime => 0x0609,
            #[cfg(feature = "v5")]
            Self::BroadcastServiceGroup => 0x060A,
            #[cfg(feature = "v5")]
            Self::BillingIdentification => 0x060B,
            #[cfg(feature = "v5")]
            Self::SourceNetworkId => 0x060D,
            #[cfg(feature = "v5")]
            Self::DestNetworkId => 0x060E,
            #[cfg(feature = "v5")]
            Self::SourceNodeId => 0x060F,
            #[cfg(feature = "v5")]
            Self::DestNodeId => 0x0610,
            #[cfg(feature = "v5")]
            Self::DestAddrNpResolution => 0x0611,
            #[cfg(feature = "v5")]
            Self::DestAddrNpInformation => 0x0612,
            #[cfg(feature = "v5")]
            Self::DestAddrNpCountry => 0x0613,
            Self::DisplayTime => 0x1201,
            Self::SmsSignal => 0x1203,
            Self::MsValidity => 0x1204,
            Self::AlertOnMessageDelivery => 0x130C,
            Self::ItsReplyType => 0x1380,
            Self::ItsSessionInfo => 0x1383,
            Self::UnknownTag(x) => *x,
        };

        u.encode(encoder)
    }
}
