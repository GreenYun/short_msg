//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

use bincode::enc::write::Writer;

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
        let tag = Tag::decode(decoder)?;
        let len = u16::decode(decoder)?;

        let mut val = Vec::with_capacity(len.into());

        for _ in 0..len {
            let u = u8::decode(decoder)?;
            val.push(u);
        }

        Ok(Self { tag, len, val })
    }
}

impl bincode::Encode for TLV {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        self.tag.encode(encoder)?;
        self.len.encode(encoder)?;
        encoder.writer().write(&self.val)
    }
}

/// SMPP Optional Parameter Tag
#[derive(Clone, Debug, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum Tag {
    DestAddrSubunit            = 0x0005,
    DestNetworkType            = 0x0006,
    DestBearerType             = 0x0007,
    DestTelematicsId           = 0x0008,
    SourceAddrSubunit          = 0x000D,
    SourceNetworkType          = 0x000E,
    SourceBearerType           = 0x000F,
    SourceTelematicsId         = 0x0010,
    QosTimeToLive              = 0x0017,
    PayloadType                = 0x0019,
    AdditionalStatusInfoText   = 0x001D,
    ReceiptedMessageId         = 0x001E,
    MsMsgWaitFacilities        = 0x0030,
    PrivacyIndicator           = 0x0201,
    SourceSubaddress           = 0x0202,
    DestSubaddress             = 0x0203,
    UserMessageReference       = 0x0204,
    UserResponseCode           = 0x0205,
    SourcePort                 = 0x020A,
    DestPort                   = 0x020B,
    SarMsgRefNum               = 0x020C,
    LanguageIndicator          = 0x020D,
    SarTotalSegments           = 0x020E,
    SarSegmentSeqnum           = 0x020F,
    ScInterfaceVersion         = 0x0210,
    CallbackNumPresInd         = 0x0302,
    CallbackNumAtag            = 0x0303,
    NumberOfMessages           = 0x0304,
    CallbackNum                = 0x0381,
    DpfResult                  = 0x0420,
    SetDpf                     = 0x0421,
    MsAvailabilityStatus       = 0x0422,
    NetworkErrorCode           = 0x0423,
    MessagePayload             = 0x0424,
    DeliveryFailureReason      = 0x0425,
    MoreMessagesToSend         = 0x0426,
    MessageState               = 0x0427,
    #[cfg(feature = "v5")]
    CongestionState            = 0x0428,
    UssdServiceOp              = 0x0501,
    #[cfg(feature = "v5")]
    BroadcastChannelIndicator  = 0x0600,
    #[cfg(feature = "v5")]
    BroadcastContentType       = 0x0601,
    #[cfg(feature = "v5")]
    BroadcastContentTypeInfo   = 0x0602,
    #[cfg(feature = "v5")]
    BroadcastMessageClass      = 0x0603,
    #[cfg(feature = "v5")]
    BroadcastRepNum            = 0x0604,
    #[cfg(feature = "v5")]
    BroadcastFrequencyInterval = 0x0605,
    #[cfg(feature = "v5")]
    BroadcastAreaIdentifier    = 0x0606,
    #[cfg(feature = "v5")]
    BroadcastErrorStatus       = 0x0607,
    #[cfg(feature = "v5")]
    BroadcastAreaSuccess       = 0x0608,
    #[cfg(feature = "v5")]
    BroadcastEndTime           = 0x0609,
    #[cfg(feature = "v5")]
    BroadcastServiceGroup      = 0x060A,
    #[cfg(feature = "v5")]
    BillingIdentification      = 0x060B,
    #[cfg(feature = "v5")]
    SourceNetworkId            = 0x060D,
    #[cfg(feature = "v5")]
    DestNetworkId              = 0x060E,
    #[cfg(feature = "v5")]
    SourceNodeId               = 0x060F,
    #[cfg(feature = "v5")]
    DestNodeId                 = 0x0610,
    #[cfg(feature = "v5")]
    DestAddrNpResolution       = 0x0611,
    #[cfg(feature = "v5")]
    DestAddrNpInformation      = 0x0612,
    #[cfg(feature = "v5")]
    DestAddrNpCountry          = 0x0613,
    DisplayTime                = 0x1201,
    SmsSignal                  = 0x1203,
    MsValidity                 = 0x1204,
    AlertOnMessageDelivery     = 0x130C,
    ItsReplyType               = 0x1380,
    ItsSessionInfo             = 0x1383,
}

impl bincode::Decode for Tag {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let u = u16::decode(decoder)?;
        let r = num_traits::FromPrimitive::from_u16(u);
        match r {
            None => Err(bincode::error::DecodeError::UnexpectedVariant {
                type_name: "Tag",
                allowed: bincode::error::AllowedEnumVariants::Allowed(&[]),
                found: u as u32,
            }),
            Some(t) => Ok(t),
        }
    }
}

impl bincode::Encode for Tag {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let u = *self as u16;

        u.encode(encoder)
    }
}
