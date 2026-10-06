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

//! What the proofs panel reports on, as this boot established it.

use crate::kernel_verify::CryptoVerifyResult;
use crate::security::SecurityContext;

/// The kernel's verification, the platform's switches, and the three regions
/// the loader carries for the kernel's check of the loader. Each region is the
/// bytes as read, `None` when the loader found none.
pub struct Proofs<'a> {
    pub crypto: &'a CryptoVerifyResult,
    pub security: &'a SecurityContext,
    pub tcg_log: Option<&'a [u8]>,
    pub trailer: Option<&'a [u8]>,
    pub record: Option<&'a [u8]>,
}
