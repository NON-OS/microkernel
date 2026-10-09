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

//! `git clone` as a terminal job. The clone is nonos_git's own steps in
//! order (the ref advertisement, the pack, the write into the store), each
//! run against a transport that leaves its request for the job
//! (`git::transport::ask`); the job carries it over the chosen network a
//! tick at a time and runs the step again with the answer. The window keeps
//! painting through a transfer of tens of megabytes, says how far it has
//! got, and Ctrl+C ends it with the connection closed and nothing written.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_git::{clone_into, discover, fetch, CloneRequest, ObjectId, TransportError};
use nonos_http::{parse_response, parse_url};
use nonos_route_link::Route;
use nonos_tls::rtc_now;

use super::super::repo::storage;
use super::fail::say_failure;
use crate::command::output::Output;
use crate::git::{Stepped, VfsStorage};
use crate::jobs::JobProgress;
use crate::mixnet::{Exchange, Poll};
use crate::term::state::State;

/// A clone stops at the tip by default. Whole histories are large, so the
/// depth is stated rather than left to run.
const DEPTH: u32 = 1;
/// What a remote can make the terminal allocate for one request: a depth-1
/// clone of this kernel is a 33 MB pack (`git::transport::round_trip`).
const MAX_RESPONSE: usize = 64 * 1024 * 1024;
/// How often a transfer says how much has come.
const REPORT_EVERY: usize = 2 * 1024 * 1024;
const COMMAND: &str = "git clone";

enum Step {
    Refs,
    Pack(ObjectId),
    /// The pack is here; the next tick writes it, after this one has said so.
    Write(ObjectId, Vec<u8>),
}

pub struct CloneJob {
    transport: Stepped,
    storage: VfsStorage,
    route: Route,
    rtc: u64,
    url: String,
    into: String,
    branch: String,
    step: Option<Step>,
    /// The request being carried, and the exchange carrying it.
    carrying: Option<(Vec<u8>, Exchange)>,
    reported: usize,
}

pub fn prepare(state: &mut State, argv: &[&[u8]]) -> Option<CloneJob> {
    let Some(url) = argv.first().and_then(|a| core::str::from_utf8(a).ok()) else {
        Output::new(&mut state.scrollback).writeln(b"usage: git clone <https url> [branch]");
        return None;
    };
    let Some(remote) = parse_url(url) else {
        Output::new(&mut state.scrollback).writeln(b"git clone: only https urls are supported");
        return None;
    };
    let branch = argv.get(1).and_then(|a| core::str::from_utf8(a).ok()).unwrap_or("main");
    let Some(into) = remote.last_segment().map(String::from) else {
        Output::new(&mut state.scrollback).writeln(b"git clone: that url names no directory");
        return None;
    };
    let route = Route::chosen();
    // The chosen network is not running: say so before anything is written.
    if let Route::Down(why) = route {
        let line = format!("git clone: {why}, so nothing was sent");
        Output::new(&mut state.scrollback).writeln(line.as_bytes());
        return None;
    }
    let line = format!("Cloning into {into} {}", route.name());
    Output::new(&mut state.scrollback).writeln(line.as_bytes());
    Some(CloneJob {
        transport: Stepped::new(remote),
        storage: storage(state),
        route,
        rtc: rtc_now(),
        url: String::from(url),
        into,
        branch: String::from(branch),
        step: Some(Step::Refs),
        carrying: None,
        reported: 0,
    })
}

