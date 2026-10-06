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
import json, os, re, sys
def main() -> int:
    if len(sys.argv) != 3:
        raise SystemExit("usage: bench_boot_log.py <serial.log> <out.json>")
    log_path, out_path = sys.argv[1], sys.argv[2]
    markers = {"capsules_spawned": "[INIT] Capsules spawned", "zk_attest_ok": "[ZK-ATTEST] ok",
               "zk_attest_fail": "[ZK-ATTEST] FAIL", "panic": "[PANIC]", "fatal": "[FATAL]"}
    data = {
        "schema": "nonos.bench.bootlog.v1",
        "log": log_path,
        "status": "gap",
        "markers": {key: 0 for key in markers},
        "bench_events": [],
        "phase_ms": {},
        "phase_samples_ms": {},
        "latency_status": "gap",
    }
    if not os.path.exists(log_path) or os.path.getsize(log_path) == 0:
        data["reason"] = "serial log missing or empty"
    else:
        bench = re.compile(r"^\[BENCH\] ([0-9]+) ([A-Za-z0-9_.:-]+)")
        with open(log_path, "r", encoding="utf-8", errors="replace") as src:
            for nr, line in enumerate(src, 1):
                for key, needle in markers.items():
                    if needle in line: data["markers"][key] += 1
                match = bench.search(line)
                if match: data["bench_events"].append([match.group(2), nr, int(match.group(1))])
        if data["markers"]["panic"] or data["markers"]["fatal"]:
            data["status"] = "fail"
        elif data["markers"]["capsules_spawned"]:
            data["status"] = "pass"
        else:
            data["reason"] = "ready marker missing"
        phase_text = ("kernel_core_ms kernel_entry kernel_core_ready", "microkernel_init_ms microkernel_init_start microkernel_core_ready",
                      "userspace_entry_ms microkernel_main_start init_enter_userspace", "first_spawn_ms capsule_spawn_start capsule_runqueue_ok",
                      "capsule_preflight_ms capsule_spawn_start capsule_preflight_ok", "capsule_install_ms capsule_preflight_ok capsule_runqueue_ok",
                      "first_surface_register_ms microkernel_main_start surface_register_first", "first_surface_present_ms microkernel_main_start surface_present_first",
                      "first_gpu_transfer_ms microkernel_main_start virtio_gpu_transfer_first", "first_gpu_scanout_ms microkernel_main_start virtio_gpu_scanout_first",
                      "first_gpu_flush_ms microkernel_main_start virtio_gpu_flush_first", "gpu_scanout_to_flush_ms virtio_gpu_scanout_first virtio_gpu_flush_first",
                      "first_input_post_ms microkernel_main_start input_post_first", "first_input_drain_ms microkernel_main_start input_drain_first", "input_route_ms input_post_first input_drain_first", "capsule_attest_ms capsule_spawn_start capsule_attest_ok", "capsule_attest_to_runqueue_ms capsule_attest_ok capsule_runqueue_ok")
        for name, a, b in (item.split() for item in phase_text):
            pending, samples = None, []
            for ev, _, ms in data["bench_events"]:
                if ev == a or ev.startswith(a + ":"):
                    pending = ms
                elif (ev == b or ev.startswith(b + ":")) and pending is not None and ms >= pending:
                    samples.append(ms - pending)
                    pending = None
            if samples:
                data["phase_ms"][name] = samples[0]
                data["phase_samples_ms"][name] = samples
        if data["phase_ms"]: data["latency_status"] = "measured"
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    with open(out_path, "w", encoding="utf-8") as out:
        json.dump(data, out, indent=2, sort_keys=True)
        out.write("\n")
    print(out_path)
    return 0 if data["status"] == "pass" else 2
if __name__ == "__main__": raise SystemExit(main())
