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

use alloc::string::String;
use alloc::vec::Vec;

use super::types::TlsCtx;

/*
 * The one TLS connection stash() holds between image fetches. The pool takes
 * it into its own list of kept connections as soon as the image is done, so
 * the slot is always free for the next. The ciphertext buffer persists, and
 * `consumed` marks where the next response begins in the plaintext.
 */
pub struct KeptConn {
    pub host: String,
    pub port: u16,
    pub handle: u32,
    pub tls: TlsCtx,
    pub buf: Vec<u8>,
    pub consumed: usize,
    /* Client application records sent so far: the next request's sequence. */
    pub tx_seq: u64,
    pub used: u8,
}

/*
 * A connection is recycled before its buffer grows past this, and before
 * sequence reuse could ever be in question.
 */
pub const MAX_KEEP_BYTES: usize = 768 * 1024;
pub const MAX_KEEP_USES: u8 = 32;
