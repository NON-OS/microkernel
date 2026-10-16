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

//! Whose the primary is: the compositor's, the process its endpoint is
//! registered to, asked at each get_primary. It was its first asker's, so a
//! capsule that asked first had the screen (`respawn_tests` for the rest).

use nonos_libc::hold;

use super::frames::{ask, flush_body};
use super::model::{served, COMPOSITOR, OTHER};
use super::respawn_tests::RESPAWNED;
use crate::serve::{COMPOSITOR_SERVICE, E_BUSY, OP_FLUSH, OP_GET_PRIMARY_SURFACE};

#[test]
fn a_capsule_that_is_not_the_compositor_is_refused_the_primary() {
    let mut s = served(64, 48);
    let flush = flush_body(0, 0, 1, 1);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush).0, E_BUSY, "nobody was handed it");
    let first = ask(&mut s.d, OTHER, OP_GET_PRIMARY_SURFACE, &[]).0;
    assert_eq!(first, E_BUSY, "a capsule that asked first was handed the primary");
    assert_eq!(ask(&mut s.d, OTHER, OP_FLUSH, &flush).0, E_BUSY);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[]).0, 0);
    assert_eq!(ask(&mut s.d, OTHER, OP_GET_PRIMARY_SURFACE, &[]).0, E_BUSY, "a second owner");
    assert_eq!(ask(&mut s.d, OTHER, OP_FLUSH, &flush).0, E_BUSY);
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[]).0, 0, "asked again");
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush).0, 0);
}

#[test]
fn nobody_is_handed_the_primary_while_no_compositor_runs() {
    let mut s = served(64, 48);
    // The name gone, or held by no process, as the kernel holds an endpoint
    // it registers before the process that will own it has a pid.
    for holder in [None, Some(0)] {
        hold(COMPOSITOR_SERVICE, holder);
        for pid in [0, COMPOSITOR, OTHER, RESPAWNED] {
            let got = ask(&mut s.d, pid, OP_GET_PRIMARY_SURFACE, &[]).0;
            assert_eq!(got, E_BUSY, "{pid:#x} handed the primary, name held by {holder:?}");
        }
    }
}

#[test]
fn the_name_asked_for_is_the_one_the_compositors_endpoint_holds() {
    let mk = include_str!("../../../compositor/Capsule.mk");
    let name = std::str::from_utf8(COMPOSITOR_SERVICE).expect("a name");
    assert!(mk.contains(&format!("CAPSULE_SERVICE_ENDPOINT := service:4310:{name}\n")));
}
