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
 * Why a file was not brought in: the words the Terminal shows, and the exit
 * status (`crate::exit`) the run ends in, which the installer reads.
 */

use alloc::string::String;

use crate::errno::said;
use crate::exit::{
    of_errno, ANYONE_NOT_UP, ANYONE_NO_EXIT, ANYONE_UNREACHABLE, MIRROR_UNREACHABLE, NO_NETWORK,
    NYM_NO_EXIT, NYM_UNREACHABLE, REFUSED, UNKEPT,
};
use crate::net::Route;

pub struct Refusal {
    pub status: i32,
    pub said: String,
}

impl Refusal {
    /* The kernel refused the stream with `rc`. */
    pub fn kernel(rc: i64) -> Refusal {
        Refusal { status: of_errno(rc), said: said(rc) }
    }

    /* The route the bytes must come by is not running. */
    pub fn no_network(why: &str) -> Refusal {
        let said = alloc::format!(
            "no network to download it through: {why}. Start that network, or ask for a \
             direct download with `qwen get --direct`: faster, but the mirror sees this \
             machine's address"
        );
        Refusal { status: NO_NETWORK, said }
    }

    /* The file's name is too long for the volume to keep a mark beside. */
    pub fn unkept() -> Refusal {
        let why = "its name is too long for the data volume to keep";
        Refusal { status: UNKEPT, said: String::from(why) }
    }

    /*
     * The route runs and no connection through it reached any mirror
     * before a byte came this run: the network is not reachable from this
     * machine, said by name, with what to do. `kept` is what the kernel
     * holds of the file from before.
     */
    pub fn unreachable(route: Route, kept: &str) -> Refusal {
        let (status, net) = match route {
            Route::Nym(_) => (NYM_UNREACHABLE, "Nym is"),
            Route::Anon(_) => (ANYONE_UNREACHABLE, "Anyone is"),
            _ => (MIRROR_UNREACHABLE, "no mirror is"),
        };
        let said = alloc::format!(
            "{net} not reachable from this machine: no connection reached a mirror. Check the \
             network, or choose another in Settings{kept}"
        );
        Refusal { status, said }
    }

    /*
     * Through an anonymity network, every try in a row found no exit that
     * answered (get/retry.rs). Another try may find one; a direct download
     * is the other choice, and what it costs is said.
     */
    pub fn no_exit(route: Route, kept: &str) -> Refusal {
        let (status, none) = match route {
            Route::Anon(_) => (ANYONE_NO_EXIT, "No Anyone circuit reached the mirror"),
            _ => (NYM_NO_EXIT, "No Nym exit answered"),
        };
        let said = alloc::format!(
            "{none}; try again, or download direct with `qwen get --direct` (the mirror sees this \
             machine's address){kept}"
        );
        Refusal { status, said }
    }

    /* Anyone did not come up in the time a download waits for it. */
    pub fn anyone_not_up() -> Refusal {
        let said = String::from(
            "Anyone did not build a circuit within 3 minutes; try again, or download direct with \
             `qwen get --direct` (the mirror sees this machine's address)",
        );
        Refusal { status: ANYONE_NOT_UP, said }
    }

    /* Any other reason, in words. */
    pub fn other(why: String) -> Refusal {
        Refusal { status: REFUSED, said: why }
    }
}
