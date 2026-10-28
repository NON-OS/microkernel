// NONOS Operating System (AGPL-3.0-or-later)

//! `__rtw89_fw_download` (Linux v6.12 rtw89 `fw.c`): the CPU restarted in
//! download mode, the H2C path seen ready, the header sent as an H2C
//! (`__rtw89_fw_download_hdr`, the part size stamped 2020) and the
//! download path seen ready, each section in 2020-byte chunks
//! (`__rtw89_fw_download_main`), then the firmware's own "init ready"
//! awaited.

use super::cpu::{self, *};
use super::header::Header;
use super::FwFail;
use crate::dma::DmaMem;
use crate::h2c::packet::{h2c_hdr, Cmd, H2C_HDR};
use crate::h2c::send::post;
use crate::h2c::Fw;
use crate::regs::{get, Mmio};
use crate::ring::tx::TxRing;
use crate::wait::{delay_ms, until_us};

/// `FWDL_SECTION_PER_PKT_LEN`.
pub const CHUNK: usize = 2020;
/// `FWDL_WAIT_CNT`, in microseconds.
pub const WAIT_US: u64 = 400_000;
/// `RTW89_FWDL_WCPU_FW_INIT_RDY`.
pub const INIT_RDY: u8 = 7;
pub const FWHDR_DL: Cmd = Cmd { cat: 1, class: 3, func: 0 };
/// One attempt; `super::download` makes rtw89's five.
pub fn once<M: DmaMem>(
    r: &impl Mmio,
    ring: &mut TxRing<M>,
    fw: &mut Fw,
    suit: &[u8],
    hdr: &Header,
) -> Result<(), FwFail> {
    cpu::disable(r);
    if !cpu::enable_for_download(r) {
        return Err(FwFail::CpuBusy);
    }
    let path = |bit| until_us(WAIT_US, || r.read8(R_AX_WCPU_FW_CTRL) as u32 & bit != 0);
    if !path(B_AX_H2C_PATH_RDY) {
        return Err(FwFail::H2cPath);
    }
    let mut pkt = [0u8; H2C_HDR + 512];
    let n = hdr.sent_len;
    pkt[..H2C_HDR].copy_from_slice(&h2c_hdr(FWHDR_DL, fw.h2c_seq, n, false, false));
    pkt[H2C_HDR..H2C_HDR + n].copy_from_slice(&suit[..n]);
    pkt[H2C_HDR + 28..H2C_HDR + 30].copy_from_slice(&(CHUNK as u16).to_le_bytes());
    post(r, ring, &pkt[..H2C_HDR + n], false).map_err(FwFail::Send)?;
    if !path(B_AX_FWDL_PATH_RDY) {
        return Err(FwFail::FwdlPath);
    }
    r.write32(R_AX_HALT_H2C_CTRL, 0);
    r.write32(R_AX_HALT_C2H_CTRL, 0);
    for s in &hdr.sections[..hdr.count] {
        for chunk in suit[s.offset..s.offset + s.len].chunks(CHUNK) {
            post(r, ring, chunk, true).map_err(FwFail::Send)?;
        }
    }
    *fw = Fw::default();
    delay_ms(5);
    let sts = || get(r.read8(R_AX_WCPU_FW_CTRL) as u32, B_AX_WCPU_FWDL_STS_MASK) as u8;
    if until_us(WAIT_US, || sts() == INIT_RDY) {
        return Ok(());
    }
    Err(FwFail::Status(sts()))
}
