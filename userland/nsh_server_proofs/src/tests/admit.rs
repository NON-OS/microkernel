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

//! The frames around the login that the real client never sends, and the
//! host's answer to an admitted login: `AUTH_OK` names what the session
//! capsule was given, never more than the grant, and a host that cannot
//! start the session says `BUSY`.

use crate::kit::raw_run::RawRun;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::K;
use nsh_roles::CapSet;
use nsh_server::Event;
use nsh_wire::{CapNames, Kind, Reader};

pub(super) const PROTOCOL: (u8, [u8; 2]) = (0x7F, [1, 0]);

pub(super) fn told(frames: &[(u8, Vec<u8>)]) -> Vec<(u8, [u8; 2])> {
    frames.iter().map(|(k, b)| (*k, b[..2].try_into().expect("a code"))).collect()
}

pub(super) fn admitted() -> (RawRun, CapSet) {
    let mut r = RawRun::proved();
    let ua = r.raw.user_auth(&GrantB::default(), K::User);
    let f = r.raw.seal(Kind::UserAuth, &ua);
    r.send(&f);
    let Some(Event::Admit(a)) = r.events.last().cloned() else { panic!("not admitted") };
    (r, a.caps)
}

pub(super) fn caps_of(names: &[&str]) -> CapNames<'static> {
    let mut sorted = names.to_vec();
    sorted.sort_unstable();
    let mut out = vec![sorted.len() as u8];
    for n in sorted {
        out.push(n.len() as u8);
        out.extend_from_slice(n.as_bytes());
    }
    let bytes: &'static [u8] = Box::leak(out.into_boxed_slice());
    CapNames::read(&mut Reader::new(bytes)).expect("names")
}

#[test]
fn any_frame_but_user_auth_is_answered_protocol() {
    for inner in [vec![Kind::Input.byte(), 1], vec![Kind::AuthOk.byte()], vec![0x99]] {
        let mut r = RawRun::proved();
        let f = r.raw.seal_inner(&inner);
        assert_eq!(told(&r.frames(&f)), [PROTOCOL]);
        assert!(r.server.is_over());
    }
}
