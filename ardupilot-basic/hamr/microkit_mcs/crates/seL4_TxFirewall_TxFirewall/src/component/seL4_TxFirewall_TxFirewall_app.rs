// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_TxFirewall_TxFirewall_api::*;
use vstd::prelude::*;

#[cfg(feature = "sel4")]
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::bridge::seL4_TxFirewall_TxFirewall_GUMBOX as gumbox;
use firewall_core::{Arp, EthFrame, EthernetRepr, Ipv4Packet, PacketType};
mod config;
use crate::SW::SW_RawEthernetMessage_DIM_0;
use GumboLib::*;

#[verus_verify]
pub struct seL4_TxFirewall_TxFirewall {
  // PLACEHOLDER MARKER STATE VARS
}

#[verus_verify]
impl seL4_TxFirewall_TxFirewall {
  pub fn new() -> Self
  {
    Self {
      // PLACEHOLDER MARKER STATE VAR INIT
    }
  }

  #[verus_spec(
    ensures
      // PLACEHOLDER MARKER INITIALIZATION ENSURES
  )]
  pub fn initialize<API: seL4_TxFirewall_TxFirewall_Put_Api> (
    &mut self,
    api: &mut seL4_TxFirewall_TxFirewall_Application_Api<API>)
  {
  }

  #[verus_spec(
    requires
      // BEGIN MARKER TIME TRIGGERED REQUIRES
      // assume AADL_Requirement
      //   All outgoing event ports must be empty
      old(api).EthernetFramesTxOut0.is_none(),
      old(api).EthernetFramesTxOut1.is_none(),
      old(api).EthernetFramesTxOut2.is_none(),
      old(api).EthernetFramesTxOut3.is_none(),
      // END MARKER TIME TRIGGERED REQUIRES
    ensures
      // BEGIN MARKER TIME TRIGGERED ENSURES
      // guarantee hlr_07_tx0_can_send_valid_arp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn0.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesTxIn0.unwrap()) ==>
        final(api).EthernetFramesTxOut0.is_some() &&
          (final(api).EthernetFramesTxIn0.unwrap() == final(api).EthernetFramesTxOut0.unwrap().amessage) &&
          GumboLib::valid_output_arp_size_spec(final(api).EthernetFramesTxOut0.unwrap()),
      // guarantee hlr_12_tx0_can_send_valid_ipv4
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn0.is_some() && GumboLib::valid_ipv4_spec(final(api).EthernetFramesTxIn0.unwrap()) ==>
        final(api).EthernetFramesTxOut0.is_some() &&
          (final(api).EthernetFramesTxIn0.unwrap() == final(api).EthernetFramesTxOut0.unwrap().amessage) &&
          GumboLib::valid_output_ipv4_size_spec(final(api).EthernetFramesTxIn0.unwrap(), final(api).EthernetFramesTxOut0.unwrap()),
      // guarantee hlr_14_tx0_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn0.is_some() && !(GumboLib::tx_allow_outbound_frame_spec(final(api).EthernetFramesTxIn0.unwrap())) ==>
        final(api).EthernetFramesTxOut0.is_none(),
      // guarantee hlr_16_tx0_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn0.is_some() || final(api).EthernetFramesTxOut0.is_none(),
      // guarantee hlr_07_tx1_can_send_valid_arp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn1.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesTxIn1.unwrap()) ==>
        final(api).EthernetFramesTxOut1.is_some() &&
          (final(api).EthernetFramesTxIn1.unwrap() == final(api).EthernetFramesTxOut1.unwrap().amessage) &&
          GumboLib::valid_output_arp_size_spec(final(api).EthernetFramesTxOut1.unwrap()),
      // guarantee hlr_12_tx1_can_send_valid_ipv4
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn1.is_some() && GumboLib::valid_ipv4_spec(final(api).EthernetFramesTxIn1.unwrap()) ==>
        final(api).EthernetFramesTxOut1.is_some() &&
          (final(api).EthernetFramesTxIn1.unwrap() == final(api).EthernetFramesTxOut1.unwrap().amessage) &&
          GumboLib::valid_output_ipv4_size_spec(final(api).EthernetFramesTxIn1.unwrap(), final(api).EthernetFramesTxOut1.unwrap()),
      // guarantee hlr_14_tx1_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn1.is_some() && !(GumboLib::tx_allow_outbound_frame_spec(final(api).EthernetFramesTxIn1.unwrap())) ==>
        final(api).EthernetFramesTxOut1.is_none(),
      // guarantee hlr_16_tx1_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn1.is_some() || final(api).EthernetFramesTxOut1.is_none(),
      // guarantee hlr_07_tx2_can_send_valid_arp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn2.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesTxIn2.unwrap()) ==>
        final(api).EthernetFramesTxOut2.is_some() &&
          (final(api).EthernetFramesTxIn2.unwrap() == final(api).EthernetFramesTxOut2.unwrap().amessage) &&
          GumboLib::valid_output_arp_size_spec(final(api).EthernetFramesTxOut2.unwrap()),
      // guarantee hlr_12_tx2_can_send_valid_ipv4
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn2.is_some() && GumboLib::valid_ipv4_spec(final(api).EthernetFramesTxIn2.unwrap()) ==>
        final(api).EthernetFramesTxOut2.is_some() &&
          (final(api).EthernetFramesTxIn2.unwrap() == final(api).EthernetFramesTxOut2.unwrap().amessage) &&
          GumboLib::valid_output_ipv4_size_spec(final(api).EthernetFramesTxIn2.unwrap(), final(api).EthernetFramesTxOut2.unwrap()),
      // guarantee hlr_14_tx2_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn2.is_some() && !(GumboLib::tx_allow_outbound_frame_spec(final(api).EthernetFramesTxIn2.unwrap())) ==>
        final(api).EthernetFramesTxOut2.is_none(),
      // guarantee hlr_16_tx2_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn2.is_some() || final(api).EthernetFramesTxOut2.is_none(),
      // guarantee hlr_07_tx3_can_send_valid_arp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn3.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesTxIn3.unwrap()) ==>
        final(api).EthernetFramesTxOut3.is_some() &&
          (final(api).EthernetFramesTxIn3.unwrap() == final(api).EthernetFramesTxOut3.unwrap().amessage) &&
          GumboLib::valid_output_arp_size_spec(final(api).EthernetFramesTxOut3.unwrap()),
      // guarantee hlr_12_tx3_can_send_valid_ipv4
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=5
      final(api).EthernetFramesTxIn3.is_some() && GumboLib::valid_ipv4_spec(final(api).EthernetFramesTxIn3.unwrap()) ==>
        final(api).EthernetFramesTxOut3.is_some() &&
          (final(api).EthernetFramesTxIn3.unwrap() == final(api).EthernetFramesTxOut3.unwrap().amessage) &&
          GumboLib::valid_output_ipv4_size_spec(final(api).EthernetFramesTxIn3.unwrap(), final(api).EthernetFramesTxOut3.unwrap()),
      // guarantee hlr_14_tx3_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn3.is_some() && !(GumboLib::tx_allow_outbound_frame_spec(final(api).EthernetFramesTxIn3.unwrap())) ==>
        final(api).EthernetFramesTxOut3.is_none(),
      // guarantee hlr_16_tx3_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).EthernetFramesTxIn3.is_some() || final(api).EthernetFramesTxOut3.is_none(),
      // END MARKER TIME TRIGGERED ENSURES
  )]
  pub fn timeTriggered<API: seL4_TxFirewall_TxFirewall_Full_Api> (
    &mut self,
    api: &mut seL4_TxFirewall_TxFirewall_Application_Api<API>)
  {
    // Tx0 ports
    if let Some(frame) = api.get_EthernetFramesTxIn0() {
        if let Some(eth) = Self::get_frame_packet(&frame) {
            if let Some(size) = can_send_packet(&eth.eth_type) {
                let out = SW::SizedEthernetMessage_Impl {
                    sz: size,
                    amessage: frame,
                };
                api.put_EthernetFramesTxOut0(out);
            }
        }
    }

      // Tx1 ports
      if let Some(frame) = api.get_EthernetFramesTxIn1() {
          if let Some(eth) = Self::get_frame_packet(&frame) {
              if let Some(size) = can_send_packet(&eth.eth_type) {
                  let out = SW::SizedEthernetMessage_Impl {
                      sz: size,
                      amessage: frame,
                  };
                  api.put_EthernetFramesTxOut1(out);
              }
          }
      }

      // Tx2 ports
      if let Some(frame) = api.get_EthernetFramesTxIn2() {
          if let Some(eth) = Self::get_frame_packet(&frame) {
              if let Some(size) = can_send_packet(&eth.eth_type) {
                  let out = SW::SizedEthernetMessage_Impl {
                      sz: size,
                      amessage: frame,
                  };
                  api.put_EthernetFramesTxOut2(out);
              }
          }
      }

      // Tx3 ports
      if let Some(frame) = api.get_EthernetFramesTxIn3() {
          if let Some(eth) = Self::get_frame_packet(&frame) {
              if let Some(size) = can_send_packet(&eth.eth_type) {
                  let out = SW::SizedEthernetMessage_Impl {
                      sz: size,
                      amessage: frame,
                  };
                  api.put_EthernetFramesTxOut3(out);
              }
          }
      }
  }

  pub fn notify(
    &mut self,
    channel: microkit_channel)
  {
    // this method is called when the monitor does not handle the passed in channel
    match channel {
      _ => {
        log_warn_channel(channel)
      }
    }
  }
}

