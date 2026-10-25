// NONOS Operating System (AGPL-3.0-or-later)

//! PBKDF2 and PRF-SHA1 against IEEE 802.11-2020 Annex J.

use nonos_wifi_core::wpa::pbkdf2::pbkdf2_sha1;
use nonos_wifi_core::wpa::prf::prf;
use nonos_wifi_core::wpa::ptk::pmk;

use super::vectors::h;

#[test]
fn pbkdf2_psk_matches_annex_j4() {
    let cases: [(&[u8], &[u8], &str); 3] = [
        (b"password", b"IEEE", "f42c6fc52df0ebef9ebb4b90b38a5f902e83fe1b135a70e23aed762e9710a12e"),
        (
            b"ThisIsAPassword",
            b"ThisIsASSID",
            "0dc0d6eb90555ed6419756b9a15ec3e3209b63df707dd508d14581f8982721af",
        ),
        (
            b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            b"ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ",
            "becb93866bb8c3832cb777c2f559807c8c59afcb6eae734885001300a981cc62",
        ),
    ];
    for (pass, ssid, want) in cases {
        assert_eq!(pmk(pass, ssid).to_vec(), h(want), "{}", String::from_utf8_lossy(pass));
        let mut out = [0u8; 32];
        pbkdf2_sha1(pass, ssid, 4096, &mut out);
        assert_eq!(out.to_vec(), h(want));
    }
}

#[test]
fn prf_sha1_matches_annex_j3() {
    let cases: [(Vec<u8>, Vec<u8>, &str); 3] = [
        (
            vec![0x0b; 20],
            b"Hi There".to_vec(),
            "bcd4c650b30b9684951829e0d75f9d54b862175ed9f00606e17d8da35402ffee
             75df78c3d31e0f889f012120c0862beb67753e7439ae242edb8373698356cf5a",
        ),
        (
            b"Jefe".to_vec(),
            b"what do ya want for nothing?".to_vec(),
            "51f4de5b33f249adf81aeb713a3c20f4fe631446fabdfa58244759ae58ef9009
             a99abf4eac2ca5fa87e692c440eb40023e7babb206d61de7b92f41529092b8fc",
        ),
        (
            vec![0xaa; 20],
            vec![0xdd; 50],
            "e1ac546ec4cb636f9976487be5c86be17a0252ca5d8d8df12cfb04735252 49ce
             9dd8d177ead710bc9b590547239107aef7b4abd43d87f0a68f1cbd9e2b6f7607",
        ),
    ];
    for (key, data, want) in cases {
        let want = h(want);
        let mut out = [0u8; 64];
        prf(&key, b"prefix", &data, &mut out);
        assert_eq!(out.to_vec(), want);
        // PRF-384, as the PTK uses, is the first 48 bytes of the same stream.
        let mut out384 = [0u8; 48];
        prf(&key, b"prefix", &data, &mut out384);
        assert_eq!(out384.to_vec(), want[..48].to_vec());
    }
}
