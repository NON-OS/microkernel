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


//! Client authorization keys, as callers hand them over.

use crate::onion::address::{is_onion, parse, SUFFIX};
use crate::onion::client_auth::parse_key;
use crate::trace;

use super::super::state::Manager;

/// Keys held at once.
const KEYS_MAX: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyError {
    /// Not a key line in the fork's `.auth_private` format.
    Malformed,
    /// Another caller gave the key held for that service, and only it may
    /// replace or take it back.
    NotYours,
    /// KEYS_MAX keys are held.
    Full,
}

/// Keep the key in `line` for its service, for `owner`. A key for the same
/// service from the same owner replaces it. The cached descriptor for that
/// service is dropped, since it was opened without the key.
pub fn set_client_key(state: &mut Manager, line: &[u8], owner: u32) -> Result<(), KeyError> {
    let key = parse_key(line).ok_or(KeyError::Malformed)?;
    if let Some(at) = state.client_keys.iter().position(|(k, _)| k.identity == key.identity) {
        if state.client_keys[at].1 != owner {
            return Err(KeyError::NotYours);
        }
        state.client_keys.remove(at);
    } else if state.client_keys.len() >= KEYS_MAX {
        return Err(KeyError::Full);
    }
    state.desc_cache.forget_service(&key.identity);
    state.client_keys.push((key, owner));
    trace::say(b"onion client authorization key held");
    Ok(())
}

/// Take back the key `owner` gave for the service `address` names.
pub fn forget_client_key(state: &mut Manager, address: &[u8], owner: u32) -> Result<(), KeyError> {
    let identity = if is_onion(address) {
        parse(address)
    } else {
        let mut host = address.to_vec();
        host.extend_from_slice(SUFFIX);
        parse(&host)
    }
    .ok_or(KeyError::Malformed)?;
    let at = state.client_keys.iter().position(|(k, _)| k.identity == identity).ok_or(KeyError::Malformed)?;
    if state.client_keys[at].1 != owner {
        return Err(KeyError::NotYours);
    }
    state.client_keys.remove(at);
    state.desc_cache.forget_service(&identity);
    trace::say(b"onion client authorization key dropped");
    Ok(())
}
