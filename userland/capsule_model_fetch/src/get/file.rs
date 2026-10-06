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

use super::mirrors::feed_all;
use super::progress::{Progress, Whole};
use super::refusal::Refusal;
use crate::catalogue::File;
use crate::errno::said;
use crate::feed::{begin, finish, pause, Start};
use crate::net::Route;
use crate::serve;
use crate::status_wire::CHECKING;

const EBADMSG: i64 = -74;

pub enum Got {
    Already,
    Fetched,
}

pub fn fetch(route: Route, file: &File, whole: Whole) -> Result<Got, Refusal> {
    if !file.keepable() {
        return Err(Refusal::unkept());
    }
    let (name, sha, bytes) = (file.volume_name(), &file.sha256, file.bytes);
    let (mut mirror, mut restarts) = (0, 0);
    loop {
        let mut at = match begin(&name, sha, bytes, false).map_err(Refusal::kernel)? {
            Start::Done => return Ok(Got::Already),
            Start::From(at) => at,
        };
        /*
         * Bytes are still to come and the network they must come by is not
         * running: the stream is put down with its mark, and nothing is
         * asked of any other network.
         */
        if let Route::Down(why) = route {
            return Err(match pause() {
                Ok(_) => Refusal::no_network(why),
                /* Said too: what came before this run may not be kept. */
                Err(e) => Refusal::no_network(&format!(
                    "{why}; the download could not be put down either: {}",
                    said(e)
                )),
            });
        }
        let mut show = Progress::new(&file.name, bytes, at, whole);
        feed_all(route, file, &mut at, &mut mirror, &mut show)?;
        show.end();
        serve::stage(CHECKING);
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
            Err(e) => return Err(Refusal::kernel(e)),
        }
    }
}
