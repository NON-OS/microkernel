// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! net_core's link protocol: raw Ethernet frames onto the air, below the
//! route the person chose, so served to net.core's owner alone; anyone else
//! is refused in the link's own shape (nonos_wifi_core::netif::gate).

use nonos_wifi_core::netif::{link_admits, refuse_link};

use super::net_core::net_core_pid;
use super::radio::Radio;

/// The reply's length in `out`, if there is one.
pub(super) fn link_serve(
    radio: &mut Radio,
    sender: u32,
    req: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    if !link_admits(sender, net_core_pid()) {
        return Some(refuse_link(req, out));
    }
    radio.serve_link(req, out)
}
