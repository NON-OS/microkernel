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

//! A kernel refusal in plain words, for the call that refused. Each errno is
//! read as its call documents it.

/// EPERM from a DeviceSecret call: the bit is missing, or the capsule holding
/// it was not proven by the vendor root.
const DEVICE_SECRET: &str = "only the vendor-signed nonos.prove may ask the TPM for this";

pub fn file(e: i64) -> &'static str {
    match e {
        -2 => "is not on the data volume: run the fetcher, or import it",
        -1 => "cannot be read: this window lacks FileSystem",
        -13 => "cannot be read: the data volume is locked",
        -11 => "cannot be read yet: the disk is not ready",
        _ => "could not be read from the data volume",
    }
}

pub fn ek(e: i64) -> &'static str {
    match e {
        -1 => DEVICE_SECRET,
        -19 => "this machine has no TPM",
        -13 => "the TPM refused to give its endorsement key",
        _ => "the TPM gave no endorsement key",
    }
}

pub fn slots(e: i64) -> &'static str {
    match e {
        -2 => "the kernel did not admit the loader, or an image carries no trailer",
        -1 => DEVICE_SECRET,
        _ => "the kernel gave no boot slots",
    }
}

pub fn secret(e: i64) -> &'static str {
    match e {
        -2 => "the secret stays sealed: this boot carried no approval",
        -13 => "the TPM refused this chain's policy: it is not an approved chain",
        -19 => "the TPM cannot derive the secret on this machine",
        -1 => DEVICE_SECRET,
        _ => "the kernel gave no device secret",
    }
}

pub fn save(e: i64) -> &'static str {
    match e {
        -1 => "this window lacks StreamImport",
        -28 | -12 => "the data volume is full",
        -16 => "another file is being written to the data volume",
        -13 => "the data volume is locked",
        _ => "the proof could not be saved to the data volume",
    }
}
