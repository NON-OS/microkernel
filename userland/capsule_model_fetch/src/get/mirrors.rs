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

/* Feeding a file on from where it stands, from the first mirror that
 * serves it; the next is taken whenever one fails without bringing a byte.
 * Each try is a new connection, and `retry` says when to try again and
 * when, and as what, to give up. */

use alloc::format;
use alloc::string::String;

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::pour::pour;
use super::progress::Progress;
use super::put::Stop;
use super::refusal::Refusal;
use super::retry::{End, Kind, Next, Retry};
use crate::catalogue::File;
use crate::errno::said;
use crate::feed::pause;
use crate::http::{open, Fault};
use crate::net::Route;
use crate::out::size;
use crate::serve;

/* Feed `file` from `*at` to its end, starting at mirror `*mirror`. */
pub fn feed_all(
    route: Route,
    file: &File,
    at: &mut u64,
    mirror: &mut usize,
    show: &mut Progress,
) -> Result<(), Refusal> {
    let (start, anonymous) = (*at, matches!(route, Route::Nym(_) | Route::Anon(_)));
    let mut retry = Retry::new(mk_uptime_ms());
    while *at < file.bytes {
        let (before, url) = (*at, &file.mirrors[*mirror % file.mirrors.len()]);
        let fault = match open(route, url, *at, file.bytes) {
            Ok(mut body) => {
                retry.opened = true;
                match pour(&mut body, at, file.bytes, show) {
                    Ok(()) => continue,
                    Err(Stop::Volume(e)) => return Err(Refusal::kernel(e)),
                    Err(Stop::Mirror(f)) => f,
                }
            }
            Err(f) => f,
        };
        let kind = if matches!(fault, Fault::NoSession(_)) { Kind::NoSession } else { Kind::Other };
        let (progressed, now) = (*at > before, mk_uptime_ms());
        let next = retry.after(anonymous, file.mirrors.len(), progressed, *at > start, kind, now);
        /*
         * Said with where it goes on from: the bytes the kernel holds are
         * kept, and the next try, or the next `qwen get`, asks the mirror
         * for the rest only.
         */
        let tail = match next {
            Next::Wait { n, of, .. } => format!("; trying again ({n} of {of}), from {}", size(*at)),
            Next::GiveUp(_) => format!("; {} of {} kept", size(*at), size(file.bytes)),
        };
        show.note(&format!("  {}: {}{tail}", host(url), fault.said()));
        if !progressed {
            *mirror += 1;
        }
        match next {
            Next::Wait { ms, n, of } => wait(ms, n, of),
            Next::GiveUp(end) => return Err(given_up(route, end)),
        }
    }
    Ok(())
}

/* The refusal a file given up ends in, with what the kernel keeps of it. */
fn given_up(route: Route, end: End) -> Refusal {
    let kept = pause();
    let tail = match kept {
        Ok(n) if n > 0 => format!("; the {} that came are kept, and the next try goes on from there", size(n)),
        Ok(_) => String::new(),
        Err(e) => format!("; the kernel kept none of it: {}", said(e)),
    };
    match end {
        End::Unreachable => Refusal::unreachable(route, &tail),
        End::NoExit => Refusal::no_exit(route, &tail),
        End::NoMirror => Refusal::other(format!("no mirror served it{tail}")),
    }
}

/* Wait `ms` before try `n` of `of`, answering the store the while. */
fn wait(ms: i64, n: u32, of: u32) {
    serve::waiting(n, of);
    let until = mk_uptime_ms().saturating_add(ms);
    while mk_uptime_ms() < until {
        serve::answer();
        mk_idle_ms(50);
    }
    serve::going_on();
}

/* The host a mirror URL names, for a line about it. */
fn host(url: &str) -> &str {
    let rest = url.strip_prefix("https://").unwrap_or(url);
    rest.split('/').next().unwrap_or(rest)
}
