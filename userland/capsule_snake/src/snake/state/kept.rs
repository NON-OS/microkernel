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

// Where the ranks and awards ended up, said on the Ranks screen so ranks that
// never reached the disk are not taken for ranks that did.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kept {
    // Nothing read or written yet that needs saying.
    Quiet,
    // Written and committed to the disk store.
    Disk,
    // Written to the file service but not to the disk, for the reason given.
    Session(&'static str),
    // Not written at all, for the reason given.
    NotSaved(&'static str),
    // Ranks were stored but could not be read back, for the reason given.
    Unread(&'static str),
}

// The transport error the vfs client raises when nothing answers.
const TRANSPORT: &str = "vfs ipc failed";
// What the vfs client says when a file cannot be opened: on the first run
// there is simply no ranks file yet.
const NO_FILE: &str = "vfs open failed";
// What the vfs client says for EACCES, which a persist gets on a boot where
// the person chose at setup to keep nothing on disk.
const DENIED: &str = "access denied";

pub const NO_SERVICE: &str = "the file service did not answer";
pub const AMNESIC: &str = "this boot keeps nothing on disk";
pub const DAMAGED: &str = "the stored file is damaged";

pub fn reason(err: &'static str) -> &'static str {
    if err == TRANSPORT {
        return NO_SERVICE;
    }
    err
}

// One file: written into the file service, then committed to disk.
pub fn of_file(write: Result<(), &'static str>, persist: Result<(), &'static str>) -> Kept {
    if let Err(err) = write {
        return Kept::NotSaved(reason(err));
    }
    match persist {
        Ok(()) => Kept::Disk,
        Err(DENIED) => Kept::Session(AMNESIC),
        Err(err) => Kept::Session(reason(err)),
    }
}

fn weight(kept: Kept) -> u8 {
    match kept {
        Kept::Quiet => 0,
        Kept::Disk => 1,
        Kept::Session(_) => 2,
        Kept::NotSaved(_) | Kept::Unread(_) => 3,
    }
}

// Two files saved together are only as kept as the less kept of the two.
pub fn worse(a: Kept, b: Kept) -> Kept {
    if weight(b) > weight(a) {
        b
    } else {
        a
    }
}

// What a read of one stored file says. A file that is not there is the first
// run and says nothing; one that is there but did not read or decode means
// ranks that exist are not on the screen.
pub fn of_read(read: Result<(), &'static str>, decoded: bool) -> Kept {
    match read {
        Err(NO_FILE) => Kept::Quiet,
        Err(err) => Kept::Unread(reason(err)),
        Ok(()) if decoded => Kept::Quiet,
        Ok(()) => Kept::Unread(DAMAGED),
    }
}

// The line the Ranks screen shows: the caption, the reason (empty when there
// is none) and whether it is a warning.
pub fn line(kept: Kept) -> (&'static [u8], &'static [u8], bool) {
    match kept {
        Kept::Quiet => (b"", b"", false),
        Kept::Disk => (b"Ranks saved to disk", b"", false),
        Kept::Session(why) => (b"Ranks kept until power off: ", why.as_bytes(), true),
        Kept::NotSaved(why) => (b"Ranks not saved: ", why.as_bytes(), true),
        Kept::Unread(why) => (b"Stored ranks not read: ", why.as_bytes(), true),
    }
}
