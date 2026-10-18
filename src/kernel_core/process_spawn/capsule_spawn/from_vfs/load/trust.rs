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

//! The trust anchor and the clock a loaded capsule's certificate is
//! checked against.

use super::super::error::LoadError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust, NonosTrustAnchorPolicy, BAKED_TRUST_ANCHOR_POLICY,
};

pub(super) fn trust_and_now() -> Result<(NonosTrustAnchorPolicy, Option<u64>), LoadError> {
    let trust = decode_trust(BAKED_TRUST_ANCHOR_POLICY).map_err(|_| LoadError::TrustAnchor)?;
    /*
     * Enforce the certificate validity window against the wall clock, not
     * uptime-since-boot: a real `valid_from_ms` is a wall-clock epoch, so
     * comparing it to uptime rejected every runtime-loaded capsule as
     * NotYetValid. Before the clock is set the gate returns None and the
     * signature and trust anchor still gate the load.
     */
    let now_ms = super::super::validity_clock::validity_now_ms(crate::sys::unix_ms());
    Ok((trust, now_ms))
}
