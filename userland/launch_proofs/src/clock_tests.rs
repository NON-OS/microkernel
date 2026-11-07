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

//! libc's clock-page reader against the kernel's writer: the same layout,
//! the kernel's milliseconds, every malformed page refused, and no torn
//! read while the kernel writes on another thread.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

use super::clock_layout as k;
use super::clock_reader as r;

fn page() -> Vec<AtomicU64> {
    (0..k::WORDS).map(|_| AtomicU64::new(0)).collect()
}

fn values(hz: u64, boot: u64) -> k::Values {
    k::Values { flags: k::FLAG_USER_COUNTER, hz, boot_count: boot }
}

#[test]
fn reader_and_writer_share_one_layout() {
    assert_eq!(
        (k::MAGIC, k::VERSION, k::W_MAGIC, k::W_VERSION, k::W_SEQ, k::W_FLAGS, k::W_HZ, k::W_BOOT_COUNT, k::WORDS),
        (r::MAGIC, r::VERSION, r::W_MAGIC, r::W_VERSION, r::W_SEQ, r::W_FLAGS, r::W_HZ, r::W_BOOT_COUNT, r::WORDS)
    );
    assert_eq!(k::FLAG_USER_COUNTER, r::FLAG_USER_COUNTER);
}

#[test]
fn a_written_page_reads_as_the_kernels_milliseconds() {
    let p = page();
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..20_000 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let hz = 1 + seed % 6_000_000_000;
        let boot = seed.rotate_left(17) >> 8;
        let count = boot.wrapping_add(seed >> 20);
        k::publish(&p, &values(hz, boot));
        let want = super::clock_scale::ticks_to_ms(count.saturating_sub(boot), hz);
        assert_eq!(r::read(&p, || count), Some(want));
        assert_eq!(k::ms_since(count, boot, hz), want);
    }
}

#[test]
fn an_unwritten_or_foreign_page_is_refused() {
    let p = page();
    assert_eq!(r::read(&p, || 5), None, "zero page");
    k::publish(&p, &values(1_000, 0));
    assert_eq!(r::read(&p, || 5_000), Some(5_000));
    p[k::W_MAGIC].store(0xdead, Ordering::Relaxed);
    assert_eq!(r::read(&p, || 5_000), None, "magic");
    k::publish(&p, &values(1_000, 0));
    p[k::W_VERSION].store(k::VERSION + 1, Ordering::Relaxed);
    assert_eq!(r::read(&p, || 5_000), None, "version");
    k::publish(&p, &k::Values { flags: 0, hz: 1_000, boot_count: 0 });
    assert_eq!(r::read(&p, || 5_000), None, "counter not readable from user code");
    k::publish(&p, &values(0, 0));
    assert_eq!(r::read(&p, || 5_000), None, "no rate");
    assert_eq!(r::read(&p[..3], || 5_000), None, "short page");
}

#[test]
fn a_write_that_never_settles_is_given_up_on() {
    let p = page();
    k::publish(&p, &values(1_000, 0));
    p[k::W_SEQ].fetch_add(1, Ordering::Relaxed); // odd: mid-write
    assert_eq!(r::read(&p, || 9), None);
}

#[test]
fn reads_during_writes_are_never_torn() {
    // Two consistent states; a torn read would mix them.
    let a = values(1_000, 0); // count 10_000 -> 10_000 ms
    let b = values(4_000, 2_000); // count 10_000 -> 2_000 ms
    let p: Arc<Vec<AtomicU64>> = Arc::new(page());
    k::publish(&p, &a);
    let stop = Arc::new(AtomicBool::new(false));
    let writer = {
        let (p, stop) = (p.clone(), stop.clone());
        thread::spawn(move || {
            let mut flip = false;
            while !stop.load(Ordering::Relaxed) {
                k::publish(&p, if flip { &a } else { &b });
                flip = !flip;
            }
        })
    };
    let mut read = 0u64;
    let mut gave_up = 0u64;
    for _ in 0..2_000_000 {
        match r::read(&p, || 10_000) {
            Some(ms) => {
                assert!(ms == 10_000 || ms == 2_000, "torn read: {ms}");
                read += 1;
            }
            None => gave_up += 1,
        }
    }
    stop.store(true, Ordering::Relaxed);
    writer.join().expect("writer");
    assert!(read > 0, "every read gave up ({gave_up})");
}
