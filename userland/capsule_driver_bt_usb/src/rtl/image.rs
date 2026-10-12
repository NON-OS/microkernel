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

//! What is downloaded. A first-layout file gives the chip's patch with
//! the file's firmware version in its last four octets; a second-layout
//! file gives its subsections for this ROM and key. The config follows as
//! it is, when there is one.

use alloc::vec::Vec;

use super::blobs::{pinned, Blobs};
use super::chip::{Chip, Format};
use super::epatch::patch_for;
use super::project::project;
use super::refused::{Layout, Refused};
use super::rtbt::image_for;

pub fn image<B: Blobs>(chip: &Chip, rom: u8, key: u8, blobs: &mut B) -> Result<Vec<u8>, Refused> {
    let name = chip.firmware.file;
    let file = pinned(blobs, chip.firmware)?.ok_or(Refused::Missing(name))?;
    // The pin says these are the right octets; the project says the table
    // pairs them with the right chip. Both layouts end in the same block.
    if project(&file) != Some(chip.project) {
        return Err(Refused::Layout(name, Layout::Project));
    }
    let mut out = match chip.format {
        Format::Realtech => {
            let patch = patch_for(&file, rom).map_err(|l| Refused::Layout(name, l))?;
            let mut out = patch.body.to_vec();
            out.extend_from_slice(&patch.version.to_le_bytes());
            out
        }
        Format::RtbtCore => image_for(&file, rom, key).map_err(|l| Refused::Layout(name, l))?,
    };
    let config = match chip.config {
        Some(pin) => match pinned(blobs, pin)? {
            None if chip.config_needed => return Err(Refused::Missing(pin.file)),
            found => found,
        },
        None => None,
    };
    if let Some(c) = &config {
        out.extend_from_slice(c);
    }
    Ok(out)
}
