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
"""Run the whole extraction pipeline over many kernel files at once.

Doing this a crate at a time is why the surface grew slowly. For each file this
generates the mirror crate, type checks it, lowers it with Charon, translates it
with Aeneas, and keeps the ones that survive all four. A file that fails any step
is dropped with the reason rather than left half wired, because a crate that does
not extract is worse than no crate: it sits in CI failing for a reason nobody
reads.

It prints the CI block for the crates that worked, so the workflow edit is a
paste rather than a transcription.
"""

import argparse
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def run(cmd, cwd=None, timeout=300):
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True,
                           timeout=timeout)
        return p.returncode, (p.stdout or '') + (p.stderr or '')
    except subprocess.TimeoutExpired:
        return 124, 'timed out'


def slug(path):
    """A crate-safe name from a kernel path."""
    parts = Path(path).with_suffix('').parts
    tail = [p for p in parts if p not in ('src',)][-2:]
    return re.sub(r'[^a-z0-9_]', '_', '_'.join(tail).lower())


def sweep_one(root, source, outdir, leandir, keep):
    name = slug(source)
    crate = 'nonos_x_%s' % name
    module = Path(source).stem
    lean_mod = ''.join(w.capitalize() for w in name.split('_'))[:24] or 'X'
    cdir = outdir / name

    rc, out = run([sys.executable, str(HERE / 'mirror_crate.py'), source,
                   '--crate', crate, '--out', str(cdir), '--module', module,
                   '--lean-module', lean_mod,
                   '--emit-lean', str(leandir / ('%sRefinement.lean' % lean_mod)),
                   '--repo-root', str(root)], cwd=root)
    if rc != 0:
        return None, 'generate: %s' % out.strip().splitlines()[-1:]
    starts = re.findall(r"--start-from '([^']+)'", out)
    if not starts:
        return None, 'no forwardable functions'

    rc, out = run(['cargo', 'check', '--quiet'], cwd=cdir)
    if rc != 0:
        return None, 'cargo check failed'

    args = ['charon', 'cargo', '--preset=aeneas']
    for s in starts:
        args += ['--start-from', s]
    args += ['--dest-file', '%s.llbc' % name]
    rc, out = run(args, cwd=cdir, timeout=600)
    if rc != 0:
        return None, 'charon failed'

    dest = Path('/tmp/sweep') / name
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    rc, out = run(['aeneas', '-backend', 'lean', '%s.llbc' % name,
                   '-dest', str(dest)], cwd=cdir, timeout=600)
    if rc != 0:
        return None, 'aeneas failed'
    produced = list(dest.glob('*.lean'))
    if not produced:
        return None, 'aeneas produced nothing'

    target = leandir / ('%s.lean' % lean_mod)
    shutil.copy(produced[0], target)
    if not keep:
        pass
    return {'name': name, 'crate': crate, 'module': module,
            'lean': lean_mod, 'starts': starts,
            'generated': produced[0].name, 'dir': str(cdir)}, None


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n', 1)[0])
    ap.add_argument('files', nargs='+', help='kernel files to mirror')
    ap.add_argument('--repo-root', default='.')
    ap.add_argument('--out', default='verification/extraction/sweep')
    ap.add_argument('--lean-dir',
                    default='verification/extraction/lean/NonosExtraction')
    ap.add_argument('--keep-llbc', action='store_true')
    ap.add_argument('--report', default=None, help='write a JSON summary here')
    args = ap.parse_args()

    root = Path(args.repo_root).resolve()
    outdir = root / args.out
    leandir = root / args.lean_dir
    outdir.mkdir(parents=True, exist_ok=True)

    good, bad = [], []
    for f in args.files:
        info, why = sweep_one(root, f, outdir, leandir, args.keep_llbc)
        if info:
            good.append(info)
            print('ok    %-52s %3d functions' % (f, len(info['starts'])))
        else:
            bad.append((f, why))
            print('skip  %-52s %s' % (f, why))

    total = sum(len(g['starts']) for g in good)
    print('\ncrates kept %d, functions %d, skipped %d' % (len(good), total, len(bad)))

    if good:
        print('\nCI block:')
        for g in good:
            print('          cd ../../sweep/%s' % g['name'])
            print('          charon cargo --preset=aeneas \\')
            for s in g['starts']:
                print("            --start-from '%s' \\" % s)
            print('            --dest-file %s.llbc' % g['name'])
            print('          mkdir -p /tmp/regen-%s' % g['name'])
            print('          aeneas -backend lean %s.llbc -dest /tmp/regen-%s'
                  % (g['name'], g['name']))
            print('          diff -u ../../lean/NonosExtraction/%s.lean '
                  '/tmp/regen-%s/%s' % (g['lean'], g['name'], g['generated']))

    if args.report:
        Path(args.report).write_text(json.dumps(
            {'kept': good, 'skipped': bad}, indent=2))
    return 0


if __name__ == '__main__':
    sys.exit(main())
