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

//! At start: HEST's GHES sources kept for polling and for the Error Device's
//! Notify, and BERT read once: the fatal error the last boot ended in,
//! printed before anything else is served.

use super::phys::Phys;
use super::read::read_block;
use super::state::{Apei, Source};
use crate::sys::acpi::table;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use nonos_aml::apei::{bert, hest, POLLED, SCI};
use nonos_aml::platform::DeviceInfo;

/// The Error Device (18.3.2.7.2), which firmware notifies for SCI sources.
const ERROR_DEVICE: &[u8] = b"PNP0C33";

pub fn start(devices: &[DeviceInfo]) -> Apei {
    let mut phys = Phys::default();
    if let Some((at, len)) = table(*b"BERT", 0).ok().and_then(|t| bert(&t)) {
        if read_block(&mut phys, at, u64::from(len), b"last boot ended in a hardware error, ") == 0
        {
            Line::new(b"BERT: no error from the last boot").end();
        }
    }
    let sources: alloc::vec::Vec<Source> = table(*b"HEST", 0)
        .map(|t| hest(&t))
        .unwrap_or_default()
        .into_iter()
        .filter(|s| s.enabled)
        .filter_map(|s| s.ghes.map(|g| Source { id: s.id, ghes: g, next_ms: now_ms() }))
        .collect();
    let count = |k| sources.iter().filter(|s| s.ghes.notify == k).count() as u64;
    let error_devices = devices
        .iter()
        .filter(|d| d.hid.as_ref().is_some_and(|h| h.bytes() == ERROR_DEVICE))
        .map(|d| d.id)
        .collect();
    Line::new(b"HEST: GHES sources ")
        .dec(sources.len() as u64)
        .str(b" (polled ")
        .dec(count(POLLED))
        .str(b", SCI ")
        .dec(count(SCI))
        .str(b"; others are listed and not served)")
        .end();
    Apei { sources, phys, error_devices }
}
