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
// The one place the active quote source is chosen. The swap screen calls this,
// never a concrete impl, so wiring a pool means swapping the body here and
// nothing else. Today it returns `Unwired`, which answers None.

use super::quote_provider::QuoteProvider;
use super::unwired::Unwired;

pub fn quote() -> impl QuoteProvider {
    Unwired
}
