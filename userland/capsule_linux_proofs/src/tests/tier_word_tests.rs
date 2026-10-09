// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! The kernel's tier allowlist, which every `qwen` request from a terminal
//! is checked against, a run on the terminal or a window: each word it
//! takes names a tier this personality ships, so a request the kernel
//! admits never reaches `run` as a name nothing answers to.

use crate::models::apps::{all as apps, app};

/*
 * Compiled from the kernel's own file, not a copy of its table.
 */
#[path = "../../../../src/userspace/capsule_linux/terminal/tier.rs"]
mod kernel_tier;

use kernel_tier::{argv, package, parse};

#[test]
fn every_word_the_kernel_takes_is_a_shipped_tier() {
    let mut i = 0u8;
    while let Some(name) = package(i) {
        assert!(app(&name).is_some(), "{name} is not shipped");
        assert_eq!(parse(&name.as_bytes()["qwen-".len()..]), Some(i), "{name}");
        assert_eq!(argv(i).unwrap(), ["run", name.as_str(), "cli"]);
        i += 1;
    }
    assert_eq!(usize::from(i), apps().count());
    assert_eq!(parse(b""), Some(0));
    assert_eq!(parse(b"small\0\0"), Some(0));
    assert_eq!(parse(b"window"), None);
}

/*
 * A tier from the word a person types to the model it opens, for the
 * smallest tier of each Qwen family (Qwen3 0.6B is the stick tier, which
 * answers offline), a mid tier and the everyday Qwen3 4B: `qwen <word>` on
 * the terminal (the word setup keeps in the policy store), `qwen window
 * <word>`, and the Store's `linux.qwen-<word>`, whose Open the kernel
 * reads with `package_arg`, reach the same shipped tier, the same pinned
 * model and a chat build this CPU runs.
 */
#[test]
fn each_way_to_ask_reaches_the_tier_and_its_pinned_model() {
    use crate::console::run_mode::{parse as run_request, Mode};
    use crate::models::apps::CHAT;
    use crate::models::pinned::all as pins;
    for word in ["small", "qwen3-0.6b", "qwen3-1.7b", "qwen3-4b"] {
        let index = parse(word.as_bytes()).expect(word);
        let name = package(index).unwrap();
        assert_eq!(name, alloc::format!("qwen-{word}"));
        // The Store's listing names the same tier.
        let listing = alloc::format!("linux.{name}");
        let opened = crate::listing_family::package_arg(listing.strip_prefix("linux.").unwrap());
        assert_eq!(opened.as_deref(), Some(name.as_str()), "{word}");
        let tier = app(listing.strip_prefix("linux.").unwrap()).expect(word);
        // A chat build this CPU runs comes first: v3 on hardware with AVX2,
        // v2 without, the plain build under QEMU's software CPU.
        use crate::models::chat_pick::{pick, CHAT_PLAIN, CHAT_V2};
        assert_eq!(pick(tier.program, false, true)[0].0, CHAT, "{word}");
        assert_eq!(pick(tier.program, false, false)[0].0, CHAT_V2, "{word}");
        assert_eq!(pick(tier.program, true, true)[0].0, CHAT_PLAIN, "{word}");
        assert_eq!((tier.tier(), tier.program), (word, CHAT));
        // Every file it needs is pinned under it, and none is left out.
        let pinned: alloc::vec::Vec<_> =
            pins().filter(|p| p.tier == word).map(|p| p.name).collect();
        assert_eq!(tier.models, &pinned[..], "{word}");
        let model = [&b"/models"[..], tier.models[0]].concat();
        // On the terminal: the run the kernel asks for, without the window.
        let asked = argv(index).unwrap().join("\0");
        let (asked, mode) = run_request(asked.as_bytes()).expect(word);
        assert_eq!((asked.as_str(), mode), (name.as_str(), Mode::Cli));
        assert_eq!(mode.tier_args(tier.args), [b"-m".to_vec(), model.clone()]);
        // In a window, as `qwen window` and the Store open it.
        let window = Mode::Window.tier_args(tier.args);
        assert!(window.windows(2).any(|w| w[0] == b"-ui" && w[1] == b"window"), "{word}");
        assert!(window.windows(2).any(|w| w[0] == b"-m" && w[1] == model), "{word}");
    }
}
