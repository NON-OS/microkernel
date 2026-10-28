// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_core_start` (Linux v6.12 rtw89 `core.c`) after the probe: the
//! MAC init, the PHY tables, the WL-only coexistence set-up
//! (`rtw89_btc_ntfy_init`'s chip part), and the DM init. rtw89 then starts
//! the HCI's interrupts; this driver polls its rings instead.

use super::dm_init::dm_init;
use super::mac_init::mac_init;
use super::phy_init::{phy_init, phy_of};
use super::probe::Probe;
use super::Stop;
use crate::coex;
use crate::dma::DmaMem;
use crate::h2c::Fw;
use crate::phy::state::Phy;
use crate::regs::Mmio;
use crate::rfk::Rfk;
use crate::ring::Rings;

/// What the running chip keeps from its start.
pub struct Started {
    pub phy: Phy,
    pub rfk: Rfk,
    /// Whether the firmware took every RF register page.
    pub rf_pages_sent: bool,
}

pub fn start<M: DmaMem>(
    r: &impl Mmio,
    rings: &mut Rings<M>,
    fw: &mut Fw,
    p: &Probe,
) -> Result<Started, Stop> {
    mac_init(r, rings, fw, p)?;
    let mut phy = phy_of(p);
    let rf_pages_sent = phy_init(r, &mut rings.ch12, fw, &mut phy)?;
    coex::init(r, phy.rfe).map_err(|_| Stop::Coex)?;
    let mut rfk = Rfk::default();
    dm_init(r, &mut rings.ch12, fw, &mut phy, &mut rfk, p)?;
    Ok(Started { phy, rfk, rf_pages_sent })
}
