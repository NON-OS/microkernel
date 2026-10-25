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

//! The ACS words each hop reads on a modelled machine, as acs_read.rs
//! reads them: the real layout choice, then the real word reading.

use super::acs_layout::layout;
use super::acs_tree::Bdf;
use super::acs_words::words;
use super::laptops::Part;
use super::words_tests::read;

/// The ACS words each hop reads, as acs_read.rs reads them.
pub(super) fn acs(parts: &[Part]) -> impl FnMut(Bdf, u16) -> Option<(u16, u16)> + '_ {
    move |at, needed| {
        let p = parts.iter().find(|p| p.func.at == at)?;
        let l = layout(p.ids.0, p.ids.1, p.kind, at.0);
        match &p.acs {
            Some(r) => read(r, l, needed),
            None => words(l, needed, false, |_| 0, |_, _| {}),
        }
    }
}
