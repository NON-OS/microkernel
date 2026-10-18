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

use nonos_payment_proto::Names;

use super::deliver::Delivery;
use super::fees::Fees;
use super::held::Held;
pub use super::pending_step::Step;
use super::read::Named;
use super::reading::Reading;
use super::terms::arming::Guard;
use super::terms::asked::Record;
use super::voucher_form::VoucherForm;
use super::watch::Watch;

pub struct Pending {
    pub names: Names,
    /// The market's terms, or why there are none to confirm yet.
    pub read: Result<Named, &'static str>,
    /// The market call to make next; None once the terms are read.
    pub reading: Option<Reading>,
    /// The market's names for what is bought, read for the names asked.
    pub record: Option<Record>,
    /// Whether Buy may act yet: the consent drawn, the wallet focused.
    pub guard: Guard,
    /// Whether the screen shows Buy armed, as of the last look at focus.
    pub armed: bool,
    pub fees: Fees,
    pub watch: Watch,
    pub delivery: Delivery,
    pub form: VoucherForm,
    /// A channel the voucher listing already has (v1.4.48).
    pub held: Held,
    pub step: Step,
    /// The fresh address a refund goes to, drawn when the person confirms.
    pub refund_to: [u8; 20],
    /// A dispute or refund under way from refundTo (v1.4.51 to v1.4.53).
    pub escrow: Option<super::escrowing::Escrowing>,
}
