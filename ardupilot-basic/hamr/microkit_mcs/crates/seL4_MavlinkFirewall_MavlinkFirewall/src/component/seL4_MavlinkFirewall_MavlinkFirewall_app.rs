// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_MavlinkFirewall_MavlinkFirewall_api::*;
use vstd::prelude::*;

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
    log_info("initialize entrypoint invoked");
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
    // This default implementation returns `true`, which is safe but weak:
    // * In `assume` contexts, returning `false` may allow Verus to prove `false`.
    // * To obtain meaningful guarantees, developers should strengthen this
    //   specification to reflect the intended semantics of the GUMBO spec function.
    true
  }

  /// Developer-supplied Verus realization of the GUMBO spec function `test`.
  /// 
  /// This function may be freely refined as long as it remains a pure Verus `spec fn`.
  pub open spec fn msg_is_mav_cmd_flash_bootloader__developer_verus(msg: SW::UdpPayload) -> (res: bool)
  {
    // This default implementation returns `true`, which is safe but weak:
    // * In `assume` contexts, returning `false` may allow Verus to prove `false`.
    // * To obtain meaningful guarantees, developers should strengthen this
    //   specification to reflect the intended semantics of the GUMBO spec function.
    true
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
    // This default implementation returns `true`, which is safe but weak:
    // * In `assume` contexts, returning `false` may allow GUMBOX to prove `false`.
    // * To obtain meaningful guarantees, developers should strengthen this
    //   specification to reflect the intended semantics of the GUMBO spec function.
    true
  }

  /// Developer-supplied GUMBOX realization of the GUMBO spec function `test`.
  /// 
  /// This function may be freely refined.
  pub exec fn msg_is_mav_cmd_flash_bootloader__developer_gumbox(msg: SW::UdpPayload) -> (res: bool)
    ensures
      res == msg_is_mav_cmd_flash_bootloader__developer_verus(msg),
  {
    // This default implementation returns `true`, which is safe but weak:
    // * In `assume` contexts, returning `false` may allow GUMBOX to prove `false`.
    // * To obtain meaningful guarantees, developers should strengthen this
    //   specification to reflect the intended semantics of the GUMBO spec function.
    true
  }

}
