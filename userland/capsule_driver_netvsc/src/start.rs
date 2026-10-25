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

//! The bring-up on an open channel, in the order netvsp's initialize takes
//! it (netvsp lib.rs:5134-5345): INIT for the newest protocol version the
//! host takes; from version 2 on, the NDIS configuration (the MTU), which
//! must come before the receive buffer; the NDIS version; the receive
//! buffer and the send buffer, each a GPADL first; then RNDIS, which netvsp
//! takes once the rest is in place: INITIALIZE, the host-assigned address,
//! the packet filter (without which netvsp drops every frame, lib.rs:5620),
//! and the link.

use nonos_vmbus::{Host, Shared};

use crate::constants::layout::MAX_SECTIONS;
use crate::constants::nvsp::*;
use crate::constants::rndis::{
    FILTER, MEDIA_CONNECTED, OID_802_3_PERMANENT_ADDRESS, OID_GEN_CURRENT_PACKET_FILTER,
    OID_GEN_MEDIA_CONNECT_STATUS, OID_GEN_RNDIS_CONFIG_PARAMETER, STATUS_SUCCESS as RNDIS_OK,
};
use crate::constants::timing::INIT_MS;
use crate::link::Link;
use crate::{nvsp, rndis};

/// What the bring-up learned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Started {
    pub version: u32,
    /// The address the host assigned the adapter.
    pub host_mac: [u8; 6],
    pub link_up: bool,
}

/// The two buffers' GPADLs, made before the host is told of them.
pub struct Gpadls<'a> {
    pub relid: u32,
    pub recv: (u32, &'a [u64]),
    pub send: (u32, &'a [u64]),
}

fn version<H: Host, M: Shared>(l: &mut Link<H, M>) -> Result<u32, &'static str> {
    for v in VERSIONS {
        let a = l.call(&nvsp::init(v), INIT_MS).map_err(|_| "host silent at INIT")?;
        if nvsp::init_taken(&a) {
            return Ok(v);
        }
    }
    Err("no protocol version in common")
}

/// The NVSP half, through the buffers.
fn buffers<H: Host, M: Shared>(l: &mut Link<H, M>, g: &Gpadls, v: u32) -> Result<(), &'static str> {
    if v >= V2 {
        l.call(&nvsp::ndis_config(), INIT_MS).map_err(|_| "host silent at NDIS config")?;
    }
    let ndis = if v >= V5 { NDIS_6_30 } else { NDIS_6_1 };
    l.call(&nvsp::ndis_version(ndis), INIT_MS).map_err(|_| "host silent at NDIS version")?;
    nonos_vmbus::gpadl(&mut l.host, g.relid, g.recv.0, g.recv.1).map_err(|_| "receive buffer GPADL refused")?;
    let a = l
        .call(&nvsp::buffer(SEND_RECEIVE_BUFFER, g.recv.0, RECEIVE_BUFFER_ID), INIT_MS)
        .map_err(|_| "host silent at the receive buffer")?;
    let (size, count) = nvsp::receive_buffer(&a).ok_or("receive buffer refused")?;
    let span = (size as u64).checked_mul(count as u64).ok_or("receive buffer refused")?;
    if span > l.bufs.recv.len() as u64 {
        return Err("receive sections past the buffer");
    }
    nonos_vmbus::gpadl(&mut l.host, g.relid, g.send.0, g.send.1).map_err(|_| "send buffer GPADL refused")?;
    let a = l
        .call(&nvsp::buffer(SEND_SEND_BUFFER, g.send.0, SEND_BUFFER_ID), INIT_MS)
        .map_err(|_| "host silent at the send buffer")?;
    let section = nvsp::send_buffer(&a).ok_or("send buffer refused")? as usize;
    let sections = (l.bufs.send.len() / section).min(MAX_SECTIONS);
    if sections == 0 {
        return Err("send sections past the buffer");
    }
    (l.bufs.section, l.bufs.sections) = (section, sections);
    Ok(())
}

/// The whole bring-up.
pub fn run<H: Host, M: Shared>(l: &mut Link<H, M>, g: &Gpadls) -> Result<Started, &'static str> {
    let v = version(l)?;
    buffers(l, g, v)?;
    let (status, _) = l.control(&rndis::initialize(1), 1, INIT_MS).map_err(|_| "host silent at RNDIS INITIALIZE")?;
    if status != RNDIS_OK {
        return Err("RNDIS INITIALIZE refused");
    }
    let (status, mac) =
        l.control(&rndis::query(2, OID_802_3_PERMANENT_ADDRESS), 2, INIT_MS).map_err(|_| "host silent at its address")?;
    let host_mac: [u8; 6] = mac.get(..6).and_then(|m| m.try_into().ok()).ok_or("no host-assigned address")?;
    if status != RNDIS_OK {
        return Err("host-assigned address refused");
    }
    let filter = FILTER.to_le_bytes();
    let (status, _) = l
        .control(&rndis::set(3, OID_GEN_CURRENT_PACKET_FILTER, &filter), 3, INIT_MS)
        .map_err(|_| "host silent at the packet filter")?;
    if status != RNDIS_OK {
        return Err("packet filter refused");
    }
    let (status, media) = l
        .control(&rndis::query(4, OID_GEN_MEDIA_CONNECT_STATUS), 4, INIT_MS)
        .map_err(|_| "host silent at the link")?;
    let link_up = status == RNDIS_OK && media.get(..4) == Some(&MEDIA_CONNECTED.to_le_bytes()[..]);
    Ok(Started { version: v, host_mac, link_up })
}

/// Ask the host to take `mac` as the station address (Linux's
/// rndis_filter_set_device_mac): whether it said yes.
pub fn offer_mac<H: Host, M: Shared>(l: &mut Link<H, M>, mac: &[u8; 6]) -> Result<bool, &'static str> {
    let info = rndis::network_address(mac);
    let (status, _) = l
        .control(&rndis::set(5, OID_GEN_RNDIS_CONFIG_PARAMETER, &info), 5, INIT_MS)
        .map_err(|_| "host silent at the station address")?;
    Ok(status == RNDIS_OK)
}
