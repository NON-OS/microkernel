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

//! `http <url>` as a terminal job: the request is carried a step a tick
//! (`mixnet::exchange`), the window keeps painting, and Ctrl+C ends it with
//! the connection closed and the prompt back.

use alloc::vec::Vec;

use nonos_http::{parse_response, RequestBuilder};
use nonos_route_link::Route;
use nonos_tls::rtc_now;

use super::emit::emit;
use super::url::parse_url;
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::mixnet::{Exchange, Poll, Stage};
use crate::term::state::State;

const USAGE: &[u8] =
    b"usage: http <url>   e.g. http example.com  |  http https://host/path  |  http host:8080";
const MAX_BODY: usize = 64 * 1024;
const AGENT: &str = "nonos-terminal";

pub struct HttpJob {
    exchange: Exchange,
    /// The stage last told to the person, so each is said once.
    shown: Option<Stage>,
}

/// Parse the url and set the request up; a bad url is reported here and
/// there is no job.
pub fn prepare(state: &mut State, args: &[&[u8]]) -> Option<HttpJob> {
    let Some(&raw) = args.first() else {
        state.scrollback.push_error(USAGE);
        return None;
    };
    let Some(url) = parse_url(raw) else {
        state.scrollback.push_error(b"http: bad url");
        return None;
    };
    // A short .anyone name is only as good as the signed list it came from.
    if nonos_route_link::is_short_anyone(&url.host) {
        state.scrollback.push_line(nonos_route_link::SHORT_NOTICE.as_bytes());
    }
    let route = Route::chosen();
    if !matches!(route, Route::Down(_)) {
        let mut line = Vec::from(&b"http: connecting to "[..]);
        line.extend_from_slice(url.host.as_bytes());
        line.push(b' ');
        line.extend_from_slice(route.name().as_bytes());
        state.scrollback.push_line(&line);
    }
    let request = RequestBuilder::get(&url.host, &url.path).user_agent(AGENT).build();
    let exchange =
        Exchange::new(route, &url.host, url.port, url.secure, rtc_now(), request.bytes, MAX_BODY);
    Some(HttpJob { exchange, shown: Some(Stage::Connecting) })
}

impl HttpJob {
    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        let raw = match self.exchange.step() {
            Poll::Pending => {
                self.tell(out);
                return JobProgress::Running;
            }
            Poll::Ready(Ok(raw)) => raw,
            Poll::Ready(Err(why)) => return fail(out, why),
        };
        match parse_response(&raw) {
            Ok(response) => {
                emit(out, &response);
                JobProgress::Done(0)
            }
            Err(_) => fail(out, "malformed response"),
        }
    }

    /// One line as the request reaches a new stage, so a slow network shows
    /// what it is waiting on rather than nothing.
    fn tell(&mut self, out: &mut Output<'_>) {
        let stage = self.exchange.stage();
        if self.shown == Some(stage) {
            return;
        }
        self.shown = Some(stage);
        match stage {
            Stage::Handshake => out.writeln(b"http: securing the connection"),
            Stage::Receiving => out.writeln(b"http: waiting for the answer"),
            Stage::Connecting | Stage::Sending => {}
        }
    }
}

fn fail(out: &mut Output<'_>, why: &str) -> JobProgress {
    let mut line = Vec::from(&b"http: "[..]);
    line.extend_from_slice(why.as_bytes());
    out.writeln_error(&line);
    JobProgress::Done(1)
}
