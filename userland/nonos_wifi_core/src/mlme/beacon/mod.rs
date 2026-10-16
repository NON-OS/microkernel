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

//! Choose the BSS and its security from the target network's beacon (or probe
//! response), and start authentication: Open System for the PSK AKMs, an SAE
//! commit for SAE. The beacon's RSNE and RSNXE are kept whole for the
//! downgrade check in message 3, and its rates for the association request.

mod select;
mod start_sae;
