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

//! Sleeping on the display's interrupt. The engine stays pure: the
//! capsule gives it a `Park` over its interrupt grant, the proofs a
//! scripted one.

/// Why a park returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Woke {
    /// The function's interrupt was delivered.
    Irq,
    /// An unrelated wake; nothing is known.
    Early,
    /// The whole time passed with no interrupt.
    TimedOut,
    /// The kernel refused the grant: it is gone.
    Refused,
}

pub trait Park {
    /// Whether an interrupt is bound at all; no flip arms one otherwise.
    fn bound(&self) -> bool;
    /// Sleep until the function's interrupt, at most `ms`.
    fn wait(&mut self, ms: u64) -> Woke;
    /// Tell the broker the interrupt was handled.
    fn ack(&mut self);
}

/// How a pipe's flips are waited for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blank {
    /// Sleep on its vertical blank interrupt.
    Irq,
    /// Poll PLANE_SURFLIVE: no interrupt here, or it never fired.
    Poll,
}

/// No interrupt at all: every flip is polled.
pub struct NoPark;

impl Park for NoPark {
    fn bound(&self) -> bool {
        false
    }
    fn wait(&mut self, _ms: u64) -> Woke {
        Woke::Refused
    }
    fn ack(&mut self) {}
}
