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
"""Fuzz run records, in the layout fuzz/README.md describes.

    fuzz_record.py target --log L --crate C --target T --seconds S
                          --corpus-before N --corpus-after M --artifacts DIR --out FILE
        one target's summary from its libFuzzer log
    fuzz_record.py run <summaries dir> <root>
        fuzz/results/<date>-<sha>/summary.json under <root>, from every
        target summary in the directory, and the table to the job summary
    fuzz_record.py rebuild <root>
        fuzz/history.csv and fuzz/LATEST.md from every fuzz/results/*/ under <root>
"""

import argparse
import csv
import datetime
import glob
import json
import os
import re
import sys

STAT = re.compile(r"^stat::(\w+):\s+(\d+)", re.M)
COV = re.compile(r"cov: (\d+) ft: (\d+) corp: (\d+)/")
# The inputs libFuzzer writes when a run fails; slow-unit-* fails nothing.
FAILURES = ("crash", "leak", "timeout", "oom")
FIELDS = ["date", "sha", "event", "run_id", "crate", "target", "status", "seconds", "execs",
          "execs_per_sec", "cov", "features", "corpus", "new_entries", "crashes"]


def target_summary(a):
    text = open(a.log, encoding="utf-8", errors="replace").read() if os.path.exists(a.log) else ""
    stats = {k: int(v) for k, v in STAT.findall(text)}
    cov = COV.findall(text)
    crashes = sorted(f"{a.crate}/fuzz/artifacts/{a.target}/{os.path.basename(p)}" for kind in FAILURES
                     for p in glob.glob(os.path.join(a.artifacts, f"{kind}-*")) if os.path.isfile(p))
    if crashes:
        status = "crash"
    elif "stat::number_of_executed_units" in text:
        status = "pass"
    else:
        status = "error"
    return {
        "schema": "nonos.fuzz.target.v1",
        "crate": a.crate, "target": a.target, "status": status, "seconds": a.seconds,
        "execs": stats.get("number_of_executed_units"),
        "execs_per_sec": stats.get("average_exec_per_sec"),
        "new_units_added": stats.get("new_units_added"),
        "peak_rss_mb": stats.get("peak_rss_mb"),
        "cov": int(cov[-1][0]) if cov else None,
        "features": int(cov[-1][1]) if cov else None,
        "corpus_before": a.corpus_before, "corpus_after": a.corpus_after,
        "new_entries": max(0, a.corpus_after - a.corpus_before),
        "crashes": crashes,
    }


def stamp():
    sha = os.environ.get("GITHUB_SHA", "unknown")
    date = datetime.datetime.now(datetime.timezone.utc)
    return sha, date


def free_dir(base):
    """base, or base-2, base-3 ... when a run of the same commit and day is kept already."""
    path, n = base, 1
    while os.path.exists(path):
        n += 1
        path = f"{base}-{n}"
    return path


def run_record(src, root):
    sha, date = stamp()
    targets = []
    for p in sorted(glob.glob(os.path.join(src, "**", "*.json"), recursive=True)):
        with open(p, encoding="utf-8") as f:
            item = json.load(f)
        if item.get("schema") == "nonos.fuzz.target.v1":
            targets.append(item)
    out = free_dir(os.path.join(root, "fuzz", "results", f"{date:%Y-%m-%d}-{sha[:12]}"))
    os.makedirs(out)
    record = {
        "schema": "nonos.fuzz.run.v1",
        "sha": sha, "date": date.isoformat(timespec="seconds"),
        "event": os.environ.get("GITHUB_EVENT_NAME", "local"),
        "ref": os.environ.get("GITHUB_REF_NAME", ""),
        "run_id": os.environ.get("GITHUB_RUN_ID", ""),
        "run_url": run_url(),
        "targets": sorted(targets, key=lambda t: (t["crate"], t["target"])),
    }
    with open(os.path.join(out, "summary.json"), "w", encoding="utf-8") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    table = markdown_table(record)
    with open(os.path.join(out, "summary.md"), "w", encoding="utf-8") as f:
        f.write(table)
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a", encoding="utf-8") as f:
            f.write(table)
    print(out)
    return 0


def run_url():
    server, repo, run = (os.environ.get(k, "") for k in ("GITHUB_SERVER_URL", "GITHUB_REPOSITORY", "GITHUB_RUN_ID"))
    return f"{server}/{repo}/actions/runs/{run}" if server and repo and run else ""


