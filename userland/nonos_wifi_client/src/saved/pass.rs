/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Which passphrases a saved network can be given.

use super::list::PASS_MAX;

/// Whether `pass` is one the network's kind of security can use. Pinned to
/// WPA3, SAE takes any password of 1 to 64 bytes; otherwise the join may be
/// WPA2, whose passphrase is 8 to 63 bytes or 64 hex digits. Empty is never
/// a passphrase: no kind of saved network goes without one.
pub(super) fn usable(pass: &[u8], sae: bool) -> bool {
    let psk = (8..=63).contains(&pass.len())
        || (pass.len() == PASS_MAX && pass.iter().all(u8::is_ascii_hexdigit));
    if sae {
        !pass.is_empty() && pass.len() <= PASS_MAX
    } else {
        psk
    }
}
