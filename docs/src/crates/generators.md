# testkit-generators

Soroban-aware property testing strategies for `proptest` and `arbitrary`.

## Available Strategies

| Strategy | Type | Range |
|----------|------|-------|
| `token_amount()` | `i128` | 0 to 1e15 |
| `ledger_sequence()` | `u32` | 1 to 10,000,000 |
| `timestamp()` | `u64` | Sep 2020 to May 2033 |

## Usage with proptest

```rust
use proptest::prelude::*;
use testkit_generators::strategies;

proptest! {
    #[test]
    fn transfer_preserves_total_supply(
        amount in strategies::token_amount(),
        seq in strategies::ledger_sequence(),
    ) {
        // amount is always a valid token balance
        // seq is always a realistic ledger sequence number
    }
}
```

## Event Fuzzing

`EventFuzzHarness` produces valid SDK v28 `ContractEvent` values and mutates
them to verify that event parsers reject malformed payloads cleanly.

```rust
use testkit_generators::event_fuzz::{
    fuzzed_event, valid_transfer_event, EventFuzzHarness, MutationOperator,
};

// Start from a well-formed event and apply a single mutation.
let harness = EventFuzzHarness::new(valid_transfer_event());
let malformed = harness.mutate(MutationOperator::DropField, 42);

// Or drive a property test across every mutation operator.
proptest! {
    #[test]
    fn parser_never_panics(event in fuzzed_event(valid_transfer_event())) {
        // Downstream decoding must return an error, never panic.
        let _ = TransferEvent::try_from_event(&event);
    }
}
```

Supported mutation operators:

| Operator | Effect |
|----------|--------|
| `DropField` | Removes a topic, or clears the data when no topics remain |
| `TypeShift` | Replaces a value with a different `ScVal` variant |
| `ByteTruncate` | Halves the length of bytes, strings, symbols, or vectors |
| `SignFlip` | Flips the sign of a number or toggles a boolean |

`EventFuzzHarness::shrink` deterministically reduces a malformed payload after
a failure, mirroring proptest's shrinking so failures are reported with the
smallest event that still reproduces the bug. `shrink_candidates` returns each
intermediate step for debugging.

## Custom Strategies

Compose built-in strategies with proptest combinators:

```rust
use proptest::prelude::*;
use testkit_generators::strategies;

fn transfer_args() -> impl Strategy<Value = (i128, i128)> {
    (strategies::token_amount(), strategies::token_amount())
        .prop_filter("sender must have sufficient balance", |(a, b)| a >= b)
}
```
