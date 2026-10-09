// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The firmware's contexts for one access point, put up before the first
//! frame goes out and taken down in reverse, in the order Linux v6.12 runs
//! them for a station on the MLD API:
//!
//! 1. the PHY context on the channel, then its receive chains (RLC)
//!    (`iwl_mvm_phy_ctxt_add`);
//! 2. the link pointed at that PHY, then activated with the BSS's ACK rates
//!    (`__iwl_mvm_mld_assign_vif_chanctx`);
//! 3. the access point as station 0 (`iwl_mvm_mld_add_sta`), with `mfp` set
//!    until it is authorized;
//! 4. its management and data queues (`iwl_mvm_tvqm_enable_txq`), the ring
//!    starting at the write pointer each reply names;
//! 5. session protection, waited for until the firmware says it is on the
//!    channel (`iwl_mvm_mac_mgd_prepare_tx`; this driver waits, so the
//!    authentication frame is not queued for a radio still elsewhere).
//!
//! After association the MAC context is marked associated with the AID, the
//! station entry takes the AID, and the link the ERP preamble and slot of
//! the association response's capability (`iwl_mvm_mld_vif_cfg_changed_station`,
//! `BSS_CHANGED_ERP_SLOT`). Once the keys are in, the station entry takes
//! the negotiated `mfp` and session protection is cancelled. A context a
//! failed step left half up is taken down exactly as far as it went: every
//! step records itself in `stage` only once the firmware answered it.
//! Teardown is best effort (a firmware that stopped answering cannot be made
//! to), and flushes the station's queues before removing them, as
//! `iwl_mvm_mld_rm_sta` does.

use super::super::cmds::{LINK_CONFIG_CMD, LONG_GROUP, MAC_CONFIG_CMD, MAC_CONF_GROUP};
use super::super::station::ids::{
    DATA_PATH_GROUP, PHY_CONTEXT_CMD, RLC_CONFIG_CMD, SCD_QUEUE_CONFIG_CMD, SESSION_PROTECTION_CMD,
    STA_CONFIG_CMD, STA_REMOVE_CMD, TXPATH_FLUSH,
};
use super::super::station::link::{link_config, LinkParams, LINK_ID, MODIFY_ACTIVE, MODIFY_RATES_INFO};
use super::super::station::mac::mac_config;
use super::super::station::phy::{phy_context, rlc_config, ACTION_ADD, ACTION_MODIFY, ACTION_REMOVE, PHY_ID};
use super::super::station::queue::{add_queue, flush_sta, parse_queue_reply, remove_queue, DATA_TID, MGMT_TID};
use super::super::station::session::{session_protection, Session, JOIN_SESSION_MS};
use super::super::station::sta::{sta_config, sta_remove, AP_STA_MASK};
use super::super::region::{Clock, Region};
use super::super::txq::TXQ_ENTRIES;
use super::fw::{CommandFailed, Fw};
use super::target::Target;
use crate::regs::Mmio;

/// How long the firmware may take to put the radio on the channel.
pub const SESSION_WAIT_MS: u32 = 1000;

/// `WLAN_CAPABILITY_SHORT_PREAMBLE` and `_SHORT_SLOT_TIME`.
const CAP_SHORT_PREAMBLE: u16 = 1 << 5;
const CAP_SHORT_SLOT: u16 = 1 << 10;

/// How far the contexts are up, each step only once the firmware answered.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Stage {
    Down,
    Phy,
    LinkActive,
    Station,
    MgmtQueue,
    DataQueue,
    OnChannel,
    Associated,
}

/// Why the contexts could not be put up.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetupError {
    Command(CommandFailed),
    /// A queue reply that was not the 8-byte structure.
    BadQueueReply,
    /// The firmware refused the session, or never put the radio on channel.
    NoSession(Option<Session>),
    /// The transmit region could not be cleared.
    Region,
}

impl From<CommandFailed> for SetupError {
    fn from(c: CommandFailed) -> Self {
        SetupError::Command(c)
    }
}

/// The contexts for one access point.
pub struct Bss {
    pub target: Target,
    /// The station's own address.
    pub addr: [u8; 6],
    /// The antennas the PHY configuration and the NVM both allow.
    pub tx_ant: u8,
    pub rx_ant: u8,
    pub stage: Stage,
    /// The association id, once associated.
    pub aid: u16,
    /// Session protection is running.
    pub session: bool,
}

impl Bss {
    pub fn new(target: Target, addr: [u8; 6], tx_ant: u8, rx_ant: u8) -> Self {
        Self { target, addr, tx_ant, rx_ant, stage: Stage::Down, aid: 0, session: false }
    }

    fn link(&self, phy: Option<u32>, active: bool, modify_mask: u32, capability: u16) -> LinkParams {
        let t = &self.target;
        LinkParams {
            addr: self.addr,
            phy,
            active,
            modify_mask,
            cck_ack: t.rates.cck_ack,
            ofdm_ack: t.rates.ofdm_ack,
            short_preamble: capability & CAP_SHORT_PREAMBLE != 0,
            short_slot: t.channel > 14 || capability & CAP_SHORT_SLOT != 0,
            beacon_int: t.beacon_int,
            dtim_period: t.dtim_period,
        }
    }

