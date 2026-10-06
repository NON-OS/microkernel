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

//! The rig the join proofs run on: the modeled device booted to ALIVE with
//! a transmit region, a firmware responder that answers the join's commands
//! (session protection with its notification, as the firmware sends it),
//! and a scripted access point that runs Open System or SAE authentication,
//! association, the four-way handshake and, once keyed, CCMP data, built
//! from the same shared core the station runs (its SAE, key derivation and
//! the authenticator messages of `nonos_wifi_core_proofs`). Each refusal and
//! silence the proofs need is a switch on the access point or the responder.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt, CCMP_HDR_LEN, MIC_LEN};
use nonos_wifi_core::dot11::data::protect;
use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};
use nonos_wifi_core::eapol::parse::{parse as parse_key, KEY_INFO_PAIRWISE, KEY_INFO_SECURE};
use nonos_wifi_core::mlme::{Entropy, JoinRequest, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeStation, SaeStep};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;

use crate::ap_sim::{gtk_kde, message1, message3, AP_RSNE_MIXED, GTK};
use crate::gen3::bringup::boot;
use crate::gen3::dev::Dev;
use crate::gen3::dram_map::classify;
use crate::gen3::join::fw::Fw;
use crate::gen3::join::inbox::{Inbox, Queues};
use crate::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::gen3::plan::{control_len, RB_REGION};
use crate::gen3::txq::{TxQueue, TXQ_STRIDE, TX_REGION};
use crate::gen3::ucode::Ucode;
use crate::gen3_model::{reply, Mem, Model, Out, Polls, Seen, TxModel};

pub static SO_GF: &[u8] = include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

pub const STA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub const AP: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
pub const SSID: &[u8] = b"home";
pub const PASS: &[u8] = b"correct horse";
pub const SNONCE: [u8; 32] = [0x52; 32];
pub const ANONCE: [u8; 32] = [0xA1; 32];
pub const TX_DEV: u64 = 0x6000_0000;

/// The WPA2-PSK RSNE an access point without WPA3 beacons.
pub const RSNE_PSK: [u8; 22] = [48, 20, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 2, 0, 0];

pub fn entropy() -> Entropy {
    let mut sae = [0u8; SAE_ENTROPY];
    for (i, b) in sae.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    Entropy { snonce: SNONCE, sae }
}

pub fn request(policy: JoinPolicy) -> JoinRequest<'static> {
    JoinRequest { our_mac: STA, ssid: SSID, passphrase: PASS, policy, entropy: entropy() }
}

/// A beacon from the access point on channel 6 with these elements after
/// the SSID, the 802.11b/g rates (CCK basic) and the DS parameter.
pub fn beacon(elements: &[&[u8]]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, 8), [0xFF; 6], AP, AP, 0).unwrap();
    f.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&[0x64, 0, 0x31, 0x04]); // 100 TU; ESS, privacy, short preamble
    f.extend_from_slice(&[0, SSID.len() as u8]);
    f.extend_from_slice(SSID);
    f.extend_from_slice(&[1, 8, 0x82, 0x84, 0x8B, 0x96, 0x0C, 0x12, 0x18, 0x24]);
    f.extend_from_slice(&[3, 1, 6]);
    f.extend_from_slice(&[5, 4, 0, 2, 0, 0]); // TIM: DTIM period 2
    for e in elements {
        f.extend_from_slice(e);
    }
    f
}

/// An RX MPDU notification carrying `frame`, with the firmware's status.
pub fn rx(frame: &[u8], status: u32) -> Out {
    let mut p = vec![0u8; 64];
    p[0..2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    p[12..16].copy_from_slice(&(status | 0x3).to_le_bytes());
    p.extend_from_slice(frame);
    Out { cmd: 0xC1, group: 0, seq: 0x8000, payload: p }
}

fn mgmt(subtype: u8, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, subtype), STA, AP, AP, 0).unwrap();
    f.extend_from_slice(body);
    f
}

