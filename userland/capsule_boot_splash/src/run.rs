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

//! The splash's whole life: raised on the compositor, redrawn at each new
//! size the person asks for, and given back when they are done.

use nonos_libc::{mk_attest_status, mk_uptime_ms, AttestStatus};

use crate::{display, follow, input, interact, look, proto, ready, scene, show};

pub fn run() -> i32 {
    let Some(comp) = ready::wait_compositor() else { return 2 };
    let Ok(size) = display::query(comp, 2) else { return 4 };
    let mut att = AttestStatus::default();
    let rc = mk_attest_status(&mut att);
    let chain = look::chain(&att, rc);
    let (mut up, mut look) = match show::raise(comp, size, att, chain) {
        Ok(u) => u,
        Err(code) => return code,
    };
    let mut f = follow::Follow::new(size, mk_uptime_ms());
    let grab = proto::lookup(b"input_router");
    if let Some(rp) = grab {
        let _ = input::subscribe(rp, 10);
        let _ = input::grab(rp, 11);
    }
    while let Some(next) = interact::interact(comp, up.base, &look, &mut f) {
        match show::raise(comp, next, att, chain) {
            Ok((u, l)) => {
                show::give_back(&up);
                (up, look) = (u, l);
            }
            // Not drawn at the new size: the old drawing stays, as it was.
            Err(_) => f.size = up.size,
        }
    }
    if let Some(rp) = grab {
        let _ = input::release(rp, 12);
    }
    let _ = scene::remove(comp, 5);
    show::give_back(&up);
    0
}
