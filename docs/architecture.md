# Architecture

- Chain events: Offered, Accepted, Paid, SignerRevoked and Closed.
- The event store is SQLite. The schema version is 2.
- Match statuses: matched, matched without the document, payment without an order,
  payment without a bill, ambiguous, document and obligation disagree.
- Report kinds include: bill without a payment, amount difference, buyer approval
  (the checker did not run on chain), order expiring with open bills, self-check.
- The beancount export writes these entries: `custom "warrant-invoice"`, Offered, Paid and
  Closed transactions, and a balance assertion with tolerance 0.000001 USDC.
- The export replaces the file. It never edits your other ledger files.
