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

//! DRAIN SIGN, TOPUP and CLOSE (v1.4.7): the calldata against encodings
//! built here by hand, the bound on another settle(), and the gas checks.

mod calldata;
mod caps;
mod channel_tx;
mod fixture;
mod flow;
mod flow_claim;
mod flow_open;
mod plan;
mod ret;
mod retry;
