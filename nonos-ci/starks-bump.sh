#!/usr/bin/env bash
# Follow STARKs main, the trust path pinned once as the `starks` input of
# flake.lock, and propose the move as a pull request; never merge it.
#
#   move   nix flake update starks; writes old=, new= and moved= to
#          $GITHUB_OUTPUT
#   check  tools/nonos-starks-sync writes the new commit into every
#          Cargo.lock, then nix flake check; writes verdict=green|red
#   pr     commit on starks/<new>, push it, and open or update the pull
#          request with the verdict (needs GH_TOKEN, OLD, NEW, VERDICT)
#
# Usage: starks-bump.sh <move|check|pr>

set -euo pipefail
cd "$(dirname "$0")/.."

output() { echo "$1" >> "${GITHUB_OUTPUT:-/dev/stdout}"; }

case "${1:?usage: starks-bump.sh <move|check|pr>}" in
    move)
        old="$(jq -r .nodes.starks.locked.rev flake.lock)"
        nix flake update starks
        new="$(jq -r .nodes.starks.locked.rev flake.lock)"
        output "old=${old}"
        output "new=${new}"
        if [ "${old}" = "${new}" ]; then
            echo "STARKs main is still ${old}; nothing to do"
            output "moved=false"
        else
            echo "STARKs main moved: ${old} -> ${new}"
            output "moved=true"
        fi
        ;;
    check)
        nix develop --command python3 tools/nonos-starks-sync 2>&1 | tee sync.log
        if nix flake check -L --keep-going 2>&1 | tee check.log; then
            output "verdict=green"
        else
            output "verdict=red"
        fi
        ;;
    pr)
        : "${OLD:?}" "${NEW:?}" "${VERDICT:?}"
        branch="starks/${NEW:0:9}"
        git config user.name "github-actions[bot]"
        git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
        git checkout -B "${branch}"
        git add flake.lock
        git ls-files -m -z '*Cargo.lock' '*Cargo.toml' | xargs -0 -r git add --
        git commit -q -m "starks: follow main to ${NEW:0:9}" \
            -m "https://github.com/NON-OS/STARKs/compare/${OLD}...${NEW}"
        git push -f origin "${branch}"
        {
            echo "STARKs main moved from \`${OLD:0:9}\` to \`${NEW:0:9}\`:"
            echo "https://github.com/NON-OS/STARKs/compare/${OLD}...${NEW}"
            echo
            echo "flake.lock moves the \`starks\` input; tools/nonos-starks-sync wrote the commit into every Cargo.lock."
            echo
            echo "nix flake check: **${VERDICT}**"
            echo
            echo '```'
            grep -hE "^error|cannot follow" check.log sync.log | tail -40 || true
            echo '```'
            echo
            echo "This is the trust path. Read the STARKs change before merging."
        } > body.md
        if gh pr view "${branch}" >/dev/null 2>&1; then
            gh pr edit "${branch}" --body-file body.md
        else
            gh pr create --head "${branch}" --base main --title "starks: follow main to ${NEW:0:9}" --body-file body.md
        fi
        ;;
    *)
        echo "starks-bump: unknown step $1" >&2
        exit 2
        ;;
esac
