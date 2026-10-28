// NONOS Operating System (AGPL-3.0-or-later)

//! Tune both RF paths to a channel: rtw88 `rtw8822c_set_channel`
//! (rtw8822c.c:2426): the baseband for the channel (`bb`), the MAC's channel
//! fields (`mac`), RF register 0x18 on both paths (`rf`), then the IGI
//! toggled. This is the whole retune, so the serve loop's `set_rf` on each
//! scan hop does all of it: the baseband's spur and filter settings differ
//! between 2.4 GHz channels on this chip (11 to 14 above all).

pub mod bb;
mod mac;
pub mod rf18;

use super::igi::toggle_igi;
use crate::regs::Mmio;

/// Channel bandwidth. The radio parks on 20 MHz channels.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bw {
    W20,
}

/// Retune to the 2.4 GHz `channel` at `bw`. A channel outside 1..=14 is
/// left alone: this driver runs no 5 GHz band.
pub fn set_rf<M: Mmio>(mmio: &M, channel: u8, bw: Bw) {
    if !(1..=14).contains(&channel) {
        return;
    }
    let Bw::W20 = bw;
    bb::set_2g_20(mmio, channel);
    mac::set_20(mmio);
    rf18::set(mmio, channel);
    toggle_igi(mmio);
}