/// A data frame from the access point to the station, in the clear.
pub fn from_ap(payload_with_snap: &[u8], seq: u16) -> Vec<u8> {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&STA);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&(seq << 4).to_le_bytes());
    f.extend_from_slice(payload_with_snap);
    f
}

pub fn eapol_from_ap(eapol: &[u8], seq: u16) -> Vec<u8> {
    let mut body = vec![0xAA, 0xAA, 0x03, 0, 0, 0, 0x88, 0x8E];
    body.extend_from_slice(eapol);
    from_ap(&body, seq)
}

/// What the access point is to do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Security {
    Wpa2,
    Sae { h2e: bool },
}

#[derive(Default)]
pub struct Switches {
    /// Answer authentication with this status.
    pub refuse_auth: Option<u16>,
    /// Answer association with this status.
    pub refuse_assoc: Option<u16>,
    /// Answer nothing at all.
    pub silent: bool,
    /// Answer authentication, then nothing.
    pub silent_after_auth: bool,
    /// Answer association (and message 1), then nothing.
    pub silent_after_m1: bool,
    /// Deauthenticate (reason 15) instead of sending message 3.
    pub deauth_instead_of_m3: bool,
    /// Run SAE with another password.
    pub wrong_password: bool,
    /// Key the four-way handshake with another PMK (a wrong WPA2 passphrase).
    pub wrong_pmk: bool,
    /// Deliver protected frames as the firmware decrypted them.
    pub fw_decrypts: bool,
    /// After message 1, a message 1 from another BSS.
    pub stray_m1: bool,
    /// Lose the first authentication frame, as a busy channel does.
    pub drop_first_auth: bool,
}

/// The scripted access point.
pub struct Ap {
    pub sec: Security,
    pub sw: Switches,
    pub rsne: Vec<u8>,
    pub rsnxe: Option<Vec<u8>>,
    sae: Option<SaeStation>,
    pub pmk: [u8; 32],
    pub ptk: [u8; 48],
    pub keyed: bool,
    pub seq: u16,
    pub pn: u64,
    /// The last protected frame it sent.
    pub last: Option<Vec<u8>>,
    /// Plaintext bodies (from LLC/SNAP) of protected frames it decrypted.
    pub got: Vec<Vec<u8>>,
    /// Every frame the station sent it.
    pub heard: Vec<Vec<u8>>,
    pub deauths: Vec<Vec<u8>>,
    pub eapol_replies: Vec<Vec<u8>>,
    dropped_auth: bool,
}

impl Ap {
    pub fn new(sec: Security, sw: Switches) -> Self {
        let (rsne, rsnxe) = match sec {
            Security::Wpa2 => (RSNE_PSK.to_vec(), None),
            Security::Sae { h2e } => (AP_RSNE_MIXED.to_vec(), h2e.then(|| vec![244, 1, 0x20])),
        };
        Self {
            sec,
            sw,
            rsne,
            rsnxe,
            sae: None,
            pmk: [0; 32],
            ptk: [0; 48],
            keyed: false,
            seq: 0,
            pn: 0,
            last: None,
            got: Vec::new(),
            heard: Vec::new(),
            deauths: Vec::new(),
            eapol_replies: Vec::new(),
            dropped_auth: false,
        }
    }

    pub fn beacon(&self) -> Vec<u8> {
        let mut els: Vec<&[u8]> = vec![&self.rsne];
        if let Some(x) = &self.rsnxe {
            els.push(x);
        }
        beacon(&els)
    }

    fn akm(&self) -> Akm {
        match self.sec {
            Security::Wpa2 => Akm::Psk,
            Security::Sae { .. } => Akm::Sae,
        }
    }

    pub fn tk(&self) -> [u8; 16] {
        self.ptk[32..48].try_into().unwrap()
    }

    fn next_seq(&mut self) -> u16 {
        self.seq = self.seq.wrapping_add(1) & 0x0FFF;
        self.seq
    }

