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

//! The bring-up files, each the shipping one.

#[path = "../../../capsule_driver_rtl8139/src/init/finish.rs"]
mod finish;
#[path = "../../../capsule_driver_rtl8139/src/init/mac.rs"]
mod mac;
#[path = "../../../capsule_driver_rtl8139/src/init/reset.rs"]
mod reset;
#[path = "../../../capsule_driver_rtl8139/src/init/run.rs"]
mod run;
#[path = "../../../capsule_driver_rtl8139/src/init/rx_setup.rs"]
mod rx_setup;
#[path = "../../../capsule_driver_rtl8139/src/init/tx_setup.rs"]
mod tx_setup;

pub use finish::finish;
pub use run::bring_up;
pub use rx_setup::restart as restart_rx;
pub use tx_setup::TCR;
