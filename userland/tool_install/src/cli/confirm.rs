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

//! The confirmation: what is about to be erased, then the word typed back.
//! `--yes` skips the prompt for a script that already named the disk on
//! its command line; naming it once is the confirmation in that case.

use std::io::{BufRead, Write};

use nonos_blk_client::{Contents, Disk};

use super::source::bytes;

pub fn confirm(d: &Disk, word: &str, yes: bool) -> bool {
    let name = d.identity.map(|i| i.model_str().to_string());
    println!("disk     {}", name.as_deref().unwrap_or(d.label()));
    println!("bus      {}", d.label());
    if let Some(i) = d.identity {
        println!("serial   {}", i.serial_str());
    }
    println!("size     {}", bytes(d.bytes()));
    println!("holds    {}", d.contents.text());
    match d.contents {
        Contents::Nonos => println!("this disk holds NONOS already; it will be replaced"),
        Contents::Blank => {}
        _ => println!("everything on this disk is erased"),
    }
    if yes {
        return true;
    }
    print!("type {word} to erase it and install, anything else to stop: ");
    let _ = std::io::stdout().flush();
    let mut typed = String::new();
    if std::io::stdin().lock().read_line(&mut typed).is_err() {
        println!("no confirmation read; nothing written");
        return false;
    }
    if typed.trim() != word {
        println!("not confirmed; nothing written");
        return false;
    }
    true
}
