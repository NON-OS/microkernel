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

//! The one way out of a kernel panic or a fatal kernel fault: stop the other
//! CPUs, say so, wipe, power off (`zerostate::panic_wipe_and_off`). A fault
//! taken on the way, before the wipe, comes back here and goes straight to
//! the wipe; one taken after it parks. Serial lines here take no lock: the
//! fault may have been raised holding COM1's.

use core::sync::atomic::{AtomicU64, Ordering};

use super::stage::{next, word, Next, IDLE, STOPPING, WIPING};

static STAGE: AtomicU64 = AtomicU64::new(IDLE);

/// Take the path; `paint` runs only for the CPU that takes it first.
pub(crate) fn end(paint: impl FnOnce()) -> ! {
    let me = crate::arch::cpu::get_cpu_id() as u64 + 1;
    let claim =
        STAGE.compare_exchange(IDLE, word(STOPPING, me), Ordering::AcqRel, Ordering::Acquire);
    // A claim that failed found the word another entry left.
    let found = claim.err().unwrap_or(IDLE);
    match next(found, me) {
        Next::Full => {
            paint();
            wipe_and_off(me)
        }
        Next::Wipe => wipe_and_off(me),
        Next::Park => crate::arch::halt_loop(),
    }
}

/// Whether a panic or fatal fault has taken the path (zerostate's
/// `terminate` reads this before its own once-guard).
pub(crate) fn panic_stage_taken() -> bool {
    super::stage::taken(STAGE.load(Ordering::Acquire))
}

/// A fatal kernel fault, from an exception handler's kernel branch.
pub(crate) fn fatal() -> ! {
    end(|| say(b"[ZEROSTATE] fatal kernel fault\r\n"))
}

fn wipe_and_off(me: u64) -> ! {
    crate::smp::send_panic_ipi();
    if crate::smp::others_halted() {
        say(b"[ZEROSTATE] panic: cpus halted, wiping, powering off\r\n");
    } else {
        say(b"[ZEROSTATE] panic: a cpu did not halt; wiping, powering off\r\n");
    }
    STAGE.store(word(WIPING, me), Ordering::Release);
    crate::security::zerostate::panic_wipe_and_off()
}

fn say(line: &[u8]) {
    for &b in line {
        crate::sys::serial::core::write_byte(b);
    }
}
