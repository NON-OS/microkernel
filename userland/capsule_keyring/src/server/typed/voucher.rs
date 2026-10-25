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

//! Entry 1: the meter's pay-per-use voucher. Only the meter asks,
//! and only for its own key: the payer is whoever signs.

use super::entry::Entry;
use super::field::{Field, Kind};

const fn f(kind: Kind) -> Field {
    Field { kind, is_signer: false }
}

pub static VOUCHER: Entry = Entry {
    id: 1,
    typed: &nonos_nox_config::VOUCHER,
    fields: &[
        f(Kind::Bytes32),
        Field { kind: Kind::Address, is_signer: true },
        f(Kind::Address),
        f(Kind::Bytes32),
        f(Kind::Bytes32),
        f(Kind::Uint(256)),
        f(Kind::Uint(64)),
        f(Kind::Uint(64)),
        f(Kind::Bytes32),
    ],
    callers: &[b"payment"],
};
