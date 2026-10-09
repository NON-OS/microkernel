// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that the RTL8821CE `RtlLink` satisfies the shared `LinkPort` contract
//! net_core drives. An unassociated link reports down, offers no MAC and refuses
//! to transmit; once associated it reports its MAC and link up. A transmitted
//! Ethernet frame is framed by the station and laid into the TX ring with the
//! hardware CCMP security type set, so the radio encrypts from the CAM, and the
//! write-index doorbell is rung. A received 802.11 MPDU is pulled off the RX ring
//! and parsed back to the exact Ethernet frame that produced it. Driven through
//! the real `nonos_wifi_core::netif::LinkPort` trait against modeled DMA and a
//! modeled card; only the card actually moving bytes is left for on-silicon
//! bring-up.
//!
//! The staged receive frame is what this chip hands up (rtw88 reads the same
//! layout): the access point's FromDS frame, Protected bit still set, decrypted
//! in place with the CCMP header and MIC left on, the 4-byte FCS after it, and
//! a descriptor whose security type says AES and whose SWDEC bit is clear. An
//! earlier version of this fixture staged an unprotected station-to-AP frame,
//! which the link now refuses as it must (`an_unprotected_frame_is_refused_*`).

use std::cell::RefCell;
use std::rc::Rc;

use crate::fw::dma::DmaMem;
use crate::link::{RtlLink, RxBuffers};
use crate::regs::Mmio;
use crate::rx::regs::REG_RXBD_IDX_MPDUQ;
use crate::rx::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::tx::desc::TXDESC_LEN;
use crate::tx::regs::{REG_TXBD_IDX_BEQ, TRX_BD_HW_IDX_SHIFT};

use crate::ap_sim::{gtk_kde, group1, message1, message3, ANONCE, AP_RSNE_MIXED, GTK, SNONCE};
use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt};
use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

const STA_MAC: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
const AP_MAC: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
const PTK: [u8; 16] = [0x5A; 16];

// A modeled DMA region: readable and writable bytes with a bus address. The
// backing store is shared so a test can inspect what the driver laid down after
// the link has taken ownership of the region.
#[derive(Clone)]
struct Dma {
    mem: Rc<RefCell<Vec<u8>>>,
    dev: u64,
}

impl Dma {
    fn new(len: usize, dev: u64) -> Self {
        Self { mem: Rc::new(RefCell::new(vec![0u8; len])), dev }
    }
}

impl DmaMem for Dma {
    fn capacity(&self) -> usize {
        self.mem.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.dev
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        self.mem.borrow_mut()[offset..offset + src.len()].copy_from_slice(src);
    }
}

// The receive buffers as a CPU-readable region. The poll path only reads them, so
// a plain vector suffices; the harness stages a frame before the link is built.
struct RxBuf {
    mem: Vec<u8>,
    dev: u64,
}

impl RxBuf {
    fn new(dev: u64) -> Self {
        Self { mem: vec![0u8; RX_DESC_COUNT as usize * RX_BUF_STRIDE], dev }
    }

    // Stage a decrypted 802.11 MPDU behind a clean data descriptor in `slot`.
    fn stage(&mut self, slot: u32, mpdu: &[u8]) {
        self.stage_w0(slot, mpdu, 0);
    }

    // Stage a frame behind a descriptor with extra word-0 bits (the security
    // type, SWDEC).
    fn stage_w0(&mut self, slot: u32, mpdu: &[u8], w0_extra: u32) {
        let base = RxState::buffer_offset(slot);
        let mut d = rx_desc(mpdu.len() as u16);
        let w0 = u32::from_le_bytes([d[0], d[1], d[2], d[3]]) | w0_extra;
        d[0..4].copy_from_slice(&w0.to_le_bytes());
        self.mem[base..base + 24].copy_from_slice(&d);
        self.mem[base + 24..base + 24 + mpdu.len()].copy_from_slice(mpdu);
    }
}

impl RxBuffers for RxBuf {
    fn bytes(&self) -> &[u8] {
        &self.mem
    }
    fn device_addr(&self) -> u64 {
        self.dev
    }
}

// A clean data RX descriptor: length only, no driver info, no shift, no errors,
// not a firmware command. The frame body sits at the fixed 24-byte offset.
fn rx_desc(pkt_len: u16) -> [u8; 24] {
    let w0 = (pkt_len as u32) & 0x3FFF;
    let mut d = [0u8; 24];
    d[0..4].copy_from_slice(&w0.to_le_bytes());
    d
}

// A modeled card. Both ring index registers report a settable hardware pointer in
// their high bits; every write is recorded in a shared log so a test can assert
// the doorbell after the card has moved into the link.
#[derive(Clone)]
struct Card {
    tx_rp: u32,
    rx_wp: u32,
    writes: Rc<RefCell<Vec<(usize, u32)>>>,
}

