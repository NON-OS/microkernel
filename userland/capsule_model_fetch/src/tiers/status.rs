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

/*
 * Where a tier stands on this machine, asked of the kernel file by file
 * without holding a stream or reading a byte of the volume.
 */

use crate::catalogue::{File, Tier};
use crate::feed::{begin, Start};

const ENOSPC: i64 = -28;

pub enum Status {
    Installed,
    /* Bytes of it on the volume, some of it still to come. */
    Partial(u64),
    Available,
    /* The volume has no room for what is still to come. */
    NoRoom,
    /* A file name too long for the volume to keep a mark beside. */
    Unkept,
    /* The kernel refused to say; its errno. */
    Unknown(i64),
}

pub fn status(tier: &Tier) -> Status {
    if !tier.files.iter().all(File::keepable) {
        return Status::Unkept;
    }
    let (mut have, mut whole) = (0u64, 0usize);
    for f in &tier.files {
        match begin(&f.volume_name(), &f.sha256, f.bytes, true) {
            Ok(Start::Done) => (have, whole) = (have + f.bytes, whole + 1),
            Ok(Start::From(at)) => have += at,
            Err(ENOSPC) => return Status::NoRoom,
            Err(e) => return Status::Unknown(e),
        }
    }
    match (whole == tier.files.len(), have) {
        (true, _) => Status::Installed,
        (false, 0) => Status::Available,
        (false, have) => Status::Partial(have),
    }
}
