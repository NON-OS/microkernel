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

//! The install: find the disk by its word, confirm, write in steps with a
//! line every ten percent, read back, print the receipt, restart if asked.

use nonos_blk_client::scan;
use nonos_disk::Plan;
use nonos_libc::{crypto_random, mk_admin_reboot};

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
        eprintln!("install: that disk has no working driver");
        return 2;
    };
    if !confirm(d, word, yes) {
        return 0;
    }
    let image = match Image::load() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("install: {e}");
            return 3;
        }
    };
    let mut entropy = [0u8; 36];
    if crypto_random(entropy.as_mut_ptr(), entropy.len()) < 0 {
        eprintln!("install: the kernel gave no entropy for the disk identifiers");
        return 3;
    }
    let plan = match Plan::new(device.sectors, &image.as_disk_image(), entropy) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("install: {e:?}");
            return 3;
        }
    };
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
