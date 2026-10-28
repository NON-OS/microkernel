// NONOS Operating System (AGPL-3.0-or-later)

//! What the running image speaks beyond what its family's layout decides,
//! read from it when the bring-up starts (`up`).

use super::super::cmds::{CAPA_CDB_SUPPORT, CAPA_MLD_API_SUPPORT, LONG_GROUP, SCAN_REQ_UMAC};
use super::super::ucode::Ucode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Speaks {
    /// Two LMACs (`IWL_UCODE_TLV_CAPA_CDB_SUPPORT`): a 5 GHz PHY context
    /// names LMAC 1.
    pub cdb: bool,
    /// SCAN_REQ_UMAC's version: 17, or 15 (the 22000 family's image), whose
    /// channels carry their band in `v2.band` rather than in their flags
    /// (`iwl_mvm_umac_scan_cfg_channels_v7`).
    pub scan: u8,
}

impl Speaks {
    /// What a transport speaks before an image is read: the AX211 image's.
    pub const DEFAULT: Self = Self { cdb: false, scan: 17 };

    /// What `u` speaks, or why this driver does not run it: every image it
    /// runs has the MLD API (the 22000 family's at API 77 too, as Linux
    /// v6.12's `iwl_mvm_has_mld_api` reads it) and one of the two scan
    /// request versions it encodes.
    pub fn of(u: &Ucode<'_>) -> Result<Self, &'static str> {
        if !u.capa(CAPA_MLD_API_SUPPORT) {
            return Err("firmware without the MLD API");
        }
        match u.cmd_version(LONG_GROUP, SCAN_REQ_UMAC) {
            Some(scan @ (15 | 17)) => Ok(Self { cdb: u.capa(CAPA_CDB_SUPPORT), scan }),
            _ => Err("scan request version other than 15 or 17"),
        }
    }
}
