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
//! HTTP/1.1 for a client, with no I/O of its own.

//! The request for a download, or for the rest of one.

use alloc::format;
use alloc::string::String;

use crate::url::DlUrl;

/// The same agent the browser and the model fetcher send (Tor Browser's
/// own): a string of its own would be a crowd of one.
const AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0";

/// A GET for `url`, from byte `from` when resuming. Identity encoding, so
/// what is written to the file is the file; the connection closes after.
pub fn request(url: &DlUrl, from: u64) -> String {
    let host = if url.port == 443 { url.host.clone() } else { format!("{}:{}", url.host, url.port) };
    let range = if from > 0 { format!("Range: bytes={from}-\r\n") } else { String::new() };
    format!(
        "GET {} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: {AGENT}\r\nAccept: audio/mpeg,audio/*;q=0.9,*/*;q=0.5\r\nAccept-Encoding: identity\r\n{range}Connection: close\r\n\r\n",
        url.target
    )
}
