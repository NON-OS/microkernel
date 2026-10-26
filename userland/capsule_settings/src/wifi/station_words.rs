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
//! The words the Network page shows for where the wired card's address came
//! from (station.rs), and how many rows that takes. Pure; held in
//! capsule_settings_proofs.

/// What the page says for a source: the value, and the line under the
/// label when the value needs one. None shows nothing.
pub fn said(source: u32) -> Option<(&'static str, Option<&'static str>)> {
    match source {
        1 => Some(("Private, new each boot", None)),
        2 | 3 => Some(("Assigned by Hyper-V", Some(HYPER_V))),
        _ => None,
    }
}

/// Why Hyper-V's address is used, as the lead worded it.
pub const HYPER_V: &str =
    "Hyper-V drops a chosen address unless MAC spoofing is on for this VM's adapter.";

/// The address row's entries for a source: none when there is nothing to
/// say, so no label stands empty; one, with or without its note line.
pub fn rows(source: Option<u32>) -> (usize, bool) {
    match source.and_then(said) {
        Some((_, note)) => (1, note.is_some()),
        None => (0, false),
    }
}
