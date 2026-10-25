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
//! What the attestation panel says, each line from what this boot recorded
//! and nothing else: the firmware's secure boot for the bootloader, the
//! loader's signature check for the kernel, and the kernel's attestation
//! mode for capsules. A development image admits capsules on their Merkle
//! path alone, and says so. Pure; held in boot_splash_proofs.

const OK: u32 = 0xFF00_CC66;
const ACCENT: u32 = 0xFF00_D4AA;
const WARN: u32 = 0xFFFF_AA00;
const DIM: u32 = 0xFF5B_6B78;

/// `zk_verified` values, as libc's attest.rs names them.
const ZK_STARK: u8 = 1;
const ZK_PATH_ONLY: u8 = 2;

/// The parts of MkAttestStatus's record the panel reads.
#[derive(Clone, Copy)]
pub struct Chain {
    pub secure_boot: u8,
    pub kernel_sig_ok: u8,
    pub zk_verified: u8,
}

/// A line's text and colour.
pub type Line = (&'static [u8], u32);

/// The bootloader, kernel and capsule lines and the verdict under them;
/// `None` when the kernel gave no record.
pub fn lines(chain: Option<Chain>) -> [Line; 4] {
    let Some(c) = chain else {
        return [
            (b"[?] bootloader  not reported", DIM),
            (b"[?] kernel      not reported", DIM),
            (b"[?] capsules    not reported", DIM),
            (b"NO RECORD", DIM),
        ];
    };
    let boot: Line = match c.secure_boot {
        1 => (b"[+] bootloader  secure boot verified", OK),
        _ => (b"[-] bootloader  secure boot off", WARN),
    };
    let kernel: Line = match c.kernel_sig_ok {
        1 => (b"[+] kernel      ed25519 verified", OK),
        _ => (b"[!] kernel      signature not verified", WARN),
    };
    let capsules: Line = match c.zk_verified {
        ZK_STARK => (b"[#] capsules    zk attested", ACCENT),
        ZK_PATH_ONLY => (b"[-] capsules    path only (development)", WARN),
        _ => (b"[!] capsules    not attested", WARN),
    };
    // Attested only when the kernel's signature and the STARK both held;
    // secure boot is its own line, as a test machine may run without it.
    let verdict: Line = match (c.kernel_sig_ok, c.zk_verified) {
        (1, ZK_STARK) => (b"ATTESTED", OK),
        _ => (b"UNVERIFIED", WARN),
    };
    [boot, kernel, capsules, verdict]
}