impl Card {
    fn new(tx_rp: u32, rx_wp: u32) -> Self {
        Self { tx_rp, rx_wp, writes: Rc::new(RefCell::new(Vec::new())) }
    }
    fn wrote(&self, off: usize) -> bool {
        self.writes.borrow().iter().any(|&(o, _)| o == off)
    }
}

impl Mmio for Card {
    fn read8(&self, _o: usize) -> u8 {
        0
    }
    fn write8(&self, _o: usize, _v: u8) {}
    fn read16(&self, _o: usize) -> u16 {
        0
    }
    fn write16(&self, off: usize, val: u16) {
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read32(&self, off: usize) -> u32 {
        if off == REG_TXBD_IDX_BEQ {
            self.tx_rp << TRX_BD_HW_IDX_SHIFT
        } else if off == REG_RXBD_IDX_MPDUQ {
            self.rx_wp << TRX_BD_HW_IDX_SHIFT
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

// Build a link over fresh DMA. Returns the link, a handle on the TX buffer store
// so a test can read the descriptor the driver emitted, and the card handle.
fn link_with(card: Card, rx: RxBuf) -> (RtlLink<Card, Dma, RxBuf>, Dma) {
    let tx_ring = Dma::new(1 << 12, 0x1000_0000);
    let tx_buffers = Dma::new(1 << 16, 0x2000_0000);
    let rx_ring = Dma::new(1 << 12, 0x3000_0000);
    let tx_handle = tx_buffers.clone();
    let link = RtlLink::new(card, tx_ring, tx_buffers, rx_ring, rx, STA_MAC);
    (link, tx_handle)
}

// A minimal Ethernet frame to `dst` from `src` with one payload byte.
fn eth_frame(dst: [u8; 6], src: [u8; 6], ethertype: [u8; 2], payload: &[u8]) -> Vec<u8> {
    let mut e = Vec::new();
    e.extend_from_slice(&dst);
    e.extend_from_slice(&src);
    e.extend_from_slice(&ethertype);
    e.extend_from_slice(payload);
    e
}

#[test]
fn an_unassociated_link_is_down_with_no_mac() {
    let (link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    assert!(!link.link_up(), "no association means link down");
    assert_eq!(link.mac(), None, "no MAC is offered until associated");
}

#[test]
fn an_unassociated_link_refuses_to_transmit() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    let eth = [0u8; 64];
    assert!(!link.send_tx(&eth), "an unassociated station cannot frame a packet");
}

#[test]
fn association_brings_the_link_up_and_publishes_the_mac() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);
    assert!(link.link_up(), "associated means link up");
    assert_eq!(link.mac(), Some(STA_MAC), "the station MAC is published once up");
}

#[test]
fn transmit_encrypts_the_packet_in_software_and_kicks_the_queue() {
    let card = Card::new(0, 0);
    let probe = card.clone();
    let (mut link, tx) = link_with(card, RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);

    let eth = eth_frame(AP_MAC, STA_MAC, [0x08, 0x00], &[0xAB]);
    assert!(link.send_tx(&eth), "an associated station frames and enqueues");

    /* Descriptor word 1 bits 22..23: no radio cipher; the MPDU is already CCMP. */
    let mem = tx.mem.borrow();
    let w1 = u32::from_le_bytes([mem[4], mem[5], mem[6], mem[7]]);
    assert_eq!((w1 >> 22) & 0x3, 0, "the TX descriptor asks the radio for no cipher");
    assert_eq!(mem[TXDESC_LEN + 1] & 0x40, 0x40, "the MPDU is marked Protected");
    let body = &mem[TXDESC_LEN..TXDESC_LEN + 96];
    assert!(!body.windows(6).any(|w| w == [0xAA, 0xAA, 0x03, 0, 0, 0]), "no plaintext LLC");
    drop(mem);

    // The write-index doorbell was rung so the card sees the new descriptor.
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "the BE queue write index is kicked");
}

/// RX descriptor word 0: security type AES (`RX_DESC_ENC_AES`, bits 20-22).
const W0_ENC_AES: u32 = 4 << 20;
/// RX descriptor word 0: the chip left decryption to software.
const W0_SWDEC: u32 = 1 << 27;

// The access point's frame for `eth` (sent to us), as this chip hands it up
// after decrypting it in place: FromDS, Protected, the CCMP header (packet
// number `pn`), the plaintext, the MIC and the FCS still on.
fn chip_decrypted(eth: &[u8], pn: u8) -> Vec<u8> {
    let mut f = vec![0x08, 0x42, 0, 0];
    f.extend_from_slice(&eth[0..6]); // addr1: the destination, us
    f.extend_from_slice(&AP_MAC); // addr2: the BSSID
    f.extend_from_slice(&eth[6..12]); // addr3: the source
    f.extend_from_slice(&[0x20, 0x00]);
    f.extend_from_slice(&[pn, 0, 0, 0x20, 0, 0, 0, 0]);
    f.extend_from_slice(&[0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00]);
    f.extend_from_slice(&eth[12..]);
    f.extend_from_slice(&[0x4D; 8]); // MIC
    f.extend_from_slice(&[0xFC; 4]); // FCS
    f
}

#[test]
fn receive_recovers_the_original_ethernet_frame() {
    // The AP sends an Ethernet packet to us; the chip decrypts it in place.
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x06], &[0xDE, 0xAD, 0xBE, 0xEF]);
    let mpdu = chip_decrypted(&eth, 1);

    // Stage it in RX slot 0 and tell the card one frame is ready.
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &mpdu, W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);

    let mut out = [0u8; 1600];
    let n = link.poll_rx(&mut out).expect("a staged frame is delivered");
    assert_eq!(&out[..n], &eth[..], "the exact Ethernet frame is recovered");
}

