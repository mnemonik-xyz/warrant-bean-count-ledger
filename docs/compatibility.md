# Compatibility

The identifiers come from the `warrant-policy` crate at the tag in `tools/warrant-ids/Cargo.toml`.
The golden vectors in `tests/data/vectors.json` must equal the values that the pinned core computes.

If the vector test fails after a tag change:

1. The core changed an identifier or a normalisation rule.
2. Do not edit the vectors to silence the failure.
3. Check the core release notes and the escrow contract version.
4. Rewrite the vectors with `WARRANT_WRITE_VECTORS=1` only after you confirm the change.
