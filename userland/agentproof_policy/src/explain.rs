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

//! What a compiled policy allows, in plain words: capsule_linux's own
//! allowed lines, then what each means for a program run in it.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::boxed::allowed::allowed;
use crate::compile::Compiled;

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn explain(c: &Compiled) -> Vec<String> {
    let word = String::from_utf8_lossy(&c.word).into_owned();
    let mut out = Vec::new();
    out.push(format!("policy   {} {}", word, hex(&c.hash)));
    out.push(format!("run      linux --box {word} <program> [arguments]"));
    out.extend(allowed(&c.policy, None).into_iter().map(|l| format!("allowed  {l}")));
    out.push(String::new());
    out.extend(plain(c));
    out
}

fn plain(c: &Compiled) -> Vec<String> {
    let folders: Vec<String> =
        c.policy.folders.iter().map(|f| String::from_utf8_lossy(f).into_owned()).collect();
    [
        format!("It may write only inside {}.", folders.join(", ")),
        "Any other write, make, rename or removal is refused, and counted as write.".to_string(),
        "Its folders are new when it starts and wiped when it ends.".to_string(),
        "It reads the image, read-only.".to_string(),
        network(c),
        "It may start other programs, and each is held to this same box.".to_string(),
        "A program gets what both this box and its own policy allow; a folder in /home or \
         /root is writable only when the program's own policy gives it a home."
            .to_string(),
        "The report's counts are capsule_linux's enforcement, not the kernel's sealed record."
            .to_string(),
    ]
    .to_vec()
}

fn network(c: &Compiled) -> String {
    let refused = "A socket, connection, bind or datagram the box does not allow is refused, \
                   and counted as network.";
    match c.policy.dests.is_empty() {
        true => format!("It has no network. {refused}"),
        false => format!(
            "It may open TCP connections to the destinations listed, and to no other, each over \
             the anonymity network the system uses (the Nym mixnet, or Anyone when chosen), never \
             directly. A name is answered inside NONOS, never looked up outside. It sends no \
             UDP. {refused}"
        ),
    }
}
