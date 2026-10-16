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
//! Where the swap screen's numbers come from. Only the quote has a caller,
//! and no pool is wired to it yet: it answers None, and Swap stays hidden
//! from home until one is (`screen::home_actions`).

mod active;
mod quote_provider;
mod unwired;

pub use active::quote;
pub use quote_provider::QuoteProvider;
