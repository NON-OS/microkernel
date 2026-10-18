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

//! The display wire, from the shipping files. The capsule's own module
//! re-exports only what its loop uses; named here as siblings, the handlers
//! are reachable to the tests through the paths the loop takes. The loop
//! itself never returns and is left out: `turn` is one pass of it.

#[path = "../../../capsule_driver_bga/src/serve/announce.rs"]
mod announce;
#[path = "../../../capsule_driver_bga/src/serve/answer.rs"]
mod answer;
#[path = "../../../capsule_driver_bga/src/serve/compositor.rs"]
mod compositor;
#[path = "../../../capsule_driver_bga/src/serve/flush.rs"]
mod flush;
#[path = "../../../capsule_driver_bga/src/serve/held.rs"]
mod held;
#[path = "../../../capsule_driver_bga/src/serve/primary.rs"]
mod primary;
#[path = "../../../capsule_driver_bga/src/serve/rect.rs"]
mod rect;
#[path = "../../../capsule_driver_bga/src/serve/scanout.rs"]
mod scanout;
#[path = "../../../capsule_driver_bga/src/serve/transfer.rs"]
mod transfer;
#[path = "../../../capsule_driver_bga/src/serve/turn.rs"]
mod turn;
#[path = "../../../capsule_driver_bga/src/serve/wire/mod.rs"]
mod wire;

pub use announce::{announce, SERVICE_NAME, SERVICE_PORT};
pub use answer::answer;
pub use compositor::COMPOSITOR_SERVICE;
pub use held::PRIMARY_RESOURCE_ID;
pub use turn::{turn, WAIT_MS};
pub use wire::{
    Reply, E_BAD_OP, E_BUSY, E_DEVICE, E_INVAL, OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_SET_SCANOUT,
    OP_TRANSFER_TO_HOST, REPLY_LEN, RX_LEN,
};
