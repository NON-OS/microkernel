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
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::run::Run;
use nsh_client::Refusal;
use nsh_server::{busy, AttestFault, Why};
use nsh_wire::{Reason, WireError};

#[test]
fn any_frame_but_the_hello_is_out_of_order() {
    let mut r = RawRun::new();
    assert!(r.send(&[3, 0, 0, 0, 0x7F, 5, 0]).is_empty());
    assert_eq!(r.refused(), [Why::Wire(WireError::Order)]);
    let mut r = RawRun::new();
    assert!(r.send(&[1, 0, 0, 0, 0x99]).is_empty());
    assert_eq!(r.refused(), [Why::Wire(WireError::UnknownType)]);
}

#[test]
fn the_hello_fed_a_byte_at_a_time_is_the_same_hello() {
    let mut r = RawRun::new();
    let hello = r.raw.hello.clone();
    let mut out = Vec::new();
    for b in &hello {
        out.extend(r.send(std::slice::from_ref(b)));
    }
    let frames = r.raw.handshake(&out);
    assert_eq!(frames.len(), 1);
    assert!(r.refused().is_empty());
}

#[test]
fn a_busy_host_says_busy_and_the_client_reads_it() {
    let mut run = Run::new(&GrantB::default(), Table::shipped());
    let _ = run.client.take_output();
    let err = run.client.feed(&busy(), &mut |_| true);
    assert_eq!(err, Err(Refusal::Aborted(Reason::Busy)));
}

#[test]
fn a_kernel_that_cannot_attest_is_told_busy() {
    for fault in [AttestFault::Kernel(-11), AttestFault::Malformed] {
        let kernel = Kernel { fault: Some(fault), ..Kernel::honest() };
        let mut run =
            Run::with(&GrantB::default(), config(), kernel, Keys::default(), Table::shipped());
        run.pump();
        assert_eq!(run.c_err, Some(Refusal::Aborted(Reason::Busy)));
        assert_eq!(run.refused(), [Why::Attest(fault)]);
    }
}