    /// Deliver `frame` as the firmware would: protected frames decrypted
    /// (CCMP header kept, MIC gone) when the firmware has the key.
    pub fn deliver(&self, frame: &[u8]) -> Out {
        let protected = frame[1] & 0x40 != 0;
        if !(protected && self.sw.fw_decrypts && self.keyed) {
            return rx(frame, 0);
        }
        let plain = decrypt(frame, &self.tk()).expect("the access point's own frame decrypts");
        let mut f = frame[..MAC_HEADER_LEN + CCMP_HDR_LEN].to_vec();
        f.extend_from_slice(&plain);
        rx(&f, (2 << 8) | (1 << 6))
    }

    /// A protected data frame to the station carrying `snap_body`.
    pub fn protected(&mut self, snap_body: &[u8]) -> Out {
        let seq = self.next_seq();
        let clear = from_ap(snap_body, seq);
        self.pn += 1;
        let f = protect(&clear, self.pn, &self.tk()).unwrap();
        self.last = Some(f.clone());
        self.deliver(&f)
    }

    /// The last protected frame again, as a replay.
    pub fn deliver_replay(&self) -> Out {
        self.deliver(self.last.as_ref().expect("a frame was sent"))
    }

    /// A management frame protected under the pairwise key.
    pub fn protected_mgmt(&mut self, clear: &[u8]) -> Out {
        self.pn += 1;
        let f = encrypt(clear, self.pn, &self.tk()).unwrap();
        self.deliver(&f)
    }

    /// Answer one frame the station sent.
    pub fn on_tx(&mut self, f: &[u8]) -> Vec<Out> {
        self.heard.push(f.to_vec());
        if self.sw.silent {
            return vec![];
        }
        match f[0] {
            0xB0 => self.on_auth(f),
            0x00 => self.on_assoc(),
            0xC0 => {
                self.deauths.push(f.to_vec());
                vec![]
            }
            0x08 => self.on_data(f),
            _ => vec![],
        }
    }

    fn on_auth(&mut self, f: &[u8]) -> Vec<Out> {
        if self.sw.drop_first_auth && !self.dropped_auth {
            self.dropped_auth = true;
            return vec![];
        }
        let b = &f[MAC_HEADER_LEN..];
        let (alg, seq, status) =
            (u16::from_le_bytes([b[0], b[1]]), u16::from_le_bytes([b[2], b[3]]), u16::from_le_bytes([b[4], b[5]]));
        if let Some(code) = self.sw.refuse_auth {
            return vec![rx(&mgmt(11, &[b[0], b[1], 2, 0, code as u8, (code >> 8) as u8]), 0)];
        }
        let out = match (alg, seq) {
            (0, 1) => {
                self.pmk = pmk(if self.sw.wrong_pmk { b"another passphrase" } else { PASS }, SSID);
                vec![rx(&mgmt(11, &[0, 0, 2, 0, 0, 0]), 0)]
            }
            (3, 1) => {
                let Security::Sae { h2e } = self.sec else { return vec![] };
                let pass: &[u8] = if self.sw.wrong_password { b"not the password" } else { PASS };
                let pwe = if h2e {
                    pwe_from_pt(&derive_pt(SSID, pass, None).unwrap(), &AP, &STA).unwrap()
                } else {
                    derive_pwe(pass, &vec![9u8; pass.len()], &AP, &STA).unwrap()
                };
                // As hostapd does: its commit, then its confirm at once.
                let mut ap = SaeStation::start(pwe, h2e, &[0x33; 32], &[0x44; 32]).unwrap();
                let SaeStep::Send { body: confirm, .. } = ap.on_frame(1, status, &b[6..]) else { return vec![] };
                let (_, s, commit) = ap.commit_frame();
                self.sae = Some(ap);
                let mut body = vec![3, 0, 1, 0];
                body.extend_from_slice(&s.to_le_bytes());
                body.extend_from_slice(&commit);
                let mut conf = vec![3, 0, 2, 0, 0, 0];
                conf.extend_from_slice(&confirm);
                vec![rx(&mgmt(11, &body), 0), rx(&mgmt(11, &conf), 0)]
            }
            (3, 2) => {
                let Some(ap) = self.sae.as_mut() else { return vec![] };
                if matches!(ap.on_frame(2, status, &b[6..]), SaeStep::Accepted) {
                    self.pmk = ap.pmk().unwrap().0;
                }
                vec![]
            }
            _ => vec![],
        };
        if self.sw.silent_after_auth {
            self.sw.silent = true;
        }
        out
    }

