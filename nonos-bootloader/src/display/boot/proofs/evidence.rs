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

//! What the loader carries from the ESP for the kernel's check of the loader.
//! The loader reads both files and hands them on; it verifies neither.

use nonos_boot_measure::record::parse;

use super::rows::Row;
use super::state::State;
use crate::display::text::Text;

/// `EFI/nonos/bootloader.trailer`, the loader's own v4 trailer.
pub fn bootloader(trailer: Option<&[u8]>) -> Row {
    let t = Text::new();
    let (state, detail) = match trailer.filter(|b| !b.is_empty()) {
        None => (State::Absent, t.push(b"no EFI/nonos/bootloader.trailer")),
        Some(b) => {
            (State::Present, t.push(b"trailer ").size(b.len()).push(b", checked by the kernel"))
        }
    };
    Row { label: b"BOOTLOADER", state, detail }
}

/// `EFI/nonos/boot_root.approval`, read with the kernel's own parser only for
/// its epoch, bytes 32..40 little-endian. Its signature is the kernel's check.
pub fn boot_root(record: Option<&[u8]>) -> Row {
    let t = Text::new();
    let (state, detail) = match record.filter(|b| !b.is_empty()) {
        None => (State::Absent, t.push(b"no EFI/nonos/boot_root.approval")),
        Some(b) => match parse(b) {
            Ok(r) => (
                State::Present,
                t.push(b"epoch ").dec(r.epoch).push(b", signature checked by the kernel"),
            ),
            Err(e) => (
                State::Failed,
                t.push(b"does not parse, code ").dec(e.code() as u64).push(b", ").size(b.len()),
            ),
        },
    };
    Row { label: b"BOOT-ROOT RECORD", state, detail }
}
