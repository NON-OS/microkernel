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

//! The names the image's own capsules (namespace `systems.nonos`) declare,
//! service and reply alike, as their Capsule.mk files give them (L52). A
//! capsule a publisher signs may take none of them, nor a numbered instance
//! of one. A store app could otherwise take `input_router` in the second
//! between the router's exit and its restart, or `app.settings` before the
//! person opened Settings, and every check that trusts a name (the HELD table,
//! the ACPI service's per-op senders, the shell's input source) believed it.
//! `kernel_proofs` holds the list to the Capsule.mk files.

const NAMES: &str = include_str!("system_names.txt");

/// Whether `name` is one of the image's own, or a numbered instance of one.
pub(crate) fn is_system_name(name: &str) -> bool {
    NAMES.lines().filter(|sys| !sys.is_empty()).any(|sys| {
        name.strip_prefix(sys).is_some_and(|rest| {
            let digits = rest.strip_prefix('.').unwrap_or("");
            rest.is_empty() || (!digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
        })
    })
}
