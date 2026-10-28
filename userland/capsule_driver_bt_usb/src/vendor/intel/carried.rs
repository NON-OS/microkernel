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

//! The firmware the tree carries. Every file is byte for byte
//! linux-firmware's intel/ copy at d0039fd797d7776d89f00c22c45754914a96706e
//! (WHENCE: "Licence: Redistributable. See LICENCE.ibt_firmware", its text
//! beside them in `nonos-bootloader/firmware/intel/`), unmodified as that
//! licence asks, pinned by sha256 in scripts/baselines/upstream.sha256.
//! Each file answers to every name WHENCE links to it.

use super::files::*;

pub struct Carried {
    pub sfi: &'static [u8],
    pub ddc: &'static [u8],
}

/// A stem prefix, the names after it, and the files they all name.
const NO_DDC: &[u8] = &[];

type Row = (&'static str, &'static [&'static str], &'static [u8], &'static [u8]);

const TABLE: [Row; 8] = [
    // HrP, the AX201 (CNVi on Qu and QuZ): BT_Quasar_REL53843.
    (
        "ibt-19-",
        &["0-4", "0-0", "0-1", "0-3", "16-4", "32-0", "32-1", "32-4", "240-1", "240-4"],
        HRP_SFI,
        HRP_DDC,
    ),
    // CcP, the AX200: BT_CyclonePeak_A0_REL53636. ibt-20-0-3 is byte for
    // byte ibt-20-1-3 (one sha256), so one copy answers to both names.
    ("ibt-20-", &["1-3", "0-3"], CCP_13_SFI, CCP_13_DDC),
    ("ibt-20-", &["1-4"], CCP_14_SFI, CCP_14_DDC),
    // TyP, the AX210: BT_Solar_REL82200.
    ("ibt-", &["0041-0041"], TYP_SFI, TYP_DDC),
    // Solar CNVi with a GF radio, the AX211: BT_Solar_REL82200.
    ("ibt-1040-", &["0041", "4150", "1050"], SLR_SFI, SLR_DDC),
    // BlazarU, the BE200: BT_BlazarU_S_REL125585.
    ("ibt-", &["0291-0291"], BZRU_SFI, BZRU_DDC),
    // BlazarI B0 with an FM radio, the BE201 on Meteor Lake and Arrow Lake
    // CNVi over USB: its loader, which has no DDC, then the firmware the
    // loader names by its id: BT_BlazariFmp_S_REL62.
    ("ibt-0190-0291-", &["iml"], BZRI_IML, NO_DDC),
    ("ibt-0190-0291-", &["usb"], BZRI_SFI, BZRI_DDC),
];

/// The carried firmware for a file stem, `None` when the tree has none.
pub fn carried(stem: &str) -> Option<Carried> {
    let named = |r: &&Row| stem.strip_prefix(r.0).is_some_and(|s| r.1.contains(&s));
    let row = TABLE.iter().find(named)?;
    Some(Carried { sfi: row.2, ddc: row.3 })
}
