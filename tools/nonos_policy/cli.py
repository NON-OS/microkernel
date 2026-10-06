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
"""The boot-root verbs of nonos-policy-approve.

    nonos-policy-approve boot-root --root bootloader_attest_root.bin --epoch N \\
        --key policy_p256.pem --out boot_root.approval
    nonos-policy-approve boot-root-show --record boot_root.approval --pub policy_p256.pub.pem

The key is the release's P-256 policy key, the one that approves kernels for the
device secret. Like every verb here, these never create a key.
"""

from . import boot_root


def sign(args, read_root, load_key):
    key, _, _ = load_key(args.key, private=True)
    record = boot_root.sign(key, read_root(args.root), args.epoch)
    with open(args.out, "wb") as f:
        f.write(record)
    print(f"root      {record[:32].hex()}")
    print(f"epoch     {args.epoch}")
    print(f"message   {boot_root.message(record[:32], args.epoch).hex()}")
    print(f"wrote {args.out}")


def show(args, load_key):
    pub, _, _ = load_key(args.pub, private=False)
    root, epoch = boot_root.verify(pub, open(args.record, "rb").read())
    print(f"root      {root.hex()}")
    print(f"epoch     {epoch}")
    print("signature verifies under this key")


def add(sub, read_root, load_key):
    p = sub.add_parser("boot-root", help="sign the boot-root record for a bootloader tree")
    p.add_argument("--root", required=True, help="the 32-byte bootloader attestation root")
    p.add_argument("--epoch", required=True, type=int, help="the release's rollback index")
    p.add_argument("--key", required=True, help="P-256 policy key, PEM")
    p.add_argument("--out", required=True, help="where to write the 104-byte record")
    p.set_defaults(fn=lambda a: sign(a, read_root, load_key))
    p = sub.add_parser("boot-root-show", help="verify a boot-root record and print it")
    p.add_argument("--record", required=True, help="a boot_root.approval file")
    p.add_argument("--pub", required=True, help="P-256 policy public key, PEM")
    p.set_defaults(fn=lambda a: show(a, load_key))
