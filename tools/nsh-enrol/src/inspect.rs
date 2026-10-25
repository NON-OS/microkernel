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

//! What a checked document says, for an operator to read before enrolling
//! the machine: its PCRs, the applications its firmware started in PCR 4 in
//! the order they ran, its DMA posture and its registry.

use nsh_attest::consts::QUOTED_PCRS_V4;

use crate::hex::of;
use crate::Error;

pub fn inspect(bytes: &[u8], challenge: Option<[u8; 32]>) -> Result<String, Error> {
    let (doc, log) = crate::check::checked(bytes, challenge)?;
    let mut out = String::from("document   version 5: parsed, quote signed, log replayed\n");
    let mut line = |k: &str, v: String| out.push_str(&format!("{k:<10} {v}\n"));
    line("challenge", of(doc.challenge));
    line("ak", of(doc.ak_public));
    for (i, v) in QUOTED_PCRS_V4.iter().zip(doc.pcrs) {
        line(&format!("pcr {i}"), of(v));
    }
    for a in &log.applications {
        line("pcr 4 app", of(a));
    }
    let dma = (doc.iommu_vendor, doc.iommu_enforcing, doc.unconfined_grants, doc.identity_devices);
    line(
        "dma",
        format!("vendor {} enforcing {} unconfined {} identity {}", dma.0, dma.1, dma.2, dma.3),
    );
    line("dma held", (nsh_attest::steps::dma(&doc).is_ok()).to_string());
    line("registry", format!("{} capsules, complete {}", doc.capsule_count, doc.registry_complete));
    line("root", of(doc.registry_root));
    line("log", format!("{} bytes", doc.log.len()));
    Ok(out)
}
