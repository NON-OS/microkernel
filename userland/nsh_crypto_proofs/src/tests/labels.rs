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

//! The labels and the attestation challenge, against vectors/schedule.txt.

use super::schedule::chain;
use crate::{arr, sched};
use nsh_crypto::{attest_challenge, labelled, Label};

#[test]
fn the_labels_and_the_challenge_match() {
    let th = chain();
    assert_eq!(attest_challenge(&th[2], &[0xED; 32], &[0x4D; 1952]), arr(&sched("challenge")));
    let names = ["host-sig", "user-sig", "record-sig", "record-ckpt", "grant", "approve", "enrol"];
    let labels = [
        Label::HostSig,
        Label::UserSig,
        Label::RecordSig,
        Label::RecordCkpt,
        Label::Grant,
        Label::Approve,
        Label::Enrol,
    ];
    for (n, l) in names.iter().zip(labels) {
        assert_eq!(labelled(l, b"inner"), arr::<64>(&sched(&format!("label {n}"))), "{n}");
    }
}
