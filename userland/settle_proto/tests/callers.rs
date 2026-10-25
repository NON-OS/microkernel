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

use nonos_settle_proto::*;

fn registry(name: &[u8]) -> Option<u32> {
    let table: [(&[u8], u32); 6] = [
        (b"payment", 10),
        (b"app.nonos_wallet.1", 11),
        (b"app.store", 12),
        (b"market.index", 13),
        (b"installer", 14),
        (b"app.linux.run", 15),
    ];
    table.iter().find(|(n, _)| *n == name).map(|(_, p)| *p)
}

#[test]
fn each_op_answers_only_its_callers() {
    let may = |op, pid| admits(op, pid, registry);
    let all = [10, 11, 12, 13, 14, 15];
    let allowed = |op| all.iter().filter(|p| may(op, **p)).copied().collect::<Vec<u32>>();
    assert_eq!(allowed(OP_HEALTHCHECK), all.to_vec());
    assert_eq!(allowed(OP_BROADCAST), vec![10, 11]);
    for op in [OP_CHANNEL_STATE, OP_GAS, OP_OPENED, OP_READ, OP_INTENT] {
        assert_eq!(allowed(op), vec![10, 11, 12, 13, 14], "op {op}");
    }
    assert_eq!(allowed(OP_FETCH), vec![10, 12, 13], "each form has its own list");
    assert_eq!(allowed(OP_LISTING), vec![12, 13]);
    assert_eq!(allowed(OP_SETTLE), vec![11, 14]);
    assert_eq!(allowed(3), Vec::<u32>::new(), "a retired op");
    assert!(!may(OP_HEALTHCHECK, 0) && !may(OP_READ, 0), "pid zero is no sender");
}

#[test]
fn each_fetch_form_has_its_own_callers() {
    let names = |body: &[u8]| fetch_callers(body).to_vec();
    assert_eq!(names(&[FETCH_ESCROW, 0]), vec![&b"payment"[..]]);
    assert_eq!(names(&[FETCH_RELEASE]), vec![&b"app.store"[..], b"market.index"]);
    assert_eq!(names(&[FETCH_PAGE]).len(), 3, "a page goes to its key's holder");
}
