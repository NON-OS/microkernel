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

//! A market program installed in the box its release names (plan/30). The
//! policy is checked before the package is fetched, bound to the program the
//! install records once it is in the store, written beside it and vouched
//! for. Any failure fails the install and leaves nothing of it.

use nonos_libc::mk_debug;

use super::super::record::program::forget;
use super::super::{install, uninstall, Why};
use super::arg::MARKET_LEN;
use super::bind::bind;
use super::check::gate;

/// `market` is None for a release with no guest_policy: only a shipped tier,
/// whose box is the personality's own table's, installs without one.
pub fn install_from(
    pkg: &str,
    pin: &[u8; 32],
    market: Option<&[u8; MARKET_LEN]>,
) -> Result<(), Why> {
    let Some(policy) = gate(pkg, market).map_err(refused)? else {
        return install(pkg, pin);
    };
    // Its programs land held: until bind, nothing of it starts unboxed.
    super::hold::hold(&policy.program_blake3);
    install(pkg, pin)?;
    if let Err(why) = bind(pkg, &policy) {
        let why = refused(why);
        // Whatever of it would not go stays held: its policy is not vouched.
        if uninstall(pkg).is_err() && !forget(pkg) {
            say(&alloc::format!("[LINUX] {pkg}: its record stays, refused until uninstalled\n"));
        }
        return Err(why);
    }
    Ok(())
}

pub(super) fn refused(why: &str) -> Why {
    say(&alloc::format!("[LINUX] refused the market's policy: {why}\n"));
    Why::Policy
}

fn say(line: &str) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
