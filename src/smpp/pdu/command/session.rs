//Copyright (c) 2022 GreenYun Organization
//SPDX-License-Identifier: MIT

//! These operations are used to establish and maintain a SMPP session.

use crate::smpp::pdu::typedef::{COctet, TLV};

pub use bind::*;

pub mod bind {
    //! The purpose of the SMPP bind operation is to register an instance of an
    //! ESME with the SMSC (v5: MC) system and request a SMPP session over
    //! this network connection for the submission or delivery of messages.
    //! Thus, the Bind operation may be viewed as a form of MC login request
    //! to authenticate the ESME entity wishing to establish a connection.
    //!
    //! As described previously, an ESME may bind to the SMSC (v5: MC) as a
    //! Transmitter (called ESME Transmitter), a Receiver (called ESME
    //! Receiver), or a Transceiver (called ESME Transceiver). There are
    //! three SMPP bind PDUs to support the various modes of operation,
    //! namely ***bind_transmitter***, ***bind_transceiver*** and
    //! ***bind_receiver***. The *command_id* field setting specifies which
    //! PDU is being used.
    //!
    //! An ESME may bind as both a SMPP Transmitter and Receiver using separate
    //! ***bind_transmitter*** and ***bind_receiver*** operations (having first
    //! established two separate network connections). Alternatively an ESME can
    //! also bind as a Transceiver having first established a single network
    //! connection.

    use crate::smpp::pdu::typedef::{COctet, TLV};

    #[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
    pub struct Bind {
        /// Identifies the ESME system requesting to bind as a transceiver with
        /// the SMSC (v5: MC).
        pub system_id: COctet,
        /// The password may be used by the SMSC (v5: MC) to authenticate the
        /// ESME requesting to bind.
        pub password: COctet,
        /// Identifies the type of ESME system requesting to bind as a
        /// transceiver with the SMSC (v5: MC).
        pub system_type: COctet,
        /// Identifies the version of the SMPP protocol supported by the ESME.
        pub interface_version: u8,
        /// Type of Number (TON) for ESME address(es) served via this SMPP
        /// transceiver session.
        ///
        /// Set to NULL (Unknown) if not known.
        pub addr_ton: u8,
        /// Numbering Plan Indicator (NPI) for ESME address(es) served via this
        /// SMPP transceiver session.
        ///
        /// Set to NULL (Unknown) if not known.
        pub addr_npi: u8,
        /// A single ESME address or a range of ESME addresses served via this
        /// SMPP transceiver session. This field may be used by the SMSC
        /// (v5: MC) for authentication, verification or routing
        /// purposes.
        ///
        /// Set to NULL if not known.
        pub address_range: COctet,
    }

    #[derive(Clone, Debug)]
    pub struct BindResp {
        /// SMSC (v5: MC) identifier. Identifies the SMSC (v5: MC) to the ESME.
        pub system_id: COctet,
        ///  SMPP version supported by SMSC (v5: MC).
        pub sc_interface_version: Option<TLV>,
    }

    impl bincode::Decode for BindResp {
        fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
            let system_id = COctet::decode(decoder)?;
            let sc_interface_version = TLV::decode(decoder).ok();

