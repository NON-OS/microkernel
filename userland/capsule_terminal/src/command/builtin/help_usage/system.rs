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
//! Usage of the commands that report on processes, capsules and the machine.

pub(crate) const SYSTEM: &[(&[u8], &[u8], &[u8])] = &[
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
        b"audit",
        b"audit [--agent <name>]",
        b"who can reach the network, storage and devices now, and where that authority came from",
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
];