    /// Put the contexts up, through session protection.
    pub fn setup<M: Mmio, R: Region + ?Sized, C: Clock>(&mut self, fw: &mut Fw<'_, '_, M, R, C>) -> Result<(), SetupError> {
        let ch = self.target.channel;
        fw.command(LONG_GROUP, PHY_CONTEXT_CMD, &phy_context(ACTION_ADD, ch))?;
        self.stage = Stage::Phy;
        fw.command(DATA_PATH_GROUP, RLC_CONFIG_CMD, &rlc_config(self.rx_ant))?;
        // Before association the link has no ERP state yet (Linux's
        // defaults: long preamble, and a short slot only on 5 GHz).
        let named = link_config(ACTION_MODIFY, &self.link(Some(PHY_ID), false, 0, 0));
        fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &named)?;
        let active = link_config(ACTION_MODIFY, &self.link(Some(PHY_ID), true, MODIFY_ACTIVE | MODIFY_RATES_INFO, 0));
        fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &active)?;
        self.stage = Stage::LinkActive;
        fw.command(MAC_CONF_GROUP, STA_CONFIG_CMD, &sta_config(LINK_ID, self.target.bssid, 0, true))?;
        self.stage = Stage::Station;
        for (tid, stage) in [(MGMT_TID, Stage::MgmtQueue), (DATA_TID, Stage::DataQueue)] {
            let q = if tid == MGMT_TID { &mut fw.q.mgmt } else { &mut fw.q.data };
            if !q.clear(fw.tx) {
                return Err(SetupError::Region);
            }
            let cmd = add_queue(AP_STA_MASK, tid, TXQ_ENTRIES, q.bc(fw.tx), q.tfds(fw.tx));
            let reply = fw.command(DATA_PATH_GROUP, SCD_QUEUE_CONFIG_CMD, &cmd)?;
            let given = parse_queue_reply(&reply).ok_or(SetupError::BadQueueReply)?;
            let q = if tid == MGMT_TID { &mut fw.q.mgmt } else { &mut fw.q.data };
            q.start(given.queue, given.write_ptr);
            self.stage = stage;
        }
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
            Err(why) => Err(SetupError::Command(CommandFailed { group: MAC_CONF_GROUP, cmd: SESSION_PROTECTION_CMD, why })),
        }
    }

    /// Ask for session protection.
    pub fn protect<M: Mmio, R: Region + ?Sized, C: Clock>(&mut self, fw: &mut Fw<'_, '_, M, R, C>) -> Result<(), CommandFailed> {
        fw.command(MAC_CONF_GROUP, SESSION_PROTECTION_CMD, &session_protection(ACTION_ADD, JOIN_SESSION_MS))?;
        self.session = true;
        Ok(())
    }

    /// Associated with `aid`; `capability` is the association response's.
    pub fn associated<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        aid: u16,
        capability: u16,
    ) -> Result<(), CommandFailed> {
        self.aid = aid;
        fw.command(MAC_CONF_GROUP, MAC_CONFIG_CMD, &mac_config(ACTION_MODIFY, self.addr, Some(aid)))?;
        self.stage = Stage::Associated;
        fw.command(MAC_CONF_GROUP, STA_CONFIG_CMD, &sta_config(LINK_ID, self.target.bssid, aid, true))?;
        let erp = link_config(ACTION_MODIFY, &self.link(Some(PHY_ID), true, MODIFY_RATES_INFO, capability));
        fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &erp)?;
        Ok(())
    }

    /// The keys are in: the station entry takes the negotiated protection and
    /// the session is cancelled.
    pub fn authorized<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        pmf: bool,
    ) -> Result<(), CommandFailed> {
        fw.command(MAC_CONF_GROUP, STA_CONFIG_CMD, &sta_config(LINK_ID, self.target.bssid, self.aid, pmf))?;
        self.unprotect(fw);
        Ok(())
    }

    // Cancel session protection if it is running (best effort).
    fn unprotect<M: Mmio, R: Region + ?Sized, C: Clock>(&mut self, fw: &mut Fw<'_, '_, M, R, C>) {
        if self.session {
            self.session = false;
            let _ = fw.command(MAC_CONF_GROUP, SESSION_PROTECTION_CMD, &session_protection(ACTION_REMOVE, 0));
        }
    }

    /// Take the contexts down as far as they went (best effort).
    pub fn teardown<M: Mmio, R: Region + ?Sized, C: Clock>(&mut self, fw: &mut Fw<'_, '_, M, R, C>) {
        self.unprotect(fw);
        if self.stage >= Stage::Associated {
            let _ = fw.command(MAC_CONF_GROUP, MAC_CONFIG_CMD, &mac_config(ACTION_MODIFY, self.addr, None));
        }
        if self.stage >= Stage::MgmtQueue {
            let _ = fw.command(LONG_GROUP, TXPATH_FLUSH, &flush_sta());
        }
        if self.stage >= Stage::DataQueue {
            let _ = fw.command(DATA_PATH_GROUP, SCD_QUEUE_CONFIG_CMD, &remove_queue(AP_STA_MASK, DATA_TID));
        }
        if self.stage >= Stage::MgmtQueue {
            let _ = fw.command(DATA_PATH_GROUP, SCD_QUEUE_CONFIG_CMD, &remove_queue(AP_STA_MASK, MGMT_TID));
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
            let _ = fw.command(LONG_GROUP, PHY_CONTEXT_CMD, &phy_context(ACTION_REMOVE, self.target.channel));
        }
        self.stage = Stage::Down;
        fw.inbox.frames.clear();
        fw.inbox.session = None;
    }
}
