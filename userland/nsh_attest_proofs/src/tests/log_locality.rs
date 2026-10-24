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

//! The startup-locality event: PCR 0 starts at the locality, once, before
//! anything is extended into it, for a locality a PC starts from.

use nsh_attest::log::replay;
use nsh_attest::LogFault;

use super::passes;
use crate::build::tcg::EV_NO_ACTION;
use crate::build::tcg_event::{digest, event};
use crate::build::tcg_log::Log;
use crate::build::Spec;

fn locality(pcr: u32, body: &[u8]) -> Vec<u8> {
    let zeros = vec![(0x0004, vec![0; 20]), (0x000B, vec![0; 32])];
    event(pcr, EV_NO_ACTION, &zeros, body)
}

fn body(l: u8) -> Vec<u8> {
    [&b"StartupLocality\0"[..], &[l]].concat()
}

fn started_at(l: u8) -> Log {
    let mut log = Log::empty();
    log.bytes.extend(locality(0, &body(l)));
    log.pcrs[0][31] = l;
    log.ext(0, 8, digest(b"crtm")).ext(4, 0x8000_0003, crate::build::tcg_log::loader());
    log
}

#[test]
fn a_tpm_started_from_locality_3_or_4_starts_pcr_0_there() {
    for l in [0, 3, 4] {
        passes(Spec { log: started_at(l), ..Spec::default() });
    }
    let mut log = started_at(3);
    log.pcrs[0] = Log::empty().ext(0, 8, digest(b"crtm")).pcrs[0];
    let r = replay(&log.bytes).expect("replays").pcrs[0];
    assert_ne!(r, log.pcrs[0], "the locality is not ignored");
}

#[test]
fn a_misplaced_repeated_or_malformed_locality_is_refused() {
    let after = [Log::empty().ext(0, 8, digest(b"crtm")).bytes.clone(), locality(0, &body(3))];
    let twice = [started_at(3).bytes, locality(0, &body(3))];
    let lists = [
        after.concat(),
        twice.concat(),
        [Log::empty().bytes, locality(0, &body(2))].concat(),
        [Log::empty().bytes, locality(1, &body(3))].concat(),
        [Log::empty().bytes, locality(0, &[body(3), vec![0]].concat())].concat(),
        [Log::empty().bytes, locality(0, b"StartupLocality\0")].concat(),
    ];
    for (i, log) in lists.iter().enumerate() {
        assert_eq!(replay(log).map(|_| ()), Err(LogFault::Locality), "case {i}");
    }
}