#[test]
fn receive_on_an_empty_ring_yields_nothing() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "no frame queued means no delivery");
}

// The access point's frame for `eth` in the clear: FromDS, not protected.
fn from_ap_plain(eth: &[u8]) -> Vec<u8> {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&eth[0..6]);
    f.extend_from_slice(&AP_MAC);
    f.extend_from_slice(&eth[6..12]);
    f.extend_from_slice(&[0x10, 0x00]);
    f.extend_from_slice(&[0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00]);
    f.extend_from_slice(&eth[12..]);
    f
}

fn with_fcs(mut f: Vec<u8>) -> Vec<u8> {
    f.extend_from_slice(&[0xFC; 4]);
    f
}

#[test]
fn an_unprotected_frame_is_refused_on_a_protected_link() {
    // What this fixture used to stage: plaintext data, as if decryption had
    // also cleared the Protected bit. On a protected link that is an
    // injection, and the stack must not see it.
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[1, 2, 3, 4]);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &with_fcs(from_ap_plain(&eth)));
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "refused");
    assert_eq!((link.stats().rx_refused, link.stats().rx_eth), (1, 0), "and counted");
}

#[test]
fn a_frame_the_chip_left_encrypted_is_decrypted_in_software() {
    // SWDEC set, or no security type at all: the frame is still ciphertext
    // and the station decrypts it under the pairwise key.
    for w0 in [W0_ENC_AES | W0_SWDEC, 0] {
        let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], b"software");
        let enc = encrypt(&from_ap_plain(&eth), 5, &PTK).expect("protected");
        let mut rx = RxBuf::new(0x4000_0000);
        rx.stage_w0(0, &with_fcs(enc), w0);
        let (mut link, _tx) = link_with(Card::new(0, 1), rx);
        link.associate(AP_MAC, PTK);
        let mut out = [0u8; 1600];
        let n = link.poll_rx(&mut out).expect("decrypted and delivered");
        assert_eq!(&out[..n], &eth[..]);
    }
}

#[test]
fn ciphertext_the_chip_claims_to_have_decrypted_is_not_delivered() {
    // A frame the station would read as plaintext must really be plaintext:
    // ciphertext marked decrypted fails the LLC/SNAP check, never reaches the
    // stack as garbage.
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], b"cipher");
    let enc = encrypt(&from_ap_plain(&eth), 5, &PTK).expect("protected");
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &with_fcs(enc), W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
}

#[test]
fn a_replayed_frame_is_refused() {
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[9, 9]);
    let f = chip_decrypted(&eth, 1);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &f, W0_ENC_AES);
    rx.stage_w0(1, &f, W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 2), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_some(), "the first copy is delivered");
    assert!(link.poll_rx(&mut out).is_none(), "the replay of packet number 1 is not");
    assert_eq!(link.stats().rx_refused, 1);
}

#[test]
fn a_frame_larger_than_the_caller_buffer_is_dropped_not_cut() {
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[7; 64]);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &chip_decrypted(&eth, 1), W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 32];
    assert!(link.poll_rx(&mut out).is_none(), "no truncated packet reaches the stack");
    assert_eq!(link.stats().rx_refused, 1);
}

#[test]
fn poll_raw_hands_up_the_frame_without_its_fcs() {
    let beacon = [0x80u8; 40];
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &beacon);
    rx.stage(1, &[0x80, 0x00, 0x00]);
    let (mut link, _tx) = link_with(Card::new(0, 2), rx);
    let mut out = [0u8; 1600];
    assert_eq!(link.poll_raw(&mut out), Some(36), "four bytes of FCS dropped");
    assert_eq!(link.poll_raw(&mut out), None, "a frame shorter than an FCS is dropped");
}

