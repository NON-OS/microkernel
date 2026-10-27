// NONOS Operating System (AGPL-3.0-or-later)

//! The WiFi CPU's firmware: linux-firmware's `rtw89/rtw8852b_fw-1.bin`
//! (format 1, firmware 0.29.29.15; the file Linux v6.12 asks for first,
//! `RTW8852B_FW_FORMAT_MAX` 1), linked in and pinned by sha256 in
//! `scripts/baselines/upstream.sha256`. Its suits use header format 0,
//! which `header` parses; nothing of the file's format-1 container is
//! needed beyond the entry table `mfw` reads.

pub mod cpu;
mod download;
pub mod header;
pub mod mfw;

pub use download::{CHUNK, FWHDR_DL, INIT_RDY};

use crate::dma::DmaMem;
use crate::h2c::send::SendFail;
use crate::h2c::Fw;
use crate::regs::Mmio;
use crate::ring::tx::TxRing;

pub const IMAGE: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/realtek/rtw8852b_fw-1.bin");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FwFail {
    /// The file holds no normal suit for this cut.
    NoSuit,
    BadHeader,
    /// The CPU was still running when the download began.
    CpuBusy,
    H2cPath,
    FwdlPath,
    Send(SendFail),
    /// The firmware's status after the download: 2 checksum, 3 security,
    /// 4 cut mismatch, anything else not ready.
    Status(u8),
}

/// The suit this chip runs, and its header.
pub fn prepare(file: &[u8], cv: u8) -> Result<(&[u8], header::Header), FwFail> {
    let s = mfw::select_normal(file, cv).ok_or(FwFail::NoSuit)?;
    let suit = &file[s.offset..s.offset + s.size];
    let hdr = header::parse(suit).ok_or(FwFail::BadHeader)?;
    Ok((suit, hdr))
}

/// `rtw89_fw_download`: the whole download, tried five times.
pub fn download<M: DmaMem>(
    r: &impl Mmio,
    ring: &mut TxRing<M>,
    fw: &mut Fw,
    suit: &[u8],
    hdr: &header::Header,
) -> Result<(), FwFail> {
    let mut last = FwFail::CpuBusy;
    for _ in 0..5 {
        match download::once(r, ring, fw, suit, hdr) {
            Ok(()) => return Ok(()),
            Err(e) => last = e,
        }
    }
    Err(last)
}
