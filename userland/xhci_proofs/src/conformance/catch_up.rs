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

//! A device model on a thread of its own, kept in step with its driver at
//! each PCI Command write. A config write is not posted: it completes only
//! once the device has taken the driver's earlier register writes. So each
//! write waits, on the driver's thread, for the model to finish a look it
//! began after the write was asked for.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use nonos_libc::pci_command_reg;

/// How long a write waits for a model that stopped looking.
const CATCH_UP: Duration = Duration::from_secs(5);

pub struct CatchUp {
    looks: Arc<AtomicU64>,
}

impl CatchUp {
    /// For the device of the calling thread's driver.
    pub fn on_this_thread() -> Self {
        let looks = Arc::new(AtomicU64::new(0));
        let seen = Arc::downgrade(&looks);
        pci_command_reg().before_write(move || caught_up(&seen));
        Self { looks }
    }

    /// The model finished a look.
    pub fn looked(&self) {
        self.looks.fetch_add(1, Ordering::SeqCst);
    }
}

/// Wait for one look begun after now to finish: the one under way may have
/// read the registers before the driver's last write. A model gone, its
/// `CatchUp` dropped, is not waited for.
fn caught_up(looks: &Weak<AtomicU64>) {
    let Some(looks) = looks.upgrade() else { return };
    let (from, deadline) = (looks.load(Ordering::SeqCst), Instant::now() + CATCH_UP);
    while looks.load(Ordering::SeqCst) < from + 2 && Instant::now() < deadline {
        std::thread::yield_now();
    }
}
