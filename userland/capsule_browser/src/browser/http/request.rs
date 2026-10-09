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

use alloc::string::String;

use crate::browser::url::Url;

/*
 * The browser says it is Firefox 128 ESR on Windows, word for word the
 * string Tor Browser sends from every platform. A browser reached over a
 * mixnet is only as anonymous as the crowd it blends into, and a string of
 * its own ("nonos-browser/0.1") was a crowd of one that sites also served
 * reduced pages to. navigator.userAgent in the script prelude says the same
 * thing (dom_prelude_1.inc); a proof checks the two agree.
 */
/// The User-Agent every request carries.
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0";

/* GET, or a form POST when a urlencoded body rides along; either asks to close. */
pub fn build(url: &Url, post: Option<&str>, cookie: Option<&str>) -> String {
    request(url, post, false, cookie)
}

/*
 * GET on a connection the fetch machine keeps for the next request to the
 * same host, in the clear or over TLS: that request then skips the connect
 * and, over TLS, the handshake.
 */
pub fn build_keep_alive(url: &Url, cookie: Option<&str>) -> String {
    request(url, None, true, cookie)
}

fn request(url: &Url, post: Option<&str>, keep_alive: bool, cookie: Option<&str>) -> String {
    let mut r = String::new();
    r.push_str(if post.is_some() { "POST " } else { "GET " });
    r.push_str(crate::browser::url::request_target(url));
    r.push_str(" HTTP/1.1\r\nHost: ");
    r.push_str(&crate::browser::url::authority(url));
    r.push_str("\r\nUser-Agent: ");
    r.push_str(USER_AGENT);
    r.push_str("\r\n");
    r.push_str("Accept: text/html,text/plain,application/json,*/*;q=0.1\r\n");
    r.push_str("Accept-Language: en-US,en;q=0.5\r\n");
    r.push_str("Accept-Encoding: gzip, deflate\r\n");
    /* The jar never holds a CR or LF, so a value cannot add a header. */
    if let Some(c) = cookie.filter(|c| !c.is_empty()) {
        r.push_str("Cookie: ");
        r.push_str(c);
        r.push_str("\r\n");
    }
    r.push_str(if keep_alive { "Connection: keep-alive\r\n" } else { "Connection: close\r\n" });
    match post {
        Some(body) => {
            r.push_str("Content-Type: application/x-www-form-urlencoded\r\n");
            r.push_str("Content-Length: ");
            r.push_str(&alloc::format!("{}", body.len()));
            r.push_str("\r\n\r\n");
            r.push_str(body);
        }
        None => r.push_str("\r\n"),
    }
    r
}
