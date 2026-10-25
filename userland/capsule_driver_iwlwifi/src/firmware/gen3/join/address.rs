// NONOS Operating System (AGPL-3.0-or-later)

//! A join under the network's own address. Bring-up added the firmware's MAC
//! and link contexts under the boot's address. Linux never changes the
//! address of an interface that is up: it removes the interface and adds it
//! again. So does this before each join: the link removed, then the MAC, then
//! both added again under the network's address, as bring-up adds them.

use alloc::vec;

use super::super::cmds::MAC_CONF_GROUP;
use super::super::cmds::{link_config_add, mac_config_add, LINK_CONFIG_CMD, MAC_CONFIG_CMD};
use super::super::region::{Clock, Region};
use super::super::station::link::{CTXT_INVALID, LINK_CONFIG_LEN, LINK_ID, MAC_ID};
use super::super::station::phy::ACTION_REMOVE;
use super::bss::SetupError;
use super::exchange::Progress;
use super::fw::{CommandFailed, Fw};
use super::run::{join, JoinEnd, Joined};
use crate::regs::Mmio;
use nonos_wifi_core::mlme::JoinRequest;

/// Put the station's contexts under `addr`.
pub fn readdress<M: Mmio, R: Region + ?Sized, C: Clock>(
    fw: &mut Fw<'_, '_, M, R, C>,
    addr: [u8; 6],
) -> Result<(), CommandFailed> {
    let mut link = vec![0u8; LINK_CONFIG_LEN];
    for (at, v) in [(0, ACTION_REMOVE), (4, LINK_ID), (8, MAC_ID), (12, CTXT_INVALID)] {
        link[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }
    fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &link)?;
    let mut mac = [0u8; 52];
    mac[0..4].copy_from_slice(&MAC_ID.to_le_bytes());
    mac[4..8].copy_from_slice(&ACTION_REMOVE.to_le_bytes());
    fw.command(MAC_CONF_GROUP, MAC_CONFIG_CMD, &mac)?;
    fw.command(MAC_CONF_GROUP, MAC_CONFIG_CMD, &mac_config_add(addr))?;
    fw.command(MAC_CONF_GROUP, LINK_CONFIG_CMD, &link_config_add(addr))?;
    Ok(())
}

/// `join`, under `req.our_mac`: the contexts readdressed first.
pub fn join_as<M: Mmio, R: Region + ?Sized, C: Clock>(
    fw: &mut Fw<'_, '_, M, R, C>,
    req: &JoinRequest<'_>,
    beacon: &[u8],
    heard_on: u8,
    tx_ant: u8,
    rx_ant: u8,
    progress: &mut Progress,
) -> Result<Joined, JoinEnd> {
    readdress(fw, req.our_mac).map_err(|c| JoinEnd::Setup(SetupError::Command(c)))?;
    join(fw, req, beacon, heard_on, tx_ant, rx_ant, progress)
}
