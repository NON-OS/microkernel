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

//! A purchase the store named, as the wallet holds it, and where it stands.

use alloc::boxed::Box;

use nonos_payment_proto::{Names, OrderMade, OrderNew};

use super::fees::Fees;
use super::read::Named;
use super::reading::Reading;
use super::watch::Watch;

pub struct Pending {
    pub names: Names,
    /// The market's terms, or why there are none to confirm yet.
    pub read: Result<Named, &'static str>,
    /// The market call to make next; None once the terms are read.
    pub reading: Option<Reading>,
    pub fees: Fees,
    pub watch: Watch,
    pub step: Step,
    /// The fresh address a refund goes to, drawn when the person confirms.
    pub refund_to: [u8; 20],
}

pub enum Step {
    Consent,
    Offering(u32),
    Ordering(u32, Box<OrderNew>),
    /// The order is made: its deposit address waits for the NOX.
    Fund(Box<OrderMade>),
    Failed(&'static str),
}
