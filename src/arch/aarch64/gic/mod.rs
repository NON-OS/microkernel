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

mod acked;
mod announce;
mod api;
mod cpu;
mod device;
pub mod distributor;
mod enable;
mod frames;
pub mod icc;
mod in_service;
pub mod irq_handlers;
pub mod redistributor;
pub mod select;
mod serve;
mod sgi;
mod state;
mod sysregs;
mod trigger;
pub mod v2;
pub mod version;
mod wait;

pub use acked::Acked;
pub use api::{init_gic, init_gic_cpu};
pub use distributor::GicDistributor;
pub use enable::{disable_irq, enable_irq};
pub use icc::set_priority_mask;
pub use in_service::{
    complete as complete_interrupt, enter as enter_service, leave as leave_service,
};
pub use irq_handlers::{
    dispatch as dispatch_irq, register as register_irq_handler,
    register_for_capsule as register_irq_handler_for_capsule,
    unregister_for_capsule as unregister_irq_handler_for_capsule,
};
pub use redistributor::GicRedistributor;
pub use serve::acknowledge_interrupt;
pub use sgi::{send_sgi, send_sgi_all_others};
pub use trigger::configure_spi;
pub use version::GicVersion;
