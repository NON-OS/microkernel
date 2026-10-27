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

//! wl_data_offer.receive, and the primary selection offer's: the client
//! passes a descriptor to be written, which goes to the source the offer
//! stands for as its send, or is written the desktop's text (desk.rs).

use crate::linux::file::install;
use crate::linux::guest::{Fd, Guest};

use super::super::args::Args;
use super::super::shm::take_fd;
use super::state::DESKTOP;
use super::which::Which;

pub fn receive(guest: &mut Guest, which: Which, offer: u32, args: &mut Args<'_>) {
    let mime = args.string();
    // The request's own descriptor is taken whatever becomes of it, so the
    // next request that carries one is not given this one.
    let fd = take_fd(guest);
    let (Some(mime), Some(fd)) = (mime, fd) else { return };
    match super::set::target(&guest.objects, &guest.scene.clip, which, offer) {
        Some(DESKTOP) => super::desk::paste(guest, mime, fd as u64),
        Some(source) => forward(guest, (which, source), mime, fd as u64),
        None => {}
    }
}

/// The source is given a descriptor of its own onto what the client
/// passed, in the control data of the read that carries its send, as a
/// compositor passes it. Both are this guest's: the client closes the one
/// it sent once its request has gone, as libwayland does, and the source
/// closes its own once it has written, which ends what the reader reads.
fn forward(guest: &mut Guest, (which, source): (Which, u32), mime: &[u8], fd: u64) {
    let Some(mut copy) = guest.fds.get(fd as usize).filter(|f| f.is_open()).map(Fd::clone_of)
    else {
        return;
    };
    // libwayland reads its descriptors with MSG_CMSG_CLOEXEC.
    copy.cloexec = true;
    let Some(copy) = install(guest, copy) else { return };
    guest.display.give.push(copy as u32);
    super::set::forward(which, source, mime, &mut guest.display.to_client);
}
