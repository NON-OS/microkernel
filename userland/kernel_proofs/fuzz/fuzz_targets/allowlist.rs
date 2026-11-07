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

//! The per-capsule syscall allowlist against the capability tables it is
//! derived from. A native capsule is killed on a call outside its list, so a
//! call the tables allow and the list refuses is a capsule killed for nothing,
//! and one the list allows and the tables refuse is a hole: both are crashes
//! here. Each 32-byte step of the input is a call in a capsule's sequence:
//! the number, the token's capability bits, and a word whose low bit says
//! whether the caller may launch apps on another's behalf. The number is any
//! u64, so ids outside the registry come up as often as registered ones.
//! The token is signed as install signs it, so the MAC check `for_token`
//! runs first passes; the same token unsigned must permit nothing.

#![no_main]

use kernel_proofs::capabilities::bits::{bits_to_caps, caps_to_bits};
use kernel_proofs::capabilities::CapabilityToken;
use kernel_proofs::syscall::contract::allowlist::{allowed, decide, depends_on_launcher};
use kernel_proofs::syscall::contract::{allowlist_for, allowlist_for_unsigned, is_allowed, is_allowed_as_shell};
use kernel_proofs::syscall::numbers::SyscallNumber;
use libfuzzer_sys::fuzz_target;

fn word(b: &[u8]) -> u64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&b[..8]);
    u64::from_le_bytes(w)
}

fn never() -> bool {
    false
}

fn always() -> bool {
    true
}

fuzz_target!(|data: &[u8]| {
    for step in data.chunks_exact(32) {
        let nr = word(&step[0..]);
        let bits = caps_to_bits(&bits_to_caps(word(&step[8..])));
        let launches = word(&step[16..]) & 1 == 1;
        let token = CapabilityToken::with_caps(&bits_to_caps(bits));
        let list = allowlist_for(&token);
        let unsigned = allowlist_for_unsigned(&token);
        assert!(!allowed(&unsigned, nr, false) && !allowed(&unsigned, nr, true), "an unsigned token allowed {nr:#x}");
        let (plain, shell) = match SyscallNumber::from_u64(nr) {
            Some(n) => (is_allowed(&token, n), is_allowed_as_shell(&token, n)),
            None => (false, false),
        };
        assert_eq!(allowed(&list, nr, false), plain, "{nr:#x} with {bits:#x}, not a launcher");
        assert_eq!(allowed(&list, nr, true), shell, "{nr:#x} with {bits:#x}, a launcher");
        // The dispatcher's form asks the launcher question only when it matters,
        // and answers as the plain form does either way.
        assert_eq!(decide(&list, nr, never), plain);
        assert_eq!(decide(&list, nr, always), shell);
        assert_eq!(depends_on_launcher(&list, nr), plain != shell);
        let expected = if launches { shell } else { plain };
        assert_eq!(allowed(&list, nr, launches), expected);
    }
});
