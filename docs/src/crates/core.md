# testkit-core

Core utilities for budget tracking, error decoding, and storage inspection.

## Budget Snapshots

Capture and compare CPU/memory consumption between operations:

```rust
use testkit_core::budget::BudgetSnapshot;

let before = BudgetSnapshot::capture(&env.host().budget_cloned());
// ... invoke contract
let after = BudgetSnapshot::capture(&env.host().budget_cloned());
let diff = before.diff(&after);

println!("CPU instructions: {}", diff.cpu_insns);
println!("Memory bytes: {}", diff.mem_bytes);
```

## Error Decoding

Translate opaque Soroban error codes into human-readable messages:

```rust
use testkit_core::error::DecodedError;

let err = DecodedError {
    code: 12,
    message: "Insufficient balance".to_string(),
    context: Some("transfer".to_string()),
};
println!("{}", err); // "Soroban Error [12]: Insufficient balance (context: transfer)"
```

## Storage Inspection

Examine contract storage entries and their tiers:

```rust
use testkit_core::storage::{inspect_storage, StorageTier};

let entries = inspect_storage(&env, &contract_id);
for entry in &entries {
    println!("{:?} storage: {}", entry.tier, entry.key);
}
```

## Storage Namespace Auditing

When contracts compose multiple modules or traits, bare symbol keys such as
`"admin"` can collide and silently overwrite another module's state.
`NamespaceAuditor` records keys per module and reports collisions and
un-namespaced keys.

```rust
use testkit_core::namespace_auditor::NamespaceAuditor;

let mut auditor = NamespaceAuditor::new(&env, &contract_id);
auditor
    .record("ownable", "ownable::admin")
    .record("pausable", "pausable::paused");

let report = auditor.audit();
assert!(!report.has_collisions(), "Storage collision: {:?}", report.collisions());
println!("{report}");
```

Keys are considered namespaced when they contain a namespace separator
(`::`, `:`, `.`, `/`) such as `DataKey::Admin` or `ownable:admin`. Bare keys
like `admin` are flagged as collision risks. A collision is reported when the
same key is written by two independent modules. `report.modules()` returns the
full key map, while `NamespaceAuditor::from_entries` infers module ownership
from a `StorageEntry` snapshot.
