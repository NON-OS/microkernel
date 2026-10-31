#!/usr/bin/env bash
# The benchmark job's run, in the flake's shell after provision-trust.sh:
# make nonos-mk-bench with the build and boot commands CI measures, into
# NONOS_BENCH_OUT. The run's summary.md also goes to the job summary.
#
# Reads NONOS_BENCH_OUT, NONOS_BENCH_BOOT_RUNS, BENCH_STRICT (true|false) and
# BENCH_BOOT (true|false).

set -euo pipefail
cd "$(dirname "$0")/.."

: "${NONOS_BENCH_OUT:?}"
mkdir -p "${NONOS_BENCH_OUT}"
export NONOS_BENCH_BUILD_CMD="cargo run --release --manifest-path nonos-verify/Cargo.toml -- build"
# The boot command builds the kernel and proves the kernel and the loader (one
# STARK slot each) before QEMU starts, about five minutes on a four-core
# runner, so the deadline leaves the boot itself room.
export NONOS_BENCH_BOOT_TIMEOUT="${NONOS_BENCH_BOOT_TIMEOUT:-900}"
export NONOS_BENCH_STRICT=0
if [ "${BENCH_STRICT:-true}" = "true" ]; then export NONOS_BENCH_STRICT=1; fi
if [ "${BENCH_BOOT:-true}" != "true" ]; then export NONOS_BENCH_SKIP_BOOT=1; fi
export NONOS_BENCH_BOOT_CMD="make PLAN_A_SERIAL=${NONOS_BENCH_OUT}/boot/qemu-serial.log PLAN_A_RESULT=${NONOS_BENCH_OUT}/boot/plan-a-result.log nonos-mk-plan-a-runtime"

status=0
make nonos-mk-bench || status=$?
if [ -n "${GITHUB_STEP_SUMMARY:-}" ] && [ -f "${NONOS_BENCH_OUT}/summary.md" ]; then
    cat "${NONOS_BENCH_OUT}/summary.md" >> "${GITHUB_STEP_SUMMARY}"
fi
exit "${status}"
