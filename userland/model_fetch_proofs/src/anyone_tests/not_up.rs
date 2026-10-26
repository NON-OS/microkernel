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

//! Anyone not up in time, as the installer and the store read it.

/* Anyone not up in time is its own reason, with Retry and the direct choice. */
#[test]
fn anyone_not_up_is_its_own_reason() {
    use crate::install::fetch_exit::ANYONE_NOT_UP;
    use crate::install::model_dep::fetched;
    use crate::install::why::Why;
    use crate::store::progress::Progress;
    assert_eq!(fetched(i64::from(ANYONE_NOT_UP)), Err(Why::ANYONE_NOT_UP));
    let p = Progress::Failed(Why::ANYONE_NOT_UP.code() as u8);
    assert!(p.retryable());
    let said = String::from_utf8_lossy(p.sentence().unwrap().0).into_owned();
    assert!(
        said.starts_with("Anyone did not build a circuit within 3 minutes; retry, or press d"),
        "{said}"
    );
    assert!(crate::store::route_offer::ROUTE_REASONS.contains(&33));
}
