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
"""The allow-list as the capsules' sources say it, to hold against the
committed allowlist.json: a pinned address or port that changes in the
source fails the run until a reviewer commits the new list.

  - names: the TLS names Nym's directory client pins (pinned.rs)
  - Nym's directory API: its addresses (live.rs), on HTTPS_PORT (fetch_at.rs)
  - Nym's bootstrap gateways, each with its port (state/bootstrap.rs)
  - Anyone's directory authorities, each with its dir_port (authority/list.rs);
    their consensus names the relays, so they are marked authorities"""

import re
from pathlib import Path

from .allow import FORMAT

NYM = Path("userland/capsule_net_nym/src")
ANON = Path("userland/capsule_net_anon/src/directory/authority/list.rs")
QUAD = r"\[(\d{1,3}), (\d{1,3}), (\d{1,3}), (\d{1,3})\]"


def _ip(m, at=1):
    return ".".join(m.group(i) for i in range(at, at + 4))


def scrape(root=Path(".")):
    read = lambda p: (root / p).read_text()
    names = re.findall(r'\("([a-z0-9.-]+)", ' + QUAD + r"\)", read(NYM / "directory_sync/pinned.rs"))
    https = re.search(r"const HTTPS_PORT: u16 = (\d+);", read(NYM / "directory_sync/fetch_at.rs"))
    api = re.search(r"API_ADDRESSES: &\[\[u8; 4\]\] = &\[(.*?)\];", read(NYM / "directory_sync/live.rs"), re.S)
    if not (names and https and api):
        raise ValueError("a pinned name, the HTTPS port or the API addresses moved in the source")
    peers = [{"ip": _ip(m), "port": int(https.group(1)), "what": "Nym directory API"}
             for m in re.finditer(QUAD, api.group(1))]
    for m in re.finditer(r"\(\(" + QUAD + r", (\d+)\)", read(NYM / "state/bootstrap.rs")):
        peers.append({"ip": _ip(m), "port": int(m.group(5)), "what": "Nym bootstrap gateway"})
    for m in re.finditer(r"address: " + QUAD + r",\s*dir_port: (\d+),", read(ANON)):
        peers.append({"ip": _ip(m), "port": int(m.group(5)), "what": "Anyone directory authority",
                      "authority": True})
    if not any(p.get("authority") for p in peers) or len(peers) < 3:
        raise ValueError("the gateways or the authorities moved in the source")
    peers.sort(key=lambda p: (p["what"], tuple(int(x) for x in p["ip"].split(".")), p["port"]))
    return {"format": FORMAT, "names": sorted({n[0] for n in names}), "peers": peers}
