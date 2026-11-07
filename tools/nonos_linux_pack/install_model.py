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

"""The install-time model in , as arithmetic
the project can run again with measured throughputs: today's path (an index,
then one GET per package, one at a time) against an image's (a list, then
one GET per chunk, `width` at a time). Every rate is a parameter; nothing
here is a measurement."""

from typing import NamedTuple

MB = 1_000_000


class Link(NamedTuple):
    mb_s: float   # one stream's throughput, MB/s
    rtt: float    # seconds a GET costs before its bytes flow, on a kept stream


class Device(NamedTuple):
    verify_s_mb: float   # today: RSA chain, checksums, inflate, per MB of .apk
    decode_s_mb: float   # an image: xz decode and BLAKE3, per MB of member tar
    place_s_mb: float    # packing into the store, per MB installed
    fixed_s: float       # policy gate and bind, records: per install


# The monero runs on the project's KVM host (owner-kvm), 
KVM = Device(verify_s_mb=1.9 / 30.2, decode_s_mb=2 * 1.31 / 73.6, place_s_mb=3.1 / 73.8, fixed_s=29.0)
INDEX_BYTES = 473_606 + 1_848_333


def today(link: Link, dev: Device, gets: int, apk: int, installed: int) -> dict:
    net = 2 * link.rtt + INDEX_BYTES / MB / link.mb_s + gets * link.rtt + apk / MB / link.mb_s
    rest = apk / MB * dev.verify_s_mb + installed / MB * dev.place_s_mb + dev.fixed_s
    return {"net": net, "device": rest, "total": net + rest}


def image(link: Link, dev: Device, size: int, chunk: int, raw: int, installed: int,
          width: int = 1, cached: int = 0) -> dict:
    """`cached` chunks are already here: a re-install asks only for the rest."""
    chunks = -(-size // chunk)
    left = max(chunks - cached, 0)
    moved = min(size, left * chunk)
    rounds = -(-left // max(width, 1))
    net = (link.rtt if left else 0) + rounds * link.rtt + moved / MB / (link.mb_s * max(width, 1))
    rest = raw / MB * dev.decode_s_mb + installed / MB * dev.place_s_mb + dev.fixed_s
    return {"net": net, "device": rest, "total": net + rest}
