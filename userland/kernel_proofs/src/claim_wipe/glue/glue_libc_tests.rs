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


//! libc's side of a claim answered Resetting (L80): the kernel's only EAGAIN
//! for a claim, which libc asks again on until a deadline the model's own,
//! its count of tries only a backstop. The count must outlast the deadline
//! however a wait is cut short: a sleep ends early only for a delivery to the
//! claimant, whose inboxes hold at most so many until it reads them
//! (inboxes), and every other try sleeps its whole 10 ms. Each function
//! pinned whole (whole).

use super::super::model_attempt::{RESET_MS, TRY_CAP};
use super::super::phase_rule::MS;
use super::code::flat;
use super::whole::{num, whole};

const LIBC: &str = include_str!("../../../../libc/src/broker/device.rs");
const ERRNO: &str = include_str!("../../../../../src/syscall/microkernel/device_errno.rs");
const ERRNOS: &str = include_str!("../../../../../src/syscall/microkernel/errnos.rs");

const CLAIM: [&str; 5] = [
    "pub extern \"C\" fn mk_device_claim(device_id: u64) -> i64 {",
    "let until = crate::time::Deadline::after_ms(RESET_MS); for _ in 0..TRY_CAP {",
    "let got = call_raw(N_MK_DEVICE_CLAIM, [device_id, 0, 0, 0, 0, 0]);",
    "if got != EAGAIN || until.expired() { return got; } let _ = crate::mk_idle_ms(10); }",
    "call_raw(N_MK_DEVICE_CLAIM, [device_id, 0, 0, 0, 0, 0]) }",
];

const ERRNOS_OF: [&str; 8] = [
    "pub(super) fn claim_errno(e: ClaimError) -> i64 { match e {",
    "ClaimError::AlreadyClaimed | ClaimError::Held => ERRNO_BUSY,",
    "ClaimError::Busy | ClaimError::GroupHeld => ERRNO_BUSY,",
    "ClaimError::UnknownDevice | ClaimError::Disabled => ERRNO_NODEV,",
    "ClaimError::NotHolder | ClaimError::NotClaimed => ERRNO_INVAL,",
    "ClaimError::Unconfined | ClaimError::Refused | ClaimError::Firmware => ERRNO_PERM,",
    "ClaimError::OutOfScope => ERRNO_PERM,",
    "ClaimError::Resetting => ERRNO_AGAIN, } }",
];

#[test]
fn libc_asks_again_for_as_long_as_the_model_says() {
    let l = flat(LIBC);
    whole(&l, "mk_device_claim", &CLAIM);
    let e = flat(ERRNO);
    whole(&e, "claim_errno", &ERRNOS_OF);
    assert!(e.contains("use super::super::errnos::{ERRNO_AGAIN, ERRNO_BUSY,"), "another EAGAIN");
    assert_eq!(num::<i64>(&l, "EAGAIN", 2), num::<i64>(&flat(ERRNOS), "ERRNO_AGAIN", 1));
    let (reset_ms, tries) = (num::<u64>(&l, "RESET_MS", 2), num::<u64>(&l, "TRY_CAP", 2));
    let early = super::inboxes::held();
    assert!(tries >= early + reset_ms / 10, "{tries} tries spent before {reset_ms} ms");
    assert_eq!((reset_ms * MS, tries), (RESET_MS, u64::from(TRY_CAP)), "libc's not the model's");
}
