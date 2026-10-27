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

//! A bring-up step as data, so each phase is a table the proofs can read
//! and run against a modelled chip.

use super::reg::Reg;

/// A MAC-side register, or one of the PHY's (a word, through phy.rs).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target {
    Mac(Reg),
    Phy(u16),
}

/// How long to look for a value: `tries` looks, `every_ms` apart. With
/// `fail` the bring-up stops, named, when it never comes; without, it goes
/// on, as Linux v6.12 does after the same wait.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Wait {
    pub tries: u16,
    pub every_ms: u16,
    pub fail: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    /// Read, clear the first mask's bits, set the second's, write back.
    Edit(Target, u32, u32),
    /// Write the value whole.
    Put(Target, u32),
    /// Look until the value under the mask is one of those listed.
    Until(Target, u32, &'static [u32], Wait),
    /// Edit the second register (clear, then set) only when the first has
    /// a bit of the first mask.
    EditIf(Target, u32, Target, u32, u32),
    /// Wait this many milliseconds.
    Pause(u16),
}

pub const fn clear(r: Reg, bits: u32) -> Op {
    Op::Edit(Target::Mac(r), bits, 0)
}

pub const fn set(r: Reg, bits: u32) -> Op {
    Op::Edit(Target::Mac(r), 0, bits)
}

pub const fn put(r: Reg, value: u32) -> Op {
    Op::Put(Target::Mac(r), value)
}

/// Look until every bit of `bits` is set (`want` = `bits`) or clear (0).
pub const fn until(r: Reg, bits: u32, want: &'static [u32], wait: Wait) -> Op {
    Op::Until(Target::Mac(r), bits, want, wait)
}