    fn on_assoc(&mut self) -> Vec<Out> {
        let status = self.sw.refuse_assoc.unwrap_or(0);
        let mut body = vec![0x31, 0x04];
        body.extend_from_slice(&status.to_le_bytes());
        body.extend_from_slice(&0xC001u16.to_le_bytes());
        let mut out = vec![rx(&mgmt(1, &body), 0)];
        if status == 0 {
            let s = self.next_seq();
            out.push(rx(&eapol_from_ap(&message1(self.akm(), 1, &ANONCE), s), 0));
        }
        if self.sw.stray_m1 {
            let mut other = eapol_from_ap(&message1(self.akm(), 2, &[0xB2; 32]), 7);
            other[10] ^= 0x80;
            other[16] ^= 0x80;
            out.push(rx(&other, 0));
        }
        if self.sw.silent_after_m1 {
            self.sw.silent = true;
        }
        out
    }

    fn on_data(&mut self, f: &[u8]) -> Vec<Out> {
        let body = if f[1] & 0x40 != 0 {
            let Some(plain) = decrypt(f, &self.tk()) else { return vec![] };
            plain
        } else {
            f[MAC_HEADER_LEN..].to_vec()
        };
        if body.len() < 8 || body[6..8] != [0x88, 0x8E] {
            if f[1] & 0x40 != 0 {
                self.got.push(body);
            }
            return vec![];
        }
        let eapol = &body[8..];
        self.eapol_replies.push(eapol.to_vec());
        let Some(k) = parse_key(eapol) else { return vec![] };
        if k.key_info & KEY_INFO_PAIRWISE == 0 {
            return vec![]; // a group message 2
        }
        if k.key_info & KEY_INFO_SECURE != 0 {
            self.keyed = true; // message 4
            return vec![];
        }
        if self.sw.deauth_instead_of_m3 {
            return vec![rx(&mgmt(12, &[15, 0]), 0)];
        }
        self.ptk = self.akm().derive_ptk(&self.pmk, &AP, &STA, &ANONCE, &k.nonce);
        let mut kd = self.rsne.clone();
        if let Some(x) = &self.rsnxe {
            kd.extend_from_slice(x);
        }
        kd.extend_from_slice(&gtk_kde(1, &GTK));
        let s = self.next_seq();
        vec![rx(&eapol_from_ap(&message3(self.akm(), 2, &ANONCE, &self.ptk, &kd), s), 0)]
    }
}

/// The modeled device, booted, with its transmit region.
pub struct Rig {
    pub model: Model,
    pub tx: Rc<Mem>,
    pub ap: Rc<RefCell<Ap>>,
    fw_regions: Vec<Rc<Mem>>,
}

/// The session protection notification the firmware sends.
pub fn session_notif(status: u32, start: u32) -> Out {
    let p: Vec<u8> = [0u32, status, start, 0].iter().flat_map(|w| w.to_le_bytes()).collect();
    Out { cmd: 0xFB, group: 0x3, seq: 0x8000, payload: p }
}

/// How the responder answers session protection.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SessionReply {
    Start,
    Refuse,
    Never,
}

pub fn rig(sec: Security, sw: Switches) -> Rig {
    rig_with(sec, sw, SessionReply::Start)
}

