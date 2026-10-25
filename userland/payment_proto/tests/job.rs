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

use nonos_payment_proto::*;

#[test]
fn a_job_is_named_by_four_bytes_and_op_15_reads_it() {
    assert_eq!((OP_JOB, EBUSY), (15, -16));
    assert_eq!(job_id(0x0102_0304), [4, 3, 2, 1]);
    assert_eq!(parse_job_id(&job_id(77)), Some(77));
    assert_eq!(parse_job_id(&[1, 2, 3]), None);
    assert_eq!(parse_job_id(&[1, 2, 3, 4, 5]), None);
}
