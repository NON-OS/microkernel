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

//! The least free memory a proof left the machine, watched from a thread.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use super::header;

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
