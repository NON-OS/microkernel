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
use alloc::vec::Vec;

use super::{Phase, TlsCtx};
use crate::browser::url::Url;

pub struct Fetch {
    pub url: Url,
    pub handle: u32,
    pub phase: Phase,
    pub buf: Vec<u8>,
    pub socks: Vec<u8>,
    pub tls: Option<TlsCtx>,
    /* Where the connection is being made to, while it is being made. */
    pub dial: Option<super::Dial>,
    pub started_ms: i64,
    /* When bytes last arrived or the phase last moved; silence runs from here. */
    pub progress_ms: i64,
    /* Bytes received for this request, handshake included. */
    pub received: usize,
    pub error: Option<&'static str>,
    /* The alert byte a server refused the handshake with. */
    pub tls_alert: Option<u8>,
    /* A navigation that must not be pushed onto history. */
    pub suppress: bool,
    /* The key an <img> source is stored under, and redirects taken for it. */
    pub image: Option<String>,
    pub hops: u8,
    /* Form body for a POST navigation; None sends a GET. */
    pub post: Option<String>,
    pub js_req: bool,
    pub css: bool,
    /* Family key of an @font-face download; zero otherwise. */
    pub font: u32,
    pub script: bool,
    /* A script's place in document order. */
    pub order: u32,
    /* Plaintext of earlier responses on this connection, before this one. */
    pub rx_consumed: usize,
    /* Client record sequence this request was sealed at. */
    pub tx_seq: u64,
    /* Requests already served on this connection. */
    pub keep_uses: u8,
    /* The request asked to keep the connection for another. */
    pub keep: bool,
}
