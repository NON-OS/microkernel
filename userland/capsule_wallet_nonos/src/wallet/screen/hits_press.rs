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

//! The things on a screen that can be pressed, by what pressing them does.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Press {
    Send,
    Receive,
    Swap,
    Shield,
    Settings,
    Accounts,
    Back,
    Dismiss,
    Footer(u8),
    /* The Shield screens: a screen to open, an asset, a size, a field. */
    Go(u8),
    Asset(u8),
    Pick(u8),
    Field(u8),
    /* Where a withdrawal goes: 0 a fresh address of this wallet, 1 the meter. */
    Dest(u8),
    /* Ethereum mainnet (0) or Sepolia (1). */
    Network(u8),
    /* A row of a settings list, by its index. */
    Row(u8),
    /* The Stake and Unstake tabs, a lock term, a staked position. */
    Tab(u8),
    Term(u8),
}