#[verus_verify(external_body)]
pub fn log_info(msg: &str)
{
  log::info!("{0}", msg);
}

#[verus_verify(external_body)]
pub fn log_warn_channel(channel: u32)
{
  log::warn!("Unexpected channel: {0}", channel);
}

// PLACEHOLDER MARKER GUMBO METHODS

// Application helpers
verus! {
fn can_send_packet(packet: &PacketType) -> (r: Option<u16>)
      requires
          (packet is Ipv4) ==> (firewall_core::ipv4_valid_length(*packet))
      ensures
          (packet is Arp || packet is Ipv4) == r.is_some(),
          packet is Arp ==> (r == Some(64u16)),
          packet is Ipv4 ==> (r == Some((packet->Ipv4_0.header.length + EthernetRepr::SIZE) as u16)),
  {
      match packet {
          PacketType::Arp(_) => Some(64u16),
          PacketType::Ipv4(ip) => Some(ip.header.length + EthernetRepr::SIZE as u16),
          PacketType::Ipv6 => {
              log_info("IPv6 packet: Throw it away.");
              None
          }
      }
  }



  impl seL4_TxFirewall_TxFirewall {
    fn get_frame_packet(frame: &SW::RawEthernetMessage) -> (r: Option<EthFrame>)
    requires
        frame@.len() == SW_RawEthernetMessage_DIM_0
    ensures
        valid_arp_spec(*frame) == firewall_core::res_is_arp(r),
        valid_ipv4_spec(*frame) == firewall_core::res_is_ipv4(r),
        valid_ipv4_spec(*frame) ==> firewall_core::ipv4_length_bytes_match(frame, r),
    {
        let eth = EthFrame::parse(frame);
        if eth.is_none() {
            log_info("Malformed packet. Throw it away.")
        }
        eth
    }
}
}
