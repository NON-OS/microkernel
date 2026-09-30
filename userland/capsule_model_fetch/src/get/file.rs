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
 * One file: its stream begun, or taken up where the kernel's mark stands,
 * fed from its mirrors, and finished by the kernel's check of the SHA-256.
 * Bytes that are not the pin are discarded by the kernel, and the file is
 * fetched again from the start from the next mirror, while one is left.
 */

use alloc::format;
use alloc::string::String;

use super::mirrors::feed_all;
use super::progress::Progress;
use crate::catalogue::File;
use crate::errno::said;
use crate::feed::{begin, finish, Start};
use crate::net::Route;

const EBADMSG: i64 = -74;

pub enum Got {
    Already,
    Fetched,
}

pub fn fetch(route: Route, file: &File) -> Result<Got, String> {
    if !file.keepable() {
        return Err(String::from("its name is too long for the data volume to keep"));
    }
    let (name, sha, bytes) = (file.volume_name(), &file.sha256, file.bytes);
    let (mut mirror, mut restarts) = (0, 0);
    loop {
        let mut at = match begin(&name, sha, bytes, false).map_err(said)? {
            Start::Done => return Ok(Got::Already),
            Start::From(at) => at,
        };
        let mut show = Progress::new(&file.name, bytes, at);
        feed_all(route, file, &mut at, &mut mirror, &mut show)?;
        show.end();
        match finish() {
            Ok(_) => return Ok(Got::Fetched),
            Err(EBADMSG) if restarts + 1 < file.mirrors.len() => {
                show.note(&format!(
                    "  {}: {}; again from the next mirror",
                    file.name,
                    said(EBADMSG)
                ));
                (restarts, mirror) = (restarts + 1, mirror + 1);
            }
            Err(e) => return Err(said(e)),
        }
    }
}
