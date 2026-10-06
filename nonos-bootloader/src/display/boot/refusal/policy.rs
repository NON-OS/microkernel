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

//! Refusals from the security policy, one per reason security/enforce gives.
//! Shown once that reason reaches the screen (letter of 2026-10-01).

use super::advice::Advice;

const fn a(title: &'static [u8], what: &'static [u8], remedy: &'static [u8]) -> Advice {
    Advice { title, what, remedy }
}

const FIRMWARE: &[u8] = b"Turn it on in the firmware settings, or choose an entry the menu shows as ready.";

pub const POLICY: [(&[u8], Advice); 9] = [
    (b"SecureBoot required", a(b"Secure Boot is off", b"Hardened boots only under UEFI Secure Boot.", FIRMWARE)),
    (b"PlatformKey required", a(b"No Secure Boot platform key", b"Hardened needs the firmware's platform key (PK) enrolled.", FIRMWARE)),
    (b"SignatureDB required", a(b"No Secure Boot db", b"Hardened needs the firmware's signature database (db) present.", FIRMWARE)),
    (b"TPM required", a(b"No TPM 2.0", b"Hardened measures the boot into a TPM 2.0 and found none.", FIRMWARE)),
    (b"HW RNG required", a(b"No hardware random number generator", b"Every boot needs a hardware random source for its keys; none answered.", b"Turn on RDRAND or the TPM in the firmware. A virtual machine needs virtio-rng.")),
    (b"BLAKE3 health check failed", a(b"BLAKE3 self-test failed", b"The loader's own BLAKE3 gave a wrong answer on a known input.", b"This machine or the loader is damaged. Write the boot media again; if it repeats, report it.")),
    (b"Ed25519 health check failed", a(b"Ed25519 self-test failed", b"The loader's own Ed25519 gave a wrong answer on a known input.", b"This machine or the loader is damaged. Write the boot media again; if it repeats, report it.")),
    (b"signing keys not loaded", a(b"No release keys in this loader", b"The loader carries no key to check the kernel's signature with.", b"Use an official N\xD8NOS release, or rebuild the loader with its keys.")),
    (b"zero signing keys", a(b"No release keys in this loader", b"The loader carries no key to check the kernel's signature with.", b"Use an official N\xD8NOS release, or rebuild the loader with its keys.")),
];
