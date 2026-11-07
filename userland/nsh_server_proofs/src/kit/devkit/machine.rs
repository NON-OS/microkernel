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

//! The machine an enrolment names: its TPM attestation key, its PCRs 0, 1,
//! 2 and 7, and the boot applications PCR 4 may hold besides the loader.
//! The fixture's machine is the proof spec's; a live one is read from the
//! attestation document nsh-probe kept, with the loader named, since PCR 4
//! may record more applications than the loader.

use crate::kit::attest_build::ak::Key;
use crate::kit::host::spec;
use nsh_attest::log::replay;
use nsh_attest::Doc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    pub ak: [u8; 64],
    pub pcrs: [[u8; 32]; 4],
    pub boot_apps: Vec<[u8; 32]>,
}

impl Machine {
    /// The proof spec's machine, which the fixture enrolment names.
    pub fn fixture() -> Machine {
        let s = spec();
        let l = &s.log.pcrs;
        Machine { ak: Key::new(1).public(), pcrs: [l[0], l[1], l[2], l[7]], boot_apps: s.boot_apps }
    }

    /// The machine a document describes, or why it cannot be read. The
    /// loader must be one of the applications PCR 4 records.
    pub fn from_doc(doc: &[u8], loader: &[u8; 32]) -> Result<Machine, String> {
        let d = Doc::parse(doc).map_err(|e| format!("the document does not parse: {e:?}"))?;
        let apps = replay(d.log)
            .map_err(|e| format!("the event log does not replay: {e:?}"))?
            .applications;
        if !apps.contains(loader) {
            return Err("PCR 4 does not record that loader".into());
        }
        let pcr = |i: u8| d.pcr(i).copied().ok_or_else(|| format!("the document has no PCR {i}"));
        Ok(Machine {
            ak: *d.ak_public,
            pcrs: [pcr(0)?, pcr(1)?, pcr(2)?, pcr(7)?],
            boot_apps: apps.into_iter().filter(|a| a != loader).collect(),
        })
    }
}
