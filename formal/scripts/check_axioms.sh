#!/usr/bin/env bash
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU Affero General Public License as published by the Free
# Software Foundation, either version 3 of the License, or (at your option) any
# later version. See <https://www.gnu.org/licenses/>.
#
# Prints the axiom closure of every headline theorem and fails when one rests
# on sorryAx or on any axiom beyond propext, Classical.choice and Quot.sound,
# or when the formal/ sources contain sorry, admit or an axiom declaration.
# Run from formal/ after `lake build`. Writes the profile to $1 when given.
set -euo pipefail

out="${1:-/dev/stdout}"
profile="$(lake env lean Axioms.lean)"
printf '%s\n' "$profile" > "$out"

expected=24
lines=$(printf '%s\n' "$profile" | grep -c "depends on axioms\|does not depend on any axioms" || true)
if [ "$lines" -ne "$expected" ]; then
  echo "check_axioms: expected $expected axiom lines, got $lines" >&2
  printf '%s\n' "$profile" >&2
  exit 1
fi

bad=$(printf '%s\n' "$profile" | grep -o "\[[^]]*\]" | tr -d '[]' | tr ',' '\n' | sed 's/^ *//' \
  | grep -v -x -e propext -e Classical.choice -e Quot.sound -e '' || true)
if [ -n "$bad" ]; then
  echo "check_axioms: unexpected axioms: $bad" >&2
  exit 1
fi

hits=$(grep -rnwE 'sorry|admit' --include='*.lean' Formal Formal.lean Axioms.lean || true)
axioms=$(grep -rnE '^\s*axiom\s' --include='*.lean' Formal Formal.lean || true)
if [ -n "$hits$axioms" ]; then
  echo "check_axioms: escape hatch in formal/ sources:" >&2
  printf '%s\n%s\n' "$hits" "$axioms" >&2
  exit 1
fi
echo "check_axioms: ok" >&2
