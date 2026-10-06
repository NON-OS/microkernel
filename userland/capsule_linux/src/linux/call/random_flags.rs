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

//! getrandom's flags, checked as Linux checks them. Pure, so the host
//! proofs hold it.

use crate::linux::abi::errno;

const GRND_NONBLOCK: u64 = 1;
const GRND_RANDOM: u64 = 2;
const GRND_INSECURE: u64 = 4;

/// The flags are an unsigned int: an unknown one is EINVAL, and so is
/// GRND_RANDOM with GRND_INSECURE, which ask for opposite things. Every
/// accepted combination draws from the kernel's one generator, which never
/// blocks once the machine is up, so none changes what is returned.
pub fn random_flags(flags: u64) -> Result<(), i64> {
    let flags = u64::from(flags as u32);
    let both = GRND_RANDOM | GRND_INSECURE;
    if flags & !(GRND_NONBLOCK | both) != 0 || flags & both == both {
        return Err(errno::EINVAL);
    }
    Ok(())
}
