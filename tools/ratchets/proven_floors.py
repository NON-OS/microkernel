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
"""The numbers proven_functions.py holds the tree to."""

# Raise these when the numbers improve. They may never be lowered, except when
# the code a proof covered is deleted: 1756a547 removed the zk_kernel field and
# range code with the curve proof, and its five extracted functions with it
# (987 to 982 proven, 494 to 492 substantive). 074a3a9d deleted the three
# paging statistics getters memory_paging_stats_query extracted (982 to 979).
# Removing the compat MSI builders left MsiInfo::max_vectors with no caller,
# and it went with them (979 to 978).
FLOOR = 978
GAP_CEILING = 0
# Functions carrying a property beyond "this wrapper is its method". That
# wrapper theorem is real, and it is what ties a manifest entry to the method a
# theorem talks about, but on its own it says nothing about behaviour. Counting
# the two together would be the inflation this file exists to stop.
SUBSTANTIVE_FLOOR = 492
