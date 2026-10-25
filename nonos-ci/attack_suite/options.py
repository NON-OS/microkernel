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


"""The command line's options."""

import argparse

from .rules import BOOTS


def parser(doc):
    ap = argparse.ArgumentParser(description=doc.split("\n", 1)[0], epilog=doc.split("\n\n", 1)[1],
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--log", action="append", default=[], help="a serial log; repeat once per boot")
    ap.add_argument("--boot", action="append", choices=sorted(BOOTS),
                    help="judge only the attacks this boot carries; repeat for more (default: all)")
    ap.add_argument("--report", metavar="PATH", help="write the text report here")
    ap.add_argument("--json", metavar="PATH", help="write the report as JSON here")
    ap.add_argument("--mutants", action="store_true", help="prove every mutant of the logs fails")
    ap.add_argument("--list", action="store_true", help="print the attacks, rows and dropped rows")
    ap.add_argument("--self-test", action="store_true", help="the unit tests and mutants, with no boot")
    return ap
