// NONOS Operating System (AGPL-3.0-or-later)

//! Drive the firmware download on real hardware. This is the thin plumbing the
//! provable download core cannot be: it maps the two DMA regions the reserved
//! page path needs (a staging buffer for one chunk plus its TX descriptor, and
//! the beacon-queue buffer-descriptor ring), points the card at the ring, then
//! brackets `fw::download` with the prologue and epilogue. The firmware image is
//! embedded so the capsule carries its own blob. Everything downstream of the
//! `stage` closure is proven off-silicon in `rtl8821ce_proofs`; the DMA mapping
//! and the card actually running the firmware are the on-silicon boundary.

use super::image::{check_image, FW};
use super::map::map_regions;
use super::say::{finish, say_download};
use crate::fw::dma::DmaMem;
use crate::fw::regs::REG_TXBD_DESA_BCNQ;
use crate::fw::{download, prep, rsvd, DownloadError};
use crate::regs::{Mmio, Regs};

/// Why the firmware download could not be set up or completed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FwLoadError {
    /// The embedded image is not an RTL8821C firmware, or is malformed.
    BadImage,
    /// A DMA region could not be mapped through the broker.
    Dma,
    /// The download itself failed; carries the stage it stopped at.
    Download(DownloadError),
    /// The firmware never reported ready after the sections were delivered.
    NotReady,
}

/// Download the embedded firmware into the powered chip. `device_id` and
/// `claim_epoch` are the same claim the register window was mapped under.
pub fn load(regs: &Regs, device_id: u64, claim_epoch: u64) -> Result<(), FwLoadError> {
    check_image()?;
    let (stage_buf, ring) = map_regions(device_id, claim_epoch)?;

    // Burst sizes first, as rtw88 sets them before the ring addresses (pci.c:403).
    crate::mac::set_dma_burst(regs);
    // Point the card at the beacon-queue ring before the first kick reads it.
    regs.write32(REG_TXBD_DESA_BCNQ, ring.device_addr() as u32);
    regs.write32(REG_TXBD_DESA_BCNQ + 4, (ring.device_addr() >> 32) as u32);

    let backup = prep::begin(regs);
    let result = download(regs, FW, |chunk| rsvd::stage_chunk(regs, &stage_buf, &ring, chunk));
    if let Err(e) = result {
        prep::abort(regs, backup);
        say_download(e);
        return Err(FwLoadError::Download(e));
    }
    finish(prep::finish(regs, backup))
}
