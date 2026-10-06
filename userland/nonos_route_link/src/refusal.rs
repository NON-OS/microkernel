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
 * What a person is told when an anonymity network will not carry a
 * connection. Each is a sentence they can act on, and none of them is
 * followed by another way out: the request simply did not leave.
 */

/* The proxy a stream goes through. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Proxy {
    /* net.socks5, the SOCKS front of the Nym mixnet. */
    Nym,
    /* net.anon, the Anyone onion network. */
    Anyone,
    /* net.anon, to an .anyone onion service inside it. */
    AnyoneService,
    /* net.anon, to a service named by a short .anyone name. */
    AnyoneName,
}

/* The reply code both proxies give while their network has not come up. */
pub const REP_NOT_YET: u8 = 3;

impl Proxy {
    /*
     * A refused CONNECT, from its reply code. The same code means different
     * things on each: net.socks5 answers 3 with no mixnet session and 4
     * with no exit for the host; net.anon answers 3 with no directory, link
     * or circuit, and 4 when the exit could not resolve the name.
     */
    pub fn refused(self, rep: u8) -> &'static str {
        match (self, rep) {
            (Proxy::Nym, 1) => "the Nym mixnet could not build the request",
            (Proxy::Nym, 2) => "the Nym exit's rules refuse that destination",
            (Proxy::Nym, 3) => "the Nym mixnet is not connected yet",
            (Proxy::Nym, 4) => "the Nym mixnet has no exit for that destination",
            (Proxy::Nym, 5) => "the Nym gateway refused the request",
            (Proxy::Nym, 6) => "the request expired crossing the Nym mixnet",
            (Proxy::Nym, _) => "the Nym mixnet refused the connection",
            (Proxy::Anyone, 1) => "the Anyone network could not open the stream",
            (Proxy::Anyone, 2) => "the Anyone exit's policy refuses that destination",
            (Proxy::Anyone, 3) => "the Anyone network is not connected yet, no circuit is built",
            (Proxy::Anyone, 4) => "the Anyone exit could not resolve the host",
            (Proxy::Anyone, 5) => "the host refused the connection from the Anyone exit",
            (Proxy::Anyone, 6) => "the Anyone exit timed out reaching the host",
            (Proxy::Anyone, _) => "the Anyone network refused the connection",
            (Proxy::AnyoneService, 3) => {
                "the Anyone network is not connected yet, no circuit is built"
            }
            (Proxy::AnyoneService, 4) => {
                "no descriptor answers for that .anyone address, or it is not a valid one"
            }
            (Proxy::AnyoneService, 5) => {
                "none of the .anyone service's introduction points answered"
            }
            (Proxy::AnyoneService, 6) => "the .anyone service did not answer in time",
            (Proxy::AnyoneService, _) => "the .anyone service could not be reached",
            (Proxy::AnyoneName, 3) => "the Anyone network is not connected yet, no circuit is built",
            (Proxy::AnyoneName, 4) => {
                "that short .anyone name is not in the signed list, the list is still being \
                 fetched, or the name changed service this boot and is refused; try again in a \
                 moment, or use the full address"
            }
            (Proxy::AnyoneName, r) => Proxy::AnyoneService.refused(r),
        }
    }

    /* The proxy turned the greeting away: every one of its slots is in use. */
    pub fn full(self) -> &'static str {
        match self {
            Proxy::Nym => "net.socks5 is serving as many programs as it can, try again shortly",
            Proxy::Anyone => "net.anon is serving as many programs as it can, try again shortly",
            Proxy::AnyoneService | Proxy::AnyoneName => {
                "net.anon is serving as many programs as it can, try again shortly"
            }
        }
    }

    /* The proxy stopped answering, or a call to it failed each time. */
    pub fn silent(self) -> &'static str {
        match self {
            Proxy::Nym => "net.socks5 stopped answering",
            Proxy::Anyone => "net.anon stopped answering",
            Proxy::AnyoneService | Proxy::AnyoneName => "net.anon stopped answering",
        }
    }

    /* An answer that is not one: no marker, a wrong one, or too long. */
    pub fn garbled(self) -> &'static str {
        match self {
            Proxy::Nym => "net.socks5 sent an answer this cannot read",
            Proxy::Anyone => "net.anon sent an answer this cannot read",
            Proxy::AnyoneService | Proxy::AnyoneName => "net.anon sent an answer this cannot read",
        }
    }

    /* The proxy no longer holds the conversation: it was restarted. */
    pub fn lost(self) -> &'static str {
        match self {
            Proxy::Nym => {
                "net.socks5 no longer holds this connection, most likely because it was \
                 restarted; try again"
            }
            Proxy::Anyone | Proxy::AnyoneService | Proxy::AnyoneName => {
                "net.anon no longer holds this connection, most likely because it was \
                 restarted; try again"
            }
        }
    }

    /* The proxy ended the conversation before the stream was open. */
    pub fn ended(self) -> &'static str {
        match self {
            Proxy::Nym => "the Nym mixnet ended the connection before it opened",
            Proxy::Anyone => "the Anyone network ended the connection before it opened",
            Proxy::AnyoneService | Proxy::AnyoneName => "the Anyone network ended the connection before it opened",
        }
    }

    /* The stream did not open within its bound. */
    pub fn slow(self) -> &'static str {
        match self {
            Proxy::Nym => "the Nym mixnet did not open the connection in time",
            Proxy::Anyone => "the Anyone network did not open the connection in time",
            Proxy::AnyoneService | Proxy::AnyoneName => "the Anyone network did not open the connection in time",
        }
    }
}

/* A host name SOCKS cannot carry: empty, or longer than 255 bytes. */
pub const BAD_HOST: &str = "the host name is empty or too long to send";

/* Bytes written after the far end finished. */
pub const FINISHED: &str = "the far end closed the connection";

/* More came back than was read, past what is held for a reader. */
pub const OVERRUN: &str = "the far end sent more than was read";

/* A frame asked to carry more than one frame holds; writes are split first. */
pub const TOO_LONG: &str = "a write too long for one frame";
