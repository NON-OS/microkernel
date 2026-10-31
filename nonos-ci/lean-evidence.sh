#!/usr/bin/env bash
# The Lean specification's evidence, after `lake build` in verification/lean:
# the axiom closure of every flagship theorem as Lean prints it (a `sorry`
# in any of them surfaces as sorryAx and fails), the theorem inventory, and
# a manifest naming the commit and toolchain they were checked against.
#
# Usage: lean-evidence.sh <out dir>

set -euo pipefail

out="$(mkdir -p "${1:?usage: lean-evidence.sh <out dir>}" && cd "$1" && pwd)"
cd "$(dirname "$0")/../verification/lean"
commit="${GITHUB_SHA:-$(git rev-parse HEAD)}"
toolchain="$(cat lean-toolchain)"

lake env lean AxiomProfile.lean | tee "${out}/axioms.txt"
if grep -q sorryAx "${out}/axioms.txt"; then
    echo "::error::a flagship theorem depends on sorryAx"
    exit 1
fi

grep -rn '^theorem ' Nonos/ > "${out}/theorems.txt"
count() { { grep -rh "^$1 " Nonos --include='*.lean' || true; } | wc -l | tr -d ' '; }
thms="$(count theorem)"
lemmas="$(count lemma)"
mods="$(find Nonos -name '*.lean' | wc -l | tr -d ' ')"
loc="$(find Nonos -name '*.lean' -exec cat {} + | wc -l | tr -d ' ')"
hatches="$(grep -rEn '\bsorry\b|\badmit\b|^axiom ' Nonos --include='*.lean' | grep -vc 'no sorry anywhere' || true)"

{
    echo "commit: ${commit}"
    echo "toolchain: ${toolchain}"
    echo "theorems: ${thms}"
    echo "escape hatches:"
    grep -rEn '\bsorry\b|\badmit\b|^axiom ' Nonos.lean Nonos/ | grep -v 'no sorry anywhere' || echo "  none"
} > "${out}/summary.txt"
cat "${out}/summary.txt"

{
    echo "# NONOS Lean proof inventory"
    echo
    echo "commit \`${commit}\`, toolchain \`${toolchain}\`"
    echo
    echo "${thms} theorems, ${lemmas} lemmas across ${mods} modules, ${loc} lines of proof, ${hatches:-0} escape hatches."
    echo
    find Nonos -name '*.lean' | LC_ALL=C sort | while IFS= read -r f; do
        echo "## ${f} ($(grep -c -e '^theorem ' -e '^lemma ' "${f}" || true))"
        grep -hoE '^(theorem|lemma) [^ :({]+' "${f}" | sed 's/^/  - /' || true
        echo
    done
} > "${out}/inventory.md"

jq -n --arg commit "${commit}" --arg toolchain "${toolchain}" \
    --argjson theorems "${thms}" --argjson lemmas "${lemmas}" --argjson modules "${mods}" \
    --argjson proof_loc "${loc}" --argjson escape_hatches "${hatches:-0}" \
    '{commit: $commit, toolchain: $toolchain, theorems: $theorems, lemmas: $lemmas,
      modules: $modules, proof_loc: $proof_loc, escape_hatches: $escape_hatches}' \
    | tee "${out}/manifest.json"
