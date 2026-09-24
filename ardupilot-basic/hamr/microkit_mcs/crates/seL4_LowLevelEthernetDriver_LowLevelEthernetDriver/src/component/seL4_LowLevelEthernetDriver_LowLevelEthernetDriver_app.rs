// This file will not be overwritten if HAMR codegen is rerun

use data::*;
use crate::bridge::seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_api::*;
use vstd::prelude::*;
use log::{trace, debug};

#[cfg(feature = "zcu102")]
use sel4_driver_interfaces::HandleInterrupt;
#[cfg(feature = "zcu102")]
use sel4_microkit_base::memory_region_symbol;
#[cfg(feature = "zcu102")]
use smoltcp::{
    phy::{Device, RxToken, TxToken},
    time::Instant,
};

#[cfg(feature = "zcu102")]
use eth_driver_core::{DmaDef, Driver};

mod config;

#[cfg(feature = "zcu102")]
type PlatformDriver = Driver;

#[cfg(feature = "qemu-virtio")]
pub struct QemuVirtioDriver;

#[cfg(feature = "qemu-virtio")]
type PlatformDriver = QemuVirtioDriver;

#[verus_verify]
pub struct seL4_LowLevelEthernetDriver_LowLevelEthernetDriver {
  // PLACEHOLDER MARKER STATE VARS
  drv: PlatformDriver,
}

#[verus_verify]
impl seL4_LowLevelEthernetDriver_LowLevelEthernetDriver {
  pub fn new() -> Self
  {
    Self {
      // PLACEHOLDER MARKER STATE VAR INIT
      drv: new_driver(),
    }
  }

