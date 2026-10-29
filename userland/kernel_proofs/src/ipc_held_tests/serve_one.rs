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

//! K.3, K.4: MkServeOne serves the run role alone, for a pair SERVES lists,
//! one endpoint a process; and no capsule loaded at run time declares a
//! served name, whatever its namespace.

use super::serve_rule::serves::SERVES;
use super::serve_rule::{admit_serve, is_serve_port, served_endpoint, serves_none};
use super::serve_rule::{ServeRefusal::*, SERVE_NAME_MAX};
use crate::proofs::proofs_4::name_rule::{baked_apps::APPS, baked_names::*, rule::*};

#[test]
fn only_the_run_role_serves_and_only_a_listed_pair() {
    let key = SERVES[0].1;
    assert_eq!(admit_serve(b"app.simplex", &key, true), Ok(0));
    assert_eq!(admit_serve(b"app.simplex", &key, false), Err(NotTheRun), "not the run role");
    let mut other = key;
    other[31] ^= 1;
    assert_eq!(admit_serve(b"app.simplex", &other, true), Err(NotPaired), "another listing");
    assert_eq!(admit_serve(b"app.terminal", &key, true), Err(NotPaired), "another capsule");
    let long = [b'a'; SERVE_NAME_MAX + 1];
    for bad in [&b""[..], b"App.simplex", b"1app", b".app", b"app simplex", b"app\0", &long] {
        assert_eq!(admit_serve(bad, &key, true), Err(BadName), "{bad:?}");
    }
}

#[test]
fn a_served_name_is_the_kernels_alone() {
    for (i, &(capsule, _)) in SERVES.iter().enumerate() {
        let baked = CORE.iter().chain(APPS).any(|&(name, _)| name == capsule);
        assert!(baked, "{capsule} is not reserved in the image");
        let (name, port) = served_endpoint(i);
        assert_eq!(name, format!("linux.serve.{capsule}"));
        assert!(is_serve_port(port) && !is_serve_port(port + SERVES.len() as u32));
        let reply = format!("endpoint.{name}.reply");
        for (ns, publisher) in
            [("systems.nonos.app.linux", false), ("linux.serve", true), ("linux", true)]
        {
            for (n, is_reply) in [(name.as_str(), false), (reply.as_str(), true)] {
                let got = admit_loaded(n, is_reply, ns, publisher, &[CORE, APPS], FAMILIES);
                assert_eq!(got, Err(NameRefusal::Reserved), "{n} under {ns}");
            }
        }
    }
}

#[test]
fn a_process_serves_one_endpoint() {
    let run = ["app.linux.run.net", "endpoint.app.linux.run.net.reply"];
    assert!(serves_none(run.into_iter()), "a run that serves nothing yet");
    let serving = ["app.linux.run.net", "linux.serve.app.simplex"];
    assert!(!serves_none(serving.into_iter()), "a second served endpoint");
    assert!(!serves_none(["linux.serve.app.other"].into_iter()));
}
