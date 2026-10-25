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

use nonos_nox_config::{
    DELIVER_MARGIN, DISPUTE_WINDOW, MAX_DELIVERY, ORDER_MIN_LEAD, UNBOND_DELAY_MIN,
};

#[test]
fn escrow_times_are_plan_27s() {
    assert_eq!(MAX_DELIVERY, 604_800, "7 days");
    assert_eq!(DISPUTE_WINDOW, 259_200, "3 days");
    assert_eq!(DELIVER_MARGIN, 3_600, "1 hour");
    assert_eq!(ORDER_MIN_LEAD, 86_400, "24 hours: a fresh note ripens in about 6 (v1.4.17)");
    assert_eq!(UNBOND_DELAY_MIN, 604_800 + 259_200 + 86_400);
    const { assert!(DELIVER_MARGIN < ORDER_MIN_LEAD && ORDER_MIN_LEAD < MAX_DELIVERY) };
}
