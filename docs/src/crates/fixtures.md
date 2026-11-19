# testkit-fixtures

Reusable test context builder inspired by Foundry's `setUp()` pattern.

## Basic Usage

```rust
use testkit_fixtures::TestContext;

let ctx = TestContext::new();
// ctx.env — pre-configured Env with mock_all_auths
// ctx.admin — generated admin address
// ctx.users — empty vec, add users as needed
```

## Builder Pattern

Configure your test context declaratively:

```rust
use testkit_fixtures::builder::TestContextBuilder;

let ctx = TestContextBuilder::new()
    .with_users(5)           // Pre-generate 5 user addresses
    .without_mock_auths()     // Disable automatic auth mocking
    .build();
```

## Network Profiles

By default the test host is unmetered so local iteration stays fast. Use a
network profile to emulate the resource ceilings and ledger parameters of a
real network:

```rust
use testkit_fixtures::builder::TestContextBuilder;
use testkit_fixtures::NetworkProfile;

let ctx = TestContextBuilder::new()
    .with_network_profile(NetworkProfile::Mainnet)
    .build();

// Panics if this invocation exceeds real Stellar Mainnet limits.
client.heavy_computation();
```

| Profile | CPU instructions | Memory | Ledger fees |
|---------|------------------|--------|-------------|
| `Local` | Unlimited | Unlimited | Base reserve 0 |
| `Testnet` | 100,000,000 | 40 MiB | 0.5 XLM base reserve |
| `Mainnet` | 400,000,000 | 40 MiB | 0.5 XLM base reserve, rent-bearing TTLs |

The builder also installs the profile's base reserve, minimum temporary and
persistent entry TTLs, and maximum entry TTL, so storage and rent assumptions
are exercised in the same way as on-chain. When an invocation exceeds a metered
profile's CPU, memory, or ledger-entry limits, the test panics with a message
identifying the exceeded resource.

`NetworkProfile` exposes the raw values via `cpu_instructions()`,
`mem_bytes()`, `base_reserve()`, `rent_fee_per_kib()`, and
`invocation_limits()` if you need to assert on them directly.

## Extending Fixtures

Create project-specific fixtures by composing TestContext:

```rust
struct TokenTestEnv {
    ctx: TestContext,
    token_id: BytesN<32>,
}

impl TokenTestEnv {
    fn setup() -> Self {
        let ctx = TestContextBuilder::new().with_users(3).build();
        let token_id = ctx.env.register(TokenContract, ());
        Self { ctx, token_id }
    }
}
```
