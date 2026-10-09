/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi drivers this client speaks to, and whether each can join.
//!
//! A join carries the passphrase. A driver that cannot run one answers -38
//! and drops the body, so the passphrase crossed to it for nothing, and
//! net_core's autojoin sent every saved one there at every boot. The client
//! now answers -38 itself for such a driver and sends nothing.
//! `iwlwifi_proofs` holds the table to what the iwlwifi driver answers.

/// What a join is answered with when the driver cannot run one: the
/// driver's own refusal, and this client's when it does not ask.
pub const CANNOT_JOIN: i32 = -38;

/// A registered Wi-Fi driver service.
pub struct Service {
    pub name: &'static [u8],
    /// The adapter name the panels show.
    pub label: &'static str,
    /// Whether the driver runs a join.
    pub joins: bool,
}

/// The RTL8821CE is asked first, as before the iwlwifi driver could join, so
/// a machine with both cards keeps the radio it joined with. The iwlwifi
/// driver joins WPA2 and WPA3 networks on AX211 (the SO family); on a part
/// it has no air path for, or without randomness, it answers a join with
/// `CANNOT_JOIN`.
pub const SERVICES: &[Service] = &[
    Service { name: b"driver.rtl8821ce0", label: "Realtek RTL8821CE", joins: true },
    Service { name: b"driver.iwlwifi0", label: "Intel Wi-Fi", joins: true },
];
