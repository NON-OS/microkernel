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
"""Regenerate every extraction crate from source and fail on any drift.

The extraction job used to carry one inline Charon invocation per crate. That
stops being readable somewhere around the tenth, so the crates live in
verification/extraction/crates.json and this walks them.

Drift is the whole point. A kernel edit that changes an extracted function
changes the generated Lean, the diff fails, and the proofs have to be looked at
again before the change can land. A crate that silently stopped extracting would
defeat that, so a crate that fails to build is an error and not a skip.
"""

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


def run(cmd, cwd, timeout=900):
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True,
                           timeout=timeout)
        return p.returncode, (p.stdout or '') + (p.stderr or '')
    except subprocess.TimeoutExpired:
        return 124, 'timed out'
    except FileNotFoundError as exc:
        return 127, str(exc)


def regen_one(root, crate, workdir, write):
    cdir = root / crate['dir']
    if not (cdir / 'Cargo.toml').is_file():
        return 'no crate at %s' % crate['dir']

    args = ['charon', 'cargo', '--preset=aeneas']
    for s in crate['starts']:
        args += ['--start-from', s]
    llbc = '%s.llbc' % crate.get('llbc', crate['name'])
    args += ['--dest-file', llbc]
    rc, out = run(args, cdir)
    if rc != 0:
        return 'charon failed: %s' % out.strip().splitlines()[-1:]

    dest = workdir / crate['name']
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    extra = ['-split-files'] if crate['lean'] == 'Policy' else []
    rc, out = run(['aeneas', '-backend', 'lean'] + extra + [llbc, '-dest', str(dest)],
                  cdir)
    if rc != 0:
        return 'aeneas failed: %s' % out.strip().splitlines()[-1:]

    wanted = crate['generated']
    if isinstance(wanted, str):
        wanted = [wanted]
    leanroot = root / 'verification/extraction/lean'
    for fname in wanted:
        produced = dest / fname
        if not produced.is_file():
            found = sorted(q.name for q in dest.glob('*.lean'))
            return 'expected %s, got %s' % (fname, found)
        committed = (leanroot / 'Policy' / fname) if crate['lean'] == 'Policy' \
            else (leanroot / 'NonosExtraction' / ('%s.lean' % crate['lean']))
        if write:
            committed.write_text(produced.read_text())
            continue
        if not committed.is_file():
            return 'no committed module at %s' % committed
        if committed.read_text() != produced.read_text():
            rc, diff = run(['diff', '-u', str(committed), str(produced)], root)
            return 'drift in %s:\n%s' % (fname, diff)
    return None


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n', 1)[0])
    ap.add_argument('--repo-root', default='.')
    ap.add_argument('--manifest', default='verification/extraction/crates.json')
    ap.add_argument('--only', help='regenerate a single crate by name')
    ap.add_argument('--write', action='store_true',
                    help='update the committed modules instead of diffing them')
    args = ap.parse_args()

    root = Path(args.repo_root).resolve()
    crates = json.loads((root / args.manifest).read_text())['crates']
    if args.only:
        crates = [c for c in crates if c['name'] == args.only]
        if not crates:
            print('no crate named %s' % args.only, file=sys.stderr)
            return 1

    failures = []
    with tempfile.TemporaryDirectory(prefix='nonos-regen-') as tmp:
        work = Path(tmp)
        for c in crates:
            why = regen_one(root, c, work, args.write)
            if why:
                failures.append((c['name'], why))
                print('FAIL  %-34s %s' % (c['name'], why.splitlines()[0]))
            else:
                print('ok    %-34s %3d entry points' % (c['name'], len(c['starts'])))

    total = sum(len(c['starts']) for c in crates)
    print('\n%d crates, %d entry points, %d failures'
          % (len(crates), total, len(failures)))
    if failures:
        print('\nA failure here means the kernel changed under an extracted '
              'function, or a crate stopped building. Both need the proofs '
              'looked at before this lands.', file=sys.stderr)
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
