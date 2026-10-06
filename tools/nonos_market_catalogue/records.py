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
"""The JSON records a listing is made of."""

FREE = {"kind": "free", "amount_atomic": "0", "period_seconds": 0}
NOX = {"symbol": "NOX", "decimals": 18, "chain_id": 1, "contract_address": ""}


def validation(note: str, validator: str, when_ms: int) -> dict:
    return {
        "status": "validated",
        "note": note,
        "validator_id": validator,
        "validated_at_ms": when_ms,
    }


def release(rid, manifest, package, url, arches, caps, note, validator,
             when_ms, trailer=""):
    return {
        "release_id": rid,
        "manifest_hash": manifest,
        "package_hash": package,
        "package_url": url,
        "publisher_signature": "",
        "supported_arches": arches,
        "kernel_abi_min": 1,
        "required_capabilities": caps,
        "zk_trailer_hash": trailer,
        "validation": validation(note, validator, when_ms),
    }


def entry(listing, capsule_id, name, publisher, pubkey, text, releases):
    return {
        "listing_id": listing,
        "capsule_id": capsule_id,
        "name": name,
        "publisher_name": publisher,
        "publisher_pubkey": pubkey,
        "publisher_eth_address": "00" * 20,
        "description": text,
        "price": FREE,
        "token": NOX,
        "releases": releases,
    }
