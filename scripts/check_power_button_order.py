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
"""Hold the power button's arming to after device interrupt routing.

Arming masks the SCI's IO-APIC line and then sets PWRBTN_EN. The IO-APIC's
chips are recorded only by init_from_acpi (init_broker_irq_routing), and the
SCI's record is made earlier, at the broker's seeding. Armed from there the
mask is refused and the button stays dead (551520c30, P-05). So:

  1. register_sci (src/hardware/broker/acpi/sci.rs) never arms.
  2. src/arch/init_broker_irq_routing.rs calls arm_power_button() only in
     the Ok arm of the result init_from_acpi() returned.

    scripts/check_power_button_order.py [--self-test]
"""

import re
import sys
from pathlib import Path

SCI = "src/hardware/broker/acpi/sci.rs"
ROUTING = "src/arch/init_broker_irq_routing.rs"
ORDER = re.compile(r"=\s*crate::arch::x86_64::interrupt::ioapic::init_from_acpi\(\);\s*match routed \{\s*"
                   r"Ok\(_\)\s*=>\s*crate::hardware::broker::acpi::arm_power_button\(\)")


def register_sci_body(text):
    start = text.find("pub fn register_sci(")
    return text[start:text.find("\n}\n", start)] if start >= 0 else None


def check(sci, routing):
    failed = []
    body = register_sci_body(sci)
    if body is None:
        failed.append(f"{SCI}: register_sci not found")
    elif "arm" in re.sub(r"//[^\n]*", "", body).replace("alarm", ""):
        failed.append(f"{SCI}: register_sci arms the button, before routing is up")
    if not ORDER.search(re.sub(r"//[^\n]*", "", routing)):
        failed.append(f"{ROUTING}: arm_power_button() is not the Ok arm of init_from_acpi()")
    return failed


def self_test(sci, routing):
    early = sci.replace("    Some(id)\n}", "    fw::power_button::arm(line, false);\n    Some(id)\n}", 1)
    cases = [
        (check(sci, routing), True),
        (check(early, routing), False),
        (check(sci, routing.replace("Ok(_) => crate::hardware::broker::acpi::arm_power_button()", "Ok(_) => {}")), False),
        (check(sci, "crate::hardware::broker::acpi::arm_power_button();\n" + routing.replace("arm_power_button()", "x()")), False),
    ]
    wrong = [i for i, (failed, ok) in enumerate(cases) if (not failed) != ok]
    print(f"power-button-order: self-test {'failed on ' + str(wrong) if wrong else 'passed'}, "
          f"{sum(not ok for _, ok in cases)} mutants rejected")
    return 1 if wrong else 0


def main():
    sci, routing = Path(SCI).read_text(), Path(ROUTING).read_text()
    if "--self-test" in sys.argv[1:]:
        return self_test(sci, routing)
    failed = check(sci, routing)
    for line in failed:
        print(f"power-button-order: {line}")
    if not failed:
        print("power-button-order: the button is armed once routing is up, and only there")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
