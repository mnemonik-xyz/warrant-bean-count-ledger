//! Golden vectors shared with the Python connector. The fixture document and the expected values
//! are committed under `tests/data`. This test recomputes every value with the pinned Warrant core
//! and compares it with the committed file.
//!
//! A failure after you change the core tag means that the core changed an identifier or a
//! normalisation rule. Read `docs/compatibility.md` before you change any expected value.
//! To rewrite `tests/data/vectors.json` on purpose, set `WARRANT_WRITE_VECTORS=1`.

use serde_json::{json, Value};
use std::{fs, path::PathBuf};
use warrant_ids::*;
use warrant_policy::evidence::*;

const TAX_ID: &str = "US-12-3456789";
const CHAIN_ID: u64 = 5042002;
const ESCROW: [u8; 20] = [0x11; 20];
const CUSTOMER: [u8; 20] = [0x22; 20];
const POLICY_HASH: [u8; 32] = [0x33; 32];

fn data(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/data").join(name)
}

fn fixture() -> Vec<u8> {
    fs::read(data("invoice.xml")).expect("fixture document is missing")
}

fn committed() -> Value {
    serde_json::from_slice(&fs::read(data("vectors.json")).expect("vectors.json is missing")).unwrap()
}

fn computed() -> Value {
    let xml = fixture();
    json!({
        "note": "Computed by tools/warrant-ids/tests/vectors.rs from tests/data/invoice.xml.",
        "document": "invoice.xml",
        "facts": facts(&xml).unwrap(),
        "obligation": {"sellerTaxId": TAX_ID, "invoiceNumber": "INV-1001",
                       "hash": hex32(&obligation_id(TAX_ID, "INV-1001"))},
        "obligationNormalised": {"sellerTaxId": " us 12 3456789 ", "invoiceNumber": " INV-1001 ",
                       "hash": hex32(&obligation_id(" us 12 3456789 ", " INV-1001 "))},
        "reference": {"text": "PO-77", "hash": hex32(&reference_hash("PO-77"))},
        "taxId": {"text": TAX_ID, "hash": hex32(&tax_id_hash(TAX_ID))},
        "orderId": {"chainId": CHAIN_ID, "escrow": format!("0x{}", hex::encode(ESCROW)),
                    "customer": format!("0x{}", hex::encode(CUSTOMER)),
                    "policyHash": hex32(&POLICY_HASH), "poId": hex32(&reference_hash("PO-77")),
                    "hash": hex32(&order_id(CHAIN_ID, &ESCROW, &CUSTOMER, &POLICY_HASH, &reference_hash("PO-77")))},
    })
}

#[test]
fn committed_vectors_are_current() {
    let generated = computed();
    if std::env::var_os("WARRANT_WRITE_VECTORS").is_some() {
        fs::write(data("vectors.json"), format!("{}\n", serde_json::to_string_pretty(&generated).unwrap())).unwrap();
    }
    assert_eq!(committed(), generated, "tests/data/vectors.json is stale");
}

#[test]
fn facts_match_the_checker() {
    let xml = fixture();
    let f = facts(&xml).unwrap();
    let parsed = parse_invoice(&xml).unwrap();
    assert_eq!(f["documentHash"], hex32(&parsed.doc_hash));
    assert_eq!(f["obligationId"], hex32(&obligation_id(TAX_ID, "INV-1001")));
    assert_eq!(f["poId"], hex32(&reference_hash("PO-77")));
    assert_eq!(f["taxIdHash"], hex32(&tax_id_hash(TAX_ID)));
    assert_eq!(f["payableMinor"], 132_000);
    assert_eq!(f["currency"], "USD");
    assert_eq!(f["currencyConsistent"], true);
    assert_eq!(f["precise"], true);
    assert_eq!(f["totalsConsistent"], true);
}

#[test]
fn denied_document_is_an_error() {
    assert!(facts(b"<NotAnInvoice/>").is_err());
}

#[test]
fn order_id_matches_the_contract_layout() {
    // abi.encode of five static words, hashed with keccak256: InvoiceEscrow.orderIdFor.
    let mut encoded = Vec::new();
    let mut word = [0u8; 32];
    word[24..].copy_from_slice(&CHAIN_ID.to_be_bytes());
    encoded.extend_from_slice(&word);
    for a in [ESCROW, CUSTOMER] {
        let mut w = [0u8; 32];
        w[12..].copy_from_slice(&a);
        encoded.extend_from_slice(&w);
    }
    encoded.extend_from_slice(&POLICY_HASH);
    encoded.extend_from_slice(&reference_hash("PO-77"));
    use sha3::{Digest, Keccak256};
    let expected: [u8; 32] = Keccak256::digest(&encoded).into();
    assert_eq!(
        order_id(CHAIN_ID, &ESCROW, &CUSTOMER, &POLICY_HASH, &reference_hash("PO-77")),
        expected
    );
}
