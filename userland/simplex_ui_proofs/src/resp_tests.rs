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

//! A chat's items, links, errors and frames the window does not show.

use crate::fixture::recorded;
use crate::proto::envelope::incoming;
use crate::proto::resp::Resp;
use crate::proto::status::Status;

#[test]
fn a_chat_reads_its_items_in_order() {
    let Resp::Chat(chat) = recorded("/_get chat @2 count=50") else { panic!() };
    let ids: Vec<_> = chat.items.iter().map(|i| i.id).collect();
    assert_eq!(ids, [10, 11, 12, 13]);
    assert!(chat.items[1].sent);
    assert_eq!(chat.items[1].status, Status::Delivered);
    assert_eq!(chat.items[1].text, "it did, smp4 answers on the onion address too");
}

#[test]
fn links_errors_and_events_read() {
    let Resp::Address(a) = recorded("/_address 1") else { panic!() };
    assert!(a.starts_with("https://smp4.simplex.im/a#"));
    let Resp::Invitation(i) = recorded("/_connect 1") else { panic!() };
    assert!(i.starts_with("https://smp4.simplex.im/i#"));
    let Resp::Error(e) = recorded("/_show_address 1") else { panic!() };
    assert_eq!(e, "errorStore userContactLinkNotFound");
    let old =
        r#"{"resp":{"type":"userContactLink","contactLink":{"connReqContact":"simplex:/c"}}}"#;
    assert_eq!(incoming(old).unwrap().resp, Resp::Address("simplex:/c".into()));
    assert_eq!(incoming(old).unwrap().corr, None);
    let other = r#"{"corrId":"1","resp":{"type":"chatStarted"}}"#;
    assert_eq!(incoming(other).unwrap().resp, Resp::Other("chatStarted".into()));
    assert!(incoming(r#"{"corrId":1,"resp":{"type":"cmdOk"}}"#).is_none());
}
