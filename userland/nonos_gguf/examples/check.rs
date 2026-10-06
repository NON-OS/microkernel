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

//! Check a GGUF file on the host and print what its header says:
//!   cargo run --release --example check -- model.gguf

use std::fs::File;
use std::os::unix::fs::FileExt;

struct Disk(File);

impl nonos_gguf::ReadAt for Disk {
    fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> bool {
        self.0.read_exact_at(buf, offset).is_ok()
    }
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: check <model.gguf>");
        std::process::exit(2);
    };
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{path}: {e}");
            std::process::exit(1);
        }
    };
    let len = file.metadata().map(|m| m.len()).unwrap_or(0);
    match nonos_gguf::parse(&mut Disk(file), len, &nonos_gguf::DEFAULT) {
        Ok(s) => {
            let text = |t: Option<nonos_gguf::Text>| {
                t.map(|t| String::from_utf8_lossy(t.as_bytes()).into_owned())
            };
            println!("accepted: {path}, {len} bytes, GGUF v{}", s.version);
            println!("tensors {} keys {} alignment {}", s.tensors, s.keys, s.meta.alignment);
            println!(
                "architecture {:?} name {:?} file_type {:?}",
                text(s.meta.architecture),
                text(s.meta.name),
                s.meta.file_type
            );
            println!("data starts at {}, {} bytes of tensor data", s.data_start, s.data_bytes);
            for (ty, n) in s.by_type.iter().enumerate().filter(|(_, n)| **n > 0) {
                println!("  type {ty}: {n} tensors");
            }
        }
        Err(e) => {
            println!("refused: {path}: {e:?}");
            std::process::exit(1);
        }
    }
}
