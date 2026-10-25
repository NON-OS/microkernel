# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""Runs run-static-checks.sh's operator-key block on a fake tree, traced.
With NONOS_STATIC_SKIP_KEYS=1 no executed command may name .keys/ (the
assignments aside), and the trust list's shape must still be checked.
--selftest also shows each mutant of the block fails it."""
import os, re, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
KEY = "bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2"
RS = ('const NOX_OPERATOR_V1: [u8; 32] =\n'
      '    *include_bytes!("../../../../.keys/marketplace_operator_ed25519.pub");\n')
WIRED = 'pub static TRUSTED_OPERATORS: &[[u8; 32]] = &[NOX_OPERATOR_V1];\n'
READS = re.compile(r"^\++ (od|cat|head|tail|xxd|cmp|wc|sha256sum|'\[' !? ?-[efrsd]) ")
DEFS = ("note() { printf '[%s] %s\\n' \"$1\" \"$2\"; }\n"
        "fail_with() { printf '::error::%s\\n' \"$1\"; }\n")

def block(path):
    s = open(path).read()
    return s[s.index("# >>> operator-key block"):s.index("# <<< operator-key block")]

def run(code, skip, key, wired=True):
    with tempfile.TemporaryDirectory() as d:
        src = os.path.join(d, "userland/capsule_market/src/bootstrap_trust")
        os.makedirs(src)
        open(os.path.join(src, "keys.rs"), "w").write(RS + (WIRED if wired else ""))
        if key is not None:
            os.makedirs(os.path.join(d, ".keys"))
            open(os.path.join(d, ".keys/marketplace_operator_ed25519.pub"), "wb").write(key)
        env = {"PATH": os.environ["PATH"], **({"NONOS_STATIC_SKIP_KEYS": "1"} if skip else {})}
        r = subprocess.run(["bash", "-x", "-c", DEFS + code], cwd=d, env=env,
                           capture_output=True, text=True)
    touched = [l for l in r.stderr.splitlines() if READS.match(l) and ".keys/" in l]
    return r.stdout, touched

def holds(code):
    out, touched = run(code, True, None)
    errs = [run(code, True, None, wired=False), run(code, False, None),
            run(code, False, b"\0" * 32)]
    return ("[skip]" in out and "::error::" not in out and not touched
            and all("::error::" in o for o, _ in errs)
            and "[ok]" in run(code, False, bytes.fromhex(KEY))[0])

MUTANTS = [
    ('elif [ "${NONOS_STATIC_SKIP_KEYS:-0}" = 1 ]; then', "elif false; then"),
    ('elif [ "${NONOS_STATIC_SKIP_KEYS:-0}" = 1 ]; then', "elif true; then"),
    ('! grep -qE', "! true ||"),
    ('!= "${operator_pubkey_hex}"', '= "${operator_pubkey_hex}"'),
    ('elif [ -z "${trust_key_rel}" ] ||',
     'elif [ ! -f "${trust_key_file}" ] || [ -z "${trust_key_rel}" ] ||'),
]

def main():
    code = block(os.path.join(HERE, "run-static-checks.sh"))
    if not holds(code):
        sys.exit("check-skip-keys: the block reads .keys/ when skipped, or checks too little")
    if "--selftest" in sys.argv:
        for old, new in MUTANTS:
            if old not in code or holds(code.replace(old, new, 1)):
                sys.exit(f"check-skip-keys: mutant not caught: {new}")
    print("check-skip-keys: ok")

if __name__ == "__main__":
    main()
