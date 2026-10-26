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

use alloc::vec::Vec;

use super::gate::{admits, Sender};
use super::handlers;
use super::outcomes::Outcomes;
use crate::protocol::{
    encode_response, retired, Request, EINVAL, EPERM, OP_HEALTHCHECK, OP_INSTALL,
    OP_LIST_INSTALLED, OP_LOAD_BY_NAME, OP_LOAD_OUTCOME, OP_PKG_COMMIT, OP_PKG_QUERY,
    OP_PKG_REMOVE,
};

pub fn dispatch(req: Request<'_>, sender: Sender, kept: &mut Outcomes) -> Vec<u8> {
    if let Some(refused) = retired(&req) {
        return refused;
    }
    if !admits(req.op, sender) {
        return encode_response(req.seq, EPERM, &[]);
    }
    match req.op {
        OP_HEALTHCHECK => handlers::health(req),
        OP_INSTALL => handlers::install(req),
        OP_LOAD_BY_NAME => handlers::load_noted(req, sender.pid, kept),
        OP_LOAD_OUTCOME => handlers::load_outcome(req, sender.pid, kept),
        OP_LIST_INSTALLED => handlers::list_installed(req),
        OP_PKG_QUERY => handlers::pkg_query(req),
        OP_PKG_COMMIT => handlers::pkg_commit(req),
        OP_PKG_REMOVE => handlers::pkg_remove(req),
        _ => encode_response(req.seq, EINVAL, &[]),
    }
}
