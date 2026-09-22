// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_RxFirewall_RxFirewall_api::*;
use vstd::prelude::*;

#[verus_verify]
pub struct seL4_RxFirewall_RxFirewall {
  // PLACEHOLDER MARKER STATE VARS
}

#[verus_verify]
impl seL4_RxFirewall_RxFirewall {
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
  pub fn initialize<API: seL4_RxFirewall_RxFirewall_Put_Api> (
    &mut self,
    api: &mut seL4_RxFirewall_RxFirewall_Application_Api<API>)
  {
    log_info("initialize entrypoint invoked");
  }

  #[verus_spec(
    requires
      // BEGIN MARKER TIME TRIGGERED REQUIRES
      // assume AADL_Requirement
      //   All outgoing event ports must be empty
      old(api).VmmOut0.is_none(),
      old(api).VmmOut1.is_none(),
      old(api).VmmOut2.is_none(),
      old(api).VmmOut3.is_none(),
      old(api).MavlinkOut0.is_none(),
      old(api).MavlinkOut1.is_none(),
      old(api).MavlinkOut2.is_none(),
      old(api).MavlinkOut3.is_none(),
      // END MARKER TIME TRIGGERED REQUIRES
    ensures
      // BEGIN MARKER TIME TRIGGERED ENSURES
      // guarantee hlr_05_rx0_can_send_arp_to_vmm
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn0.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesRxIn0.unwrap()) ==>
        ((final(api).VmmOut0.is_some() &&
          (final(api).EthernetFramesRxIn0.unwrap() == final(api).VmmOut0.unwrap())) &&
          final(api).MavlinkOut0.is_none()),
      // guarantee hlr_18_rx0_can_send_mavlink_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn0.is_some() && GumboLib::valid_ipv4_udp_mavlink_spec(final(api).EthernetFramesRxIn0.unwrap()) ==>
        ((final(api).MavlinkOut0.is_some() && GumboLib::input_eq_mav_output_spec(final(api).EthernetFramesRxIn0.unwrap(), final(api).MavlinkOut0.unwrap())) &&
          final(api).VmmOut0.is_none()),
      // guarantee hlr_13_rx0_can_send_ipv4_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn0.is_some() && GumboLib::valid_ipv4_udp_port_spec(final(api).EthernetFramesRxIn0.unwrap()) ==>
        ((final(api).VmmOut0.is_some() &&
          (final(api).EthernetFramesRxIn0.unwrap() == final(api).VmmOut0.unwrap())) &&
          final(api).MavlinkOut0.is_none()),
      // guarantee hlr_15_rx0_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn0.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(final(api).EthernetFramesRxIn0.unwrap())) ==>
        (final(api).VmmOut0.is_none() && final(api).MavlinkOut0.is_none()),
      // guarantee hlr_17_rx0_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn0.is_some() ||
        (final(api).VmmOut0.is_none() && final(api).MavlinkOut0.is_none()),
      // guarantee hlr_05_rx1_can_send_arp_to_vmm
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn1.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesRxIn1.unwrap()) ==>
        ((final(api).VmmOut1.is_some() &&
          (final(api).EthernetFramesRxIn1.unwrap() == final(api).VmmOut1.unwrap())) &&
          final(api).MavlinkOut1.is_none()),
      // guarantee hlr_18_rx1_can_send_mavlink_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn1.is_some() && GumboLib::valid_ipv4_udp_mavlink_spec(final(api).EthernetFramesRxIn1.unwrap()) ==>
        ((final(api).MavlinkOut1.is_some() && GumboLib::input_eq_mav_output_spec(final(api).EthernetFramesRxIn1.unwrap(), final(api).MavlinkOut1.unwrap())) &&
          final(api).VmmOut1.is_none()),
      // guarantee hlr_13_rx1_can_send_ipv4_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn1.is_some() && GumboLib::valid_ipv4_udp_port_spec(final(api).EthernetFramesRxIn1.unwrap()) ==>
        ((final(api).VmmOut1.is_some() &&
          (final(api).EthernetFramesRxIn1.unwrap() == final(api).VmmOut1.unwrap())) &&
          final(api).MavlinkOut1.is_none()),
      // guarantee hlr_15_rx1_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn1.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(final(api).EthernetFramesRxIn1.unwrap())) ==>
        (final(api).VmmOut1.is_none() && final(api).MavlinkOut1.is_none()),
      // guarantee hlr_17_rx1_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn1.is_some() ||
        (final(api).VmmOut1.is_none() && final(api).MavlinkOut1.is_none()),
      // guarantee hlr_05_rx2_can_send_arp_to_vmm
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn2.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesRxIn2.unwrap()) ==>
        ((final(api).VmmOut2.is_some() &&
          (final(api).EthernetFramesRxIn2.unwrap() == final(api).VmmOut2.unwrap())) &&
          final(api).MavlinkOut2.is_none()),
      // guarantee hlr_18_rx2_can_send_mavlink_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn2.is_some() && GumboLib::valid_ipv4_udp_mavlink_spec(final(api).EthernetFramesRxIn2.unwrap()) ==>
        ((final(api).MavlinkOut2.is_some() && GumboLib::input_eq_mav_output_spec(final(api).EthernetFramesRxIn2.unwrap(), final(api).MavlinkOut2.unwrap())) &&
          final(api).VmmOut2.is_none()),
      // guarantee hlr_13_rx2_can_send_ipv4_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn2.is_some() && GumboLib::valid_ipv4_udp_port_spec(final(api).EthernetFramesRxIn2.unwrap()) ==>
        ((final(api).VmmOut2.is_some() &&
          (final(api).EthernetFramesRxIn2.unwrap() == final(api).VmmOut2.unwrap())) &&
          final(api).MavlinkOut2.is_none()),
      // guarantee hlr_15_rx2_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn2.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(final(api).EthernetFramesRxIn2.unwrap())) ==>
        (final(api).VmmOut2.is_none() && final(api).MavlinkOut2.is_none()),
      // guarantee hlr_17_rx2_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn2.is_some() ||
        (final(api).VmmOut2.is_none() && final(api).MavlinkOut2.is_none()),
      // guarantee hlr_05_rx3_can_send_arp_to_vmm
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn3.is_some() && GumboLib::valid_arp_spec(final(api).EthernetFramesRxIn3.unwrap()) ==>
        ((final(api).VmmOut3.is_some() &&
          (final(api).EthernetFramesRxIn3.unwrap() == final(api).VmmOut3.unwrap())) &&
          final(api).MavlinkOut3.is_none()),
      // guarantee hlr_18_rx3_can_send_mavlink_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn3.is_some() && GumboLib::valid_ipv4_udp_mavlink_spec(final(api).EthernetFramesRxIn3.unwrap()) ==>
        ((final(api).MavlinkOut3.is_some() && GumboLib::input_eq_mav_output_spec(final(api).EthernetFramesRxIn3.unwrap(), final(api).MavlinkOut3.unwrap())) &&
          final(api).VmmOut3.is_none()),
      // guarantee hlr_13_rx3_can_send_ipv4_udp
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=3
      final(api).EthernetFramesRxIn3.is_some() && GumboLib::valid_ipv4_udp_port_spec(final(api).EthernetFramesRxIn3.unwrap()) ==>
        ((final(api).VmmOut3.is_some() &&
          (final(api).EthernetFramesRxIn3.unwrap() == final(api).VmmOut3.unwrap())) &&
          final(api).MavlinkOut3.is_none()),
      // guarantee hlr_15_rx3_disallow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn3.is_some() && !(GumboLib::rx_allow_outbound_frame_spec(final(api).EthernetFramesRxIn3.unwrap())) ==>
        (final(api).VmmOut3.is_none() && final(api).MavlinkOut3.is_none()),
      // guarantee hlr_17_rx3_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=4
      final(api).EthernetFramesRxIn3.is_some() ||
        (final(api).VmmOut3.is_none() && final(api).MavlinkOut3.is_none()),
      // END MARKER TIME TRIGGERED ENSURES
  )]
  pub fn timeTriggered<API: seL4_RxFirewall_RxFirewall_Full_Api> (
    &mut self,
    api: &mut seL4_RxFirewall_RxFirewall_Application_Api<API>)
  {
    log_info("compute entrypoint invoked");
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
