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

use super::device::Device;
use super::hmb::Hmb;
use crate::admin::{AdminQueue, ControllerIdentity, NamespaceIdentity, SmartHealth};
use crate::nvm::IoQueue;

/// A controller brought up. Dropping one, as bring-up does with each it does
/// not serve, disables the controller and then releases the device, which
/// clears Bus Master before it drains its queues' memory (Device).
pub struct Driver {
    pub device: Device,
    pub admin: AdminQueue,
    pub identity: ControllerIdentity,
    pub namespace: NamespaceIdentity,
    pub health: SmartHealth,
    /// None when the namespace is not served, or once the controller left
    /// the bus or a reset failed (recover).
    pub io: Option<IoQueue>,
    /// The host memory buffer the controller was given, if it asked.
    pub hmb: Option<Hmb>,
}
