// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_MavlinkFirewall_MavlinkFirewall_api::*;
use vstd::prelude::*;

use data::{
    SW::{SW_EthIpUdpHeaders_DIM_0, SW_RawEthernetMessage_DIM_0, SW_UdpPayload_DIM_0},
};

use mavlink_parser_vest::{
    parse_mavlink_msg, MavlinkMsg, MavlinkMsgMsg, MessageIdsV1, MessageIdsV2,
};
#[cfg(verus_keep_ghost)]
use mavlink_parser_vest::spec_mavlink_msg;
use mavlink_parser_vest::{MavlinkMsgSpec as SpecMavlinkMsg, MavlinkMsgMsgSpec as SpecMavlinkMsgMsg};
use vest_lib::core::spec::SpecParser;


// Need an allocator for the vest lib
#[cfg(not(test))]
use one_shot_mutex::sync::RawOneShotMutex;
#[cfg(not(test))]
use sel4_dlmalloc::{StaticDlmalloc, StaticHeap};

extern crate alloc;

#[cfg(not(test))]
const HEAP_SIZE: usize = 16 * 1024;

#[cfg(not(test))]
static HEAP: StaticHeap<HEAP_SIZE> = StaticHeap::new();

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: StaticDlmalloc<RawOneShotMutex> = StaticDlmalloc::new(HEAP.bounds());
// Allocator END

#[verus_verify]
pub struct seL4_MavlinkFirewall_MavlinkFirewall {
  // PLACEHOLDER MARKER STATE VARS
}

#[verus_verify]
impl seL4_MavlinkFirewall_MavlinkFirewall {
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
  pub fn initialize<API: seL4_MavlinkFirewall_MavlinkFirewall_Put_Api> (
    &mut self,
    api: &mut seL4_MavlinkFirewall_MavlinkFirewall_Application_Api<API>)
  {
  }