  #[verus_spec(
    ensures
      // PLACEHOLDER MARKER INITIALIZATION ENSURES
  )]
  pub fn initialize<API: seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Put_Api> (
    &mut self,
    api: &mut seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Application_Api<API>)
  {
    initialize_driver(&mut self.drv);
  }

  #[verus_spec(
    requires
      // PLACEHOLDER MARKER TIME TRIGGERED REQUIRES
    ensures
      // PLACEHOLDER MARKER TIME TRIGGERED ENSURES
  )]
  pub fn timeTriggered<API: seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Full_Api> (
    &mut self,
    api: &mut seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Application_Api<API>)
  {
    poll_driver(&mut self.drv, api);
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

const NUM_MSGS: usize = 4;

fn get_tx<API: seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Get_Api>(
    idx: usize,
    api: &mut seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Application_Api<API>,
) -> Option<SW::SizedEthernetMessage_Impl> {
    match idx {
        0 => api.get_EthernetFramesTx0(),
        1 => api.get_EthernetFramesTx1(),
        2 => api.get_EthernetFramesTx2(),
        3 => api.get_EthernetFramesTx3(),
        _ => None,
    }
}

fn put_rx<API: seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Put_Api>(
    idx: usize,
    rx_buf: &[u8],
    api: &mut seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Application_Api<API>,
) {
    if rx_buf.len() > SW::SW_RawEthernetMessage_DIM_0 {
        return;
    }
    let mut value: SW::RawEthernetMessage = [0; SW::SW_RawEthernetMessage_DIM_0];
    value[..rx_buf.len()].copy_from_slice(rx_buf);
    match idx {
        0 => api.put_EthernetFramesRx0(value),
        1 => api.put_EthernetFramesRx1(value),
        2 => api.put_EthernetFramesRx2(value),
        3 => api.put_EthernetFramesRx3(value),
        _ => (),
    }
}

// The original microkit implementation keeps hardware/DMA operations outside
// Verus. These adapters preserve that trusted boundary; the generated component
// entrypoints and their contract markers above remain verified and unchanged.
// No functional guarantees about the hardware driver are assumed here.
verus! {
    #[cfg(feature = "zcu102")]
    #[verifier::external_type_specification]
    #[verifier::external_body]
    pub struct ExternalDriver(Driver);

    #[verifier::external_body]
    fn new_driver() -> PlatformDriver {
        create_driver()
    }

    #[verifier::external_body]
    fn initialize_driver(drv: &mut PlatformDriver) {
        #[cfg(feature = "sel4")]
        log_info("initialize entrypoint invoked");

        #[cfg(feature = "zcu102")]
        {
        drv.handle_interrupt();
        log::info!("Acked driver IRQ");
        }

        #[cfg(feature = "qemu-virtio")]
        {
            let _ = drv;
            qemu_driver_init();
        }
    }

    #[verifier::external_body]
    fn poll_driver<API: seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Full_Api>(
        drv: &mut PlatformDriver,
        api: &mut seL4_LowLevelEthernetDriver_LowLevelEthernetDriver_Application_Api<API>,
    ) {
        #[cfg(feature = "sel4")]
        trace!("compute entrypoint invoked");

        #[cfg(feature = "zcu102")]
        {
        for i in 0..NUM_MSGS {
            if let Some((rx_tok, _tx_tok)) = drv.receive(Instant::ZERO) {
                rx_tok.consume(|rx_buf| {
                    debug!("RX Packet: {:?}", &rx_buf[0..64]);
                    put_rx(i, rx_buf, api);
                });
            }
        }

        for i in 0..NUM_MSGS {
            if let Some(sz_pkt) = get_tx(i, api) {
                let size = sz_pkt.sz as usize;
                if size > 0 && size <= sz_pkt.amessage.len() {
                    // warn!("TX Packet: {:0>2X?}", &sz_pkt.message[0..size]);
                    debug!("TX Packet");
                    if let Some(tx_tok) = drv.transmit(Instant::ZERO) {
                        trace!("Valid tx token");
                        tx_tok.consume(size, |tx_buf| {
                            tx_buf.copy_from_slice(&sz_pkt.amessage[0..size]);
                            trace!("Copied from tmp to tx_buf");
                        });
                    };
                }
            }
        }

        drv.handle_interrupt();
        }

        #[cfg(feature = "qemu-virtio")]
        {
            let _ = drv;

            for i in 0..NUM_MSGS {
                let mut rx_buf: SW::RawEthernetMessage =
                    [0; SW::SW_RawEthernetMessage_DIM_0];
                let size = qemu_driver_receive(&mut rx_buf);
                if size == 0 {
                    break;
                }
                debug!("RX Packet: {} bytes", size);
                put_rx(i, &rx_buf[..size], api);
            }

            for i in 0..NUM_MSGS {
                if let Some(sz_pkt) = get_tx(i, api) {
                    let size = sz_pkt.sz as usize;
                    if size > 0 && size <= sz_pkt.amessage.len() {
                        debug!("TX Packet: {} bytes", size);
                        let _ = qemu_driver_send(&sz_pkt.amessage[..size]);
                    }
                }
            }
        }
    }
}

// Keep linker-symbol macros outside Verus's annotated items: their generated
// extern statics currently trigger a Verus erasure/lifetime-checking ICE.
#[cfg(feature = "zcu102")]
fn create_driver() -> PlatformDriver {
    {
        let dma = DmaDef {
            vaddr: memory_region_symbol!(net_driver_dma_vaddr: *mut ()),
            paddr: memory_region_symbol!(net_driver_dma_paddr: *mut ()),
            size: config::sizes::DRIVER_DMA,
        };
        Driver::new(
            memory_region_symbol!(gem_register_block: *mut ()).as_ptr(),
            dma,
        )
    }
}

#[cfg(feature = "qemu-virtio")]
fn create_driver() -> PlatformDriver {
    QemuVirtioDriver
}

#[cfg(feature = "qemu-virtio")]
extern "C" {
    fn qemu_virtio_net_init();
    fn qemu_virtio_net_receive(destination: *mut u8, capacity: usize) -> usize;
    fn qemu_virtio_net_send(source: *const u8, length: usize) -> bool;
}

#[cfg(feature = "qemu-virtio")]
fn qemu_driver_init() {
    unsafe { qemu_virtio_net_init() };
}

#[cfg(feature = "qemu-virtio")]
fn qemu_driver_receive(destination: &mut [u8]) -> usize {
    unsafe { qemu_virtio_net_receive(destination.as_mut_ptr(), destination.len()) }
}

#[cfg(feature = "qemu-virtio")]
fn qemu_driver_send(source: &[u8]) -> bool {
    unsafe { qemu_virtio_net_send(source.as_ptr(), source.len()) }
}
