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

//! The full proving run, as the capsule's main runs it on the machine: the
//! first vector from nothing, the tree it built held to the shipped cache,
//! the other three from the shipped cache. Here the shipped cache is
//! required: a build without NONOS_PERIODIC_CACHE fails, naming it. Exits 0
//! only when no line failed and all four vectors held, byte for byte.

use std::io::{self, Write};

use crate::cache::{self, SHIPPED};
use crate::serial::{line, TAG};
use crate::{pool, prove, vectors};

/// Stdout, keeping what was written so a FAIL line anywhere is known.
struct Tee {
    out: io::Stdout,
    seen: Vec<u8>,
}

impl Write for Tee {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.seen.extend_from_slice(b);
        self.out.write_all(b).map(|()| b.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

/// The run; the process's exit status.
pub fn run() -> i32 {
    let mut out = Tee { out: io::stdout(), seen: Vec::new() };
    if SHIPPED.is_empty() {
        line(&mut out, false, "no shipped cache: NONOS_PERIODIC_CACHE named no periodic.top");
        return 1;
    }
    pool::start();
    if pool::workers() == 0 {
        line(&mut out, false, "no thread could be started for the prover");
        return 1;
    }
    let cache_ok = cache::shipped_cache(&mut out);
    let mut passed = 0u32;
    let mut cache: Option<Vec<u8>> = None;
    for (i, v) in vectors::ALL.iter().enumerate() {
        if i == 1 {
            cache = Some(SHIPPED.to_vec());
        }
        cache = prove::prove(&mut out, v, cache, &mut passed);
    }
    let fail = format!("{TAG} FAIL ");
    let failed = String::from_utf8_lossy(&out.seen).lines().any(|l| l.starts_with(&fail));
    let all = passed as usize == vectors::ALL.len();
    println!("{TAG} DONE {passed} of {} vectors byte for byte", vectors::ALL.len());
    i32::from(!(all && cache_ok && !failed))
}
