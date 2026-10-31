#!/usr/bin/env bash
# Refuse to tag a commit whose own checks did not finish green: every
# completed check run on the commit must have ended success, skipped or
# neutral. Needs GH_TOKEN and GITHUB_REPOSITORY.
#
# Usage: require-green.sh <commit>

set -euo pipefail

sha="${1:?usage: require-green.sh <commit>}"
filter='.check_runs[] | select(.conclusion != null and .conclusion != "success" and .conclusion != "skipped" and .conclusion != "neutral")'
bad="$(gh api --paginate "repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?per_page=100" \
    --jq "${filter} | \"  \(.name): \(.conclusion)\"")"
if [ -n "${bad}" ]; then
    echo "${bad}"
    echo "::error::$(wc -l <<<"${bad}" | tr -d ' ') check(s) on ${sha} are not green; refusing to tag"
    exit 1
fi
echo "all completed checks on ${sha} are green"
