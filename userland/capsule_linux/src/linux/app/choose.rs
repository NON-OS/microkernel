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

//! Which policy binds a proved program, in order of who vouches for it:
//! the vendor's table compiled into this capsule, then a policy file this
//! machine vouched for at install, then the owner's default for a program
//! that came with none.

use nonos_linux_policy::{decode, unlisted, LinuxAppPolicy, LEN};

/// `table` is the vendor's policies end to end; `file`, the vouched policy
/// beside the program, if there is one. A file that does not decode, or
/// names another program, refuses the program rather than falling back to
/// the default: the default could be wider than what was published.
pub fn choose(
    program: &[u8; 32],
    table: &[u8],
    file: Option<&[u8]>,
) -> Result<LinuxAppPolicy, &'static str> {
    let vendor = table
        .chunks_exact(LEN)
        .filter_map(|p| decode(p).ok())
        .find(|p| p.program_blake3 == *program);
    if let Some(p) = vendor {
        return Ok(p);
    }
    let Some(file) = file else {
        return Ok(unlisted(*program));
    };
    let p = decode(file).map_err(|_| "its policy is malformed")?;
    if p.program_blake3 != *program {
        return Err("its policy names another program");
    }
    Ok(p)
}
