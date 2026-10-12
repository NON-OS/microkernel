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

//! A Realtek controller's setup, before the standard bring-up resets it:
//! its ROM's version and, for a second-layout chip, its key id read, its
//! patch and config loaded and checked against their pins, the image
//! downloaded, and the version read again, which a running patch changes.
//! A patch survives Reset, so bring-up then runs as on any controller.

use nonos_bt_hci::{Host, Transport};

use super::blobs::Blobs;
use super::chip::{rom, Chip, Format};
use super::frag::fragments;
use super::image::image;
use super::key::key_id;
use super::refused::Refused;
use super::rom::{download, rom_version};
use super::version;

pub enum Done<'a> {
    /// Another company's controller, or a Realtek one already patched or
    /// not in the table: bring-up goes on as before.
    NotOurs,
    Patched(&'a Chip),
}

pub fn setup<'a, T: Transport, B: Blobs>(
    host: &mut Host<T>,
    blobs: &mut B,
    chips: &'a [Chip],
) -> Result<Done<'a>, Refused> {
    let Some(chip) = rom(chips, &version::read(host)?) else {
        return Ok(Done::NotOurs);
    };
    let version = rom_version(host)?;
    // Only a second-layout file has signed subsections to choose by key.
    let key = match chip.format {
        Format::Realtech => 0,
        Format::RtbtCore => key_id(host)?,
    };
    let image = image(chip, version, key, blobs)?;
    for (index, data) in fragments(&image) {
        download(host, index, data)?;
    }
    if version::read(host)?.lmp_subversion == chip.lmp_subversion {
        return Err(Refused::NotTaken);
    }
    Ok(Done::Patched(chip))
}
