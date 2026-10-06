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

import json
import os
import sys

if len(sys.argv) not in (3, 4):
    raise SystemExit("usage: bench_compare.py <baseline-dir> <candidate-dir> [max-regression-percent]")
baseline_dir, candidate_dir = sys.argv[1], sys.argv[2]
limit = float(sys.argv[3]) if len(sys.argv) == 4 else 15.0

def load(path):
    records = {}
    jsonl = os.path.join(path, "results.jsonl")
    if not os.path.exists(jsonl):
        raise SystemExit(f"missing {jsonl}; run bench_collect.py first")
    with open(jsonl, encoding="utf-8") as src:
        for line in src:
            item = json.loads(line)
            key = item.get("name") or item.get("schema")
            if key:
                records[key] = item
    return records

base = load(baseline_dir)
candidate = load(candidate_dir)
failures = []
report = []
for key, old in sorted(base.items()):
    new = candidate.get(key)
    if not new:
        failures.append(f"{key}: missing candidate record")
        continue
    if old.get("status") == "pass" and new.get("status") != "pass":
        failures.append(f"{key}: status regressed {old.get('status')} -> {new.get('status')}")
    old_ms = old.get("elapsed_ms")
    new_ms = new.get("elapsed_ms")
    if isinstance(old_ms, int) and old_ms > 0 and isinstance(new_ms, int):
        delta = ((new_ms - old_ms) * 100.0) / old_ms
        report.append({"name": key, "baseline_ms": old_ms, "candidate_ms": new_ms, "delta_percent": round(delta, 3)})
        if delta > limit:
            failures.append(f"{key}: {delta:.2f}% slower than baseline")
    for phase, old_phase in old.get("phase_ms", {}).items():
        new_phase = new.get("phase_ms", {}).get(phase)
        if isinstance(old_phase, int) and old_phase > 0 and isinstance(new_phase, int):
            delta = ((new_phase - old_phase) * 100.0) / old_phase
            metric = f"{key}.{phase}"
            report.append({"name": metric, "baseline_ms": old_phase, "candidate_ms": new_phase, "delta_percent": round(delta, 3)})
            if delta > limit:
                failures.append(f"{metric}: {delta:.2f}% slower than baseline")
out = os.path.join(candidate_dir, "compare.json")
with open(out, "w", encoding="utf-8") as dst:
    json.dump({"schema": "nonos.bench.compare.v1", "limit_percent": limit, "failures": failures, "metrics": report}, dst, indent=2)
    dst.write("\n")
print(out)
if failures:
    for item in failures:
        print(f"REGRESSION {item}", file=sys.stderr)
    raise SystemExit(1)
