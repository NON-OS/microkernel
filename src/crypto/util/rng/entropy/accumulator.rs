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

//! The entropy capsule's bytes held for a seed, pure (K9). A batch failing
//! the health tests is refused and drops what is held; one while the source
//! is unproven is dropped. kernel_proofs::entropy_health.

use super::health::{Fault, Health};

pub(crate) const HELD: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fed {
    Held,
    Proving,
    Refused(Fault),
}

pub(crate) struct Accumulator {
    health: Health,
    held: [u8; HELD],
    len: usize,
    at: usize,
}

impl Accumulator {
    pub(crate) const fn new() -> Self {
        Self { health: Health::new(), held: [0; HELD], len: 0, at: 0 }
    }

    pub(crate) fn feed(&mut self, batch: &[u8]) -> Fed {
        let proven = self.health.proven();
        for &x in batch {
            if let Err(fault) = self.health.sample(x) {
                self.take();
                return Fed::Refused(fault);
            }
        }
        if !proven {
            return Fed::Proving;
        }
        for &x in batch {
            self.held[self.at] ^= x;
            self.at = (self.at + 1) % HELD;
        }
        self.len = (self.len + batch.len()).min(HELD);
        Fed::Held
    }

    pub(crate) fn wanted(&self) -> bool {
        !self.health.proven() || self.len < HELD
    }

    /// What is held and how many bytes of it count, then nothing held.
    pub(crate) fn take(&mut self) -> Option<([u8; HELD], usize)> {
        let out = (self.held, self.len);
        // SAFETY: each `b` is a live, exclusively borrowed byte of `held`.
        self.held.iter_mut().for_each(|b| unsafe { core::ptr::write_volatile(b, 0) });
        (self.len, self.at) = (0, 0);
        (out.1 > 0).then_some(out)
    }
}
