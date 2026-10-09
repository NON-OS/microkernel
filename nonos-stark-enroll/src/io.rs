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

//! Files and hex, with every failure a message and an exit rather than a panic.

use std::fs;
use std::io::Read;
use std::process::exit;

pub fn read(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|e| die(&format!("cannot read {path}: {e}")))
}

pub fn write(path: &str, bytes: &[u8]) {
    fs::write(path, bytes).unwrap_or_else(|e| die(&format!("cannot write {path}: {e}")));
}

pub fn read_root(path: &str) -> [u8; 32] {
    read(path).try_into().unwrap_or_else(|_| die(&format!("root {path} is not 32 bytes")))
}

pub fn hash_of(path: &str) -> [u8; 32] {
    *blake3::hash(&read(path)).as_bytes()
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn unhex32(s: &str) -> Result<[u8; 32], String> {
    if s.len() != 64 {
        return Err(format!("{s} is not 32 hex bytes"));
    }
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).map_err(|_| format!("bad hex {s}"))?;
    }
    Ok(out)
}

/// 32 bytes from the operating system's generator; a short read is fatal.
pub fn os_random() -> [u8; 32] {
    let mut out = [0u8; 32];
    fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut out))
        .unwrap_or_else(|e| die(&format!("cannot read /dev/urandom: {e}")));
    out
}

pub fn die(msg: &str) -> ! {
    eprintln!("{msg}");
    exit(1)
}
