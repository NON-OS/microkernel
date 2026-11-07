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

//! `CLIENT_HELLO` and the host's proof: only `VERSION` and `BUSY` can be
//! told before keys exist; everything else broken there is closed without
//! a word. A host that cannot attest or sign says `BUSY` or nothing, and
//! never sends a proof the client would refuse.

use crate::kit::host::{config, Kernel, Keys};
use crate::kit::raw_run::RawRun;
use crate::kit::rng::dry;
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::NOW;
use crate::kit::run::Run;
use nsh_client::Refusal;
use nsh_server::{AttestFault, Why};
use nsh_wire::Reason;

#[test]
fn entries_that_are_not_whole_records_are_never_sent() {
    for entries in [vec![], vec![0u8; 44], vec![0u8; 46]] {
        let kernel = Kernel { entries: Some(entries), ..Kernel::honest() };
        let mut run =
            Run::with(&GrantB::default(), config(), kernel, Keys::default(), Table::shipped());
        run.pump();
        assert_eq!(run.refused(), [Why::Attest(AttestFault::Malformed)]);
        assert_eq!(run.c_err, Some(Refusal::Aborted(Reason::Busy)));
    }
}

#[test]
fn host_keys_that_do_not_sign_send_no_proof() {
    let keys = Keys::default();
    keys.refuse.set(true);
    let mut run = Run::with(&GrantB::default(), config(), Kernel::honest(), keys, Table::shipped());
    run.pump();
    assert_eq!(run.refused(), [Why::Sign]);
    assert_eq!(run.c_err, None, "the client hears no proof and sends nothing");
    assert!(run.server.is_over());
}

#[test]
fn no_randomness_no_hello() {
    let mut r = RawRun::new();
    let hello = r.raw.hello.clone();
    r.server.feed(&hello, NOW, &mut dry);
    assert!(r.server.take_output().is_empty());
    assert_eq!(r.server.events(), [nsh_server::Event::Refused(Why::Rand)]);
}
