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

/*
 * A failure worth one more try: one that may well not happen twice. A name
 * the resolver said has no address ("dns failed") is not one of them: the
 * resolver answered, and asking it again at once gets the same answer after
 * the same wait. Nor is a close by the far end (closed::EXIT_CLOSED).
 */
pub fn retryable_error(msg: &str) -> bool {
    use crate::browser::fetch::proxy_fault::{PROXY_ENDED, PROXY_GONE, PROXY_LOST};
    /* A proxy that went away, lost the conversation in a restart, or ended
     * it before answering is asked again from the start: the retry looks the
     * service up afresh, so a proxy that came back is found where it is. */
    matches!(msg, "connect failed" | "send failed" | "timed out")
        || matches!(msg, PROXY_GONE | PROXY_LOST | PROXY_ENDED)
        /* net.socks5 walked away from a silent exit: the next may answer. */
        || msg == crate::browser::fetch::exits::EXIT_SILENT
}

/// Whether a navigation that failed with `msg` is tried again. Never one
/// that failed a security check, and never a form sent with POST once its
/// request may have reached the server: the server may already have acted
/// on it, and a second submission would act twice (RFC 9110 9.2.2). A POST
/// that failed before its request left is tried again as the POST it was.
pub fn retry_nav(msg: &str, post: bool, requested: bool) -> bool {
    retryable_error(msg)
        && !crate::browser::fetch::security_error::security_error(msg)
        && !(post && requested)
}
