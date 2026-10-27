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

//! What the transcript answers to a `/_send` it did not record: the core's
//! own shapes, `newChatItems` with the item as sent, then a later
//! `chatItemStatusUpdated` when the contact's client has it.

use alloc::format;
use alloc::string::String;

use crate::proto::json::{parse, push_quoted};

fn chat_info(chat: &str) -> Option<String> {
    let (sigil, id) = chat.split_at(1);
    let id: i64 = id.parse().ok()?;
    Some(match sigil {
        "@" => format!(
            "{{\"type\":\"direct\",\"contact\":{{\"contactId\":{id},\"localDisplayName\":\"\"}}}}"
        ),
        "#" => format!(
            "{{\"type\":\"group\",\"groupInfo\":{{\"groupId\":{id},\"localDisplayName\":\"\"}}}}"
        ),
        _ => return None,
    })
}

fn item(id: i64, minute: i64, text: &str, status: &str) -> String {
    let mut t = String::new();
    push_quoted(&mut t, text);
    let ts = format!("2026-10-27T18:{:02}:00.000Z", minute % 60);
    format!(
        "{{\"chatDir\":{{\"type\":\"directSnd\"}},\"meta\":{{\"itemId\":{id},\"itemTs\":\"{ts}\",\
         \"itemText\":{t},\"itemStatus\":{{\"type\":\"{status}\"}}}},\"content\":{{\"type\":\
         \"sndMsgContent\",\"msgContent\":{{\"type\":\"text\",\"text\":{t}}}}}}}"
    )
}

/// The answer to `cmd` as item `id`, and the status event that follows it;
/// None when `cmd` is not a `/_send` of one text message.
pub fn echo(cmd: &str, id: i64) -> Option<(String, String)> {
    let rest = cmd.strip_prefix("/_send ")?;
    let (chat, body) = rest.split_once(" json ")?;
    let info = chat_info(chat)?;
    let body = parse(body)?;
    let text = body.items().first()?.at(&["msgContent", "text"]).str()?;
    let pair =
        |status| format!("{{\"chatInfo\":{info},\"chatItem\":{}}}", item(id, id, text, status));
    let answer = format!("{{\"type\":\"newChatItems\",\"chatItems\":[{}]}}", pair("sndSent"));
    let later = format!("{{\"type\":\"chatItemStatusUpdated\",\"chatItem\":{}}}", pair("sndRcvd"));
    Some((answer, later))
}
