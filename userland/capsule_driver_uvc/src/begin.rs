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
//! OP_START: the peer asked of the kernel again, the stream agreed with the
//! camera, and the compositor told.

use nonos_uvc::Assembler;

use crate::device::Dev;
use crate::ops::holds;
use crate::start::start;
use crate::stream::stop;
use crate::wire::E_NODEV;

/// A stream started for `peer`: asked of the kernel again, agreed with the
/// camera, and told to the compositor. The most bytes a frame takes.
pub fn begin(d: &mut Dev, peer: u32, args: (u8, u8, u32)) -> Result<u32, i32> {
    d.session.start(peer, holds(peer))?;
    if d.stream.is_some() {
        stop(d);
        d.session.start(peer, true)?;
    }
    let (Some(mut bus), Some(b)) = (d.bus(), d.bound.as_ref()) else {
        d.session.stop();
        return Err(E_NODEV);
    };
    match start(&mut bus, &b.nic, args) {
        Ok(s) => {
            let max = s.max_frame;
            d.stream = Some((s, Assembler::new(max as usize)));
            crate::notify::capture(true);
            Ok(max)
        }
        Err(e) => {
            d.session.stop();
            Err(e)
        }
    }
}
