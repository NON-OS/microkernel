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

#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub mod kyber;

pub mod ml_dsa_65;

// Not shipped: see `unvetted`. With the feature off, `unvetted_off` refuses
// every call to these three by name.
#[cfg(feature = "pqc-unvetted")]
pub mod mceliece;
#[cfg(feature = "pqc-unvetted")]
pub mod ntru;
#[cfg(feature = "pqc-unvetted")]
pub mod sphincs;
#[cfg(feature = "pqc-unvetted")]
mod unvetted;
#[cfg(not(feature = "pqc-unvetted"))]
#[path = "unvetted_off.rs"]
mod unvetted;

pub mod quantum;
