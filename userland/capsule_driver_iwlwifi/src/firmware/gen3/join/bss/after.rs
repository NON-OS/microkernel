// NONOS Operating System (AGPL-3.0-or-later)

//! Association and authorization marked in the contexts, and session
//! protection cancelled.

use super::super::super::cmds::{LINK_CONFIG_CMD, MAC_CONFIG_CMD, MAC_CONF_GROUP};
use super::super::super::region::{Clock, Region};
use super::super::super::station::ids::STA_CONFIG_CMD;
use super::super::super::station::link::{link_config, LINK_ID, MODIFY_RATES_INFO};
use super::super::super::station::mac::mac_config;
use super::super::super::station::phy::{ACTION_MODIFY, PHY_ID};
use super::super::super::station::sta::sta_config;
use super::super::fw::{CommandFailed, Fw};
use super::{Bss, Stage};
use crate::regs::Mmio;

impl Bss {
    /// Associated with `aid`; `capability` is the association response's.
    pub fn associated<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        aid: u16,
        capability: u16,
    ) -> Result<(), CommandFailed> {
        self.aid = aid;
        fw.command(
            MAC_CONF_GROUP,
            MAC_CONFIG_CMD,
            &mac_config(ACTION_MODIFY, self.addr, Some(aid)),
        )?;
        self.stage = Stage::Associated;
        fw.command(
            MAC_CONF_GROUP,
            STA_CONFIG_CMD,
            &sta_config(LINK_ID, self.target.bssid, aid, true),
        )?;
        let erp = link_config(
            ACTION_MODIFY,
            &self.link(Some(PHY_ID), true, MODIFY_RATES_INFO, capability),
        );
        fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &erp)?;
        self.power(fw, false)
    }

    /// The keys are in: the station entry takes the negotiated protection, the
    /// session is cancelled and power management goes on.
    pub fn authorized<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        pmf: bool,
    ) -> Result<(), CommandFailed> {
        fw.command(
            MAC_CONF_GROUP,
            STA_CONFIG_CMD,
            &sta_config(LINK_ID, self.target.bssid, self.aid, pmf),
        )?;
        self.unprotect(fw);
        self.power(fw, true)
    }
}
