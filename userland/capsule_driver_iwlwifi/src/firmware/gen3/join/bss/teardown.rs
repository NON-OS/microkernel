// NONOS Operating System (AGPL-3.0-or-later)

//! The contexts taken down as far as they went.

use super::super::super::cmds::{LINK_CONFIG_CMD, LONG_GROUP, MAC_CONFIG_CMD, MAC_CONF_GROUP};
use super::super::super::region::{Clock, Region};
use super::super::super::station::ids::{
    DATA_PATH_GROUP, PHY_CONTEXT_CMD, SCD_QUEUE_CONFIG_CMD, STA_REMOVE_CMD, TXPATH_FLUSH,
};
use super::super::super::station::link::{link_config, MODIFY_ACTIVE};
use super::super::super::station::mac::mac_config;
use super::super::super::station::phy::{phy_context, ACTION_MODIFY, ACTION_REMOVE, PHY_ID};
use super::super::super::station::queue::{flush_sta, remove_queue, DATA_TID, MGMT_TID};
use super::super::super::station::sta::{sta_remove, AP_STA_MASK};
use super::super::fw::Fw;
use super::{Bss, Stage};
use crate::regs::Mmio;

impl Bss {
    /// Take the contexts down as far as they went (best effort).
    pub fn teardown<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
    ) {
        self.unprotect(fw);
        if self.stage >= Stage::Associated {
            let _ = fw.command(
                MAC_CONF_GROUP,
                MAC_CONFIG_CMD,
                &mac_config(ACTION_MODIFY, self.addr, None),
            );
            let _ = self.power(fw, false);
        }
        if self.stage >= Stage::MgmtQueue {
            let _ = fw.command(LONG_GROUP, TXPATH_FLUSH, &flush_sta());
        }
        if self.stage >= Stage::DataQueue {
            let _ = fw.command(
                DATA_PATH_GROUP,
                SCD_QUEUE_CONFIG_CMD,
                &remove_queue(AP_STA_MASK, DATA_TID),
            );
        }
        if self.stage >= Stage::MgmtQueue {
            let _ = fw.command(
                DATA_PATH_GROUP,
                SCD_QUEUE_CONFIG_CMD,
                &remove_queue(AP_STA_MASK, MGMT_TID),
            );
        }
        fw.q.mgmt.live = false;
        fw.q.data.live = false;
        if self.stage >= Stage::Station {
            let _ = fw.command(MAC_CONF_GROUP, STA_REMOVE_CMD, &sta_remove());
        }
        if self.stage >= Stage::LinkActive {
            let off = link_config(ACTION_MODIFY, &self.link(Some(PHY_ID), false, MODIFY_ACTIVE, 0));
            let _ = fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &off);
        }
        if self.stage >= Stage::Phy {
            let _ = fw.command(
                LONG_GROUP,
                PHY_CONTEXT_CMD,
                &phy_context(ACTION_REMOVE, self.target.channel),
            );
        }
        self.stage = Stage::Down;
        fw.inbox.frames.clear();
        fw.inbox.session = None;
    }
}
