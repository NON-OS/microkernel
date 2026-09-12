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

//! List the available commands, grouped, so a new user can discover the shell
//! without leaving it.

use crate::command::output::Output;

pub fn run(out: &mut Output<'_>) {
    out.writeln(b"files    ls  tree  cat  cd  pwd  mkdir  touch  rm  rmdir  mv  cp  stat");
    out.writeln(b"         find  du  basename  dirname  pull  push");
    out.writeln(b"text     head  tail  grep  wc  echo  (pipe: sort uniq[-c] cut nl tac rev)");
    out.writeln(b"shell    |  >  >>  <   alias  unalias  set  unset  env  history  clear  Ctrl-L");
    out.writeln(b"         jobs  fg  bg  exec  run/open  exit  type/which");
    out.writeln(b"editing  Ctrl-A start  Ctrl-E end  Ctrl-Left/Right by word  Tab complete");
    out.writeln(b"         Ctrl-W cut word  Ctrl-K cut to end  Ctrl-U cut line  Ctrl-Y put back");
    out.writeln(b"         Ctrl-D delete  Ctrl-R search history  Up/Down recall  Ctrl-C abandon");
    out.writeln(b"history  !! last command   !n the nth   !text the last starting with text");
    out.writeln(b"tabs     Ctrl+Shift+T new   Ctrl+Shift+W close   Ctrl+PgUp/PgDn switch");
    out.writeln(b"view     Ctrl-B side rail   Ctrl+= / Ctrl+- font size");
    out.writeln(b"system   capsules  service  ps  kill  sys  id  whoami  date  uptime  battery");
    out.writeln(b"         version  about  motd  neofetch  display  theme/profile");
    out.writeln(b"net      ping  ifconfig/ip  nslookup/host  curl/http  nym");
    out.writeln(b"apps     apps/market  install  pkg  git");
    out.writeln(b"nox      nox <cmd>   (run 'nox help' for the chain tools)");
    out.writeln(b"         help <command> for what one takes");
    tools(out);
}

/// The installed crates.io programs, listed from the table that runs them.
///
/// These are ordinary published crates, built for this system and admitted by
/// the same spawn gate as everything else. They are worth naming here because
/// nothing else on screen says they exist, and a tool nobody can discover may
/// as well not be installed.
fn tools(out: &mut Output<'_>) {
    const LEAD: &[u8] = b"tools    ";
    let mut line = [b' '; 96];
    line[..LEAD.len()].copy_from_slice(LEAD);
    let mut n = LEAD.len();
    for (typed, _) in super::tool::TOOLS {
        // Two spaces between names, matching the groups above. A name that
        // would not fit is dropped rather than wrapped: the list is a pointer
        // to what exists, not the manual.
        if n + typed.len() + 2 > line.len() {
            break;
        }
        line[n..n + typed.len()].copy_from_slice(typed);
        n += typed.len() + 2;
    }
    out.writeln(&line[..n]);
}
