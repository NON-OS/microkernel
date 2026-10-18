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

//! Seeded mutation of the signal frame a handler returns through: whatever
//! bytes sit where the guest's rsp points, rt_sigreturn's reader must return,
//! in a debug build, without a panic.

use super::mutation::damage;
use super::mutation_tests::ROUNDS;
use crate::sigframe::Entry;
use crate::sigframe_build::build;
use crate::sigframe_read::returned;

#[test]
fn a_damaged_signal_frame_never_panics_returning() {
    let mut s = 0x516E_A100u64;
    let mut base = [0u64; 18];
    base[15] = 0x7fff_ff00_0000;
    let info = [0u8; 128];
    let e = Entry {
        handler: 0x4000,
        restorer: 0x4008,
        signum: 11,
        blocked: 0,
        alt_top: None,
        stack: [0, 2, 0],
        info: &info,
    };
    let (_, buf, _) = build(&base, &e).expect("frame");
    for _ in 0..ROUNDS {
        let mut v = buf.clone();
        damage(&mut s, &mut v);
        /* rt_sigreturn reads the ucontext at the guest's rsp: any bytes there. */
        let _ = returned(&v);
        let _ = v.first().map(|_| returned(&v[v.len().min(8)..]));
    }
}
