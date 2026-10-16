// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! This driver's own protocol: what setup recorded of the card, and frames
//! built and parsed by hand (OP_MGMT_BUILD among them). It is net.core's
//! owner's alone, as the link is (server/net_core.rs); the panels speak the
//! control family.

use nonos_wifi_core::netif::link_admits;

use crate::driver::Driver;
use crate::protocol::{parse, refused, E_INVAL, E_PERM, HDR_LEN, IPC_PAYLOAD_MAX};

use super::dispatch::dispatch;
use super::net_core::net_core_pid;
use super::respond;

pub(super) fn own(driver: &mut Driver, sender_pid: u32, rx: &[u8], tx: &mut [u8]) {
    let n = rx.len();
    // This driver's own requests are read as they were before the buffer
    // grew for net_core's frames: no longer than its own limit.
    let own = &rx[..n.min(HDR_LEN + IPC_PAYLOAD_MAX)];
    let Some((req, body)) = parse(own) else {
        let _ = respond::send(sender_pid, &refused(own), E_INVAL, &[], tx);
        return;
    };
    if !link_admits(sender_pid, net_core_pid()) {
        let _ = respond::send(sender_pid, &req, E_PERM, &[], tx);
        return;
    }
    dispatch(driver, sender_pid, req, body, tx);
}
