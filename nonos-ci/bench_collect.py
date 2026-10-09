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

import csv, hashlib, json, os, sys

if len(sys.argv) != 2: raise SystemExit("usage: bench_collect.py <run-dir>")
run_dir = sys.argv[1]
records = []
for name in sorted(os.listdir(run_dir)):
    if not name.endswith(".json") or name == "manifest.json":
        continue
    path = os.path.join(run_dir, name)
    with open(path, encoding="utf-8") as src:
        item = json.load(src)
    item["_file"] = path
    records.append(item)
jsonl_path = os.path.join(run_dir, "results.jsonl")
with open(jsonl_path, "w", encoding="utf-8") as out:
    for item in records:
        out.write(json.dumps(item, sort_keys=True) + "\n")
csv_path = os.path.join(run_dir, "metrics.csv")
fields = ["name", "schema", "status", "elapsed_ms", "user_cpu_ms", "system_cpu_ms", "max_rss_kb"]
with open(csv_path, "w", newline="", encoding="utf-8") as out:
    writer = csv.DictWriter(out, fieldnames=fields)
    writer.writeheader()
    for item in records:
        row = {key: item.get(key, "") for key in fields}
        if not row["name"]:
            row["name"] = item.get("schema", "")
        writer.writerow(row)
        for key, value in item.get("phase_ms", {}).items():
            if isinstance(value, dict):
                for label, field in (("count", "count"), ("min_ms", "min"), ("avg_ms", "avg"), ("max_ms", "max")):
                    writer.writerow({"name": f"boot.{key}.run_{label}", "status": item.get("status", ""), "elapsed_ms": value.get(field, "")})
                continue
            writer.writerow({"name": f"boot.{key}", "status": item.get("status", ""), "elapsed_ms": value})
        for key, values in item.get("phase_samples_ms", {}).items():
            if not values: continue
            writer.writerow({"name": f"boot.{key}.sample_count", "status": item.get("status", ""), "elapsed_ms": len(values)})
            writer.writerow({"name": f"boot.{key}.sample_min_ms", "status": item.get("status", ""), "elapsed_ms": min(values)})
            writer.writerow({"name": f"boot.{key}.sample_max_ms", "status": item.get("status", ""), "elapsed_ms": max(values)})
            avg = round(sum(values) / len(values), 3)
            writer.writerow({"name": f"boot.{key}.sample_avg_ms", "status": item.get("status", ""), "elapsed_ms": avg})
manifest = []
for root, _, files in os.walk(run_dir):
    for name in sorted(files):
        path = os.path.join(root, name)
        rel = os.path.relpath(path, run_dir)
        with open(path, "rb") as src:
            digest = hashlib.sha256(src.read()).hexdigest()
        manifest.append({"path": rel, "sha256": digest, "bytes": os.path.getsize(path)})
manifest_path = os.path.join(run_dir, "manifest.json")
with open(manifest_path, "w", encoding="utf-8") as out:
    json.dump({"schema": "nonos.bench.manifest.v1", "artifacts": manifest}, out, indent=2)
    out.write("\n")
print(jsonl_path)
print(csv_path)
print(manifest_path)
