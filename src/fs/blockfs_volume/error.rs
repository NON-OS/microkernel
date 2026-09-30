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

use super::plan_types::PlanError;
use crate::crypto::util::argon2::Argon2Error;
use crate::fs::blockfs::BlockFsError;
use crate::fs::cryptoblock::CryptoBlockError;
use crate::hardware::block_device::BlockDeviceError;
use crate::security::keyring_capsule::KeyringCapsuleError;
use crate::security::tpm::machine_key::KeyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeError {
    Keyring(KeyringCapsuleError),
    BlockFs(BlockFsError),
    NotMounted,
    /// A whole read of a file larger than `read_all` holds; its size.
    TooLargeToReadWhole(u64),
    BadKeyLength,
    /// The disk plan is missing or names ranges it may not.
    Plan(PlanError),
    /// The TPM would not derive the volume key.
    MachineKey(KeyError),
    /// The block device refused a raw read.
    Device(BlockDeviceError),
    /// The window the plan names could not be set.
    Window(CryptoBlockError),
    /// The header ring holds sectors this key cannot open.
    Unopenable,
    /// The disk plan names no file to import.
    NoImport,
    /// The file to import does not hash to the digest the caller pinned.
    DigestMismatch,
    /// The name already holds a file verified against another digest.
    NameTaken,
    /// The key header says a passphrase keys the volume; the TPM is not asked.
    NeedsPassphrase,
    /// The passphrase does not open the volume key. Nothing was written.
    WrongPassphrase,
    /// A passphrase was offered for a volume no passphrase keys.
    NotPassphraseKeyed,
    /// The key header names a way of keying this kernel does not know.
    UnknownKeying,
    /// A volume is open already; it is not keyed again.
    AlreadyOpen,
    /// A volume exists; creating one would format over it.
    VolumeExists,
    /// Argon2id refused its parameters or found no memory.
    Stretch(Argon2Error),
    /* A record, a mark, or a file with a record: an import's alone to write. */
    ImportOnly,
}
