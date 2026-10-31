#!/usr/bin/env bash
# Commit this run's benchmark and fuzz records to the results branch, so they
# are kept with their history beside the code without touching main.
#
# The branch holds only benchmarks/ and fuzz/ (orphan history). Each attempt
# starts from the branch's tip, adds this run's records under a folder of
# their own, rebuilds history.csv and LATEST.md from every record kept, then
# commits and pushes; a push that lost a race starts over, up to five times.
# Publishing never fails the run: a final failure is a warning.
#
# Usage: publish-results.sh <branch> <bench run dir|-> <fuzz summaries dir|->
# Needs GH_TOKEN (contents: write), GITHUB_REPOSITORY and GITHUB_SHA.

set -uo pipefail

branch="${1:?usage: publish-results.sh <branch> <bench run dir|-> <fuzz summaries dir|->}"
bench="${2:--}" fuzz="${3:--}"
here="$(cd "$(dirname "$0")" && pwd)"
: "${GH_TOKEN:?}" "${GITHUB_REPOSITORY:?}" "${GITHUB_SHA:?}"

have() { [ "$1" != "-" ] && [ -d "$1" ] && [ -n "$(ls -A "$1" 2>/dev/null)" ]; }
abs() { (cd "$1" && pwd); }
if ! have "${bench}" && ! have "${fuzz}"; then
    echo "publish-results: no benchmark or fuzz records in this run"
    exit 0
fi
have "${bench}" && bench="$(abs "${bench}")"
have "${fuzz}" && fuzz="$(abs "${fuzz}")"

auth="AUTHORIZATION: basic $(printf 'x-access-token:%s' "${GH_TOKEN}" | base64 | tr -d '\n')"
echo "::add-mask::${auth#AUTHORIZATION: basic }"
remote="${NONOS_RESULTS_REMOTE:-https://github.com/${GITHUB_REPOSITORY}.git}"
g() { git -c "http.https://github.com/.extraheader=${auth}" "$@"; }

attempt() {
    local work
    work="$(mktemp -d)"
    cd "${work}" || return 1
    git init -q .
    git config user.name "github-actions[bot]"
    git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
    if g ls-remote --exit-code --heads "${remote}" "${branch}" >/dev/null; then
        g fetch -q --depth=1 "${remote}" "${branch}" || return 1
        git checkout -q -B "${branch}" FETCH_HEAD || return 1
    else
        git checkout -q --orphan "${branch}"
    fi
    mkdir -p benchmarks fuzz
    if have "${bench}"; then
        python3 "${here}/bench_record.py" record "${bench}" . || return 1
        python3 "${here}/bench_record.py" rebuild . || return 1
    fi
    if have "${fuzz}"; then
        GITHUB_STEP_SUMMARY='' python3 "${here}/fuzz_record.py" run "${fuzz}" . || return 1
        python3 "${here}/fuzz_record.py" rebuild . || return 1
    fi
    git add -A benchmarks fuzz
    local what="bench"
    if ! have "${bench}"; then what="fuzz"; fi
    git commit -q -m "${what}: $(date -u +%Y-%m-%d) ${GITHUB_SHA:0:12}" || return 1
    g push -q "${remote}" "HEAD:refs/heads/${branch}"
}

for n in 1 2 3 4 5; do
    if (attempt); then
        echo "publish-results: records pushed to ${branch}"
        exit 0
    fi
    echo "publish-results: attempt ${n} did not land; retrying from the branch tip"
    sleep $(( n * 5 ))
done
echo "::warning::benchmark and fuzz records could not be pushed to ${branch}; they are still in this run's artifacts"
exit 0
