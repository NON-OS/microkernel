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

//! The raw client brought to a stage: proved, or logged in.

use super::raw_run::RawRun;
use super::roles_build::grant::GrantB;
use super::roles_build::keys::K;
use nsh_server::Event;
use nsh_wire::Kind;

impl RawRun {
    /// The hellos and `HOST_PROOF` done: the server waits for `USER_AUTH`.
    pub fn proved() -> RawRun {
        let mut r = RawRun::new();
        let hello = r.raw.hello.clone();
        let out = r.send(&hello);
        let frames = r.raw.handshake(&out);
        assert_eq!(frames.iter().map(|f| f.0).collect::<Vec<_>>(), [Kind::HostProof.byte()]);
        r
    }

    /// Logged in with the default grant: the session runs.
    pub fn session() -> RawRun {
        let mut r = RawRun::proved();
        let ua = r.raw.user_auth(&GrantB::default(), K::User);
        let sealed = r.raw.seal(Kind::UserAuth, &ua);
        assert!(r.send(&sealed).is_empty(), "nothing before the host answers");
        let caps = match r.events.last() {
            Some(Event::Admit(a)) => a.caps,
            other => panic!("not admitted: {other:?}"),
        };
        let rng = &mut r.rng;
        r.server.begin(caps, r.now, &mut |b| rng.fill(b)).expect("begin");
        r.events.extend(r.server.events());
        let out = r.server.take_output();
        let frames = r.raw.open(&out);
        assert_eq!(frames[0].0, Kind::AuthOk.byte());
        r.raw.logged_in(&ua, &frames[0].1);
        r
    }

    pub fn refused(&self) -> Vec<nsh_server::Why> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::Refused(w) => Some(*w),
                _ => None,
            })
            .collect()
    }
}
