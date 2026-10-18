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

//! The device's PCI Command register, one per test thread. The driver
//! writes it through the broker as a plain store; the device's model reads
//! it from a thread of its own. A config write is not posted: it completes
//! only once the device has taken the driver's earlier register writes, so
//! a model may have each write wait until it has caught up (`before_write`).

use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

type Hook = Box<dyn Fn() + Send>;

pub struct PciCommandReg {
    value: AtomicU16,
    before_write: Mutex<Option<Hook>>,
}

impl PciCommandReg {
    /// The register as the driver last wrote it.
    pub fn get(&self) -> u16 {
        self.value.load(Ordering::SeqCst)
    }

    /// Run `hook` on the driver's thread before each later write lands.
    pub fn before_write(&self, hook: impl Fn() + Send + 'static) {
        *self.hook() = Some(Box::new(hook));
    }

    fn hook(&self) -> MutexGuard<'_, Option<Hook>> {
        self.before_write.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

thread_local! {
    static COMMAND: Arc<PciCommandReg> =
        Arc::new(PciCommandReg { value: AtomicU16::new(0), before_write: Mutex::new(None) });
}

/// This thread's PCI Command register, for its device's model to read.
pub fn pci_command_reg() -> Arc<PciCommandReg> {
    COMMAND.with(Arc::clone)
}

pub(crate) fn write(value: u16) {
    COMMAND.with(|c| {
        if let Some(hook) = c.hook().as_ref() {
            hook();
        }
        c.value.store(value, Ordering::SeqCst);
    });
}
