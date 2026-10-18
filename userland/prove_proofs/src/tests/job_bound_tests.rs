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


//! The job serves the op and pid init wrote in its argv, and nothing when
//! argv does not say both: the kernel writes "--nonos-prove-job", "prove" or
//! "deliver", and the asker's pid in decimal (job_argv.rs).

use nonos_proof_abi::wire::{DELIVER, PROVE};

use crate::job_bound::parse;

fn args(a: &[&str]) -> Vec<Vec<u8>> {
    a.iter().map(|s| s.as_bytes().to_vec()).collect()
}

#[test]
fn the_job_serves_the_op_and_pid_init_wrote() {
    assert_eq!(parse(&args(&["--nonos-prove-job", "deliver", "4242"])), Some((DELIVER, 4242)));
    assert_eq!(parse(&args(&["prove", "--nonos-prove-job", "prove", "7"])), Some((PROVE, 7)));
}

#[test]
fn argv_that_does_not_say_both_binds_no_one() {
    for a in [
        &["--nonos-prove-job"][..],
        &["--nonos-prove-job", "prove"],
        &["--nonos-prove-job", "isolate", "7"],
        &["--nonos-prove-job", "prove", "0"],
        &["--nonos-prove-job", "prove", "-7"],
        &["--nonos-prove-job", "prove", "4294967296"],
        &["prove", "7"],
    ] {
        assert_eq!(parse(&args(a)), None, "{a:?}");
    }
}
