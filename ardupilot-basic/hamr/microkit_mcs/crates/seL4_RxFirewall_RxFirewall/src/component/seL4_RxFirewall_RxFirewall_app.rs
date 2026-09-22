// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_RxFirewall_RxFirewall_api::*;
use vstd::prelude::*;

#[cfg(feature = "sel4")]
use log::{debug, error, info, trace, warn};
// use vstd::slice::slice_subrange;
use SW::{
    EthIpUdpHeaders, SW_EthIpUdpHeaders_DIM_0, SW_UdpPayload_DIM_0, UdpFrame_Impl, UdpPayload,
};

use crate::SW::SW_RawEthernetMessage_DIM_0;
use firewall_core::{EthFrame, IpProtocol, Ipv4ProtoPacket, PacketType, TcpRepr, UdpRepr};
use GumboLib::*;

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
    // Rx0 ports
    if let Some(frame) = api.get_EthernetFramesRxIn0() {
        if let Some(eth) = Self::get_frame_packet(&frame) {
            if can_send_to_mavlink(&eth.eth_type) {
                let output = udp_frame_from_raw_eth(frame);
                api.put_MavlinkOut0(output);
            } else if can_send_to_vmm(&eth.eth_type) {
                api.put_VmmOut0(frame);
            }
        }
    }

    // Rx1 ports
    if let Some(frame) = api.get_EthernetFramesRxIn1() {
        if let Some(eth) = Self::get_frame_packet(&frame) {
            if can_send_to_mavlink(&eth.eth_type) {
                let output = udp_frame_from_raw_eth(frame);
                api.put_MavlinkOut1(output);
            } else if can_send_to_vmm(&eth.eth_type) {
                api.put_VmmOut1(frame);
            }
        }
    }

    // Rx2 ports
    if let Some(frame) = api.get_EthernetFramesRxIn2() {
        if let Some(eth) = Self::get_frame_packet(&frame) {
            if can_send_to_mavlink(&eth.eth_type) {
                let output = udp_frame_from_raw_eth(frame);
                api.put_MavlinkOut2(output);
            } else if can_send_to_vmm(&eth.eth_type) {
                api.put_VmmOut2(frame);
            }
        }
    }

    // Rx3 ports
    if let Some(frame) = api.get_EthernetFramesRxIn3() {
        if let Some(eth) = Self::get_frame_packet(&frame) {
            if can_send_to_mavlink(&eth.eth_type) {
                let output = udp_frame_from_raw_eth(frame);
                api.put_MavlinkOut3(output);
            } else if can_send_to_vmm(&eth.eth_type) {
                api.put_VmmOut3(frame);
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
pub const MAV_UDP_SRC_PORT: u16 = 14550;
    pub const MAV_UDP_DST_PORT: u16 = 14562;

    fn udp_headers_from_raw_eth(value: SW::RawEthernetMessage) -> (r: EthIpUdpHeaders)
        ensures
            r@ =~= value@.subrange(0, SW_EthIpUdpHeaders_DIM_0 as int)
    {
        let mut headers = [0u8; SW_EthIpUdpHeaders_DIM_0];
        let mut i = 0;
        while i < SW_EthIpUdpHeaders_DIM_0
            invariant
                0 <= i <= headers@.len() < value@.len(),
                forall |j| 0 <= j < i ==> headers[j] == value[j],
            decreases
                SW_EthIpUdpHeaders_DIM_0 - i
        {
            headers.set(i, value[i]);
            i += 1;
        }
        headers
    }


    fn udp_payload_from_raw_eth(value: SW::RawEthernetMessage) -> (r: UdpPayload)
        ensures
            r@ =~= value@.subrange(SW_EthIpUdpHeaders_DIM_0 as int, SW_RawEthernetMessage_DIM_0 as int)
    {
        let mut payload = [0u8; SW_RawEthernetMessage_DIM_0-SW_EthIpUdpHeaders_DIM_0];

        let mut i = 0;
        while i < SW_UdpPayload_DIM_0
            invariant
                0 <= i <= payload@.len() <= value@.len()-SW_EthIpUdpHeaders_DIM_0,
                forall |j| 0 <= j < i ==> #[trigger] payload[j] == value[j+SW_EthIpUdpHeaders_DIM_0],
            decreases
                SW_UdpPayload_DIM_0 - i
        {
            payload.set(i, value[i+SW_EthIpUdpHeaders_DIM_0]);
            i += 1;
        }
        payload
    }

    fn udp_frame_from_raw_eth(value: SW::RawEthernetMessage) -> (r: UdpFrame_Impl)
        ensures
            r.headers@ =~= value@.subrange(0, SW_EthIpUdpHeaders_DIM_0 as int),
            r.payload@ =~= value@.subrange(SW_EthIpUdpHeaders_DIM_0 as int, SW_RawEthernetMessage_DIM_0 as int),
     {
        let headers = udp_headers_from_raw_eth(value);
        let payload = udp_payload_from_raw_eth(value);
        UdpFrame_Impl { headers, payload}
    }

    // mod config;

    // Testing
    mod config {
    pub mod tcp {
        pub const ALLOWED_PORTS: [u16; 1] = [5760u16];
    }

    pub mod udp {
        const NUM_UDP_PORTS: usize = 1;
        pub const ALLOWED_PORTS: [u16; NUM_UDP_PORTS] = [68u16];
    }
  }
    // End Testing

    const NUM_MSGS: usize = 4;



    fn port_allowed(allowed_ports: &[u16], port: u16) -> (r: bool)
        ensures
            r == allowed_ports@.contains(port),
    {
        let mut i: usize = 0;
        while i < allowed_ports.len()
            invariant
                0 <= i <= allowed_ports@.len(),
                forall |j| 0 <= j < i ==> allowed_ports@[j] != port,
            decreases
                allowed_ports@.len() - i
        {
            if allowed_ports[i] == port {
                return true;
            }
            i += 1;
        }
        false
    }

    fn udp_port_allowed(port: u16) -> (r: bool)
        ensures
            r == config::udp::ALLOWED_PORTS@.contains(port),
    {
        port_allowed(&config::udp::ALLOWED_PORTS, port)
    }

    fn tcp_port_allowed(port: u16) -> (r: bool)
        ensures
            r == config::tcp::ALLOWED_PORTS@.contains(port),
    {
        port_allowed(&config::tcp::ALLOWED_PORTS, port)
    }

    pub open spec fn packet_is_whitelisted_tcp(packet: &PacketType) -> bool
    {
        packet is Ipv4 &&
            packet->Ipv4_0.protocol is Tcp &&
            ipv4_tcp_on_allowed_port_quant(packet->Ipv4_0.protocol->Tcp_0.dst_port)
    }


    pub open spec fn packet_is_whitelisted_udp(packet: &PacketType) -> bool
    {
        packet is Ipv4 &&
            packet->Ipv4_0.protocol is Udp &&
            ipv4_udp_on_allowed_port_quant(packet->Ipv4_0.protocol->Udp_0.dst_port)
    }

    fn can_send_to_vmm(packet: &PacketType) -> (r: bool)
        requires
            config::udp::ALLOWED_PORTS =~= UDP_ALLOWED_PORTS_spec(),
        ensures
            ((packet is Arp) ||
                packet_is_whitelisted_udp(packet)
            ) == (r == true),
    {
        match packet {
            PacketType::Arp(_) => true,
            PacketType::Ipv4(ip) => match &ip.protocol {
                // Ipv4ProtoPacket::Tcp(tcp) => {
                //     let allowed = tcp_port_allowed(tcp.dst_port);
                //     if !allowed {
                //         info("TCP packet filtered out");
                //     }
                //     allowed
                // }
                Ipv4ProtoPacket::Udp(udp) => {
                    let allowed = udp_port_allowed(udp.dst_port);
                    if !allowed {
                        log_info("UDP packet filtered out");
                    }
                    allowed
                }
                _ => {
                    info_protocol(ip.header.protocol);
                    false
                }
            },
            PacketType::Ipv6 => {
                log_info("Not an IPv4 or Arp packet. Throw it away.");
                false
            },
        }
    }

    pub open spec fn packet_is_mavlink_udp(packet: &PacketType) -> bool
    {
        packet is Ipv4 &&
            packet->Ipv4_0.protocol is Udp &&
            packet->Ipv4_0.protocol->Udp_0.src_port == MAV_UDP_SRC_PORT &&
            packet->Ipv4_0.protocol->Udp_0.dst_port == MAV_UDP_DST_PORT
    }

    fn can_send_to_mavlink(packet: &PacketType) -> (r: bool)
        ensures
            (packet_is_mavlink_udp(packet)) == (r == true),
    {
        if let PacketType::Ipv4(ip) = packet {
            if let Ipv4ProtoPacket::Udp(udp) = &ip.protocol {
                return udp.src_port == MAV_UDP_SRC_PORT && udp.dst_port == MAV_UDP_DST_PORT;
            }
        }
        false
    }

  impl seL4_RxFirewall_RxFirewall {
    pub fn get_frame_packet(frame: &SW::RawEthernetMessage) -> (r: Option<EthFrame>)
        requires
            frame@.len() == SW_RawEthernetMessage_DIM_0
        ensures
            valid_arp_spec(*frame) == firewall_core::res_is_arp(r),
            valid_ipv4_udp_spec(*frame) == firewall_core::res_is_udp(r),
            valid_ipv4_tcp_spec(*frame) == firewall_core::res_is_tcp(r),
            valid_ipv4_tcp_spec(*frame) ==> firewall_core::tcp_port_bytes_match(frame, r),
            valid_ipv4_udp_spec(*frame) ==> firewall_core::udp_port_bytes_match(frame, r),
    {
        let eth = EthFrame::parse(frame);
        if eth.is_none() {
            log_info("Malformed packet. Throw it away.")
        }
        eth
    }
}


  #[verifier::external_body]
  fn info_protocol(protocol: IpProtocol) {
      log::info!("Not a TCP or UDP packet. ({:?}) Throw it away.", protocol);
  }





  pub open spec fn ipv4_udp_on_allowed_port_quant(port: u16) -> bool
  {
      exists|i:int| 0 <= i && i < UDP_ALLOWED_PORTS_spec().len() && UDP_ALLOWED_PORTS_spec()[i] == port
  }

  pub open spec fn ipv4_tcp_on_allowed_port_quant(port: u16) -> bool
  {
      exists|i:int| 0 <= i && i < TCP_ALLOWED_PORTS_spec().len() && TCP_ALLOWED_PORTS_spec()[i] == port
  }
}