  #[verus_spec(
    requires
      // BEGIN MARKER TIME TRIGGERED REQUIRES
      // assume AADL_Requirement
      //   All outgoing event ports must be empty
      old(api).Out0.is_none(),
      old(api).Out1.is_none(),
      old(api).Out2.is_none(),
      old(api).Out3.is_none(),
      // END MARKER TIME TRIGGERED REQUIRES
    ensures
      // BEGIN MARKER TIME TRIGGERED ENSURES
      // guarantee hlr_19_mav0_drop_mav_cmd_flash_bootloader
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).In0.is_some() && msg_is_wellformed(final(api).In0.unwrap().payload) &&
        msg_is_mav_cmd_flash_bootloader(final(api).In0.unwrap().payload) ==>
        final(api).Out0.is_none(),
      // guarantee hlr_20_mav0_drop_malformed_msg
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In0.is_some() && !(msg_is_wellformed(final(api).In0.unwrap().payload)) ==>
        final(api).Out0.is_none(),
      // guarantee hlr_21_mav0_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In0.is_some() || final(api).Out0.is_none(),
      // guarantee hlr_22_mav0_allow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In0.is_some() && msg_is_wellformed(final(api).In0.unwrap().payload) &&
        !(msg_is_blacklisted(final(api).In0.unwrap().payload)) ==>
        final(api).Out0.is_some() && GumboLib::mav_input_eq_output_spec(final(api).In0.unwrap(), final(api).Out0.unwrap()),
      // guarantee hlr_19_mav1_drop_mav_cmd_flash_bootloader
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).In1.is_some() && msg_is_wellformed(final(api).In1.unwrap().payload) &&
        msg_is_mav_cmd_flash_bootloader(final(api).In1.unwrap().payload) ==>
        final(api).Out1.is_none(),
      // guarantee hlr_20_mav1_drop_malformed_msg
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In1.is_some() && !(msg_is_wellformed(final(api).In1.unwrap().payload)) ==>
        final(api).Out1.is_none(),
      // guarantee hlr_21_mav1_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In1.is_some() || final(api).Out1.is_none(),
      // guarantee hlr_22_mav1_allow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In1.is_some() && msg_is_wellformed(final(api).In1.unwrap().payload) &&
        !(msg_is_blacklisted(final(api).In1.unwrap().payload)) ==>
        final(api).Out1.is_some() && GumboLib::mav_input_eq_output_spec(final(api).In1.unwrap(), final(api).Out1.unwrap()),
      // guarantee hlr_19_mav2_drop_mav_cmd_flash_bootloader
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).In2.is_some() && msg_is_wellformed(final(api).In2.unwrap().payload) &&
        msg_is_mav_cmd_flash_bootloader(final(api).In2.unwrap().payload) ==>
        final(api).Out2.is_none(),
      // guarantee hlr_20_mav2_drop_malformed_msg
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In2.is_some() && !(msg_is_wellformed(final(api).In2.unwrap().payload)) ==>
        final(api).Out2.is_none(),
      // guarantee hlr_21_mav2_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In2.is_some() || final(api).Out2.is_none(),
      // guarantee hlr_22_mav2_allow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In2.is_some() && msg_is_wellformed(final(api).In2.unwrap().payload) &&
        !(msg_is_blacklisted(final(api).In2.unwrap().payload)) ==>
        final(api).Out2.is_some() && GumboLib::mav_input_eq_output_spec(final(api).In2.unwrap(), final(api).Out2.unwrap()),
      // guarantee hlr_19_mav3_drop_mav_cmd_flash_bootloader
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=6
      final(api).In3.is_some() && msg_is_wellformed(final(api).In3.unwrap().payload) &&
        msg_is_mav_cmd_flash_bootloader(final(api).In3.unwrap().payload) ==>
        final(api).Out3.is_none(),
      // guarantee hlr_20_mav3_drop_malformed_msg
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In3.is_some() && !(msg_is_wellformed(final(api).In3.unwrap().payload)) ==>
        final(api).Out3.is_none(),
      // guarantee hlr_21_mav3_no_input
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In3.is_some() || final(api).Out3.is_none(),
      // guarantee hlr_22_mav3_allow
      //   https://loonwerks.com/INSPECTA-Open-Platform/ardupilot-basic/requirements/Inspecta-HLRs.pdf#page=7
      final(api).In3.is_some() && msg_is_wellformed(final(api).In3.unwrap().payload) &&
        !(msg_is_blacklisted(final(api).In3.unwrap().payload)) ==>
        final(api).Out3.is_some() && GumboLib::mav_input_eq_output_spec(final(api).In3.unwrap(), final(api).Out3.unwrap()),
      // END MARKER TIME TRIGGERED ENSURES
  )]
  pub fn timeTriggered<API: seL4_MavlinkFirewall_MavlinkFirewall_Full_Api> (
    &mut self,
    api: &mut seL4_MavlinkFirewall_MavlinkFirewall_Application_Api<API>)
  {
    if let Some(udp_frame) = api.get_In0() {
        if can_send(udp_frame.payload) {
            let output = raw_eth_from_udp_frame(udp_frame);
            api.put_Out0(output);
        }
    }

    if let Some(udp_frame) = api.get_In1() {
        if can_send(udp_frame.payload) {
            let output = raw_eth_from_udp_frame(udp_frame);
            api.put_Out1(output);
        }
    }

    if let Some(udp_frame) = api.get_In2() {
        if can_send(udp_frame.payload) {
            let output = raw_eth_from_udp_frame(udp_frame);
            api.put_Out2(output);
        }
    }

    if let Some(udp_frame) = api.get_In3() {
        if can_send(udp_frame.payload) {
            let output = raw_eth_from_udp_frame(udp_frame);
            api.put_Out3(output);
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

verus! {

  // BEGIN MARKER GUMBO METHODS
  /// Verus wrapper for the GUMBO spec function `test` that delegates to the developer-supplied Verus
  /// specification function that must have the following signature:
  /// 
  ///   pub open spec fn msg_is_wellformed__developer_verus(msg: SW::UdpPayload) -> (res: bool) { ... }
  /// 
  /// The semantics of the GUMBO spec function are entirely defined by the developer-supplied implementation.
  pub open spec fn msg_is_wellformed(msg: SW::UdpPayload) -> bool
  {
    msg_is_wellformed__developer_verus(msg)
  }

  /// Verus wrapper for the GUMBO spec function `test` that delegates to the developer-supplied Verus
  /// specification function that must have the following signature:
  /// 
  ///   pub open spec fn msg_is_mav_cmd_flash_bootloader__developer_verus(msg: SW::UdpPayload) -> (res: bool) { ... }
  /// 
  /// The semantics of the GUMBO spec function are entirely defined by the developer-supplied implementation.
  pub open spec fn msg_is_mav_cmd_flash_bootloader(msg: SW::UdpPayload) -> bool
  {
    msg_is_mav_cmd_flash_bootloader__developer_verus(msg)
  }

  pub open spec fn msg_is_blacklisted(msg: SW::UdpPayload) -> bool
  {
    msg_is_mav_cmd_flash_bootloader(msg)
  }
  // END MARKER GUMBO METHODS

  /// Developer-supplied Verus realization of the GUMBO spec function `test`.
  /// 
  /// This function may be freely refined as long as it remains a pure Verus `spec fn`.
  pub open spec fn msg_is_wellformed__developer_verus(msg: SW::UdpPayload) -> (res: bool)
  {
    spec_mavlink_msg().spec_parse(msg@).is_some()
  }

  /// Developer-supplied Verus realization of the GUMBO spec function `test`.
  /// 
  /// This function may be freely refined as long as it remains a pure Verus `spec fn`.
  pub open spec fn msg_is_mav_cmd_flash_bootloader__developer_verus(msg: SW::UdpPayload) -> (res: bool)
  {
    match spec_mavlink_msg().spec_parse(msg@) {
        Some((_, msg)) => spec_msg_is_flash_bootloader(msg),
        None => false,
    }
  }

}

verus! {

  /// Developer-supplied GUMBOX realization of the GUMBO spec function `test`.
  /// 
  /// This function may be freely refined.
  pub exec fn msg_is_wellformed__developer_gumbox(msg: SW::UdpPayload) -> (res: bool)
    ensures
      res == msg_is_wellformed__developer_verus(msg),
  {
    parse_mavlink_msg(&msg).is_ok()
  }

  /// Developer-supplied GUMBOX realization of the GUMBO spec function `test`.
  /// 
  /// This function may be freely refined.
  pub exec fn msg_is_mav_cmd_flash_bootloader__developer_gumbox(msg: SW::UdpPayload) -> (res: bool)
    ensures
      res == msg_is_mav_cmd_flash_bootloader__developer_verus(msg),
  {
    match parse_mavlink_msg(&msg) {
        Ok((_, msg)) => msg_is_flash_bootloader(&msg),
        Err(_) => false,
    }
  }

}

// Application helpers
verus! {
// Spec Helpers
    pub open spec fn spec_msg_is_flash_bootloader(msg: SpecMavlinkMsg) -> bool
    {
        spec_msg_v1_is_flash_bootloader(msg) || spec_msg_v2_is_flash_bootloader(msg)
    }

    pub open spec fn spec_msg_v1_is_flash_bootloader(msg: SpecMavlinkMsg) -> bool
    {
        msg.msg matches SpecMavlinkMsgMsg::MavLink1(mv1) &&
            (mv1.msgid == MessageIdsV1::CommandInt || mv1.msgid == MessageIdsV1::CommandLong) &&
            (spec_payload_get_cmd(mv1.payload) matches Some(cmd) && cmd == 42650)
    }

    pub open spec fn spec_msg_v2_is_flash_bootloader(msg: SpecMavlinkMsg) -> bool
    {
        msg.msg matches SpecMavlinkMsgMsg::MavLink2(mv2) &&
            (mv2.msgid == MessageIdsV2::CommandInt || mv2.msgid == MessageIdsV2::CommandLong) &&
            (spec_payload_get_cmd(mv2.payload) matches Some(cmd) && cmd == 42650)
    }

    pub open spec fn spec_payload_get_cmd(payload: Seq<u8>) -> Option<u16>
    {
        if payload.len() >= 30 {
            Some((payload[28] as u16) | ((payload[29] as u16) << 8))
        } else {
            None
        }
    }

    // Exec Code
    fn raw_eth_from_udp_frame(value: SW::UdpFrame_Impl) -> (r: SW::RawEthernetMessage)
        ensures
            GumboLib::mav_input_eq_output_spec(value, r),
     {
        let mut frame = [0u8; SW_RawEthernetMessage_DIM_0];

        let mut i = 0;
        while i < SW_RawEthernetMessage_DIM_0
            invariant
                0 <= i <= SW_RawEthernetMessage_DIM_0,
                forall |j: int| 0 <= j < i ==> {
                  if j < SW_EthIpUdpHeaders_DIM_0@ {
                    value.headers[j] == #[trigger] frame[j]
                  } else {
                    value.payload[j-SW_EthIpUdpHeaders_DIM_0@] == frame[j]
                  }
                },
            decreases
                SW_RawEthernetMessage_DIM_0 - i,

        {
          if i < SW_EthIpUdpHeaders_DIM_0 {
            frame.set(i, value.headers[i]);
          } else {
            frame.set(i, value.payload[i - SW_EthIpUdpHeaders_DIM_0]);
          }
            i += 1;
        }
        frame
    }

    fn can_send(payload: SW::UdpPayload) -> (r: bool)
        ensures
            (msg_is_wellformed(payload) && !msg_is_blacklisted(payload)) == (r == true)
    {
        match parse_mavlink_msg(&payload) {
            Ok((_, msg)) => !ex_msg_is_blacklisted(&msg),
            Err(_) => {
                log_info("Throw away malformed mavlink");
                false
            }
        }
    }

    fn ex_msg_is_blacklisted(msg: &MavlinkMsg) -> (r: bool)
        ensures
            r == spec_msg_is_flash_bootloader(msg.deep_view()),
    {
        let res = msg_is_flash_bootloader(msg);
        if res {
            log_info("Throw away flash bootloader command");
        }
        res
    }

    fn msg_is_flash_bootloader(msg: &MavlinkMsg) -> (r: bool)
        ensures
             r == spec_msg_is_flash_bootloader(msg.deep_view())
    {
        // Upstream Vest makes deep views opaque; expose the generated field
        // relationships before relating executable values to the specification.
        proof {
            msg.lemma_deep_view_fields();
            msg.msg.lemma_deep_view_fields();
            match &msg.msg {
                MavlinkMsgMsg::MavLink1(v1) => {
                    v1.lemma_deep_view_fields();
                    v1.msgid.lemma_deep_view();
                },
                MavlinkMsgMsg::MavLink2(v2) => {
                    v2.lemma_deep_view_fields();
                    v2.msgid.lemma_deep_view();
                },
            }
        }
        let command = match &msg.msg {
            MavlinkMsgMsg::MavLink1(v1_msg) =>
                match v1_msg.msgid {
                    MessageIdsV1::CommandInt | MessageIdsV1::CommandLong =>
                    payload_get_cmd(v1_msg.payload),
                    _ => None,
                }
            MavlinkMsgMsg::MavLink2(v2_msg) => {
                let msgid = v2_msg.msgid;
                match msgid {
                MessageIdsV2::CommandInt | MessageIdsV2::CommandLong =>
                    payload_get_cmd(v2_msg.payload),
                    _ => None
                }
            },
        };

        match command {
            Some(cmd) => cmd == 42650,
            None => false,
        }

    }

    /// Gets the command for a CommandInt or CommandLong payload
    ///
    /// Workaround for current vest deficiency
    fn payload_get_cmd(payload: &[u8]) -> (o: Option<u16>)
        ensures
            o == spec_payload_get_cmd(payload@),
    {
        if payload.len() >= 30 {
            Some((payload[28] as u16) | ((payload[29] as u16) << 8))
        } else {
            None
        }
    }

#[verifier::external_body]
pub fn log_trace(msg: &str) {
    log::trace!("{0}", msg);
}

}

#[cfg(test)]
mod upstream_vest_tests {
    use super::*;

    fn command(version: u8, msgid: u8, cmd: u16, signed: bool) -> SW::UdpPayload {
        let mut bytes = [0; SW_UdpPayload_DIM_0];
        bytes[0] = if version == 1 { 0xfe } else { 0xfd };
        bytes[1] = 30;
        let payload_offset = if version == 1 {
            bytes[3] = 1;
            bytes[4] = 1;
            bytes[5] = msgid;
            6
        } else {
            bytes[2] = u8::from(signed);
            bytes[5] = 1;
            bytes[6] = 1;
            bytes[7] = msgid;
            10
        };
        bytes[payload_offset + 28..payload_offset + 30].copy_from_slice(&cmd.to_le_bytes());
        bytes
    }

    #[test]
    fn flash_bootloader_commands_are_blocked_for_both_versions() {
        for version in [1, 2] {
            for msgid in [75, 76] {
                let bytes = command(version, msgid, 42650, false);
                assert!(msg_is_wellformed__developer_gumbox(bytes));
                assert!(msg_is_mav_cmd_flash_bootloader__developer_gumbox(bytes));
                assert!(!can_send(bytes));
            }
        }
    }

    #[test]
    fn other_commands_and_unknown_messages_remain_allowed() {
        for version in [1, 2] {
            assert!(can_send(command(version, 76, 400, false)));
            assert!(can_send(command(version, 42, 42650, false)));
        }
    }

    #[test]
    fn signed_v2_requires_the_signature_bytes() {
        let bytes = command(2, 76, 42650, true);
        assert!(parse_mavlink_msg(&bytes[..55]).is_ok());
        assert!(parse_mavlink_msg(&bytes[..54]).is_err());
        assert!(!can_send(bytes));
    }

    #[test]
    fn malformed_magic_and_truncated_payload_are_rejected() {
        let mut bytes = command(1, 76, 400, false);
        assert!(parse_mavlink_msg(&bytes[..35]).is_err());
        bytes[0] = 0;
        assert!(!msg_is_wellformed__developer_gumbox(bytes));
        assert!(!can_send(bytes));
    }
}