// A deauthentication from the AP to us with `reason`, and its FCS.
fn deauth_from_ap(reason: u16) -> Vec<u8> {
    let mut f = vec![0xC0, 0x00, 0x00, 0x00];
    f.extend_from_slice(&STA_MAC);
    f.extend_from_slice(&AP_MAC);
    f.extend_from_slice(&AP_MAC);
    f.extend_from_slice(&[0x30, 0x00]);
    f.extend_from_slice(&reason.to_le_bytes());
    with_fcs(f)
}

#[test]
fn a_deauthentication_from_the_access_point_takes_the_link_down() {
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &deauth_from_ap(7));
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert!(!link.link_up(), "the link reads down");
    assert_eq!(link.take_left(), Some(7), "with the AP's reason, for the serve loop");
    assert_eq!(link.take_left(), None, "reported once");
}

#[test]
fn a_deauthentication_from_another_access_point_is_ignored() {
    let mut f = deauth_from_ap(7);
    f[10] ^= 0x01; // transmitter: someone else
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &f);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert!(link.link_up() && link.take_left().is_none(), "the association stands");
}

// A supplicant configured for this link, with management frame protection
// as given.
fn supplicant(pmf: bool) -> Supplicant {
    Supplicant::configure(&Config {
        pmk: pmk(b"ThisIsAPassword", b"ThisIsASSID"),
        aa: AP_MAC,
        spa: STA_MAC,
        snonce: SNONCE,
        akm: Akm::Psk,
        own_rsne: &nonos_wifi_core::wpa::RSN_IE,
        own_rsnxe: None,
        ap_rsne: &AP_RSNE_MIXED,
        ap_rsnxe: None,
        pmf,
    })
}

#[test]
fn with_management_frame_protection_an_unprotected_deauthentication_is_ignored() {
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &deauth_from_ap(7));
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    link.set_supplicant(supplicant(true));
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert!(link.link_up() && link.take_left().is_none(), "a forgery cannot end the link");
}

#[test]
fn leaving_tells_the_access_point_reason_3() {
    let card = Card::new(0, 0);
    let probe = card.clone();
    let (mut link, tx) = link_with(card, RxBuf::new(0x4000_0000));
    link.send_deauth();
    assert!(!probe.wrote(REG_TXBD_IDX_BEQ), "nothing is sent before an association");
    link.associate(AP_MAC, PTK);
    link.send_deauth();
    let mem = tx.mem.borrow();
    let f = &mem[TXDESC_LEN..TXDESC_LEN + 26];
    assert_eq!(f[0], 0xC0, "a deauthentication");
    assert_eq!(&f[4..10], &AP_MAC, "to the access point");
    assert_eq!(&f[10..16], &STA_MAC, "from us");
    assert_eq!(&f[24..26], &[3, 0], "reason 3: the station is leaving");
    drop(mem);
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "and the queue is kicked");
}

#[test]
fn with_management_frame_protection_the_deauthentication_goes_protected() {
    let (mut link, tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);
    link.set_supplicant(supplicant(true));
    link.send_deauth();
    let mem = tx.mem.borrow();
    let f = &mem[TXDESC_LEN..TXDESC_LEN + 24 + 8 + 2 + 8];
    assert_eq!(f[0], 0xC0);
    assert_ne!(f[1] & 0x40, 0, "the Protected bit is set");
    assert_eq!(decrypt(f, &PTK).expect("under the pairwise key"), vec![3, 0], "reason 3 inside");
}

#[test]
fn the_group_key_handshake_is_answered_on_the_link() {
    // Finish a four-way handshake, so the supplicant holds the PTK and the
    // first group key.
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = supplicant(false);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = Akm::Psk.derive_ptk(&key, &AP_MAC, &STA_MAC, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    let mut tk = [0u8; 16];
    tk.copy_from_slice(&ptk[32..48]);

    // The AP rekeys: group message 1 arrives protected, decrypted by the chip.
    let next = [0x22u8; 16];
    let g1 = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next));
    let eth = eth_frame(STA_MAC, AP_MAC, [0x88, 0x8E], &g1);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &chip_decrypted(&eth, 1), W0_ENC_AES);
    let card = Card::new(0, 1);
    let probe = card.clone();
    let (mut link, txm) = link_with(card, rx);
    link.associate(AP_MAC, tk);
    link.set_supplicant(sup);

    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "nothing for the stack");
    assert_eq!(link.take_group_key(), Some((2, next)), "the new key, for the CAM");
    assert_eq!(link.take_group_key(), None, "handed over once");
    assert_eq!(link.stats().rekeys, 1);
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "group message 2 was queued");
    let mem = txm.mem.borrow();
    assert_ne!(mem[TXDESC_LEN + 1] & 0x40, 0, "sent protected, as the message came");
}
