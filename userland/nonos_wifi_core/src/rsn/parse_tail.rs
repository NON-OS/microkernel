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

//! The RSN element after its pairwise ciphers: the AKM suites, the RSN
//! Capabilities, the PMKID list (skipped) and the group management cipher,
//! each optional while the element has ended.

use super::parse::Rsne;
use super::reader::Reader;
use super::suite::{AKM_PSK, AKM_PSK_SHA256, AKM_SAE};

pub(super) fn tail(r: &mut Reader, out: &mut Rsne) -> Option<()> {
    for _ in 0..r.u16()? {
        match r.suite()? {
            AKM_PSK => out.akm_psk = true,
            AKM_PSK_SHA256 => out.akm_psk_sha256 = true,
            AKM_SAE => out.akm_sae = true,
            _ => {}
        }
    }
    if r.done() {
        return Some(());
    }
    out.caps = r.u16()?;
    if r.done() {
        return Some(());
    }
    let pmkids = r.u16()? as usize;
    r.skip(pmkids.checked_mul(16)?)?;
    if r.remaining() >= 4 {
        out.group_mgmt = Some(r.suite()?);
    }
    Some(())
}
