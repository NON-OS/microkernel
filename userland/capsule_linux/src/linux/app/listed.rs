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

//! A store program's listed arguments, as the install role left them beside
//! it (install/market/bind_args.rs): held to the trailer this machine minted
//! over its policy and them together, and read whole. None beside it, it
//! starts with the person's words alone; any other failure refuses the start.

use alloc::vec::Vec;

use nonos_linux_policy::args::{check, MAX_LEN, VOUCH_DOMAIN};
use nonos_linux_policy::LinuxAppPolicy;

use super::args_listed::with_listed;
use crate::linux::attest_paths::beside;
use crate::linux::file::{key, store_found, store_read};
use crate::linux::launch::Launch;
use crate::linux::origin::Origin;

const MAX_TRAILER: u32 = 4096;
const GUEST_CAPS: u64 = 0;

/// The words after argv[0]: for a store program, its listed arguments and
/// the person's (args_listed.rs); for the built-in one, the person's alone.
pub fn start_args(launch: &Launch, policy: &LinuxAppPolicy) -> Result<Vec<Vec<u8>>, &'static [u8]> {
    let person = launch.args.clone();
    let found = match launch.origin {
        Origin::BuiltIn => return Ok(person),
        Origin::Store => listed(&launch.path, policy),
    };
    match found {
        Ok(args) => Ok(with_listed(&args, person)),
        Err(why) => {
            /* Where the person who typed it reads it; the program is not named. */
            crate::linux::say::say(
                alloc::format!("linux: the program does not start: {why}\n").as_bytes(),
            );
            Err(&b"[LINUX] listed arguments refused\n"[..])
        }
    }
}

pub fn listed(path: &[u8], policy: &LinuxAppPolicy) -> Result<Vec<u8>, &'static str> {
    let at = key(&beside(path, b".nonos_linux_policy.args.bin"));
    if !store_found(&at).map_err(|_| "its listed arguments could not be looked for")? {
        return Ok(Vec::new());
    }
    let args = store_read(&at, MAX_LEN as u32 + 1)
        .map_err(|_| "its listed arguments could not be read")?;
    let trailer =
        store_read(&key(&beside(path, b".nonos_linux_policy.args.zk_trailer.bin")), MAX_TRAILER)
            .map_err(|_| "its listed arguments are not vouched for")?;
    let vouched = [VOUCH_DOMAIN, &policy.encode(), &args].concat();
    if !nonos_libc::mk_local_verify(&vouched, GUEST_CAPS, &trailer) {
        return Err("its listed arguments are not this machine's");
    }
    check(&args).map_err(|_| "its listed arguments are malformed")?;
    Ok(args)
}
