// NONOS Operating System (AGPL-3.0-or-later)
//! How the model port departs from a well-behaved one.

use std::time::Instant;

/// Whether PxCMD.CR clears when ST is cleared.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrStuck {
    No,
    /// Not until a COMRESET frees the port.
    UntilReset,
    Always,
}

/// How the port departs from a well-behaved one.
pub struct Quirks {
    /// The HBA sets PxTFD.BSY as it sends a command FIS, as Intel parts do;
    /// otherwise PxTFD changes only when a FIS arrives, as on QEMU.
    pub bsy_on_send: bool,
    pub cr_stuck: CrStuck,
    /// How long the PHY takes to talk again after a COMRESET.
    pub link_ms: u64,
    /// The drive is busy until then, from the first look on, and a COMRESET
    /// does not shorten it: PxTFD shows BSY until the drive posts its ready
    /// status. CLO clears that copy, and nothing sets it again.
    pub busy_until: Option<Instant>,
    /// How long the drive takes, once its link is back after a COMRESET, to
    /// come out of the reset and post its signature: busy until then too.
    pub reset_ms: Option<u64>,
    /// PRDBC as the HBA writes it, whatever it moved.
    pub prdbc: Option<u32>,
    /// PxIS error bits the HBA posts as it sends the next command FIS, as
    /// on an interface or host bus error, leaving the slot set and PxTFD as
    /// it was; the drive has the command all the same.
    pub fatal_on_send: Option<u32>,
    /// The HBA has CAP.SCLO, so a CLO write clears its copy of BSY and DRQ;
    /// without it the bit is reserved and does nothing.
    pub sclo: bool,
    /// The drive resets itself this long after it takes a command, as on a
    /// firmware fault or a power dip: it drops the command and sends
    /// COMINIT, and the link comes back as after a COMRESET.
    pub self_reset_ms: Option<u64>,
    /// Slots the port shows issued and queued whatever the driver does, as
    /// a broken part or firmware sharing the port would.
    pub sticky: (u32, u32),
    /// No COMRESET brings the link back: PxSSTS.DET stays at this, 0 for a
    /// disk pulled, 1 for a device the PHY cannot talk to.
    pub stuck_det: Option<u32>,
}

impl Default for Quirks {
    fn default() -> Self {
        Self {
            bsy_on_send: false,
            cr_stuck: CrStuck::No,
            link_ms: 10,
            busy_until: None,
            prdbc: None,
            fatal_on_send: None,
            reset_ms: None,
            sclo: true,
            stuck_det: None,
            self_reset_ms: None,
            sticky: (0, 0),
        }
    }
}
