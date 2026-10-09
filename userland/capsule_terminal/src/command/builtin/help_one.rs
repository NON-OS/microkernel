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

use super::help_unknown::unknown;

/// name, usage, what it does.
const USAGE: &[(&[u8], &[u8], &[u8])] = &[
    (
        b"ls",
        b"ls [-l -a -h -1 -R -t -S] [path...]",
        b"list a directory; -l long, -a hidden, -h human sizes, -R recurse, -t by time, -S by size",
    ),
    (b"tree", b"tree [path]", b"draw the shape of a directory and everything under it"),
    (b"cat", b"cat [-n] <file...>", b"print files; -n numbers the lines"),
    (b"cd", b"cd [path]", b"change directory; no argument goes to $HOME, /home/nonos"),
    (b"pwd", b"pwd", b"print the working directory"),
    (b"mkdir", b"mkdir [-p] <dir...>", b"make directories; -p makes parents too"),
    (b"touch", b"touch <path...>", b"create empty files; a file that exists is left as it is"),
    (
        b"rm",
        b"rm [-r -f] <path...>",
        b"remove; -r recurses into directories, -f ignores what is missing",
    ),
    (b"rmdir", b"rmdir <dir...>", b"remove empty directories"),
    (b"mv", b"mv <src> <dst>", b"move or rename"),
    (b"cp", b"cp [-r] <src> <dst>", b"copy; -r recurses into directories"),
    (b"stat", b"stat <path>", b"kind, size, write bit and modification time of one path"),
    (b"find", b"find [path] [-name <pat>] [-type f|d]", b"walk a tree, filtering by name or kind"),
    (b"du", b"du [path]", b"how many bytes the files under a path hold"),
    (
        b"head",
        b"head [-n <count>] <file...>",
        b"first lines of each file, ten by default; after a pipe, of the piped lines",
    ),
    (
        b"tail",
        b"tail [-n <count>] <file...>",
        b"last lines of each file, ten by default; after a pipe, of the piped lines",
    ),
    (
        b"grep",
        b"grep [-c -i -n -r -v] <pattern> <path...>",
        b"search; -i ignores case, -n numbers, -r recurses, -v inverts, -c counts; no -r in a pipe",
    ),
    (
        b"wc",
        b"wc [-l -w -c] <file...>",
        b"count lines, words, bytes, with a total for several files; after a pipe, the piped lines",
    ),
    (b"echo", b"echo [text...]", b"write the arguments back"),
    (b"sort", b"sort [-n -r -u] [file...]", b"sort lines; -n numeric, -r reverse, -u unique"),
    (b"uniq", b"uniq [-c] [file...]", b"collapse repeated neighbouring lines; -c counts each run"),
    (
        b"cut",
        b"cut [-d <char>] [-f <n>] [file...]",
        b"the n-th field of each line split on the character; a space and field 1 by default",
    ),
    (b"nl", b"nl [file...]", b"number the lines"),
    (b"tac", b"tac [file...]", b"reverse the order of the lines"),
    (b"rev", b"rev [file...]", b"reverse the characters within each line"),
    (b"type", b"type <name...>", b"say which of the four routes a name runs through"),
    (b"which", b"which <name...>", b"the same as type"),
    (b"history", b"history", b"the commands this session has run"),
    (b"jobs", b"jobs", b"what is running in the background"),
    (b"fg", b"fg <id>", b"bring a job to the foreground"),
    (
        b"bg",
        b"bg <id>",
        b"say that a job runs on; nothing is ever stopped, so every job already runs behind",
    ),
    (b"ping", b"ping <host>", b"round trip to a host"),
    (
        b"capsules",
        b"capsules",
        b"every capsule running, by name, with the capabilities the kernel granted it",
    ),
    (b"service", b"service <name>", b"the port and pid answering a service name"),
    (b"ps", b"ps", b"every process in the kernel's table, with its parent and state"),
    (
        b"kill",
        b"kill <pid|name> [signal]",
        b"signal a process by pid, or by name as kill browser; signal 9 by default",
    ),
    (b"sys", b"sys", b"version and build identity together"),
    (b"battery", b"battery", b"charge percentage, or says so when the platform reports none"),
    (b"about", b"about", b"what this terminal is and how it reaches the rest of the system"),
    (b"version", b"version", b"the release this terminal was built in, and who signed it"),
    (
        b"receipt",
        b"receipt [--hex]",
        b"every capsule as the kernel recorded it: measurement, signer, capabilities",
    ),
    (
        b"log",
        b"log [word ...]",
        b"the kernel's console lines, newest last; with words, only the lines naming one",
    ),
    (b"id", b"id", b"the same as whoami"),
    (b"whoami", b"whoami", b"the user name, then this capsule and who the kernel says signed it"),
    (b"date", b"date", b"the real time clock, as year-month-day hour:minute:second UTC"),
    (b"uptime", b"uptime", b"how long this system has been running, from the monotonic clock"),
    (b"env", b"env", b"the shell variables that are set"),
    (b"set", b"set [name value]", b"list the shell variables, or set one; use it as $name"),
    (b"unset", b"unset <name>", b"remove a shell variable"),
    (b"alias", b"alias [name expansion]", b"list the aliases, or define one"),
    (b"unalias", b"unalias <name>", b"remove an alias"),
    (b"theme", b"theme [name]", b"switch the terminal profile, or list them"),
    (b"clear", b"clear", b"empty the scrollback; Ctrl-L does the same"),
    (b"help", b"help [command]", b"the grouped list, or one command in detail"),
    (b"bench", b"bench", b"cycle costs of the kernel primitives, as percentiles"),
    (b"ifconfig", b"ifconfig", b"interfaces, addresses and link state"),
    (b"nslookup", b"nslookup <name>", b"resolve a name through the configured resolver"),
    (
        b"curl",
        b"curl <url>",
        b"fetch a URL over the chosen network; http, get and fetch are the same command",
    ),
    (b"nym", b"nym", b"mixnet client state: directory, gateway and route"),
    (b"market", b"market", b"the packages the market service offers"),
    (
        b"install",
        b"install <name> [argv...]",
        b"verify, load and start the capsule held at /capsules/<name>.* in the store",
    ),
    (
        b"pkg",
        b"pkg install <path> [--yes] | remove <name> | status",
        b"install a package, remove one, or list what is installed",
    ),
    (
        b"git",
        b"git init | clone <url> | add | status | commit -m <msg> | log | push | remote",
        b"a git client for repositories in the file store, over the chosen network",
    ),
    (b"nox", b"nox [command]", b"the nox command index, or a command by its nox name"),
    (
        b"run",
        b"run <app>",
        b"bring an app's window forward: files editor settings calc about procs term",
    ),
    (b"open", b"open <app>", b"the same as run"),
    (
        b"exec",
        b"exec <name> [argv...]",
        b"load a store capsule as this terminal's child and run it in the foreground",
    ),
    (b"exit", b"exit", b"close this terminal; quit is the same"),
    (b"qwen", b"qwen [tier] [question]", b"chat with a Qwen model on this machine, offline"),
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
    if name == b"qwen" {
        for more in super::qwen::HELP {
            out.writeln(more);
        }
    }
    true
}
