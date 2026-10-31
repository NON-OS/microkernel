#!/usr/bin/env bash
# Hold the pipelines to their own standard: every workflow and composite
# action parses as YAML and passes actionlint (which runs shellcheck over
# each inline script), every shell script under nonos-ci/ passes shellcheck,
# every action is pinned by a full commit SHA, and the fuzz target list
# matches the tree. Zero findings, warnings included.
#
# Usage: lint-ci.sh   (needs actionlint, shellcheck, python3 and yq or PyYAML)

set -euo pipefail
cd "$(dirname "$0")/.."

status=0
step() { echo "::group::$1"; }
done_step() { echo "::endgroup::"; }

step "YAML"
for f in .github/workflows/*.yml .github/actions/*/action.yml; do
    if command -v yq >/dev/null; then
        yq e '.' "${f}" >/dev/null || { echo "${f}: not valid YAML"; status=1; }
    else
        python3 -c 'import sys, yaml; yaml.safe_load(open(sys.argv[1]))' "${f}" || { echo "${f}: not valid YAML"; status=1; }
    fi
done
done_step

step "actionlint"
actionlint -no-color || status=1
done_step

step "shellcheck"
git ls-files -z 'nonos-ci/*.sh' | xargs -0 shellcheck --severity=warning || status=1
done_step

step "actions pinned by commit"
if grep -nE '^\s*-?\s*uses:\s*[^./][^@]*@' .github/workflows/*.yml .github/actions/*/action.yml \
        | grep -vE '@[0-9a-f]{40}( |$)'; then
    echo "an action above is not pinned by a full commit SHA"
    status=1
fi
done_step

step "fuzz targets"
python3 nonos-ci/fuzz_targets.py --check || status=1
done_step

exit "${status}"
