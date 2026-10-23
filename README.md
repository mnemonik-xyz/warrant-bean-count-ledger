# warrant-bean-count-ledger

A connector that books Warrant `InvoiceEscrow` events into a beancount ledger.

It reads public chain logs. It holds no Warrant key and cannot move money.

## How it works

1. `sync` reads the escrow logs over JSON-RPC and stores the events in a local SQLite file.
2. `intake` records each invoice document the buyer agent submitted.
   The `warrant-ids` helper computes the identifiers with the Warrant core.
3. `match` pairs payments with invoices by document hash, then by obligation identifier.
4. `report` lists the exceptions.
5. `export` writes the whole beancount file again from the event store.
   The same events give the same bytes.
   Include the generated file from your main ledger.

The bean-check plugin `warrant_ledger.beancount_plugin` checks the generated entries.

## Commands

```sh
cargo build -p warrant-ids --release --locked
pip install -e '.[beancount]'
warrant-ledger --help
```

## Tests

```sh
pip install -r requirements-test.txt pytest
cargo test -p warrant-ids --locked
python -m pytest
```

## Core version

`tools/warrant-ids/Cargo.toml` pins one release of the Warrant core.
See `docs/compatibility.md` before you change the tag.
