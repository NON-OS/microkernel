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
}
