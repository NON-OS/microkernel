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

//! Which builds of the chat program to try, best first, from what the CPU
//! said about itself. Pure, so capsule_linux_proofs holds the order.
//!
//! Under QEMU's software CPU the plain x86-64 build answers fastest; on
//! hardware, the x86-64-v3 build when the CPU runs it, else x86-64-v2. A
//! tier starts the first build the store holds. Every build after the best
//! one runs on this CPU too, and each says why it was chosen, so a store
//! missing the best build is never passed over in silence.

use alloc::vec::Vec;

pub const CHAT: &[u8] = b"/bin/qwenchat";
pub const CHAT_V2: &[u8] = b"/bin/qwenchat-x86_64_v2";
pub const CHAT_PLAIN: &[u8] = b"/bin/qwenchat-x86_64";

const UNDER_TCG: &[u8] = b"qwen: QEMU software CPU, running the plain x86-64 build\n";
const UNDER_TCG_V3: &[u8] = b"qwen: QEMU software CPU and no plain x86-64 build here, running \
    the x86-64-v3 build, which is slower under it\n";
const UNDER_TCG_V2: &[u8] = b"qwen: QEMU software CPU and no plain x86-64 build here, running \
    the x86-64-v2 build, which is slower under it\n";
const NO_V3_BUILD: &[u8] = b"qwen: no x86-64-v3 build here, running the x86-64-v2 build\n";
const NO_AVX2: &[u8] = b"qwen: no AVX2 on this CPU, running the x86-64-v2 build\n";
const NO_FASTER: &[u8] = b"qwen: no faster build here, running the plain x86-64 build\n";

/// The builds of `program` to try, best first, with the line said when
/// each is the one chosen. Only `CHAT` has other builds.
pub fn pick(program: &'static [u8], tcg: bool, v3: bool) -> Vec<(&'static [u8], &'static [u8])> {
    let mut out = Vec::new();
    if program != CHAT {
        out.push((program, &b""[..]));
        return out;
    }
    if tcg {
        out.push((CHAT_PLAIN, UNDER_TCG));
        if v3 {
            out.push((CHAT, UNDER_TCG_V3));
        }
        out.push((CHAT_V2, UNDER_TCG_V2));
        return out;
    }
    if v3 {
        out.push((CHAT, &b""[..]));
        out.push((CHAT_V2, NO_V3_BUILD));
    } else {
        out.push((CHAT_V2, NO_AVX2));
    }
    out.push((CHAT_PLAIN, NO_FASTER));
    out
}
