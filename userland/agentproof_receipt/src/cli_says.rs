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

//! What `agentproof-receipt verify` says of a verified receipt, line by
//! line: that it verified, whether it is from a release, and what it holds.

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::cli::hex;
use crate::verify::Verdict;

pub fn lines(v: &Verdict) -> Vec<String> {
    let mut out =
        vec![format!("receipt  verified: NAPR v0; the AK is the one pinned, with no EK chain")];
    out.push(format!("release  {} pcr9 {}", v.release, hex(&v.pcr9)));
    out.push(format!("kind     {}", v.is_release.said()));
    match v.program {
        Some(p) => out.push(format!("program  {} app {}", hex(&p), hex(&v.app_id))),
        None => out.push("program  the personality's own BusyBox, which names no program".into()),
    }
    out.push(format!("box      {} {}", v.box_word, hex(&v.box_hash)));
    if !v.enforced {
        out.push("mode     LEARN, not enforced: this run was held to no policy of yours".into());
    }
    out.push(format!("sealed   by {} as family {:016x}", hex(&v.supervisor), v.seal_no));
    out.push(match v.launcher == [0; 32] {
        true => "asked    by a capsule the receipt does not name".into(),
        false => format!("asked    by the capsule measured {}", hex(&v.launcher)),
    });
    out.extend(crate::cli_ran::lines(&v.ran));
    out.push("counts   not carried in v0".into());
    out
}
