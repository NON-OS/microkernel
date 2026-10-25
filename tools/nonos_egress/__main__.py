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
"""nonos-pcap-egress EGRESS --ingress INGRESS --allowlist FILE [--json OUT]

Exit 0 pass, 1 fail, 2 a capture or the allow-list unreadable, 3 empty (the
guest sent nothing), 4 incomplete (no DHCP, or no pinned peer reached).
--expect-silence takes an empty capture as the claim, for an image with no
network. --self-test runs the judge on captures built here."""

import argparse
import json
import sys
from pathlib import Path

from .allow import BadAllowList, load
from .judge import judge
from .pcap import Unreadable, mac_text
from .report import QEMU_NIC_MAC

EXIT = {"pass": 0, "fail": 1, "empty": 3, "incomplete": 4}
DEFAULT_ALLOW = Path(__file__).resolve().parent / "allowlist.json"


def args(argv):
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("egress", nargs="?", help="filter-dump queue=rx: every frame the guest sent")
    ap.add_argument("--ingress", help="filter-dump queue=tx: what the network sent the guest, for relays")
    ap.add_argument("--allowlist", default=str(DEFAULT_ALLOW), help="the committed allow-list")
    ap.add_argument("--nic-mac", default=mac_text(QEMU_NIC_MAC), help="the NIC's assigned address")
    ap.add_argument("--expect-silence", action="store_true", help="an empty capture is the claim")
    ap.add_argument("--json", help="write the report here as well")
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--check-allowlist", action="store_true",
                    help="exit 2 unless --allowlist is what the capsules' sources pin (scrape.py)")
    return ap.parse_args(argv)


def show(r):
    for p in r["peers"]:
        print(f"[egress] {p['proto']} {p['ip']:15} {p['port']:5} {p['frames']:6} frames  {p['why']}")
    for name, n in r["tls_server_names"].items():
        print(f"[egress] tls {name} x{n}")
    for host, n in r["http_hosts"].items():
        print(f"[egress] http {host!r} x{n}")
    for params in r["dhcp_parameter_lists"]:
        print(f"[egress] dhcp parameter request list {params}")
    print(f"[egress] guest MACs {', '.join(r['guest_macs']) or 'none'}")
    for f in r["failures"]:
        print(f"[egress] FAIL {f['kind']}: {f['what']}")
    print(f"[egress] {r['status'].upper()}: {r['guest_frames']} guest frames, "
          f"{r['firmware_frames']} firmware frames, of {r['frames']}; {r['relays_learned']} relays learned")


def check_allowlist(path):
    from .scrape import scrape

    want = scrape(Path(__file__).resolve().parent.parent.parent)
    have = json.loads(Path(path).read_text())
    if want == have:
        print(f"[egress] allow-list {path} is what the sources pin")
        return 0
    print(f"[egress] allow-list {path} is not what the sources pin; review and commit:")
    print(json.dumps(want, indent=1))
    return 2


def main(argv=None):
    a = args(argv)
    if a.self_test:
        from .selftest_damage import self_test
        return self_test()
    if a.check_allowlist:
        return check_allowlist(a.allowlist)
    if a.egress is None:
        sys.exit("an egress capture is required")
    try:
        allow = load(a.allowlist)
        egress = Path(a.egress).read_bytes()
        ingress = Path(a.ingress).read_bytes() if a.ingress else b""
        r = judge(egress, ingress, allow, bytes.fromhex(a.nic_mac.replace(":", "")))
    except (OSError, Unreadable, BadAllowList, ValueError) as e:
        print(f"[egress] cannot judge: {e}", file=sys.stderr)
        return 2
    show(r)
    if a.json:
        Path(a.json).write_text(json.dumps(r, indent=1) + "\n")
    if r["status"] == "empty" and a.expect_silence:
        return 0
    if r["status"] == "empty":
        print("[egress] EMPTY: the guest sent nothing, so the capture proves no path clean")
    elif r["status"] == "incomplete":
        print("[egress] INCOMPLETE: no DHCP, or no pinned peer reached; the boot did not get far enough")
    return EXIT[r["status"]]


if __name__ == "__main__":
    sys.exit(main())