impl CloneJob {
    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        if self.carrying.is_some() {
            return self.carry(out);
        }
        match self.step.take() {
            Some(Step::Refs) => self.refs(out),
            Some(Step::Pack(head)) => self.pack(out, head),
            Some(Step::Write(head, pack)) => self.write(out, head, &pack),
            None => JobProgress::Done(1),
        }
    }

    fn refs(&mut self, out: &mut Output<'_>) -> JobProgress {
        match discover(&mut self.transport, "git-upload-pack") {
            Ok(refs) => {
                let mut full = String::from("refs/heads/");
                full.push_str(&self.branch);
                let Some(head) = refs.iter().find(|r| r.name == full).map(|r| r.id) else {
                    return self.fail(out, TransportError::Malformed);
                };
                let line = format!("git clone: fetching {} (depth {DEPTH})", self.branch);
                out.writeln(line.as_bytes());
                self.step = Some(Step::Pack(head));
                JobProgress::Running
            }
            Err(e) => self.ask_or_fail(out, Step::Refs, e),
        }
    }

    fn pack(&mut self, out: &mut Output<'_>, head: ObjectId) -> JobProgress {
        match fetch(&mut self.transport, &[head], DEPTH) {
            Ok(pack) => {
                let line =
                    format!("git clone: writing {} KiB into {}", pack.len() / 1024, self.into);
                out.writeln(line.as_bytes());
                self.step = Some(Step::Write(head, pack));
                JobProgress::Running
            }
            Err(e) => self.ask_or_fail(out, Step::Pack(head), e),
        }
    }

    /// One step, the last: the pack is indexed and the work tree checked
    /// out. It is the store's work and the CPU's, with nothing left to wait
    /// on the network for.
    fn write(&mut self, out: &mut Output<'_>, head: ObjectId, pack: &[u8]) -> JobProgress {
        let git_dir = format!("{}/.git", self.into);
        let work_tree = format!("{}/", self.into);
        let request = CloneRequest {
            git_dir: &git_dir,
            work_tree: &work_tree,
            head,
            branch: &self.branch,
            shallow: DEPTH > 0,
            url: Some(&self.url),
        };
        match clone_into(&mut self.storage, &request, pack) {
            Ok(files) => {
                let line =
                    format!("Cloned into {}, {files} files, {}", self.into, self.route.name());
                out.writeln(line.as_bytes());
                JobProgress::Done(0)
            }
            Err(_) => self.fail(out, TransportError::Malformed),
        }
    }

    /// The step asked for something not yet here: carry it, and run the
    /// step again once it has come. Any other error is the clone's answer.
    fn ask_or_fail(&mut self, out: &mut Output<'_>, again: Step, e: TransportError) -> JobProgress {
        let Some(request) = self.transport.ask.take_asked() else {
            return self.fail(out, e);
        };
        let host = self.transport.remote.host.clone();
        let exchange =
            Exchange::new(self.route, &host, 443, true, self.rtc, request.clone(), MAX_RESPONSE);
        self.carrying = Some((request, exchange));
        self.reported = 0;
        self.step = Some(again);
        JobProgress::Running
    }

    fn carry(&mut self, out: &mut Output<'_>) -> JobProgress {
        let Some((_, exchange)) = self.carrying.as_mut() else {
            return JobProgress::Running;
        };
        let raw = match exchange.step() {
            Poll::Pending => {
                self.report(out);
                return JobProgress::Running;
            }
            Poll::Ready(Ok(raw)) => raw,
            Poll::Ready(Err(why)) => {
                self.carrying = None;
                say_failure(out, COMMAND, TransportError::Unreachable, Some(why));
                return JobProgress::Done(1);
            }
        };
        let Some((request, _)) = self.carrying.take() else {
            return JobProgress::Done(1);
        };
        /*
         * A status other than 200 is an error rather than a body: git's own
         * error pages are valid HTTP and would otherwise be read as a pack.
         */
        let response = match parse_response(&raw) {
            Ok(response) => response,
            Err(_) => return self.fail(out, TransportError::Malformed),
        };
        if response.status != 200 {
            return self.fail(out, TransportError::Status(response.status));
        }
        self.transport.ask.answered(request, response.body);
        JobProgress::Running
    }

    /// A line every couple of megabytes while a large answer arrives.
    fn report(&mut self, out: &mut Output<'_>) {
        let Some((_, exchange)) = self.carrying.as_ref() else { return };
        let got = exchange.received();
        if got < self.reported.saturating_add(REPORT_EVERY) {
            return;
        }
        self.reported = got;
        let line = format!("git clone: received {} MiB", got / (1024 * 1024));
        out.writeln(line.as_bytes());
    }

    fn fail(&mut self, out: &mut Output<'_>, e: TransportError) -> JobProgress {
        say_failure(out, COMMAND, e, None);
        JobProgress::Done(1)
    }
}
