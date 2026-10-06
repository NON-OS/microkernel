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

//! A whole information element held by value, so the supplicant stays `Copy`
//! and needs no allocation to remember the AP's RSNE and RSNXE from the beacon
//! (the reference message 3 is checked against) and its own elements (which
//! message 2 repeats). An element is at most 257 bytes (id, length, 255).

/// The largest element: one id octet, one length octet and 255 of content.
pub const IE_MAX: usize = 257;

/// One information element, or none.
#[derive(Clone, Copy)]
pub struct Ie {
    bytes: [u8; IE_MAX],
    len: usize,
}

impl Ie {
    /// No element.
    pub const NONE: Ie = Ie { bytes: [0u8; IE_MAX], len: 0 };

    /// Hold `elem` (id and length included). An element longer than any valid
    /// one is not held, which reads as absent and fails a later comparison.
    pub fn from_slice(elem: &[u8]) -> Ie {
        let mut ie = Ie::NONE;
        if elem.len() <= IE_MAX {
            ie.bytes[..elem.len()].copy_from_slice(elem);
            ie.len = elem.len();
        }
        ie
    }

    /// Hold `elem` if present.
    pub fn from_option(elem: Option<&[u8]>) -> Ie {
        elem.map(Ie::from_slice).unwrap_or(Ie::NONE)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn is_present(&self) -> bool {
        self.len != 0
    }

    /// The element, or `None` when absent.
    pub fn as_option(&self) -> Option<&[u8]> {
        self.is_present().then_some(self.as_slice())
    }

    /// Whether `other` is present exactly when this is, and byte-identical.
    pub fn matches(&self, other: Option<&[u8]>) -> bool {
        match other {
            Some(o) => self.is_present() && self.as_slice() == o,
            None => !self.is_present(),
        }
    }
}
