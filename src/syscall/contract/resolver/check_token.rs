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

#[cfg(feature = "nonos-syscall-token-mac-every-call")]
use crate::capabilities::token::verify_token;
use crate::capabilities::token::{is_token_not_revoked, CapabilityToken};

use super::error::ResolverError;

pub(super) fn check_token(token: &CapabilityToken) -> Result<(), ResolverError> {
    // The MAC was checked when the token was installed, and a token that
    // failed it holds an empty allowlist (`allowlist::for_token`).
    #[cfg(feature = "nonos-syscall-token-mac-every-call")]
    if !verify_token(token) {
        return Err(ResolverError::TokenSignatureInvalid);
    }
    if !token.not_expired() {
        return Err(ResolverError::TokenExpired);
    }
    if !is_token_not_revoked(token) {
        return Err(ResolverError::TokenRevoked);
    }
    Ok(())
}
