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

"""tools/nonos-dev-image adds a copy-only feature to nonos.toml's features
line once, keeps what is there, touches nothing else, and refuses a file
with no features line. The nsh switch asks for both nsh capsules and the
devkit's records beside the copy; no build bakes an identity in. --offline
clears the package mirror and the source recipes and nothing else; without
it the package mirror is the host's relay and nothing else moves. Nothing
here builds or copies anything."""

import importlib.machinery
import importlib.util
import os
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
TOOL = os.path.join(os.path.dirname(HERE), "nonos-dev-image")


def load():
    loader = importlib.machinery.SourceFileLoader("nonos_dev_image", TOOL)
    spec = importlib.util.spec_from_loader("nonos_dev_image", loader)
    mod = importlib.util.module_from_spec(spec)
    loader.exec_module(mod)
    return mod


class Features(unittest.TestCase):
    def setUp(self):
        self.t = load()

    def test_added_once_after_what_is_there(self):
        text = 'profile = "standard"\nfeatures = ["a"]\nsmp = true\n'
        once = self.t.with_features(text, ["b", "c"])
        self.assertEqual(once, 'profile = "standard"\nfeatures = ["a", "b", "c"]\nsmp = true\n')
        self.assertEqual(self.t.with_features(once, ["b", "c"]), once)

    def test_an_empty_line_takes_them(self):
        self.assertEqual(self.t.with_features("features = []\n", ["x"]), 'features = ["x"]\n')

    def test_no_features_line_is_refused(self):
        self.assertIsNone(self.t.with_features('profile = "standard"\n', ["x"]))

    def test_nsh_asks_for_both_capsules(self):
        self.assertEqual(set(self.t.NSH_FEATURES), {"nonos-capsule-nsh-front", "nonos-capsule-nsh-attest"})
        for d in (self.t.NSH_CLIENT, self.t.NSH_RECORDS):
            self.assertFalse(str(d).startswith(str(self.t.TREE)), "devkit files stay out of the copy")
        self.assertFalse(hasattr(self.t, "NSH_IDENTITY"), "no identity is baked")

    def test_one_run_writes_both_halves(self):
        plain = self.t.devkit_args()
        self.assertEqual(plain, [str(self.t.NSH_RECORDS), str(self.t.NSH_CLIENT)])
        live = self.t.devkit_args("req", "ab" * 32)
        self.assertEqual((live[0], live[1:3], live[5]), ("enrol", plain, "ab" * 32))
        self.assertTrue(live[3].endswith("/req/host.public") and os.path.isabs(live[3]))
        self.assertTrue(live[4].endswith("/req/enrol.doc"), "the machine's own request")

    def nsh_run(self, on):
        """What nsh() asks for with the switch `on`, nothing run."""
        done = []
        self.t.sh = lambda *argv, **kw: done.append(("sh", argv))
        self.t.commit = lambda: done.append(("commit",))
        self.t.add_features = lambda names, what: done.append(("features", tuple(names)))
        self.t.nsh(on)
        return done

    def test_no_build_bakes_an_identity(self):
        self.assertEqual(self.nsh_run(False), [], "without the switch nothing is run")
        done = self.nsh_run(True)
        runs = [step[1] for step in done if step[0] == "sh"]
        self.assertEqual(len(runs), 1)
        self.assertIn("nsh-devkit", runs[0], "the devkit writes the records beside the copy")
        self.assertNotIn("git", [r[0] for r in runs], "nothing is added to the copy")

    def test_only_the_switch_puts_the_capsules_in_the_kernel(self):
        self.assertNotIn("features", [step[0] for step in self.nsh_run(False)])
        self.assertIn(("features", self.t.NSH_FEATURES), self.nsh_run(True))

    def test_the_checkouts_nonos_toml_has_a_line_to_add_to(self):
        root = os.path.dirname(os.path.dirname(HERE))
        text = open(os.path.join(root, "nonos.toml")).read()
        self.assertIsNotNone(self.t.with_features(text, list(self.t.NSH_FEATURES)))


class Offline(unittest.TestCase):
    def setUp(self):
        self.t = load()

    def test_both_keys_are_cleared_and_nothing_else(self):
        text = ('profile = "standard"\nlinux_packages = "mirror:80"\nlinux_sources = "t/s"\n'
                'linux_debian = "kali:80"\n[store]\nlinux = true\n')
        self.assertEqual(self.t.offline(text),
                         'profile = "standard"\nlinux_packages = ""\nlinux_sources = ""\n'
                         'linux_debian = "kali:80"\n[store]\nlinux = true\n')

    def test_a_file_missing_either_key_is_refused(self):
        self.assertIsNone(self.t.offline('linux_packages = "m:80"\n'))
        self.assertIsNone(self.t.offline('linux_sources = "t"\n'))
        self.assertIsNone(self.t.offline('[store]\nlinux_packages = "m"\nlinux_sources = "t"\n'))

    def test_the_checkouts_nonos_toml_can_go_offline(self):
        root = os.path.dirname(os.path.dirname(HERE))
        text = open(os.path.join(root, "nonos.toml")).read()
        out = self.t.offline(text)
        self.assertIsNotNone(out)
        self.assertIn('linux_sources = ""\n', out)
        self.assertIn('linux_packages = ""\n', out)


class Relay(unittest.TestCase):
    def setUp(self):
        self.t = load()

    def test_the_package_mirror_is_the_hosts_relay_and_nothing_else_moves(self):
        text = ('profile = "standard"\nlinux_packages = "mirror:80"\nlinux_sources = "t/s"\n'
                'linux_debian = "kali:80"\n[store]\nlinux = true\n')
        self.assertEqual(self.t.to_relay(text),
                         'profile = "standard"\nlinux_packages = "10.0.2.2:8080"\nlinux_sources = "t/s"\n'
                         'linux_debian = "kali:80"\n[store]\nlinux = true\n')

    def test_no_mirror_is_made_up(self):
        for text in ('linux_packages = ""\nlinux_sources = ""\n', 'profile = "qemu"\n',
                     '[store]\nlinux_packages = "m:80"\n'):
            self.assertEqual(self.t.to_relay(text), text)

    def test_the_checkouts_nonos_toml_points_at_the_relay(self):
        root = os.path.dirname(os.path.dirname(HERE))
        text = open(os.path.join(root, "nonos.toml")).read()
        out = self.t.to_relay(text)
        self.assertIn('linux_packages = "10.0.2.2:8080"\n', out)
        self.assertEqual(self.t.offline(out), self.t.offline(text), "only that line moved")


if __name__ == "__main__":
    unittest.main()
