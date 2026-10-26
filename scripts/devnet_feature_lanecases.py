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
"""The self-test's cases for the test lane rule: a library's devnet lane
passes, and a capsule's lane, a lane that is not a feature lane, and a
release path that names the same arguments are each refused."""

import json

import devnet_feature_lanes as lanes
import devnet_feature_scan as scan

CAPS = {"userland/capsule_nox_settle"}


def lane(key, args="--features devnet"):
    return lanes.lanes(lanes.LANES, json.dumps({key: args}), CAPS)


def lane_cases():
    return [
        (lane("userland/nonos_nox_config+devnet"), True),
        (lane("userland/tls_proofs+devnet", "--lib --features devnet"), True),
        (lane("userland/nonos_nox_config", ""), True),
        (lane("userland/capsule_nox_settle+devnet"), False),
        (lane("userland/capsule_nox_settle/tests/x+devnet"), False),
        (lane("userland/nonos_nox_config"), False),
        (lane("userland/nonos_nox_config+devnet", "--features devnet --features devnet"), False),
        (lanes.lanes(lanes.LANES, "{", CAPS), False),
        (scan.anywhere("tools/nix/capsules.nix", 'cargoFeatures = "--features devnet";'), False),
    ]
