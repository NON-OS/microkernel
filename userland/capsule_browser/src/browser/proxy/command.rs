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

use crate::browser::net::mixnet;
use crate::browser::proxy::parse_socks5;
use crate::browser::proxy::said::{said, Outcome};
use crate::browser::state::State;

pub fn command(state: &mut State, input: &str) -> bool {
    let Some(rest) = input.trim().strip_prefix("proxy ") else {
        return false;
    };
    let line = if rest.trim() == "off" {
        state.proxy = None;
        said(Outcome::Off, mixnet::chosen())
    } else {
        match parse_socks5::parse_socks5(rest.trim()) {
            Some(cfg) => {
                let line = said(Outcome::Set(&cfg.host, cfg.port), mixnet::chosen());
                state.proxy = Some(cfg);
                line
            }
            None => said(Outcome::Bad, mixnet::chosen()),
        }
    };
    state.tell(line);
    true
}
