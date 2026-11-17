// Integration test for SigmaOS-archlinux-tpm2-pkcs11
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_archlinux_tpm2_pkcs11 as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
