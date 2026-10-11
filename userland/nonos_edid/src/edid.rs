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

use crate::blocks::{block, checksum_ok, Extensions, BLOCK, HEADER};
use crate::cta::eeodb;
use crate::error::Error;

/// An EDID whose base block passed its header and checksum, trimmed to
/// the extension blocks that are both declared and present.
#[derive(Clone, Copy, Debug)]
pub struct Edid<'a> {
    raw: &'a [u8],
    base: &'a [u8; BLOCK],
    extensions: usize,
}

impl<'a> Edid<'a> {
    /// Check the base block and count the extensions. Byte 126 declares
    /// how many follow (E-EDID 1.4 section 3.3, Table 3.1); an HDMI 2.1
    /// sink with more than one sets it to 1 and gives the real count in
    /// its HF-EEODB. Whatever is declared, only blocks actually handed
    /// over are read, so a short buffer is not an overrun.
    pub fn parse(raw: &'a [u8]) -> Result<Self, Error> {
        let base = block(raw, 0).ok_or(Error::Short)?;
        if base[..8] != HEADER {
            return Err(Error::Header);
        }
        if !checksum_ok(base) {
            return Err(Error::Checksum);
        }
        let declared = eeodb::count(raw).unwrap_or(base[126] as usize);
        let extensions = declared.min(raw.len() / BLOCK - 1);
        Ok(Self { raw: &raw[..(extensions + 1) * BLOCK], base, extensions })
    }

    /// The 128 byte base block.
    pub fn base(&self) -> &'a [u8; BLOCK] {
        self.base
    }

    /// Extension blocks present, whether their checksums hold or not.
    pub fn extension_count(&self) -> usize {
        self.extensions
    }

    /// The extension blocks whose checksums hold.
    pub fn extensions(&self) -> Extensions<'a> {
        Extensions { raw: self.raw, next: 1, end: self.extensions }
    }

    /// The bytes parse() kept: the base block and the extensions present.
    pub fn raw(&self) -> &'a [u8] {
        self.raw
    }
}
