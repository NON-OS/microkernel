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
import json, os, shlex, subprocess, sys, time
def main() -> int:
    out = os.environ.get("NONOS_BENCH_OUT") or f"target/bench/{time.strftime('%Y%m%d-%H%M%S', time.gmtime())}"; os.makedirs(out, exist_ok=True)
    py = sys.executable; failures = []
    boot_runs = max(1, int(os.environ.get("NONOS_BENCH_BOOT_RUNS", "1")))
    subprocess.check_call([py, "nonos-ci/bench_host.py", out])
    if os.environ.get("NONOS_BENCH_SKIP_BUILD") != "1":
        raw = os.environ.get("NONOS_BENCH_BUILD_CMD", "make nonos-mk-verify-fast")
        rc = subprocess.call([py, "nonos-ci/bench_time.py", "build-verify-fast", out, "--", *shlex.split(raw)])
        if rc != 0: failures.append("build-verify-fast")
    if os.environ.get("NONOS_BENCH_SKIP_BOOT") != "1":
        for idx in range(1, boot_runs + 1):
            suffix = "" if idx == 1 else f"-{idx:02d}"
            boot_dir = os.path.join(out, f"boot{suffix}")
            boot_log = os.path.join(boot_dir, "qemu-serial.log")
            boot_cmd = ["env", f"NONOS_BOOT_EVIDENCE_OUT={boot_dir}",
                        f"NONOS_BOOT_EVIDENCE_LOG={boot_log}",
                        f"NONOS_BOOT_EVIDENCE_TIMEOUT={os.environ.get('NONOS_BENCH_BOOT_TIMEOUT', '360')}",
                        f"NONOS_BOOT_EVIDENCE_CMD={os.environ.get('NONOS_BENCH_BOOT_CMD', '')}",
                        "make", "nonos-mk-boot-evidence"]
            rc = subprocess.call([py, "nonos-ci/bench_time.py", f"boot-evidence{suffix}", out, "--", *boot_cmd])
            if rc != 0: failures.append(f"boot-evidence{suffix}")
            parsed = [py, "nonos-ci/bench_boot_log.py", boot_log, os.path.join(out, f"boot-log{suffix}.json")]
            rc = subprocess.call(parsed)
            if rc != 0: failures.append(f"boot-log{suffix}")
        rc = subprocess.call([py, "nonos-ci/bench_boot_matrix.py", out, os.path.join(out, "boot-matrix.json")])
        if rc != 0: failures.append("boot-matrix")
    names = [n for n in sorted(os.listdir(out)) if n.endswith(".json") and n != "manifest.json"]
    records = []
    for name in names:
        with open(os.path.join(out, name), encoding="utf-8") as src: records.append(json.load(src))
    summary = os.path.join(out, "summary.md")
    with open(summary, "w", encoding="utf-8") as dst:
        dst.write("# NØNOS Benchmark Run\n\n")
        dst.write(f"Output: `{out}`\n\n")
        for rec in records:
            name = rec.get("name", rec.get("schema", "record")); status = rec.get("status", "recorded")
            dst.write(f"- `{name}` status `{status}` elapsed `{rec.get('elapsed_ms', '')}` ms\n")
        for rec in records:
            phases = rec.get("phase_ms", {})
            if not phases: continue
            if rec.get("schema") == "nonos.bench.boot.matrix.v1":
                dst.write("\nBoot matrix aggregates:\n")
                for key, value in sorted(phases.items()):
                    dst.write(f"- `{key}` count `{value['count']}` min `{value['min']}` avg `{value['avg']}` max `{value['max']}` ms\n")
                continue
            dst.write("\nBoot phases:\n")
            for key, value in sorted(phases.items()): dst.write(f"- `{key}` `{value}` ms\n")
            samples = rec.get("phase_samples_ms", {})
            if not samples: continue
            dst.write("\nSample aggregates:\n")
            for key, values in sorted(samples.items()):
                avg = round(sum(values) / len(values), 3)
                dst.write(f"- `{key}` count `{len(values)}` min `{min(values)}` avg `{avg}` max `{max(values)}` ms\n")
        if failures: dst.write("\nFailures:\n" + "".join(f"- `{item}`\n" for item in failures))
    subprocess.check_call([py, "nonos-ci/bench_collect.py", out])
    print(summary)
    return 1 if failures and os.environ.get("NONOS_BENCH_STRICT") == "1" else 0
if __name__ == "__main__": raise SystemExit(main())
