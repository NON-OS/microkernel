/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 */

//! The Wi-Fi control family Settings, first-boot setup and net_core speak to
//! whichever Wi-Fi driver is running (`nonos_wifi_client`): tag 0x57494649,
//! `[magic u32][op u16][request id u32]`.
//!
//! This driver claims the card and brings it out of reset, and it carries the
//! firmware images, but nothing at startup stages, loads and starts that
//! firmware, and it has no firmware scan command, no MAC or station context
//! commands and no transmit path for management frames. It cannot scan or
//! join, so the status op reports stage 8, which the client names, and every
//! other op is refused with -38 rather than left to time out.

/// The control family tag, distinct from this driver's own protocol tag.
const WIFI_MAGIC: u32 = 0x5749_4649;
const WIFI_HDR: usize = 10;
const OP_STATUS: u16 = 4;
/// The stage the client reads as "this driver cannot scan or join".
const STAGE_NO_AIR_PATH: u8 = 8;
/// ENOSYS: the op exists in the family and this driver does not run it.
const E_NO_OP: i32 = -38;

/// Answer a control request. `None` when `req` is not one, so it goes on to
/// this driver's own protocol.
pub(super) fn answer(req: &[u8], out: &mut [u8]) -> Option<usize> {
    if req.len() < WIFI_HDR || out.len() < WIFI_HDR + 4 {
        return None;
    }
    if u32::from_le_bytes([req[0], req[1], req[2], req[3]]) != WIFI_MAGIC {
        return None;
    }
    out[..WIFI_HDR].copy_from_slice(&req[..WIFI_HDR]);
    if u16::from_le_bytes([req[4], req[5]]) == OP_STATUS {
        out[WIFI_HDR] = STAGE_NO_AIR_PATH;
        return Some(WIFI_HDR + 1);
    }
    out[WIFI_HDR..WIFI_HDR + 4].copy_from_slice(&E_NO_OP.to_le_bytes());
    Some(WIFI_HDR + 4)
}
