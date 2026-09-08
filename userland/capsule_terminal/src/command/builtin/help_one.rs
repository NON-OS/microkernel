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

//! `help <command>`: what one command takes.
//!
//! The grouped list from bare `help` says a command exists. It cannot say what
//! `-r` does to `grep` or whether `head` takes a count, and until now nothing
//! could: the flags each command accepts were declared inside its own body,
//! reachable by reading the source and by no other means. A shell whose only
//! documentation is its source is a shell for the people who wrote it.
//!
//! Each entry is the usage line and one sentence. Where a command declares a
//! flag table this matches it; anything added there and not here shows up as a
//! flag the shell accepts and does not describe, which is the failure mode to
//! watch for.

use crate::command::output::Output;

/// name, usage, what it does.
const USAGE: &[(&[u8], &[u8], &[u8])] = &[
    (
        b"ls",
        b"ls [-l -a -h -1 -R -t -S] [path...]",
        b"list a directory; -l long, -a hidden, -h human sizes, -R recurse, -t by time, -S by size",
    ),
    (b"cat", b"cat [-n] <file...>", b"print files; -n numbers the lines"),
    (b"cd", b"cd [path]", b"change directory; no argument goes home"),
    (b"pwd", b"pwd", b"print the working directory"),
    (b"mkdir", b"mkdir [-p] <dir...>", b"make directories; -p makes parents too"),
    (
        b"rm",
        b"rm [-r -f] <path...>",
        b"remove; -r recurses into directories, -f ignores what is missing",
    ),
    (b"cp", b"cp [-r] <src> <dst>", b"copy; -r recurses into directories"),
    (b"mv", b"mv <src> <dst>", b"move or rename"),
    (b"find", b"find [path] [-name <pat>] [-type f|d]", b"walk a tree, filtering by name or kind"),
    (b"tree", b"tree [path]", b"draw the shape of a directory and everything under it"),
    (
        b"grep",
        b"grep [-c -i -n -r -v] <pattern> [path...]",
        b"search; -i ignores case, -n numbers, -r recurses, -v inverts, -c counts",
    ),
    (b"head", b"head [-n <count>] [file...]", b"first lines, ten by default"),
    (b"tail", b"tail [-n <count>] [file...]", b"last lines, ten by default"),
    (b"wc", b"wc [-l -w -c] [file...]", b"count lines, words, bytes"),
    (b"sort", b"sort [-n -r -u]", b"sort lines; -n numeric, -r reverse, -u unique"),
    (b"uniq", b"uniq [-c]", b"collapse repeated neighbouring lines; -c counts each run"),
    (b"tac", b"tac", b"reverse the order of the lines"),
    (b"rev", b"rev", b"reverse the characters within each line"),
    (b"stat", b"stat <path>", b"size, kind and times for one path"),
    (b"du", b"du [path]", b"how much a tree holds"),
    (b"touch", b"touch <path...>", b"create empty files, or update their time"),
    (b"echo", b"echo [text...]", b"write the arguments back"),
    (b"type", b"type <name...>", b"say which of the four routes a name runs through"),
    (b"which", b"which <name...>", b"the same as type"),
    (b"history", b"history", b"the commands this session has run"),
    (b"jobs", b"jobs", b"what is running in the background"),
    (b"fg", b"fg [id]", b"bring a job to the foreground"),
    (b"bg", b"bg [id]", b"let a stopped job carry on behind"),
    (b"ping", b"ping <host>", b"round trip to a host"),
    (b"capsules", b"capsules", b"every capsule running, with the capabilities it was granted"),
    (b"service", b"service", b"the registered services and the pids answering them"),
    (b"theme", b"theme [name]", b"switch the terminal profile, or list them"),
    (b"clear", b"clear", b"empty the scrollback; Ctrl-L does the same"),
    (b"help", b"help [command]", b"the grouped list, or one command in detail"),
    (b"bench", b"bench", b"cycle costs of the kernel primitives, as percentiles"),
    (b"uptime", b"uptime", b"how long this system has been running, from the monotonic clock"),
    (b"date", b"date", b"the real time clock, as year-month-day hour:minute:second"),
    (b"battery", b"battery", b"charge percentage, or says so when the platform reports none"),
    (b"ps", b"ps", b"the running capsules and the pids serving them"),
    (b"kill", b"kill <pid>", b"signal a process by pid"),
    (b"env", b"env", b"the shell variables that are set"),
    (b"id", b"id", b"which capsule this terminal is, and what signed it"),
    (b"sys", b"sys", b"version and build identity together"),
    (b"ifconfig", b"ifconfig", b"interfaces, addresses and link state"),
    (b"nslookup", b"nslookup <name>", b"resolve a name through the configured resolver"),
    (b"nym", b"nym", b"mixnet client state: directory, gateway and route"),
];

/// Print one command's usage. Returns false when the name is unknown, so the
/// caller can set a failure status and `help x && y` behaves.
pub fn run(out: &mut Output<'_>, name: &[u8]) -> bool {
    let Some((_, usage, what)) = USAGE.iter().find(|(n, _, _)| *n == name) else {
        return unknown(out, name);
    };
    out.writeln(usage);
    let mut line = [b' '; 118];
    let n = 2 + what.len().min(line.len() - 2);
    line[2..n].copy_from_slice(&what[..n - 2]);
    out.writeln(&line[..n]);
    true
}

/// A name with no entry is not necessarily a name with no command: the chain
/// set carries its own, and tools document themselves. Say which door to try
/// rather than only that this one is shut.
fn unknown(out: &mut Output<'_>, name: &[u8]) -> bool {
    if super::tool::is_tool(name) {
        out.writeln(b"an installed tool; run it with --help for its own options");
        return true;
    }
    if crate::event::complete::is_command_name(name) {
        out.writeln(b"a shell command with no usage entry yet; 'help' lists the groups");
        return true;
    }
    let _ = name;
    out.writeln(b"no such command; 'help' lists what there is");
    false
}
