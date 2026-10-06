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

use crate::menu::{MenuAction, SecurityMode};

/// One line of the menu: what Enter does, its name, one plain sentence, and
/// in mono what is checked before the kernel runs.
pub(super) struct Entry {
    pub action: MenuAction,
    pub label: &'static [u8],
    pub about: &'static [u8],
    pub spec: &'static [u8],
}

/*
 * Development is absent on purpose: an unsigned, unattested boot is reached
 * only through the explicit dev override, never from this menu. Every word
 * here says only what the loader and the kernel really do in that entry.
 */
pub(super) const ENTRIES: [Entry; 7] = [
    entry(
        MenuAction::Boot(SecurityMode::Standard),
        b"Standard",
        b"Boots when the kernel is signed, attested and current.",
        STD,
    ),
    entry(
        MenuAction::Boot(SecurityMode::Hardened),
        b"Hardened",
        b"Standard, and refuses to boot without Secure Boot and a TPM.",
        b"STANDARD + SECURE BOOT \xB7 PK \xB7 DB \xB7 TPM 2.0",
    ),
    entry(MenuAction::SafeMode, b"Safe Mode", b"No network, no audio, no optional apps.", STD),
    entry(MenuAction::NetworkIsolated, b"Air-Gapped", b"No network driver or service starts.", STD),
    entry(MenuAction::Recovery, b"Recovery", b"A terminal and files. No network, no setup.", STD),
    entry(
        MenuAction::Install,
        b"Install N\xD8NOS",
        b"Writes to the disk you choose, after you confirm.",
        STD,
    ),
    entry(MenuAction::Shutdown, b"Shut down", b"Power off.", b""),
];

/// What the loader checks in every entry that boots.
const STD: &[u8] = b"ED25519 \xB7 ML-DSA-65 \xB7 STARK \xB7 ROLLBACK \xB7 RNG";

const fn entry(
    action: MenuAction,
    label: &'static [u8],
    about: &'static [u8],
    spec: &'static [u8],
) -> Entry {
    Entry { action, label, about, spec }
}

/// Where `action` sits in the menu, or the first entry.
pub(super) fn index_of(action: MenuAction) -> usize {
    ENTRIES.iter().position(|e| e.action == action).unwrap_or(0)
}
