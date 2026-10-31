#!/usr/bin/env bash
# Run one fuzz target for a fixed time over its corpus, and say what it did.
#
# The corpus is <crate>/fuzz/corpus/<target> (the workflow restores it from
# the cache first, so coverage carries over between runs), seeded by the
# crate's seed.py when the target asks for it. libFuzzer writes any crashing
# input under <crate>/fuzz/artifacts/<target>/; each one fails the run with
# its reproducer named. The summary (execs, coverage, features, corpus growth,
# crashes) goes to fuzz-out/<slug>.json for fuzz_record.py.
#
# Usage: fuzz-run.sh <crate> <target> <minutes> <seed: true|false>

set -euo pipefail
cd "$(dirname "$0")/.."

crate="${1:?usage: fuzz-run.sh <crate> <target> <minutes> <seed>}"
target="${2:?}" minutes="${3:?}" seed="${4:-false}"
case "${minutes}" in ''|*[!0-9]*) echo "fuzz-run: minutes must be digits" >&2; exit 2 ;; esac
seconds=$(( minutes * 60 ))
slug="${crate//\//-}-${target}"
root="${PWD}"
out="${root}/fuzz-out"
mkdir -p "${out}"

cd "${crate}/fuzz"
corpus="corpus/${target}"
artifacts="artifacts/${target}"
mkdir -p "${corpus}" "${artifacts}"
if [ "${seed}" = "true" ]; then
    python3 seed.py --out "${corpus}"
fi
before="$(find "${corpus}" -type f | wc -l | tr -d ' ')"

status=0
cargo fuzz run "${target}" "${corpus}" -- \
    -max_total_time="${seconds}" -print_final_stats=1 -artifact_prefix="${artifacts}/" \
    2>&1 | tee "${out}/${slug}.log" || status=$?

after="$(find "${corpus}" -type f | wc -l | tr -d ' ')"
python3 "${root}/nonos-ci/fuzz_record.py" target --log "${out}/${slug}.log" \
    --crate "${crate}" --target "${target}" --seconds "${seconds}" \
    --corpus-before "${before}" --corpus-after "${after}" \
    --artifacts "${artifacts}" --out "${out}/${slug}.json" >/dev/null

found=0
# libFuzzer's failures; a slow-unit-* input is kept but fails nothing.
for crash in "${artifacts}"/crash-* "${artifacts}"/leak-* "${artifacts}"/timeout-* "${artifacts}"/oom-*; do
    [ -f "${crash}" ] || continue
    found=1
    echo "::error file=${crate}/fuzz/${crash}::${target} crashed; reproduce with: cd ${crate}/fuzz && cargo fuzz run ${target} ${crash}"
done
if [ "${found}" -eq 1 ] && [ "${status}" -eq 0 ]; then status=1; fi
if [ "${status}" -ne 0 ] && [ "${found}" -eq 0 ]; then
    echo "::error::${crate} ${target}: cargo fuzz exited ${status} without a crash input (build or harness failure); see ${slug}.log"
fi
exit "${status}"
