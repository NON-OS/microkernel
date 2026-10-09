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

//! About shows what the kernel holds, not what the build declared: the
//! Overview badge is green only when the registry records a proved
//! admission for this window, the capability count is taken from the word
//! the kernel recorded, and the Display screen names the path the compositor
//! actually presents through.

use crate::about_data::admission::{badge, of, Admission};
use crate::about_data::caps::{granted, ALL_CAPS, ATTEST_READ, DEBUG, MASK};
use crate::about_data::present::path;
use crate::about_data::trust::HYBRID_SCHEME;

const ME: u32 = 41;

#[test]
fn a_window_admitted_under_a_proof_is_verified() {
    let registry = [(7, 0), (ME, 0), (52, 255)];
    assert_eq!(of(ME, Some(&registry)), Admission::Proved);
    let enrolled = [(ME, 3)];
    assert_eq!(of(ME, Some(&enrolled)), Admission::Proved);
    assert_eq!(badge(Admission::Proved), (&b"Verified"[..], true, HYBRID_SCHEME));
}

#[test]
fn a_window_admitted_on_a_signature_alone_is_not_drawn_as_a_pass() {
    let registry = [(ME, 255)];
    assert_eq!(of(ME, Some(&registry)), Admission::SignedOnly);
    assert!(!badge(Admission::SignedOnly).1);
}

#[test]
fn a_window_the_registry_does_not_list_is_not_admitted() {
    let registry = [(7, 0), (52, 255)];
    assert_eq!(of(ME, Some(&registry)), Admission::Missing);
    let (label, pass, _) = badge(Admission::Missing);
    assert_eq!(label, b"Not admitted");
    assert!(!pass);
}

#[test]
fn a_registry_that_was_not_read_is_unknown_never_verified() {
    assert_eq!(of(ME, None), Admission::Unread);
    assert_eq!(of(0, Some(&[(0, 0)])), Admission::Unread);
    let (label, pass, _) = badge(Admission::Unread);
    assert_eq!(label, b"Unknown");
    assert!(!pass);
}

#[test]
fn the_capability_count_follows_the_word_it_is_given() {
    assert_eq!(granted(MASK), 6);
    assert_eq!(granted(MASK & !ATTEST_READ), 5);
    assert_eq!(granted(MASK | DEBUG), 7);
    assert_eq!(granted(0), 0);
    assert_eq!(granted(u64::MAX), ALL_CAPS.len() as u64);
}

#[test]
fn the_present_path_names_virtio_only_when_the_driver_announced() {
    let virtio = path(true);
    assert_eq!(virtio.hops[2], b"driver.virtio_gpu");
    assert_eq!(virtio.backend, b"compositor + driver.virtio_gpu");
    let firmware = path(false);
    assert_eq!(firmware.hops[2], b"kernel blit");
    assert_eq!(firmware.backend, b"compositor + firmware framebuffer");
}
