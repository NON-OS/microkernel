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

//! The panel's lines, one per proof or measurement, in boot order.

use super::input::Proofs;
use super::state::State;
use super::{evidence, kernel, platform, tcg};
use crate::display::text::Text;

pub const ROWS: usize = 7;

/// One line: what it is, its state word, and what backs that word.
#[derive(Clone, Copy)]
pub struct Row {
    pub label: &'static [u8],
    pub state: State,
    pub detail: Text,
}

pub fn rows(p: &Proofs<'_>) -> [Row; ROWS] {
    [
        kernel::trailer(p.crypto),
        kernel::signature(p.crypto),
        evidence::bootloader(p.trailer),
        evidence::boot_root(p.record),
        tcg::tcg_log(p.tcg_log),
        platform::secure_boot(p.security),
        platform::tpm(p.security),
    ]
}
