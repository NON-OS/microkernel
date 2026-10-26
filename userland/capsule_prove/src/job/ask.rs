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

//! What a first frame asks for: a PROVE or a DELIVER, decoded; None for any
//! other op or a body that does not decode (start.rs).

use nonos_proof_abi::wire::{DeliverRequest, ProveRequest, DELIVER, PROVE};

pub enum Ask<'a> {
    Prove(ProveRequest<'a>),
    Deliver(DeliverRequest<'a>),
}

pub fn ask<'a>(op: u16, body: &'a [u8]) -> Option<Ask<'a>> {
    match op {
        PROVE => ProveRequest::decode(body).map(Ask::Prove),
        DELIVER => DeliverRequest::decode(body).map(Ask::Deliver),
        _ => None,
    }
}