            Ok(Self {
                system_id,
                sc_interface_version,
            })
        }
    }

    impl bincode::Encode for BindResp {
        fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
            self.system_id.encode(encoder)?;
            if let Some(v) = &self.sc_interface_version {
                v.encode(encoder)
            } else {
                Ok(())
            }
        }
    }

    /// This operation is used by the SMSC (v5: MC) to signal an ESME to
    /// originate a ***bind_receiver*** request to the SMSC (v5: MC).
    #[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
    pub struct OutBind {
        /// SMSC (v5: MC) identifier. Identifies the SMSC (v5: MC) to the ESME.
        pub system_id: COctet,
        /// The password may be used by the ESME for security reasons to
        /// authenticate the SMSC (v5: MC) originating the ***outbind***.
        pub password: COctet,
    }

    /// The purpose of the SMPP ***unbind*** operation is to deregister an
    /// instance of an ESME from the SMSC (v5: MC) and inform the SMSC (v5:
    /// MC) that the ESME no longer wishes to use this network connection
    /// for the submission or delivery of messages.
    ///
    /// Thus, the ***unbind*** operation may be viewed as a form of SMSC (v5:
    /// MC) logoff request to close the current SMPP session.
    #[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
    pub struct Unbind {}

    /// The SMPP ***unbind_resp*** PDU is used to reply to an ***unbind***
    /// request. It comprises the SMPP message header only.
    ///
    /// # Note:
    ///
    /// The *command_id* field must include the Command ID value corresponding
    /// to the unbind_resp operation.
    #[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
    pub struct UnbindResp {}

    /// An ESME bound as a Transmitter is authorised to send short messages to
    /// the SMSC (v5: MC) and to receive the corresponding SMPP responses
    /// from the SMSC (v5: MC).
    ///
    /// An ESME indicates its desire not to receive (mobile) originated messages
    /// from other SME’s (e.g. mobile stations) by binding as a Transmitter.
    pub type BindTransmitter = Bind;
    /// The SMPP ***bind_transmitter_resp*** PDU is used to reply to a
    /// ***bind_transmitter*** request.
    ///
    /// # Note:
    ///
    /// The body portion of the SMPP ***bind_transmitter_resp*** PDU is not
    /// returned if the *command_status* field contains a non-zero value;
    /// i.e., if there is an error in the original ***bind_transmitter***
    /// request, the SMSC (v5: MC) *system_id* is not returned.
    pub type BindTransmitterResp = BindResp;

    /// An ESME bound as a Receiver is authorised to receive short messages from
    /// the SMSC (v5: MC) and to return the corresponding SMPP message
    /// responses to the SMSC (v5: MC).
    pub type BindReceiver = Bind;
    /// The SMPP ***bind_receiver_resp*** PDU is used to reply to a
    /// ***bind_receiver*** request.
    ///
    /// # Note:
    ///
    /// The body portion of the SMPP ***bind_receiver_resp*** PDU is not
    /// returned if the *command_status* field contains a non-zero value;
    /// i.e., if there is an error in the original ***bind_receiver***
    /// request, the SMSC (v5: MC) *system_id* is not returned.
    pub type BindReceiverResp = BindResp;

    /// An ESME bound as a Transceiver is allowed to send messages to the SMSC
    /// (v5: MC) and receive messages from the SMSC (v5: MC) over a single
    /// SMPP session.
    pub type BindTransceiver = Bind;
    pub type BindTransceiverResp = BindResp;
}

/// This PDU can be originated by either the ESME or MC and is used to
/// provide a confidence-check of the communication path between an ESME
/// and a MC. On receipt of this request the receiving party should
/// respond with an enquire_link_resp, thus verifying that the
/// application level connection between the MC and the ESME is
/// functioning. The ESME may also respond by sending any valid SMPP
/// primitive.
#[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
pub struct EnquireLink {}

/// The enquire_link_resp PDU is used to reply to an enquire_link request.
#[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
pub struct EnquireLinkResp {}

/// This message is sent by the SMSC to the ESME, when the SMSC has detected
/// that a particular mobile subscriber has become available and a delivery
/// pending flag had been set for that subscriber from a previous data_sm
/// operation. It may be used for example to trigger a data content ‘Push’ to
/// the subscriber from a WAP Proxy Server.
///
/// # Note:
///
/// There is no alert_notification_resp PDU.
#[derive(Clone, Debug)]
pub struct AlertNotification {
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: COctet,
    pub esme_addr_ton: u8,
    pub esme_addr_npi: u8,
    pub esme_addr: COctet,
    pub ms_availability_status: Vec<TLV>,
}

impl bincode::Decode for AlertNotification {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let source_addr_ton = u8::decode(decoder)?;
        let source_addr_npi = u8::decode(decoder)?;
        let source_addr = COctet::decode(decoder)?;
        let esme_addr_ton = u8::decode(decoder)?;
        let esme_addr_npi = u8::decode(decoder)?;
        let esme_addr = COctet::decode(decoder)?;

        let ms_availability_status = {
            let mut v = vec![];
            while let Ok(t) = TLV::decode(decoder) {
                v.push(t);
            }
            v
        };

        Ok(Self {
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            ms_availability_status,
        })
    }
}

impl bincode::Encode for AlertNotification {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        self.source_addr_ton.encode(encoder)?;
        self.source_addr_npi.encode(encoder)?;
        self.source_addr.encode(encoder)?;
        self.esme_addr_ton.encode(encoder)?;
        self.esme_addr_npi.encode(encoder)?;
        self.esme_addr.encode(encoder)?;

        for t in &self.ms_availability_status {
            t.encode(encoder)?;
        }

        Ok(())
    }
}

/// This is a generic negative acknowledgement to an SMPP PDU submitted with an
/// invalid message header. A generic_nack response is returned in the following
/// cases:
///
/// - Invalid *command_length*
///
///   If the receiving SMPP entity, on decoding an SMPP PDU, detects an invalid
///   *command_length* (either too short or too long), it should assume that the
///   data is corrupt. In such cases a ***generic_nack*** PDU must be returned
///   to the message originator.
///
/// - Unknown *command_id*
///
///   If an unknown or invalid *command_id* is received, a ***generic_nack***
///   PDU must also be returned to the originator.
#[derive(Clone, Debug, bincode::Decode, bincode::Encode)]
pub struct GenericNack {}
