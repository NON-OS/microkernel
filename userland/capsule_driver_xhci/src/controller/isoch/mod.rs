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

mod batch;
mod burst;
mod collect;
mod endpoint;
mod inbound;
mod input;
mod pipe;
mod queue;
mod queue_in;
mod received;
mod tds;
mod wait;

// Reached by the host proofs (userland/xhci_proofs), not by the capsule.
#[allow(unused_imports)]
pub use batch::batch_fits;
#[allow(unused_imports)]
pub use burst::burst_fields;
pub use endpoint::{isoch_endpoint, IsochEndpoint};
#[allow(unused_imports)]
pub use inbound::in_batch_fits;
pub use input::write_isoch_input;
pub use pipe::IsochPipe;
pub use queue::isoch_out;
pub use queue_in::isoch_in;
#[allow(unused_imports)]
pub use received::{lengths, pack, td_bytes};
