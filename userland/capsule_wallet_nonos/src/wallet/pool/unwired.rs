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
// No pool is connected yet. Every call answers None, so the swap screen shows
// that it has no price rather than a figure it cannot stand behind.

use crate::wallet::swap::{Reserves, Token};

use super::quote_provider::QuoteProvider;

pub struct Unwired;

impl QuoteProvider for Unwired {
    fn reserves(&self, _pay: Token, _receive: Token) -> Option<Reserves> {
        None
    }
    fn gas(&self, _pay: Token, _receive: Token) -> Option<u64> {
        None
    }
}
