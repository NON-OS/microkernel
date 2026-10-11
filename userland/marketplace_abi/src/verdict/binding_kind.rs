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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum BindingVerdict {
    Valid = 0,
    /// A schema 2 entry: there is nothing to bind.
    NoCommerce = 1,
    /// `bond_id` is not the one this payee and publisher key derive.
    WrongBond = 2,
    /// The token's chain is not one the OS signs for, or the bond contract
    /// is not deployed there, so there is no domain to check against.
    NoDomain = 3,
    /// The signature recovers to nobody, or to someone other than the payee.
    WrongSigner = 4,
}
