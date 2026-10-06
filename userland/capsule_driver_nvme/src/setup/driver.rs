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

use crate::admin::{
    reset_to_disabled, AdminQueue, ControllerIdentity, NamespaceIdentity, SmartHealth,
};
use crate::controller::ControllerInfo;
use crate::handles::BrokerHandles;
use crate::nvm::IoQueue;
use crate::regs::Regs;

pub struct Driver {
    pub _admin: AdminQueue,
    pub handles: BrokerHandles,
    pub regs: Regs,
    /// The registers as found at bring-up, for CAP.TO when it is disabled.
    pub info: ControllerInfo,
    pub identity: ControllerIdentity,
    pub namespace: NamespaceIdentity,
    pub health: SmartHealth,
    pub io: Option<IoQueue>,
    /// The host memory buffer the controller was given, if it asked; `drop`
    /// lets it go only once the controller is disabled.
    pub hmb: Option<super::hmb::Hmb>,
}

/// A controller brought up and then not chosen is disabled before its queue
/// memory and claim go (the fields drop after this), so it is never left
/// enabled with queue addresses that point at memory the broker took back.
impl Drop for Driver {
    fn drop(&mut self) {
        let disabled = reset_to_disabled(self.regs, self.info).is_ok();
        if let Some(hmb) = self.hmb.take() {
            hmb.after_disable(disabled);
        }
    }
}
