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

//! `market install <id>`: ask the system to install a listing.
//!
//! The market's verdict is asked first only to say why nothing would come of
//! asking: the kernel asks the market again before anything runs, and the
//! installer refuses any bytes but the release's pinned package, so this
//! command decides nothing.

use alloc::format;
use alloc::string::String;

use nonos_libc::{mk_app_install, mk_app_install_status};
use nonos_market_proto::reason::WHY_NOT_INSTALLED;
use nonos_market_proto::{pair_body, parse_readiness, reason, Stage, GATES, OP_INSTALL_READY};

use super::call::call;
use super::stage_text::stage_text;
use crate::command::output::Output;

const EBUSY: i64 = -16;
const EINVAL: i64 = -22;

pub(super) fn run(out: &mut Output<'_>, id: &[u8]) {
    let name = String::from_utf8_lossy(id).into_owned();
    if !id.starts_with(b"linux.") {
        out.writeln(
            format!("market: {name} is not a Linux package; NONOS capsules come with the image")
                .as_bytes(),
        );
        return;
    }
    match Stage::of(mk_app_install_status(id)) {
        Stage::Installed => {
            let said = stage_text(id, Stage::Installed);
            return out.writeln(format!("market: {name} is installed already: {said}").as_bytes());
        }
        s if s.pending() => {
            return out.writeln(format!("market: {name}: {}", stage_text(id, s)).as_bytes());
        }
        // The last try stopped for a reason this boot cannot change; an
        // uninstall that found nothing installed is not one.
        s @ Stage::Failed(code) if !reason(code).retry && code != WHY_NOT_INSTALLED => {
            return out.writeln(format!("market: {name}: {}", stage_text(id, s)).as_bytes());
        }
        _ => {}
    }
    let ready = call(OP_INSTALL_READY, &pair_body(id, b"")).map(|b| parse_readiness(&b));
    match ready {
        Ok(Some(r)) if !r.install_ready => {
            out.writeln(format!("market: {name} cannot install on this machine yet:").as_bytes());
            for (label, _) in GATES.iter().zip(r.gates.iter()).filter(|(_, pass)| !**pass) {
                out.writeln(format!("  fails {}", String::from_utf8_lossy(label)).as_bytes());
            }
            return;
        }
        Ok(Some(_)) => {}
        Ok(None) => return super::failure::Failure::Malformed.say(out, ""),
        Err(why) => return why.say(out, super::failure::NO_LISTING),
    }
    let line = match mk_app_install(id, b"") {
        0 => format!(
            "market: {name} queued; `market info {name}` shows how it goes, as does the Marketplace"
        ),
        EBUSY => format!("market: {name} is queued already, or the queue is full (EBUSY)"),
        EINVAL => format!("market: the system refused {name} as a listing id (EINVAL)"),
        e => format!("market: the system refused the install (errno {})", -e),
    };
    out.writeln(line.as_bytes());
}
