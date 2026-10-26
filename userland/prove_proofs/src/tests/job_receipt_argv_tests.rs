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

//! A receipt job is known from argv read into the stack, before any heap, so
//! it never holds the prover's floor (main.rs): the stack read names a
//! receipt exactly when the heap read binds the job to RECEIPT, or would
//! with a pid; never for a prove or deliver job, nor a word elsewhere.

use nonos_proof_abi::wire::RECEIPT;

use crate::job_bound::{parse, receipt};

fn both(a: &[&str]) -> (bool, Option<u16>) {
    let joined = a.join("\0");
    let words: Vec<Vec<u8>> = a.iter().map(|s| s.as_bytes().to_vec()).collect();
    (receipt(joined.as_bytes()), parse(&words).map(|(op, _)| op))
}

#[test]
fn the_stack_read_agrees_with_the_bound_one() {
    let cases: &[&[&str]] = &[
        &["prove", "--nonos-prove-job", "receipt", "9"],
        &["--nonos-prove-job", "receipt", "1"],
        &["--nonos-prove-job", "prove", "9"],
        &["--nonos-prove-job", "deliver", "9"],
        &["receipt", "--nonos-prove-job", "prove", "9"],
        &["--nonos-prove-job", "prove", "receipt", "9"],
        &["--nonos-prove-job", "receipts", "9"],
        &["--nonos-prove-job", "--nonos-prove-job", "receipt", "9"],
        &["receipt"],
    ];
    for a in cases {
        let (stack, bound) = both(a);
        assert_eq!(stack, bound == Some(RECEIPT), "{a:?}");
    }
}

#[test]
fn a_receipt_word_without_a_pid_is_still_a_receipt_job_that_serves_no_one() {
    for a in [&["--nonos-prove-job", "receipt"][..], &["--nonos-prove-job", "receipt", "0"]] {
        assert_eq!(both(a), (true, None));
    }
}
