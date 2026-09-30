// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub const HANDOFF_MAGIC: u32 = 0x4E_4F_4E_4F;
pub const HANDOFF_VERSION: u16 = 1;

pub mod flags {
    pub const WX: u64 = 1 << 0;
    pub const NXE: u64 = 1 << 1;
    pub const SMEP: u64 = 1 << 2;
    pub const SMAP: u64 = 1 << 3;
    pub const UMIP: u64 = 1 << 4;
    pub const IDMAP_PRESERVED: u64 = 1 << 5;
    pub const FB_AVAILABLE: u64 = 1 << 6;
    pub const ACPI_AVAILABLE: u64 = 1 << 7;
    pub const TPM_MEASURED: u64 = 1 << 8;
    pub const SECURE_BOOT: u64 = 1 << 9;
    pub const ZK_ATTESTED: u64 = 1 << 10;
    /*
     * The person chose "Install NONOS" in the boot menu. Set only after the
     * kernel passed the same signature and attestation checks as a Standard
     * boot; the kernel starts its installer before any desktop app.
     */
    pub const INSTALL_REQUESTED: u64 = 1 << 11;
    /*
     * The boot profile chosen in the menu, one bit each; Standard sets none.
     * The kernel acts on them: Air-Gapped, Safe Mode and Recovery start no
     * network driver or service, Safe Mode starts no audio and no optional
     * app, Recovery skips setup. Hardened is the stricter check done here.
     */
    pub const PROFILE_HARDENED: u64 = 1 << 12;
    pub const PROFILE_SAFE: u64 = 1 << 13;
    pub const PROFILE_AIR_GAPPED: u64 = 1 << 14;
    pub const PROFILE_RECOVERY: u64 = 1 << 15;
}
