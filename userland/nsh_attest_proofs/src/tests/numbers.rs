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

//! The fixed numbers of k1-attest-doc-v4.md and nsh-v1-attest.md, held to
//! their written values, and the fields whose stated count must be the
//! count that follows.

use nsh_attest::consts::{MAX_DOC, MAX_LOG, QUOTED_PCRS_V4, REPLAYED_PCRS};
use nsh_attest::log::replay;
use nsh_attest::{FormatFault, LogFault, Refusal};

use super::{passes, refused};
use crate::build::tcg::{header, SHA1, SHA256};
use crate::build::tcg_event::event;
use crate::build::Spec;

#[test]
fn the_written_numbers_are_the_ones_used() {
    assert_eq!((MAX_DOC, MAX_LOG), (1_114_112, 1_048_576));
    assert_eq!(QUOTED_PCRS_V4, [0, 1, 2, 4, 7, 8, 9, 14]);
    assert_eq!(REPLAYED_PCRS, [0, 1, 2, 4, 7, 14]);
    // The loader's caps, nonos-boot-measure's MAX_EVENTS and MAX_EVENT_BYTES.
    assert_eq!((nsh_attest::log::MAX_EVENTS, nsh_attest::log::MAX_EVENT_BODY), (4096, 65_536));
    let big = |p: &mut crate::build::doc::Parts| p.log.resize(1_048_577, 0);
    assert_eq!(
        refused(Spec { doc: big, ..Spec::default() }),
        Refusal::Format(FormatFault::Oversize)
    );
}

#[test]
fn pcr_7_is_replayed_even_when_enrolled_alike() {
    let s =
        Spec { pcr: vec![(7, [0x5E; 32])], cert: |c| c.pcrs[3] = [0x5E; 32], ..Spec::default() };
    assert_eq!(refused(s), Refusal::Replay(7));
}

#[test]
fn a_pcr_count_below_the_entries_that_follow_is_refused() {
    // Byte 91 is pcr_count: magic 8, version 4, 32 + 32, count 4, 3 flags,
    // grants 4, identity 4.
    let s = Spec { bytes: |d| d[91] = 7, ..Spec::default() };
    assert_eq!(refused(s), Refusal::Format(FormatFault::Value));
    passes(Spec { bytes: |d| assert_eq!(d[91], 8), ..Spec::default() });
}

#[test]
fn an_event_whose_digest_count_is_below_the_digests_is_refused() {
    let mut e = event(0, 1, &[(SHA1, vec![1; 20]), (SHA256, vec![2; 32])], b"");
    e[8] = 1;
    assert_eq!(replay(&[header(), e].concat()).map(|_| ()), Err(LogFault::Bank));
}
