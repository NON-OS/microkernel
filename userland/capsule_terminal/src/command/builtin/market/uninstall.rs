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

//! `market uninstall <id>`: ask the system to take away what installing a
//! listing put on this machine. A package's files go; a Qwen tier's model
//! files go off the data volume, and its program, part of the image, stays.
//! The system's answer comes as the listing's status, which `market info`
//! shows as it goes.

use alloc::format;
use alloc::string::String;

use nonos_libc::{mk_app_install_status, mk_app_uninstall};
use nonos_market_proto::reason::WHY_NOT_INSTALLED;
use nonos_market_proto::Stage;

use super::stage_text::stage_text;
use crate::command::output::Output;

const EBUSY: i64 = -16;
const EINVAL: i64 = -22;

pub(super) fn run(out: &mut Output<'_>, id: &[u8]) {
    let name = String::from_utf8_lossy(id).into_owned();
    if !id.starts_with(b"linux.") {
        let line = format!("market: {name} is part of this system's image; it is not uninstalled");
        return out.writeln(line.as_bytes());
    }
    let stage = Stage::of(mk_app_install_status(id));
    match stage {
        s if s.pending() => {
            return out.writeln(format!("market: {name}: {}", stage_text(id, s)).as_bytes());
        }
        /*
         * Taken away already, or an uninstall found nothing of it. Anything
         * else is asked: the status table is this boot's, so a package
         * installed before the last restart reads as never asked, and a
         * download that stopped may have left part of a model behind. The
         * personality says whether any install of it is recorded.
         */
        s @ (Stage::Removed | Stage::Failed(WHY_NOT_INSTALLED)) => {
            let said = stage_text(id, s);
            return out.writeln(format!("market: {name} is not installed: {said}").as_bytes());
        }
        _ => {}
    }
    let line = match mk_app_uninstall(id) {
        0 => format!("market: {name}: uninstall queued; `market info {name}` shows how it goes"),
        EBUSY => format!("market: {name} is queued already, or the queue is full (EBUSY)"),
        EINVAL => format!("market: the system refused {name} as a listing id (EINVAL)"),
        e => format!("market: the system refused the uninstall (errno {})", -e),
    };
    out.writeln(line.as_bytes());
}
