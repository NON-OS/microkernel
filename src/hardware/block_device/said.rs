// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Saying what each disk showed while the block layer chose, once per
//! change.

use core::sync::atomic::{AtomicU64, Ordering};

use super::backend::Backend;

/// The last line a driver's own report said, by its hash. Each disk keeps
/// its own in its row: a disk asked on every store read is said only when
/// what it shows changes.
static REPORT: AtomicU64 = AtomicU64::new(0);

pub(super) fn say(b: Backend, line: &str) {
    once((b.disk.said)(b.key), line);
    /* The stick driver cannot write to the log; its search is said here. */
    if let Some(report) = b.disk.report.and_then(|r| r()) {
        once(&REPORT, &report);
    }
}

fn once(slot: &AtomicU64, text: &str) {
    let hash = text
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3));
    if slot.swap(hash, Ordering::Relaxed) != hash {
        crate::sys::serial::println(text.as_bytes());
    }
}
