use core::cell::Cell;

use crate::init::BusSetup;
use crate::regs::Regs;
use crate::transaction::{BusState, Reach};

pub struct Driver {
    pub device_id: u64,
    pub pci_device: u16,
    pub claim_epoch: u64,
    pub mmio_grant: u64,
    /// The input clock, window and bus speed the core was brought up with,
    /// kept so a recovery can bring it up again the same way.
    pub setup: BusSetup,
    pub family: &'static str,
    pub comp_type: u32,
    pub comp_param: u32,
    /// FIFO depths the core reported at bring-up. The transfer engine pushes
    /// and issues against these, never against an assumed size.
    pub tx_depth: u32,
    pub rx_depth: u32,
    pub enabled: u32,
    pub status: u32,
    /// True when this controller was bound because the touchpad's address
    /// ACKed the setup probe; false for a fallback bind (named controller or
    /// first-fit) where the device never answered during setup.
    pub bound_by_probe: bool,
    /// The address the bind settled on: the declared one, or for a
    /// declaration with none, the first the sweep found. Zero when none.
    pub bound_addr: u8,
    /// The HID descriptor register the probe found the descriptor at, or the
    /// firmware's declaration when nothing answered yet.
    pub bound_desc_reg: u16,
    /// The bound declaration's interrupt pin and HID_INFO bits.
    pub bound_pin: u16,
    pub bound_info: u8,
    /// The addresses a client's transfer may reach (setup/sequence/settle.rs);
    /// it narrows once the pad has answered (transaction/types/narrow.rs).
    pub reach: Cell<Reach>,
    /// How many times OP_ACPI_HID was asked: each ask names the next address
    /// in reach (transaction/types/named.rs).
    pub acpi_hid_asks: Cell<usize>,
    /// Whether the bus is working, failing or stuck, between transfers.
    pub bus: Cell<BusState>,
    /// The touchpad's interrupt line, sensed for interrupt-paced reads, when
    /// the platform's GPIO layout is one this driver knows.
    pub doorbell: Option<Doorbell>,
    pub regs: Regs,
}

/// The register holding the level of the touchpad's GpioInt pin.
#[derive(Clone, Copy)]
pub struct Doorbell {
    pub regs: Regs,
    pub cfg_offset: u64,
    /// The bit of that register holding the raw line level: PADCFG0
    /// GPIORXSTATE on Intel, PIN_STS on AMD.
    pub level_bit: u32,
    /// The line is asserted high (GpioInt ActiveHigh); i2c-HID pads are
    /// almost always active low.
    pub active_high: bool,
}

impl Doorbell {
    /// True when register value `value` says a report waits: the level bit
    /// is the line before any inversion (GPIORXSTATE, PIN_STS), so the
    /// declared polarity decides which level is "asserted".
    pub fn asserted(&self, value: u32) -> bool {
        (value & self.level_bit != 0) == self.active_high
    }
}
