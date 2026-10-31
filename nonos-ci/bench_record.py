#!/usr/bin/env python3
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
"""Benchmark run records, in the layout benchmarks/README.md describes.

    bench_record.py record <run dir> <root>
        benchmarks/runs/<date>-<sha>/ under <root>: the run's results.jsonl,
        summary.md, metrics.csv and host.json, and run.json naming the commit,
        the event and the workflow run
    bench_record.py rebuild <root>
        benchmarks/history.csv (every metric of every kept run) and
        benchmarks/LATEST.md (the latest run against the one before it)
        from every benchmarks/runs/*/ under <root>
"""

import csv
import datetime
import glob
import json
import os
import shutil
import sys

KEPT = ("results.jsonl", "summary.md", "metrics.csv", "host.json")
FIELDS = ["date", "sha", "event", "run_id", "metric", "status", "elapsed_ms"]


def run_url():
    server, repo, run = (os.environ.get(k, "") for k in ("GITHUB_SERVER_URL", "GITHUB_REPOSITORY", "GITHUB_RUN_ID"))
    return f"{server}/{repo}/actions/runs/{run}" if server and repo and run else ""


def record(run_dir, root):
    missing = [n for n in KEPT if not os.path.isfile(os.path.join(run_dir, n))]
    if not os.path.isdir(run_dir) or len(missing) == len(KEPT):
        print(f"bench_record: no benchmark run in {run_dir}", file=sys.stderr)
        return 1
    sha = os.environ.get("GITHUB_SHA", "unknown")
    date = datetime.datetime.now(datetime.timezone.utc)
    base = os.path.join(root, "benchmarks", "runs", f"{date:%Y-%m-%d}-{sha[:12]}")
    out, n = base, 1
    while os.path.exists(out):
        n += 1
        out = f"{base}-{n}"
    os.makedirs(out)
    for name in KEPT:
        src = os.path.join(run_dir, name)
        if os.path.isfile(src):
            shutil.copyfile(src, os.path.join(out, name))
    with open(os.path.join(out, "run.json"), "w", encoding="utf-8") as f:
        json.dump({
            "schema": "nonos.bench.run.v1",
            "sha": sha, "date": date.isoformat(timespec="seconds"),
            "event": os.environ.get("GITHUB_EVENT_NAME", "local"),
            "ref": os.environ.get("GITHUB_REF_NAME", ""),
            "run_id": os.environ.get("GITHUB_RUN_ID", ""),
            "run_url": run_url(),
            "missing": missing,
        }, f, indent=2)
        f.write("\n")
    print(out)
    return 0


def metrics(run):
    """metric -> (status, elapsed_ms) from a kept run's metrics.csv."""
    path = os.path.join(run, "metrics.csv")
    out = {}
    if os.path.isfile(path):
        with open(path, newline="", encoding="utf-8") as f:
            for row in csv.DictReader(f):
                if row.get("name"):
                    out[row["name"]] = (row.get("status", ""), row.get("elapsed_ms", ""))
    return out


def number(v):
    try:
        return float(v)
    except (TypeError, ValueError):
        return None


def rebuild(root):
    runs = []
    for meta in glob.glob(os.path.join(root, "benchmarks", "runs", "*", "run.json")):
        with open(meta, encoding="utf-8") as f:
            runs.append((json.load(f), os.path.dirname(meta)))
    runs.sort(key=lambda r: r[0]["date"])
    with open(os.path.join(root, "benchmarks", "history.csv"), "w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        w.writeheader()
        for meta, path in runs:
            for name, (status, ms) in sorted(metrics(path).items()):
                w.writerow({"date": meta["date"], "sha": meta["sha"], "event": meta["event"],
                            "run_id": meta["run_id"], "metric": name, "status": status, "elapsed_ms": ms})
    if not runs:
        return 0
    meta, path = runs[-1]
    now = metrics(path)
    prev_meta, prev = (runs[-2][0], metrics(runs[-2][1])) if len(runs) > 1 else (None, {})
    lines = ["# Latest benchmark run", "",
             f"`{meta['sha'][:12]}`, {meta['date']}, {meta['event']}. {meta['run_url']}", "",
             (f"Against `{prev_meta['sha'][:12]}` ({prev_meta['date']}, {prev_meta['event']})."
              if prev_meta else "No earlier run is kept."), "",
             f"Run directory: `benchmarks/runs/{os.path.basename(path)}/`; every run: `benchmarks/history.csv`.", "",
             "| metric | status | ms | previous ms | change |",
             "|---|---|---:|---:|---:|"]
    for name, (status, ms) in sorted(now.items()):
        old = prev.get(name, ("", ""))[1]
        a, b = number(ms), number(old)
        change = f"{(a - b) * 100.0 / b:+.1f}%" if a is not None and b else ""
        lines.append(f"| `{name}` | {status} | {ms} | {old} | {change} |")
    with open(os.path.join(root, "benchmarks", "LATEST.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    return 0


def main():
    if len(sys.argv) == 4 and sys.argv[1] == "record":
        return record(sys.argv[2], sys.argv[3])
    if len(sys.argv) == 3 and sys.argv[1] == "rebuild":
        return rebuild(sys.argv[2])
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
