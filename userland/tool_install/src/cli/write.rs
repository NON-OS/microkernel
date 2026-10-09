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

//! The install: find the disk by its word, load the image and what this
//! boot carries, plan the whole disk, show it and confirm, write, read back,
//! print the receipt, restart if asked. The plan comes first, so a disk too
//! small is refused before a word is typed and what is written was shown.

use nonos_blk_client::scan;
use nonos_disk::{gather, Plan, ENTROPY_BYTES};
use nonos_libc::{crypto_random, mk_admin_reboot};

use super::carry::StdVfs;
use super::confirm::confirm;
use super::receipt::print_receipt;
use super::source::Image;

pub fn run(word: &str, yes: bool, reboot: bool) -> i32 {
    let disks = scan();
    let Some(d) = disks.iter().find(|d| d.confirm_word() == word) else {
        eprintln!("install: no disk answers to {word}; run `install` for the list");
        return 2;
    };
    let Some(device) = d.device else {
        let why = d.fault.as_deref().unwrap_or("it has no working driver");
        eprintln!("install: that disk is not offered: {why}");
        return 2;
    };
    if let Some(why) = d.refusal() {
        eprintln!("install: {why}; nothing written");
        return 2;
    }
    let image = match Image::load() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("install: {e}");
            return 3;
        }
    };
    let carried = gather(&mut StdVfs);
    let mut entropy = [0u8; ENTROPY_BYTES];
    if crypto_random(entropy.as_mut_ptr(), entropy.len()) < 0 {
        eprintln!("install: the kernel gave no entropy for the disk identifiers");
        return 3;
    }
    let store = carried.store.clone();
    let plan = match Plan::new(device.sectors, &image.as_disk_image(), store, entropy) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("install: {e}; nothing written");
            return 3;
        }
    };
    if !confirm(d, &plan, &carried, word, yes) {
        return 0;
    }
    let (receipt, verified) = match super::run::write_and_verify(device, plan) {
        Ok(r) => r,
        Err(code) => return code,
    };
    print_receipt(&receipt, verified);
    if reboot {
        println!("restarting");
        mk_admin_reboot();
    }
    0
}
