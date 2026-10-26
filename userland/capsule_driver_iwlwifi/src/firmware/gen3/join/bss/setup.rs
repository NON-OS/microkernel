// NONOS Operating System (AGPL-3.0-or-later)

//! The contexts put up, through session protection.

use super::super::super::cmds::{LINK_CONFIG_CMD, LONG_GROUP, MAC_CONF_GROUP};
use super::super::super::region::{Clock, Region};
use super::super::super::station::ids::{
    DATA_PATH_GROUP, PHY_CONTEXT_CMD, RLC_CONFIG_CMD, SESSION_PROTECTION_CMD, STA_CONFIG_CMD,
};
use super::super::super::station::link::{link_config, LINK_ID, MODIFY_ACTIVE, MODIFY_RATES_INFO};
use super::super::super::station::phy::{
    phy_context, rlc_config, ACTION_ADD, ACTION_MODIFY, PHY_ID,
};
use super::super::super::station::session::Session;
use super::super::super::station::sta::sta_config;
use super::super::fw::{CommandFailed, Fw};
use super::{Bss, SetupError, Stage, SESSION_WAIT_MS};
use crate::regs::Mmio;

impl Bss {
    /// Put the contexts up, through session protection.
    pub fn setup<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
    ) -> Result<(), SetupError> {
        let ch = self.target.channel;
        fw.command(LONG_GROUP, PHY_CONTEXT_CMD, &phy_context(ACTION_ADD, ch))?;
        self.stage = Stage::Phy;
        fw.command(DATA_PATH_GROUP, RLC_CONFIG_CMD, &rlc_config(self.rx_ant))?;
        // Before association the link has no ERP state yet (Linux's
        // defaults: long preamble, and a short slot only on 5 GHz).
        let named = link_config(ACTION_MODIFY, &self.link(Some(PHY_ID), false, 0, 0));
        fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &named)?;
        let active = link_config(
            ACTION_MODIFY,
            &self.link(Some(PHY_ID), true, MODIFY_ACTIVE | MODIFY_RATES_INFO, 0),
        );
        fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &active)?;
        self.stage = Stage::LinkActive;
        fw.command(
            MAC_CONF_GROUP,
            STA_CONFIG_CMD,
            &sta_config(LINK_ID, self.target.bssid, 0, true),
        )?;
        self.stage = Stage::Station;
        self.add_queues(fw)?;
        self.protect(fw)?;
        match fw.wait_session(SESSION_WAIT_MS) {
            Ok(Some(Session::Started)) => {
                self.stage = Stage::OnChannel;
                Ok(())
            }
            Ok(other) => {
                self.session = false;
                Err(SetupError::NoSession(other))
            }
            Err(why) => Err(SetupError::Command(CommandFailed {
                group: MAC_CONF_GROUP,
                cmd: SESSION_PROTECTION_CMD,
                why,
            })),
        }
    }
}