def cell(v):
    return "" if v is None else str(v)


def markdown_table(record):
    rows = record["targets"]
    bad = [t for t in rows if t["status"] != "pass"]
    lines = [f"## Fuzz run `{record['sha'][:12]}` ({record['event']})", "",
             f"{len(rows)} targets, {len(rows) - len(bad)} passed, "
             f"{sum(len(t['crashes']) for t in rows)} crash inputs. {record['run_url']}", "",
             "| crate | target | status | seconds | execs | exec/s | cov | features | corpus | new | crashes |",
             "|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for t in rows:
        lines.append(f"| `{t['crate']}` | `{t['target']}` | {t['status']} | {t['seconds']} | {cell(t['execs'])} | "
                     f"{cell(t['execs_per_sec'])} | {cell(t['cov'])} | {cell(t['features'])} | "
                     f"{t['corpus_after']} | {t['new_entries']} | {len(t['crashes'])} |")
    for t in bad:
        for c in t["crashes"]:
            lines.append(f"\n- reproduce `{t['target']}`: `cd {t['crate']}/fuzz && cargo fuzz run {t['target']} {os.path.relpath(c, t['crate'] + '/fuzz')}`")
    return "\n".join(lines) + "\n"


def rebuild(root):
    runs = []
    for p in sorted(glob.glob(os.path.join(root, "fuzz", "results", "*", "summary.json"))):
        with open(p, encoding="utf-8") as f:
            runs.append(json.load(f))
    runs.sort(key=lambda r: r["date"])
    with open(os.path.join(root, "fuzz", "history.csv"), "w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        w.writeheader()
        for r in runs:
            for t in r["targets"]:
                w.writerow({"date": r["date"], "sha": r["sha"], "event": r["event"], "run_id": r["run_id"],
                            "crate": t["crate"], "target": t["target"], "status": t["status"],
                            "seconds": t["seconds"], "execs": cell(t["execs"]),
                            "execs_per_sec": cell(t["execs_per_sec"]), "cov": cell(t["cov"]),
                            "features": cell(t["features"]), "corpus": t["corpus_after"],
                            "new_entries": t["new_entries"], "crashes": len(t["crashes"])})
    if not runs:
        return 0
    latest = runs[-1]
    prev = {}
    for r in runs[:-1]:
        for t in r["targets"]:
            prev[(t["crate"], t["target"], t["seconds"])] = t
    lines = ["# Latest fuzz run", "",
             f"`{latest['sha'][:12]}`, {latest['date']}, {latest['event']}. {latest['run_url']}", "",
             "Coverage and corpus against the previous run of each target for the same length of time.", "",
             "| crate | target | status | execs | cov | Δ cov | features | Δ features | corpus | Δ corpus | crashes |",
             "|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|"]

    def delta(new, old):
        if new is None or old is None:
            return ""
        return f"{new - old:+d}"

    for t in latest["targets"]:
        p = prev.get((t["crate"], t["target"], t["seconds"]), {})
        lines.append(f"| `{t['crate']}` | `{t['target']}` | {t['status']} | {cell(t['execs'])} | {cell(t['cov'])} | "
                     f"{delta(t['cov'], p.get('cov'))} | {cell(t['features'])} | {delta(t['features'], p.get('features'))} | "
                     f"{t['corpus_after']} | {delta(t['corpus_after'], p.get('corpus_after'))} | {len(t['crashes'])} |")
    with open(os.path.join(root, "fuzz", "LATEST.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    return 0


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    t = sub.add_parser("target")
    for name in ("--log", "--crate", "--target", "--artifacts", "--out"):
        t.add_argument(name, required=True)
    for name in ("--seconds", "--corpus-before", "--corpus-after"):
        t.add_argument(name, required=True, type=int)
    r = sub.add_parser("run")
    r.add_argument("src")
    r.add_argument("root")
    b = sub.add_parser("rebuild")
    b.add_argument("root")
    a = ap.parse_args()
    if a.cmd == "target":
        summary = target_summary(a)
        os.makedirs(os.path.dirname(os.path.abspath(a.out)), exist_ok=True)
        with open(a.out, "w", encoding="utf-8") as f:
            json.dump(summary, f, indent=2)
            f.write("\n")
        print(json.dumps(summary))
        return 0
    if a.cmd == "run":
        return run_record(a.src, a.root)
    return rebuild(a.root)


if __name__ == "__main__":
    sys.exit(main())
