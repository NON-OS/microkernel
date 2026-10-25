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
"""A fixture tree for check_release_trust: sealed, unsealed and baselined
capsules, and the mutant that drops one sealed file, which must be caught."""

import tempfile
from pathlib import Path


def self_test(unsealed, verdict):
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        entries = [{"slug": s, **{k: f"{s}.{k}" for k in ("cert", "manifest", "trailer")}}
                   for s in ("sealed", "waiting", "unshipped")]
        for k in ("cert", "manifest", "trailer"):
            (root / f"sealed.{k}").write_bytes(b"x")
        exists = lambda p: (root / p).is_file()
        shipped = ["sealed", "waiting"]
        missing = unsealed(entries, shipped, exists)
        assert missing == ["waiting"], missing
        assert verdict(missing, {"waiting"}) == [], "the baselined gap failed"
        assert len(verdict(missing, set())) == 1, "a new gap passed"
        # The mutant: a sealed capsule loses its manifest.
        (root / "sealed.manifest").unlink()
        got = verdict(unsealed(entries, shipped, exists), {"waiting"})
        assert len(got) == 1 and "sealed ships" in got[0], got
        # The seal writes the waiting set, and the baseline still names it.
        for k in ("cert", "manifest", "trailer"):
            (root / f"waiting.{k}").write_bytes(b"x")
            (root / f"sealed.{k}").write_bytes(b"x")
        got = verdict(unsealed(entries, shipped, exists), {"waiting"})
        assert len(got) == 1 and "sealed now" in got[0], got
        got = verdict([], {"waiting"}, {"sealed"})
        assert len(got) == 1 and "no longer shipped" in got[0], got
    from release_trust_lists import BEGIN, END, doc_slugs, drift, unnamed
    entries = [{"slug": "sealed"}, {"slug": "waiting"}]
    assert unnamed(entries, ["sealed", "waiting"]) == [], "a named slug was refused"
    got = unnamed(entries, ["sealed", "made-up"])
    assert len(got) == 1 and "made-up ships" in got[0], got
    listed = doc_slugs(f"x\n{BEGIN}\n`waiting`, `b-2`.\n{END}\n`outside`")
    assert listed == {"waiting", "b-2"}, listed
    assert drift({"waiting", "b-2"}, listed) == [], "equal lists drifted"
    assert len(drift({"waiting"}, listed)) == 1 and len(drift({"waiting", "b-2", "c"}, listed)) == 1
    assert len(drift(set(), doc_slugs("no markers"))) == 1, "a doc without its list passed"
    print("check_release_trust: self-test passed")
    return 0
