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
//! What the audio service said when the Sound page asked whether this
//! Ask net.nym for its cover profile when the Network page opens.

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::nym_cover::{read, request, NymCover, HDR_LEN, STATUS_LEN};

const SERVICE: &[u8] = b"net.nym";
const CALL_TIMEOUT_MS: u64 = 500;

pub fn probe_nym_cover() -> NymCover {
    let mut port: u32 = 0;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port as *mut u32, ptr::null_mut());
    if rc != 0 || port == 0 {
        return NymCover::NoService;
    }
    let req = request();
    let mut resp = [0u8; HDR_LEN + STATUS_LEN];
    let got =
        mk_ipc_call_timeout(port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len(), CALL_TIMEOUT_MS);
    if got <= 0 {
        return NymCover::NoService;
    }
    read(&resp[..got as usize])
}
