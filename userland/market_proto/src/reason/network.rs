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

//! No network, or one that reached no model mirror.

use super::{said, Reason};

/// No network to fetch a model over.
pub const WHY_NO_NETWORK: u8 = 12;
/// This boot runs no network at all, so the model fetcher, which brings a
/// model in and takes one away, does not start on it.
pub const WHY_BOOT_OFFLINE: u8 = 27;
/// The chosen network runs, and no connection through it reached a model
/// mirror before a byte came: it is not reachable from this machine. Not a
/// download that started and stopped (`WHY_MODEL_FETCH`). One code for
/// each network, so the sentence names it.
pub const WHY_NYM_UNREACHABLE: u8 = 28;
pub const WHY_ANYONE_UNREACHABLE: u8 = 29;
pub const WHY_MIRROR_UNREACHABLE: u8 = 30;

pub(super) fn network_reason(code: u8) -> Option<Reason> {
    Some(match code {
        WHY_NO_NETWORK => {
            said("No network to download the model over: the chosen one is not running", true)
        }
        WHY_BOOT_OFFLINE => said(
            "This boot runs no network (Air-Gapped, Safe Mode or Recovery), and a model is \
             downloaded or taken off the data volume only on a boot that does",
            false,
        ),
        WHY_NYM_UNREACHABLE => said(
            "Nym is not reachable from this machine: no connection through it reached the \
             model mirror. Check the network, or choose another in Settings",
            true,
        ),
        WHY_ANYONE_UNREACHABLE => said(
            "Anyone is not reachable from this machine: no connection through it reached the \
             model mirror. Check the network, or choose another in Settings",
            true,
        ),
        WHY_MIRROR_UNREACHABLE => said(
            "The model mirror is not reachable from this machine over a direct connection. \
             Check the network, or choose another in Settings",
            true,
        ),
        _ => return None,
    })
}
