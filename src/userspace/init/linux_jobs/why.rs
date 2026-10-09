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

//! What the market answered for an install, said before init acts on it: a
//! refusal that only says "not ready" leaves nothing to fix.

use alloc::format;

use crate::security::market_capsule::client::InstallReadiness;
use crate::security::market_capsule::MarketError;
use crate::sys::serial::println;

pub(super) fn say(ready: &Result<InstallReadiness, MarketError>, pinned: &Result<[u8; 32], MarketError>) {
    let r = match ready {
        Ok(v) => format!(
            "ready={} index={} url={} publisher={} validated={} arch={} attest={}",
            v.install_ready as u8,
            v.index_signature_valid as u8,
            v.package_url_present as u8,
            v.publisher_signature_present as u8,
            v.validation_passed as u8,
            v.arch_match as u8,
            v.attestation_present as u8
        ),
        Err(e) => format!("ready: {e:?}"),
    };
    let p = match pinned {
        Ok(_) => "release: pinned",
        Err(_) => "release: none",
    };
    println(format!("[LINUX-INSTALL] market says {r}, {p}").as_bytes());
}
