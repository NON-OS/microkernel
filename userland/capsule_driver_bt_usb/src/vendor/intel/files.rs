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

//! The files, linked in from `nonos-bootloader/firmware/intel/`
//! (provenance in `carried`).

pub(super) const HRP_SFI: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-19-0-4.sfi");
pub(super) const HRP_DDC: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-19-0-4.ddc");
pub(super) const TYP_SFI: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-0041-0041.sfi");
pub(super) const TYP_DDC: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-0041-0041.ddc");
pub(super) const SLR_SFI: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-1040-0041.sfi");
pub(super) const SLR_DDC: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-1040-0041.ddc");
pub(super) const BZRU_SFI: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-0291-0291.sfi");
pub(super) const BZRU_DDC: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-0291-0291.ddc");
pub(super) const CCP_13_SFI: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-20-1-3.sfi");
pub(super) const CCP_13_DDC: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-20-1-3.ddc");
pub(super) const CCP_14_SFI: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-20-1-4.sfi");
pub(super) const CCP_14_DDC: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-20-1-4.ddc");
pub(super) const BZRI_IML: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-0190-0291-iml.sfi");
pub(super) const BZRI_SFI: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-0190-0291-usb.sfi");
pub(super) const BZRI_DDC: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/ibt-0190-0291-usb.ddc");
