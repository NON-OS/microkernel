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

//! The nsh-verify-record tool's usage errors: each is exit code 2.

use super::cli_kit::{args, dir, file, FLEET};
use crate::kit::attest::TestAttester;
use crate::kit::keys;
use nsh_client::cli::run;

#[test]
fn the_tool_refuses_bad_usage() {
    let d = dir("usage");
    let key = file(&d, "key", &keys::authority().public_bytes());
    let short_key = file(&d, "short", &[0; 100]);
    let a = TestAttester::default();
    let code = |list: &[&str]| run(&args(list), &a).0;
    assert_eq!(code(&[]), 2);
    assert_eq!(code(&["r", "--fleet", FLEET, "--authority", &key]), 2, "no --doc");
    assert_eq!(code(&["r", "--doc", "d", "--authority", &key]), 2, "no --fleet");
    assert_eq!(code(&["r", "--doc", "d", "--fleet", FLEET]), 2, "no --authority");
    assert_eq!(code(&["r", "--doc", "d", "--fleet", "f1", "--authority", &key]), 2);
    assert_eq!(code(&["r", "--doc", "d", "--fleet", FLEET, "--authority", &key, "--x", "y"]), 2);
    assert_eq!(code(&["r", "--doc", "d", "--fleet", FLEET, "--authority", &key, "--ckpt"]), 2);
    assert_eq!(code(&["/no/such", "--doc", "d", "--fleet", FLEET, "--authority", &key]), 2);
    let bad_key = ["/dev/null", "--doc", "/dev/null", "--fleet", FLEET, "--authority", &short_key];
    assert_eq!(code(&bad_key), 2);
}
