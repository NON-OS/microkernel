#!/usr/bin/env bash
# Build one shard of this host's flake checks, the set `nix flake check`
# builds, so the checks run on several machines at once. The list is read
# from the flake on every run, sorted, and dealt round-robin: shard k of n
# takes the checks at positions k, k+n, k+2n ..., so the shards cover the
# whole set exactly once whatever is added or removed. Shard 1 also runs
# `nix flake check --no-build`, which evaluates every output of the flake for
# this host: the part of `nix flake check` that builds nothing.
#
# Usage: flake-check-shard.sh <shard>/<shards>
# Writes the shard's checks and verdicts to
# ci-reports/flake-check/<system>-<shard>-of-<shards>.json and, under
# Actions, a table to the job summary.

set -euo pipefail
cd "$(dirname "$0")/.."

spec="${1:?usage: flake-check-shard.sh <shard>/<shards>}"
shard="${spec%/*}" shards="${spec#*/}"
case "${shard}" in ''|*[!0-9]*) echo "flake-check-shard: bad shard ${spec}" >&2; exit 2 ;; esac
case "${shards}" in ''|*[!0-9]*) echo "flake-check-shard: bad shard ${spec}" >&2; exit 2 ;; esac
if [ "${shard}" -lt 1 ] || [ "${shard}" -gt "${shards}" ]; then
    echo "flake-check-shard: shard ${spec} out of range" >&2
    exit 2
fi

system="$(nix eval --raw --impure --expr builtins.currentSystem)"
report="ci-reports/flake-check/${system}-${shard}-of-${shards}.json"
mkdir -p "$(dirname "${report}")"

# name -> out path for this shard's checks, in one evaluation. A check that
# does not evaluate gets null, and fails below by name, as `nix flake check
# --keep-going` would report it, without stopping the others.
plan="$(mktemp)"
nix eval --json ".#checks.${system}" --apply \
    'cs: builtins.mapAttrs (_: c: let r = builtins.tryEval c.outPath; in if r.success then r.value else null) cs' \
    | jq --argjson k "${shard}" --argjson n "${shards}" \
        'to_entries | sort_by(.key) | {total: length,
          mine: [to_entries[] | select(.key % $n == $k - 1) | .value]}' > "${plan}"
total="$(jq -r .total "${plan}")"
count="$(jq -r '.mine | length' "${plan}")"
echo "flake-check-shard: ${system} shard ${spec}: ${count} of ${total} checks"
jq -r '.mine[].key | "  " + .' "${plan}"

status=0
if [ "${shard}" -eq 1 ]; then
    nix flake check --no-build -L || status=1
fi

# One nix build for the whole shard, so its checks still build in parallel.
attrs=()
while IFS= read -r name; do
    attrs+=(".#checks.${system}.${name}")
done < <(jq -r '.mine[] | select(.value != null) | .key' "${plan}")
if [ "${#attrs[@]}" -gt 0 ]; then
    nix build --no-link -L --keep-going "${attrs[@]}" || status=1
fi

# A check passed when its output is in the store.
jq -c '.mine[]' "${plan}" | while IFS= read -r entry; do
    name="$(jq -r .key <<<"${entry}")" out="$(jq -r .value <<<"${entry}")"
    if [ "${out}" = null ]; then verdict=eval-error
    elif [ -e "${out}" ]; then verdict=pass
    else verdict=fail
    fi
    jq -n --arg name "${name}" --arg verdict "${verdict}" '{name: $name, verdict: $verdict}'
done | jq -s --arg system "${system}" --arg shard "${spec}" --argjson total "${total}" \
    '{schema: "nonos.ci.flake-check-shard.v1", system: $system, shard: $shard,
      checks_total: $total, checks: .}' > "${report}"
rm -f "${plan}"

passed="$(jq '[.checks[] | select(.verdict == "pass")] | length' "${report}")"
failed="$(jq '[.checks[] | select(.verdict != "pass")] | length' "${report}")"
if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
    {
        echo "### nix flake check, ${system}, shard ${spec}"
        echo
        echo "| checks in shard | passed | failed | checks on ${system} |"
        echo "|---:|---:|---:|---:|"
        echo "| ${count} | ${passed} | ${failed} | ${total} |"
        jq -r '.checks[] | select(.verdict != "pass") | "- " + .verdict + ": `" + .name + "`"' "${report}"
    } >> "${GITHUB_STEP_SUMMARY}"
fi

[ "${failed}" -eq 0 ] || status=1
exit "${status}"
