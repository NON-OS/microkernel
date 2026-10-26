# Item 6 mutants

`mutants.py [names]` breaks each vendor rule once, runs the whole
hda_proofs crate and puts the file back. Sixteen rules, sixteen killed. C2
survived the first run: the Cirrus test compared the verbs sent with the
driver's own table, so a wrong value there passed. `cirrus_coef_tests.rs`
now checks the coefficients the codec holds against cs420x.c's values,
written out in the test, and C2, C6 and C7 are killed.

| mutant | file | result | killed by |
|---|---|---|---|
| V1 the HP rule for every vendor | `vendor_gpio.rs` | KILLED | a_cs4206_gets_its_errata_and_coefficients_in_order_then_its_mac_gpios, a_cs4208_gets_its_own_sequence_and_gpio0 |
| V2 the iMac GPIO2 rule dropped | `vendor_gpio.rs` | KILLED | a_cirrus_codec_in_a_mac_drives_its_amplifier_gpios |
| V3 the NVIDIA Macs dropped | `vendor_gpio.rs` | KILLED | a_cirrus_codec_in_a_mac_drives_its_amplifier_gpios |
| V4 the MacBook Air 7,2 dropped | `vendor_gpio.rs` | KILLED | a_cirrus_codec_in_a_mac_drives_its_amplifier_gpios |
| V5 GPIOs past the codec's own driven | `vendor_gpio.rs` | KILLED | no_gpio_outside_those_machines_or_past_the_codecs_own |
| C1 CS4206 given CS4208's sequence | `cirrus.rs` | KILLED | a_cs4206_gets_its_errata_and_coefficients_in_order_then_its_mac_gpios |
| C3 CS4208 not set up | `cirrus.rs` | KILLED | a_cs4208_gets_its_own_sequence_and_gpio0 |
| C4 coefficients sent as short verbs | `cirrus.rs` | KILLED | a_cs4206_gets_its_errata_and_coefficients_in_order_then_its_mac_gpios, a_cs4208_gets_its_own_sequence_and_gpio0 |
| C5 cirrus never run | `vendor.rs` | KILLED | a_cs4206_gets_its_errata_and_coefficients_in_order_then_its_mac_gpios, a_cs4208_gets_its_own_sequence_and_gpio0 |
| I1 power-map rule inverted | `idt.rs` | KILLED | only_a_port_whose_pin_says_nothing_is_there_is_powered_down, the_map_is_written_to_the_function_group |
| I2 92HD71 given 92HD83's ports | `idt.rs` | KILLED | each_family_has_its_ports_and_no_other_codec_has_a_map, only_a_port_whose_pin_says_nothing_is_there_is_powered_down |
| I3 idt never run | `vendor.rs` | KILLED | the_map_is_written_to_the_function_group |
| I4 the map sent to the root node | `idt.rs` | KILLED | the_map_is_written_to_the_function_group |
| C2 the DAC coefficient wrong | `cirrus_verbs.rs` | KILLED | a_cs4206_holds_the_errata_and_init_values |
| C6 an errata value wrong | `cirrus_verbs.rs` | KILLED | a_cs4206_holds_the_errata_and_init_values |
| C7 CS4208 coefficient wrong | `cirrus_verbs.rs` | KILLED | a_cs4208_holds_its_two |
