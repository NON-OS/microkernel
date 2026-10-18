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

mod linkport;
mod mac_response;
mod status;
use super::wire::{
    parse_request, write_response_header, HDR_LEN, MAX_FRAME, OP_LINK_STATUS, OP_MAC_ADDRESS,
    OP_RX_PACKET, OP_TX_PACKET, STATUS_AGAIN, STATUS_NO_LINK,
};
pub use linkport::*;
pub use mac_response::*;
use status::*;
