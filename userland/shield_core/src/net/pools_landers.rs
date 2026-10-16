//! The production pool's landers: Tor onions off NONOS, Anyone onion services on it.

/// The production landers, tried in this order, each given 20 seconds. The first was confirmed by a
/// transfer that settled through it, and each serves the production pool with the same API.
#[cfg(not(target_vendor = "nonos"))]
pub(super) const LANDERS: [&str; 5] = [
    "mforujillfk4w5h2zqgxdzendn3qb2zes4r2h57ownojshhmtcdzgdyd.onion",
    "lfpw5uoslfiqmwsc7o2d2qts3grmfnixrhae3hkgkv6x4dlxpvafp4yd.onion",
    "7ywwruseitmycfjwh2upbecetm6edej7pkxpp4xtzmtioes4ubs63kad.onion",
    "g7uyxmffgim7sneprrkdp4ecshupvd3kuaadho6f7e2gcd3gazimriyd.onion",
    "ewjq3ue43pclh6qyx3triup7org64nykbjgy7tmderlzzx27kstfdiqd.onion",
];
/// On NONOS a lander is reached as its Anyone onion service, `<56 letters>.anyone`, beside its Tor
/// onion; no network NONOS runs reaches a Tor onion. These are the four standbys, at 60, 120, 180
/// and 240 s into each root slot, in the STARK lane's order; the primary joins first
/// when its machine is back. Each still has to name this pool in /v1/info and take the
/// submitter's fee before it is handed a proof, and a handoff is followed on the lander that took
/// it. anon_ntor_proofs holds every one to the .anyone checksum.
#[cfg(target_vendor = "nonos")]
pub(super) const LANDERS: [&str; 4] = [
    "iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone",
    "5swdytwdtcspbntuofrknlqa7ls2nw2zol2o6tts2jv5hs67n4ee5nad.anyone",
    "ywhgp672zlndlpjubaq23yhbuy526335jfgl2zr74emhgoepovh56lid.anyone",
    "wusdfhdpumeamiyex3v625zled37m2lw24x4bxr3mngu75ryz7vurvad.anyone",
];
pub const LANDER_ON: bool = true;
