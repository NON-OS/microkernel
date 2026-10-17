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

//! A bond is authenticated exactly when a MITM-protecting user step ran.
//! Over every responder IO capability and MITM choice, pairing against the
//! twin records `authenticated` iff the method is not Just Works, iff the
//! user was asked something. A peer that merely drops its MITM bit cannot
//! reach Just Works while it still has the IO for passkey or numeric: this
//! host asks for MITM (`REQUEST`), so only a peer with no IO is Just Works.

use crate::pairing_tests::paired;
use crate::sim::{Peer, Run};
use nonos_bt_smp::method::{IoCap, Method};

const CAPS: [IoCap; 5] = [
    IoCap::DisplayOnly,
    IoCap::DisplayYesNo,
    IoCap::KeyboardOnly,
    IoCap::NoInputNoOutput,
    IoCap::KeyboardDisplay,
];

#[test]
fn authenticated_is_exactly_a_user_verified_method() {
    for io in CAPS {
        for mitm in [false, true] {
            let mut run = Run::new(Peer::new(io, mitm));
            let bond = paired(&mut run);
            let method = run.init.method();
            let by_user = !run.users.is_empty();
            assert_eq!(
                bond.authenticated,
                method != Method::JustWorks,
                "{io:?} mitm={mitm}: level disagrees with the method"
            );
            assert_eq!(
                bond.authenticated, by_user,
                "{io:?} mitm={mitm}: level without a user step"
            );
        }
    }
}

#[test]
fn only_a_peer_with_no_io_reaches_just_works() {
    // The host insists on MITM, so a keyboard that clears its own MITM bit
    // is still paired authenticated: it cannot talk itself down to the clear.
    for io in CAPS {
        let mut run = Run::new(Peer::new(io, false));
        let bond = paired(&mut run);
        let just_works = run.init.method() == Method::JustWorks;
        assert_eq!(just_works, io == IoCap::NoInputNoOutput, "{io:?}: Just Works off no-IO only");
        assert_eq!(bond.authenticated, io != IoCap::NoInputNoOutput, "{io:?}: level");
    }
}
