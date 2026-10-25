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

//! The gate as the kernel's text reads, and the order `stopped()` had before
//! its fix, kept as a control; the glue tests hold the first to the text.

use super::gate::{Flag, Gate, Recheck};

impl Gate {
    pub const KERNEL: Gate = Gate {
        reads: &[Flag::Held, Flag::Takers, Flag::Gone],
        first_check: true,
        recheck: Recheck::Full,
        backs_out: true,
        fence_first: true,
        counts_first: true,
        waits: true,
        sets_gone: true,
        ends: true,
        ends_last: true,
    };
    /// The kernel's gate with GONE loaded before TAKERS, as first written:
    /// a draw could miss both (L79). Kept to show the proofs see it.
    pub const GONE_FIRST: Gate =
        Gate { reads: &[Flag::Held, Flag::Gone, Flag::Takers], ..Gate::KERNEL };

    /// The loads of the check made before the count.
    pub fn first(&self) -> &'static [Flag] {
        if self.first_check {
            self.reads
        } else {
            &[]
        }
    }

    /// The loads of the check made once a draw is counted.
    pub fn again(&self) -> &'static [Flag] {
        match self.recheck {
            Recheck::Full => self.reads,
            Recheck::HeldOnly => &[Flag::Held],
            Recheck::Absent => &[],
        }
    }
}
