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
import glob, json, os, sys

def main() -> int:
    if len(sys.argv) != 3:
        raise SystemExit("usage: bench_boot_matrix.py <run-dir> <out.json>")
    run_dir, out_path = sys.argv[1], sys.argv[2]
    paths = sorted(glob.glob(os.path.join(run_dir, "boot-log*.json")))
    matrix = {"schema": "nonos.bench.boot.matrix.v1", "status": "gap", "runs": [], "markers": {}, "phase_ms": {}}
    phases = {}
    for path in paths:
        with open(path, encoding="utf-8") as src:
            rec = json.load(src)
        matrix["runs"].append({"file": path, "status": rec.get("status"), "latency_status": rec.get("latency_status")})
        for key, value in rec.get("markers", {}).items():
            matrix["markers"][key] = matrix["markers"].get(key, 0) + int(value)
        for key, value in rec.get("phase_ms", {}).items():
            if isinstance(value, int):
                phases.setdefault(key, []).append(value)
    if paths and all(run["status"] == "pass" for run in matrix["runs"]):
        matrix["status"] = "pass"
    elif paths:
        matrix["status"] = "fail"
    for key, values in sorted(phases.items()):
        matrix["phase_ms"][key] = {
            "count": len(values),
            "min": min(values),
            "avg": round(sum(values) / len(values), 3),
            "max": max(values),
        }
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    with open(out_path, "w", encoding="utf-8") as out:
        json.dump(matrix, out, indent=2, sort_keys=True)
        out.write("\n")
    print(out_path)
    return 0 if matrix["status"] == "pass" else 2

if __name__ == "__main__": raise SystemExit(main())
