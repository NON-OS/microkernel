# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""What it saves when an `until` step times out: the machine is still up, so
every CPU's registers and the code each one is running say where it stopped.
A boot that hangs once in twenty leaves this behind instead of only a serial
log that ends."""

FILE = "stuck.txt"


def monitor(q, line, cpu=None):
    """One line on the human monitor, through QMP; its text, or its error.
    Each call is a monitor of its own, so a `cpu N` line would be forgotten
    by the next: the CPU it reads is named on the call itself."""
    args = {"command-line": line} if cpu is None else {"command-line": line, "cpu-index": cpu}
    reply = q.cmd("human-monitor-command", **args)
    return reply.get("return", f"(error: {reply.get('error')})")


def dump(q, out):
    """Write the machine's state to out/stuck.txt and return its path, or
    None when the monitor has gone, as it has when QEMU itself exited."""
    path = out / FILE
    try:
        cpus = [c["cpu-index"] for c in q.cmd("query-cpus-fast").get("return", [])]
        parts = ["== info status", monitor(q, "info status"),
                 "== info registers -a", monitor(q, "info registers -a")]
        for cpu in cpus:
            parts += [f"== cpu {cpu}: x/16i $pc", monitor(q, "x/16i $pc", cpu)]
    except (OSError, ConnectionError, KeyError, ValueError) as e:
        print(f"  stuck: the monitor did not answer ({e})", flush=True)
        return None
    path.write_text("\n".join(parts) + "\n")
    print(f"  stuck: {len(cpus)} CPUs' registers and code in {FILE}", flush=True)
    return path
