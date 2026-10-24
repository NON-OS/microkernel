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

use super::{
    constants::{EXEC_RANDOMIZATION_RANGE, HEAP_RANDOMIZATION_RANGE, STACK_RANDOMIZATION_RANGE},
    entropy::draw,
    state::AslrManager,
};

impl AslrManager {
    /// Each placement is `None` when randomisation is on and no randomness
    /// answers, or when the slide would carry the address past either end
    /// of the address space; with randomisation off the base is kept.
    pub fn randomize_base(&mut self, preferred_base: u64) -> Option<u64> {
        if !self.executable_randomization {
            return Some(preferred_base);
        }
        Some(preferred_base.checked_add(self.random_offset(EXEC_RANDOMIZATION_RANGE)?)? & !0xFFF)
    }
    pub fn randomize_stack(&mut self, base_stack: u64) -> Option<u64> {
        if !self.stack_randomization {
            return Some(base_stack);
        }
        Some(base_stack.checked_sub(self.random_offset(STACK_RANDOMIZATION_RANGE)?)? & !0xFFF)
    }
    pub fn randomize_heap(&mut self, base_heap: u64) -> Option<u64> {
        if !self.heap_randomization {
            return Some(base_heap);
        }
        Some(base_heap.checked_add(self.random_offset(HEAP_RANDOMIZATION_RANGE)?)? & !0xFFF)
    }

    pub fn random_address(min: usize, max: usize, size: usize) -> Option<usize> {
        if min >= max || size == 0 || max - min <= size {
            return Some(min);
        }
        let entropy = usize::try_from(draw()? >> 16).unwrap_or_default();
        let offset = entropy % (max - min - size);
        Some((min + offset) & !0xFFF)
    }
}
