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

//! An item's delivery status, from `meta.itemStatus`.

use super::json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Sent by us, not yet taken by a relay.
    Sending,
    /// Taken by a relay.
    Sent,
    /// The contact's client said it arrived.
    Delivered,
    /// The relay or the contact refused it.
    Failed,
    /// Received and not yet read here.
    Unread,
    /// Received and read.
    Read,
}

impl Status {
    pub fn from_json(status: &Value) -> Status {
        match status.get("type").str().unwrap_or("") {
            "sndNew" => Status::Sending,
            "sndSent" => Status::Sent,
            "sndRcvd" => match status.get("msgRcptStatus").str() {
                Some("badMsgHash") => Status::Failed,
                _ => Status::Delivered,
            },
            "sndErrorAuth" | "sndError" | "sndWarning" => Status::Failed,
            "rcvNew" => Status::Unread,
            _ => Status::Read,
        }
    }

    /// The words under a sent bubble; a received one shows none.
    pub fn label(self) -> &'static str {
        match self {
            Status::Sending => "sending",
            Status::Sent => "sent",
            Status::Delivered => "delivered",
            Status::Failed => "not sent",
            Status::Unread | Status::Read => "",
        }
    }
}
