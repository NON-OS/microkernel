// NONOS Operating System (AGPL-3.0-or-later)

//! RF calibration, as far as this driver runs it (Linux v6.12 rtw89
//! `rtw8852b.c`, `rtw8852b_rfk.c`). At bring-up, `rtw8852b_rfk_init`: DPD
//! backoff, RCK on both paths, DACK, RX DCK. On joining a channel, the
//! first half of `rtw8852b_rfk_channel`: RX DCK and IQK. rtw89 then runs
//! TSSI and DPK; this driver does not (TX power stays open loop, the DPD
//! block keeps its init setting), and says so on the status line.
//! rtw89's coexistence notices around each calibration change nothing here:
//! the antenna is already granted to WL alone, as rtw89 grants it to RFK.

pub mod addck;
pub mod dack;
pub mod dack_read;
pub mod drck;
pub mod info;
pub mod iqk;
pub mod rck;
pub mod reg5;

use crate::chan::Chan;
use crate::h2c::Fw;
use crate::mac::sch::{self, SchFail};
use crate::phy::bb::Bb;
use crate::regs::Mmio;
pub use info::DackInfo;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rfk {
    pub dack: DackInfo,
    pub iqk: iqk::Iqk,
}

/// `rtw8852b_rx_dck` with its TX pause.
pub fn rx_dck(r: &impl Mmio, fw: &mut Fw) -> Result<(), SchFail> {
    let was = sch::stop(r, fw)?;
    let bb = Bb(r);
    rck::wait_rx_mode(&bb);
    rck::rx_dck(&bb);
    sch::resume(r, fw, was)
}

/// `rtw8852b_rfk_init`.
pub fn init(r: &impl Mmio, fw: &mut Fw, k: &mut Rfk) -> Result<(), SchFail> {
    let bb = Bb(r);
    rck::dpd_backoff(&bb);
    rck::rck(&bb, 0);
    rck::rck(&bb, 1);
    dack::dac_cal(r, &mut k.dack);
    rx_dck(r, fw)
}

/// RX DCK and IQK for the channel the station is about to join.
pub fn channel(r: &impl Mmio, fw: &mut Fw, k: &mut Rfk, c: Chan) -> Result<(), SchFail> {
    rx_dck(r, fw)?;
    let was = sch::stop(r, fw)?;
    rck::wait_rx_mode(&Bb(r));
    iqk::iqk(r, &mut k.iqk, c);
    sch::resume(r, fw, was)
}
