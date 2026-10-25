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


//! The whole-function pin the glue checks share passes the function as
//! pinned, and fails it changed, given a twin by any spelling of its name,
//! or copied into a string; a constant read from source is the only item of
//! its name.

use super::whole::{num, pinned};

const PIN: [&str; 2] = ["pub fn f(a: u8) -> u8 {", "if a > 0 { a } else { 1 } }"];
const SRC: &str = "use x; pub fn f(a: u8) -> u8 { if a > 0 { a } else { 1 } } fn ff() {}";

#[test]
fn a_pin_holds_the_function_whole_and_alone() {
    assert_eq!(pinned(SRC, "f", &PIN), Ok(()), "the function as pinned failed");
    let changed = SRC.replace("{ 1 }", "{ 2 }");
    let twins = [" fn f (b: u8) -> u8 { b }", " fn r#f() {}", r#" const S: &str = "fn f(";"#];
    let mut bad: Vec<String> = twins.iter().map(|t| format!("{SRC}{t}")).collect();
    bad.extend([changed, SRC.replace("pub fn f(", "pub fn g(")]);
    for b in &bad {
        assert!(pinned(b, "f", &PIN).is_err(), "passed: {b}");
    }
    assert!(pinned(SRC, "f", &PIN[..1]).is_err(), "a pin not one whole function passed");
    let n = "const N: u64 = 7; fn h() -> u64 { N + NN }";
    assert_eq!(num::<u64>(n, "N", 2), 7, "the constant misread");
    let twin = format!("{n} static N: u64 = 9;");
    assert!(std::panic::catch_unwind(|| num::<u64>(&twin, "N", 2)).is_err(), "a twin read");
}
