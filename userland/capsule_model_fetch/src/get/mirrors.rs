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
 * serves it; the next is taken whenever one fails without bringing a byte. */

use alloc::format;
use alloc::string::String;

use nonos_libc::mk_idle_ms;

use super::pour::pour;
use super::progress::Progress;
use super::put::Stop;
use crate::catalogue::File;
use crate::errno::said;
use crate::feed::pause;
use crate::http::open;
use crate::net::Route;
use crate::out::size;

/* Rounds over every mirror without a byte before a file is given up. */
const ROUNDS: usize = 3;

/* Feed `file` from `*at` to its end, starting at mirror `*mirror`. */
pub fn feed_all(
    route: Route,
    file: &File,
    at: &mut u64,
    mirror: &mut usize,
    show: &mut Progress,
) -> Result<(), String> {
    let mut idle = 0;
    while *at < file.bytes {
        let (before, url) = (*at, &file.mirrors[*mirror % file.mirrors.len()]);
        let fault = match open(route, url, *at, file.bytes) {
            Ok(mut body) => match pour(&mut body, at, file.bytes, show) {
                Ok(()) => continue,
                Err(Stop::Volume(e)) => return Err(said(e)),
                Err(Stop::Mirror(f)) => f,
            },
            Err(f) => f,
        };
        show.note(&format!("  {}: {}", host(url), fault.said()));
        (idle, *mirror) = if *at > before { (0, *mirror) } else { (idle + 1, *mirror + 1) };
        if idle >= ROUNDS * file.mirrors.len() {
            return Err(match pause() {
                Ok(n) => format!("no mirror served it; the {} that came are kept", size(n)),
                Err(e) => format!("no mirror served it, and the kernel kept none: {}", said(e)),
            });
        }
        mk_idle_ms(1_000 * idle.min(10) as u64);
    }
    Ok(())
}

/* The host a mirror URL names, for a line about it. */
fn host(url: &str) -> &str {
    let rest = url.strip_prefix("https://").unwrap_or(url);
    rest.split('/').next().unwrap_or(rest)
}