pub fn rig_with(sec: Security, sw: Switches, session: SessionReply) -> Rig {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let ctrl = Mem::new(control_len(ucode.iml.len()).unwrap(), 0x1000_0000);
    let rbs = Mem::new(RB_REGION, 0x2000_0000);
    let sizes = firmware_region_sizes(&classify(SO_GF)).unwrap();
    let fw_regions =
        sizes.iter().enumerate().map(|(i, &n)| Mem::new(n, 0x4000_0000 + (i as u64) * 0x10_0000)).collect();
    let model = Model::new(ctrl, rbs);
    let tx = Mem::new(TX_REGION, TX_DEV);
    let ap = Rc::new(RefCell::new(Ap::new(sec, sw)));
    let mut t = TxModel::new(tx.clone());
    let air_ap = ap.clone();
    t.air = Some(Box::new(move |f: &[u8]| air_ap.borrow_mut().on_tx(f)));
    model.s.borrow_mut().txq = Some(t);
    model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| {
        let action = c.payload.get(4).copied().unwrap_or(0);
        match (c.group, c.cmd, action) {
            (0x3, 0x05, 1) => match session {
                SessionReply::Start => vec![reply(c, vec![]), session_notif(1, 1)],
                SessionReply::Refuse => vec![reply(c, vec![]), session_notif(0, 0)],
                SessionReply::Never => vec![reply(c, vec![])],
            },
            _ => vec![reply(c, vec![])],
        }
    }));
    Rig { model, tx, ap, fw_regions }
}

/// The station's side of the rig: the transport, the queues and the inbox.
pub struct Station {
    pub q: Queues,
    pub inbox: Inbox,
}

impl Station {
    pub fn new() -> Self {
        Self { q: Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) }, inbox: Inbox::new() }
    }
}

/// Boot the rig to ALIVE and run `f` with the firmware the join drives.
pub fn with_fw(r: &Rig, st: &mut Station, f: impl FnOnce(&mut Fw<'_, '_, Model, Mem, Polls>)) {
    let ucode = Ucode::parse(SO_GF).unwrap();
    let layout = classify(SO_GF);
    let fw: Vec<&Mem> = r.fw_regions.iter().map(|m| m.as_ref()).collect();
    let mem = Memory { ctrl: r.model.ctrl.as_ref(), rbs: r.model.rbs.as_ref(), fw: &fw };
    let mut dev = Dev::new(&r.model, r.model.ctrl.as_ref(), r.model.rbs.as_ref());
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    boot(&mut dev, &mut c, &mem, &layout, &ucode, &Board { hw_rev: 0x370, imr_enabled: false, rf_id: 0x10D000, pnvm: None }).expect("alive");
    r.model.s.borrow_mut().commands.clear();
    let mut fw = Fw { dev: &mut dev, tx: r.tx.as_ref(), clock: &mut c, q: &mut st.q, inbox: &mut st.inbox };
    f(&mut fw);
}

impl Rig {
    /// Every command since boot, as (group, opcode).
    pub fn commands(&self) -> Vec<(u8, u8)> {
        self.model.s.borrow().commands.iter().map(|c| (c.group, c.cmd)).collect()
    }

    pub fn command_payloads(&self, group: u8, cmd: u8) -> Vec<Vec<u8>> {
        self.model.s.borrow().commands.iter().filter(|c| (c.group, c.cmd) == (group, cmd)).map(|c| c.payload.clone()).collect()
    }

    /// Every frame the station queued.
    pub fn sent(&self) -> Vec<crate::gen3_model::Sent> {
        self.model.s.borrow().txq.as_ref().unwrap().sent.clone()
    }

    pub fn bc_mismatch(&self) -> u32 {
        self.model.s.borrow().txq.as_ref().unwrap().bc_mismatch
    }
}

/// Whether `f` is an EAPOL frame to the access point in the clear.
pub fn is_clear_eapol(f: &[u8]) -> bool {
    f[0] == 0x08 && f[1] & 0x40 == 0 && f.len() >= 32 && f[24..30] == [0xAA, 0xAA, 0x03, 0, 0, 0] && f[30..32] == [0x88, 0x8E]
}

/// The station's protected data frame decrypts under `tk` to a body.
pub fn decrypts(f: &[u8], tk: &[u8; 16]) -> bool {
    f[1] & 0x40 != 0 && decrypt(f, tk).is_some() && f.len() > MAC_HEADER_LEN + CCMP_HDR_LEN + MIC_LEN
}
