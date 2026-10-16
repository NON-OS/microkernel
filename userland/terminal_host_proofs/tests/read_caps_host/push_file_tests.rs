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

//! A one-file push at 64 MiB: a file of it sent whole, one byte more refused, nothing sent.

use super::super::super::pull::target::Target;
use super::super::args::PushArgs;
use super::super::request::puts;
use super::run_file::push_file;
use crate::clients::vfs;
use crate::fixtures::{said, sized, state};
use crate::process::TERMINAL_PID;

const MAX: usize = 64 * 1024 * 1024;
const PATH: &[u8] = b"/home/nonos/big";

fn push(len: usize) -> (bool, Vec<(bool, Vec<u8>)>) {
    vfs::put(PATH, sized(len));
    let target = Target { host: b"10.0.2.2:8000".to_vec(), port: 8000, path: b"/up/big".to_vec() };
    let mut st = state();
    let a = PushArgs { target };
    let sent = push_file(&mut st, TERMINAL_PID, [10, 0, 2, 2], PATH.to_vec(), &a, Vec::new());
    (sent, st.scrollback.rows)
}

#[test]
fn push_file_sends_a_file_of_64_mib_whole() {
    let (sent, rows) = push(MAX);
    assert!(sent);
    assert_eq!(puts(), vec![(b"/up/big".to_vec(), sized(MAX))]);
    assert_eq!(rows, said(false, b"pushed /up/big"));
}

#[test]
fn push_file_refuses_a_file_one_byte_past_64_mib_and_sends_nothing() {
    let (sent, rows) = push(MAX + 1);
    assert!(!sent);
    assert!(puts().is_empty());
    assert_eq!(rows, said(true, b"larger than a push sends (64 MiB)"));
}
