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

//! A quote's signed structure, as the checks read it.

/// The first PCR selection, and how many the quote lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection<'a> {
    pub count: u32,
    pub hash: u16,
    pub bitmap: &'a [u8],
}

/// The fields of `TPMS_ATTEST` a check reads. `clockInfo` and
/// `firmwareVersion` are read past and kept nowhere: they say when, not what.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attest<'a> {
    pub signer: &'a [u8],
    pub extra: &'a [u8],
    pub selection: Selection<'a>,
    pub pcr_digest: &'a [u8],
}
