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

//! A market program's listed arguments (nonos_linux_policy::args), written
//! beside it before its policy is, and vouched for over a domain of their
//! own, the policy and the arguments, so they start the program only in the
//! box they were sold with. A release with none takes out any left before.

use nonos_linux_policy::args::VOUCH_DOMAIN;

use super::super::record::vouch::sign_beside;
use super::minted::{beside, ARGS, ARGS_TRAILER};
use crate::linux::file::{key, store_found, store_unlink, store_write};

pub(super) fn bind_args(path: &[u8], policy: &[u8], args: &[u8]) -> Result<(), &'static str> {
    if args.is_empty() {
        for suffix in [ARGS_TRAILER, ARGS] {
            let at = key(&beside(path, suffix));
            let _ = store_unlink(&at);
            if !matches!(store_found(&at), Ok(false)) {
                return Err("the listed arguments of an earlier install would not go");
            }
        }
        return Ok(());
    }
    store_write(&key(&beside(path, ARGS)), args)
        .map_err(|_| "its listed arguments could not be written")?;
    match sign_beside(path, &[VOUCH_DOMAIN, policy, args].concat(), ARGS_TRAILER) {
        true => Ok(()),
        false => Err("its listed arguments could not be vouched for"),
    }
}
