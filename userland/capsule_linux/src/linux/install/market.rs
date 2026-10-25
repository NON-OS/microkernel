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

//! A market program installed in the box its release names. The
//! policy is checked before the package is fetched, bound to the program the
//! install records once it is in the store, written beside it and vouched
//! for. Any failure fails the install and leaves nothing of it.

use nonos_libc::mk_debug;

use super::market_arg::MARKET_LEN;
use super::market_check::{admit, names_program};
use super::record::vouch::sign_beside;
use super::{install, recorded, uninstall, Why};
use crate::linux::attest_paths::beside;
use crate::linux::file::{key, store_read, store_write};
use nonos_linux_policy::LinuxAppPolicy;

const MAX_IMAGE: u32 = 64 << 20;

/// `market` is None for a release with no guest_policy: the program then runs
/// in the box the vendor table, or the owner's default, gives it.
pub fn install_from(
    pkg: &str,
    pin: &[u8; 32],
    market: Option<&[u8; MARKET_LEN]>,
) -> Result<(), Why> {
    let Some(market) = market else { return install(pkg, pin) };
    let policy = admit(market).map_err(refused)?;
    install(pkg, pin)?;
    if let Err(why) = bind(pkg, &policy) {
        let _ = uninstall(pkg);
        return Err(refused(why));
    }
    Ok(())
}

fn bind(pkg: &str, policy: &LinuxAppPolicy) -> Result<(), &'static str> {
    let path = recorded(pkg).ok_or("the package records no program")?;
    let image = store_read(&key(&path), MAX_IMAGE).map_err(|_| "its program could not be read")?;
    if !names_program(policy, &image) {
        return Err("its policy names another program");
    }
    store_write(&key(&beside(&path, b".nonos_linux_policy.bin")), &policy.encode())
        .map_err(|_| "its policy could not be written")?;
    match sign_beside(&path, &policy.encode(), b".nonos_linux_policy.zk_trailer.bin") {
        true => Ok(()),
        false => Err("its policy could not be vouched for: no enrolled root, or may not mint"),
    }
}

fn refused(why: &str) -> Why {
    let line = alloc::format!("[LINUX] refused the market's policy: {why}\n");
    let _ = mk_debug(line.as_ptr(), line.len());
    Why::Policy
}
