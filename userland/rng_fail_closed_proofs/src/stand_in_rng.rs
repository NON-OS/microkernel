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

//! The seeded RNG and the hardware sources the mounted RNG files reach,
//! each answering only while `stand_in::sources` lets it.

use crate::stand_in::{next, CPU, CPU_TAG};

pub mod entropy {
    use crate::stand_in::{next, ENTROPY, ENTROPY_TAG};
    pub(crate) fn get_entropy64_secure() -> Result<u64, ()> {
        ENTROPY.get().then(|| next(ENTROPY_TAG)).ok_or(())
    }
}

pub mod init {
    use crate::crypto::rng::{RngError, RngResult};
    pub fn ensure_initialized() -> RngResult<()> {
        crate::stand_in::SEEDED.get().then_some(()).ok_or(RngError::NotInitialized)
    }
}

pub mod state {
    /// The seeded RNG, there whenever `ensure_initialized` says so.
    pub struct Seeded;
    impl Seeded {
        pub fn next_u64(&mut self) -> u64 {
            crate::stand_in::next(crate::stand_in::SEEDED_TAG)
        }
    }
    pub struct Lock(std::sync::Mutex<Option<Seeded>>);
    impl Lock {
        pub fn lock(&self) -> std::sync::MutexGuard<'_, Option<Seeded>> {
            self.0.lock().unwrap_or_else(|p| p.into_inner())
        }
    }
    pub static GLOBAL_RNG: Lock = Lock(std::sync::Mutex::new(Some(Seeded)));
}

pub mod reseed {
    pub fn draw(rng: &mut super::state::Seeded, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            chunk.copy_from_slice(&rng.next_u64().to_le_bytes()[..chunk.len()]);
        }
    }
}

pub fn has_cpu_random() -> bool {
    CPU.get()
}
pub fn has_cpu_entropy() -> bool {
    false
}
pub fn cpu_entropy64() -> Option<u64> {
    None
}
pub fn cpu_random64() -> Option<u64> {
    CPU.get().then(|| next(CPU_TAG))
}
