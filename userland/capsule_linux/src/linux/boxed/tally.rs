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

//! What a box refused, kind by kind: a count and the first few examples.
//! One call is counted once for each kind it was refused as, however many
//! checks along its path said no (a rename asks of both its names).

use alloc::vec::Vec;

use super::kind::{Kind, KINDS};

/// Examples kept for each kind, and the longest each is kept at.
pub const EXAMPLES: usize = 3;
pub const EXAMPLE_LEN: usize = 72;

#[derive(Default)]
pub struct Tally {
    counts: [u64; KINDS],
    examples: [Vec<Vec<u8>>; KINDS],
    /// The call being answered, and the kinds already counted for it.
    call: u64,
    counted: [u64; KINDS],
}

impl Tally {
    /// A new call is answered: its refusals count afresh.
    pub fn call_begins(&mut self) {
        self.call = self.call.wrapping_add(1);
    }

    pub fn note(&mut self, kind: Kind, example: &[u8]) {
        let i = kind.index();
        // Before the first call began, every refusal counts.
        if self.call != 0 && self.counted[i] == self.call {
            return;
        }
        self.counted[i] = self.call;
        self.counts[i] = self.counts[i].saturating_add(1);
        if self.examples[i].len() < EXAMPLES {
            self.examples[i].push(printable(example));
        }
    }

    pub fn count(&self, kind: Kind) -> u64 {
        self.counts[kind.index()]
    }

    pub fn examples(&self, kind: Kind) -> &[Vec<u8>] {
        &self.examples[kind.index()]
    }

    pub fn total(&self) -> u64 {
        self.counts.iter().fold(0u64, |a, c| a.saturating_add(*c))
    }
}

/// An example as a terminal shows it: cut to EXAMPLE_LEN, and every byte
/// that would move the cursor or start an escape shown as `?`.
fn printable(example: &[u8]) -> Vec<u8> {
    let cut = &example[..example.len().min(EXAMPLE_LEN)];
    cut.iter().map(|b| if (0x20..0x7f).contains(b) { *b } else { b'?' }).collect()
}
