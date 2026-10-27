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

//! The kernel side, read as text: both calls take the image through the
//! digest proved here, from the caller's memory, serving shootdowns, with
//! the errnos the whole copy gave.

const IMAGE: &str = include_str!("../../../../src/syscall/microkernel/local_image.rs");
const SIGN: &str = include_str!("../../../../src/syscall/microkernel/local_sign.rs");
const VERIFY: &str = include_str!("../../../../src/syscall/microkernel/local_verify.rs");

#[test]
fn both_calls_hash_the_image_where_it_lies() {
    for (call, text) in [("MkLocalSign", SIGN), ("MkLocalVerify", VERIFY)] {
        assert!(text.contains("match digest_in(elf_ptr, elf_len) {"), "{call}");
        assert!(!text.contains("copy_in(elf_ptr"), "{call} still copies the image whole");
    }
    assert!(SIGN.contains("sign(&digest, caps)"));
    assert!(VERIFY.contains("verify_capsule_digest(&trailer, &digest, caps)"));
}

#[test]
fn the_digest_reads_user_memory_serves_and_keeps_the_errnos() {
    let body = &IMAGE[IMAGE.find("pub(super) fn digest_in").expect("digest_in")..];
    let body = &body[..body.find("\n}\n").expect("its end")];
    for needle in [
        "crate::usercopy::copy_from_user(addr, chunk)",
        "ptr.checked_add(at)",
        "digest::in_chunks(len, crate::smp::SERVE_UNIT, read, crate::smp::serve_shootdowns)",
        "Err(DigestError::Length) => Err(ERRNO_INVAL)",
        "Err(DigestError::NoMemory) => Err(ERRNO_NOMEM)",
        "Err(DigestError::Unreadable) => Err(ERRNO_FAULT)",
    ] {
        assert!(body.contains(needle), "missing `{needle}`");
    }
}
