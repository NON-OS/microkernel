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

//! What the kernel says about the machine (MkProcStat's header): the cores
//! online, and the memory free, watched from a thread while a proof runs so
//! the least it fell to is known. The std layer's heap keeps what it maps,
//! so what a proof took from the machine stays taken after it.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

/// The header, or None when the kernel refuses it.
fn header() -> Option<ProcStatHeader> {
    #[repr(C)]
    struct One {
        header: ProcStatHeader,
        entry: ProcStatEntry,
    }
    let mut one = One { header: ProcStatHeader::default(), entry: ProcStatEntry::default() };
    let rc = mk_proc_stat((&mut one as *mut One).cast(), 1);
    (rc >= 0).then_some(one.header)
}

pub fn cpus_online() -> u32 {
    header().map_or(0, |h| h.cpus_online)
}

pub fn mem_total_mib() -> u64 {
    header().map_or(0, |h| h.mem_total_kb >> 10)
}

pub fn mem_free_mib() -> u64 {
    header().map_or(0, |h| h.mem_free_kb >> 10)
}

/// The least free memory seen until it is stopped, sampled every 100 ms.
pub struct Watch {
    stop: Arc<AtomicBool>,
    least_kb: Arc<AtomicU64>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Watch {
    pub fn start() -> Watch {
        let stop = Arc::new(AtomicBool::new(false));
        let least_kb = Arc::new(AtomicU64::new(header().map_or(0, |h| h.mem_free_kb)));
        let (s, l) = (Arc::clone(&stop), Arc::clone(&least_kb));
        let thread = std::thread::Builder::new()
            .spawn(move || {
                while !s.load(Ordering::Relaxed) {
                    if let Some(h) = header() {
                        l.fetch_min(h.mem_free_kb, Ordering::Relaxed);
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            })
            .ok();
        Watch { stop, least_kb, thread }
    }

    /// Stop watching; the least free memory seen, in MiB.
    pub fn stop(mut self) -> u64 {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        if let Some(h) = header() {
            self.least_kb.fetch_min(h.mem_free_kb, Ordering::Relaxed);
        }
        self.least_kb.load(Ordering::Relaxed) >> 10
    }
}
