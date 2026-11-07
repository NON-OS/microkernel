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

//! A runnable, over-the-network demonstration of nsh/1, built from the same
//! fixtures the loopback proof uses. It exists so an operator can run the
//! real, attested, post-quantum handshake across a real TCP connection (the
//! `nshd` and `nsh-fixture-client` binaries), not only in-process in a test.
//! The host it serves is the proofs' honest fixture kernel; attestation is
//! the real `NONOSATT` checker and is never bypassed. No real key material is
//! involved: these are the throwaway test keys, so this is a development
//! runner only, never a production host.

pub mod client;
pub mod serve;
pub mod shell;

use crate::kit::roles_build::NOW;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

/// The system's randomness, from `/dev/urandom`.
pub fn urandom(out: &mut [u8]) -> bool {
    File::open("/dev/urandom").and_then(|mut f| f.read_exact(out)).is_ok()
}

/// The fixture clock: the role table's epoch, moving as real time does.
pub struct Clock(Instant);

impl Clock {
    pub fn start() -> Clock {
        Clock(Instant::now())
    }
    pub fn now(&self) -> u64 {
        NOW + self.0.elapsed().as_millis() as u64
    }
}
