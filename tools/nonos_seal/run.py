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

"""Running the steps: every command is printed before it runs, its output
goes to a log under the release directory, and a failure stops the seal with
the end of that log."""

import json
import os
import subprocess
import sys
import time

# A step quiet for this long says so, with the newest line of its log.
HEARTBEAT = 60
# How often a running step's log is read for progress lines.
TICK = 5
# What a command line looks like on the screen; the log keeps all of it.
SHOWN = 220


def shown(cmd):
    line = " ".join(str(c) for c in cmd)
    return line if len(line) <= SHOWN else line[:SHOWN] + f" ... ({len(line)} chars, all in the log)"


def last_line(log):
    try:
        with open(log, "rb") as f:
            f.seek(0, 2)
            f.seek(max(0, f.tell() - 4096))
            lines = [l for l in f.read().decode(errors="replace").splitlines() if l.strip()]
        return lines[-1].strip()[:120] if lines else ""
    except OSError:
        return ""


class Progress:
    """The lines a tool marks `progress:` in its log, shown as they come, so a
    step that counts its own work shows the count rather than a heartbeat."""

    def __init__(self, log):
        self.log = log
        self.at = 0
        self.partial = ""

    def new(self):
        try:
            with open(self.log, "rb") as f:
                f.seek(self.at)
                chunk = f.read().decode(errors="replace")
                self.at = f.tell()
        except OSError:
            return []
        lines = (self.partial + chunk).split("\n")
        self.partial = lines.pop()
        return [l.strip()[len("progress:"):].strip() for l in lines if l.startswith("progress:")]


class Step:
    def __init__(self, logdir):
        self.logdir = logdir
        self.count = 0
        os.makedirs(logdir, exist_ok=True)

    def run(self, *cmd, env=None, capture=False, may_fail=False):
        self.count += 1
        log = os.path.join(self.logdir, f"{self.count:03d}-{os.path.basename(str(cmd[0]))}.log")
        print("    " + shown(cmd), flush=True)
        full = dict(os.environ, **(env or {}))
        with open(log, "w") as sink:
            sink.write(" ".join(str(c) for c in cmd) + "\n\n")
            sink.flush()
            proc = subprocess.Popen([str(c) for c in cmd], env=full, text=True,
                                    stdout=subprocess.PIPE if capture else sink, stderr=sink)
            start = quiet = time.monotonic()
            progress = Progress(log)
            out = ""
            while True:
                try:
                    out, _ = proc.communicate(timeout=TICK)
                    break
                except subprocess.TimeoutExpired:
                    for line in progress.new():
                        print(f"      {line}", flush=True)
                        quiet = time.monotonic()
                    if time.monotonic() - quiet >= HEARTBEAT:
                        minutes = int(time.monotonic() - start) // 60
                        print(f"      still working, {minutes} min: {last_line(log)}", flush=True)
                        quiet = time.monotonic()
            for line in progress.new():
                print(f"      {line}", flush=True)
        if proc.returncode != 0 and may_fail:
            return None
        if proc.returncode != 0:
            with open(log, errors="replace") as f:
                tail = f.read().splitlines()[-20:]
            sys.exit("\n".join([f"  failed (exit {proc.returncode}), log {log}:"] + tail))
        if may_fail:
            return True
        return (out or "").strip() if capture else None


def nix_build(step, attr):
    """Builds a flake output from the tree as it stands, staged changes
    included, and returns its store path."""
    return step.run("nix", "build", f".#{attr}", "--no-link", "--print-out-paths", capture=True)


def nix_json(step, attr):
    return json.loads(step.run("nix", "eval", "--json", f".#{attr}", capture=True))


def say(line=""):
    print(line, flush=True)
