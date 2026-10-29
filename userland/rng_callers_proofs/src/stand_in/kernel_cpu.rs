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

// The shootdown call, the serial line and the CPU, included into `kernel`.

pub mod smp {
    pub fn serve_shootdowns() {
        super::SHOOTDOWNS.set(super::SHOOTDOWNS.get() + 1);
    }
}

pub mod sys {
    pub mod serial {
        pub fn println(line: &[u8]) {
            super::super::keep(String::from_utf8_lossy(line).into_owned());
        }
    }
}

pub mod arch {
    use std::cell::Cell;

    thread_local! {
        static CPU: Cell<Option<u64>> = const { Cell::new(None) };
    }

    /// Whether the CPU's random instruction answers on this thread, and from
    /// which value it counts up. It is absent unless a test turns it on.
    pub fn cpu_answers(from: Option<u64>) {
        CPU.set(from);
    }

    pub mod cpu_random {
        pub fn random_u64() -> Option<u64> {
            let v = super::CPU.get()?;
            super::CPU.set(Some(v + 1));
            Some(v)
        }
    }

    /// The cycle counter: no seed for a secret; a call counts as weak.
    pub fn read_time_counter() -> u64 {
        crate::stand_in::rng::note_weak();
        42
    }

    pub fn disable_interrupts() {}
}

/// The kernel's one way out of a fatal stop, which the canary's failure path
/// takes; no test reaches it.
pub mod boot {
    pub mod panic {
        pub fn fatal() -> ! {
            panic!("the kernel stopped")
        }
    }
}
