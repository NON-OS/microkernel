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

mod deliver;
pub mod fetch;
mod frame;
mod header;
mod prove;
mod status;

pub use deliver::{DeliverRequest, DELIVER, DELIVER_DATA_MAX, DELIVER_HEAD};
pub use frame::{reply, reply_body, request, ReplyError, FRAME_MAX};
pub use header::{Header, HDR_LEN, PROVE_MAGIC, STATUS_LEN, VERSION};
pub use prove::{job_of, ProveRequest, GET, GET_MAX, HEALTH, PROVE, PROVE_BODY, STATUS};
pub use status::{
    status_body, Stage, NOT_ENROLLED, NOT_VENDOR, OUTSIDE, POLICY, PROVER, STALE_REGISTRY,
};
