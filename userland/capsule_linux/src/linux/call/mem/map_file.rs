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

//! A private file mapping.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::map_exec::proven;
use super::map_fill::{fill_from, fill_read};
use super::map_req::MapReq;
use super::prot::PROT_EXEC;
use super::prot_span::protect_span;

pub fn file(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    /*
     * Asked before a page is allocated, not after the bytes are in place: a
     * mapping that would be refused should cost the guest nothing, and a
     * half-filled span left behind by a late refusal is memory the guest still
     * holds and did not ask to keep.
     */
    let proved = (req.prot & PROT_EXEC != 0).then(|| proven(guest, req.fd));
    if let Some(None) = proved {
        return errno::fail(errno::EPERM);
    }
    if !req.make_room(guest, at, span) || guest.map(at, span, true, false) < 0 {
        return errno::fail(errno::ENOMEM);
    }
    /* Linux reloads a file's bytes after MADV_DONTNEED; this capsule cannot. */
    guest.mark_kept(at, span);
    if let Some(Some(bytes)) = proved {
        if fill_from(guest, &bytes, req, at) < 0 {
            return errno::fail(errno::EACCES);
        }
        return finish(guest, req, at, span);
    }
    if fill_read(guest, req, at) < 0 {
        return errno::fail(errno::EACCES);
    }
    /* Not proved, since nothing asked to run it: it stays that way. */
    guest.mark_unproven(at, span);
    finish(guest, req, at, span)
}

fn finish(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    if protect_span(guest, at, span, req.prot) < 0 {
        return errno::fail(errno::EACCES);
    }
    errno::ok(at)
}
