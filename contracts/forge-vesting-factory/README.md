# forge-vesting-factory

A factory contract that manages multiple independent vesting schedules in a single deployment.

---

## When to Use Factory vs Standalone

| | `forge-vesting` | `forge-vesting-factory` |
| :--- | :--- | :--- |
| Schedules per deployment | 1 | Unlimited |
| Deployment cost | One per beneficiary | One total |
| Pause / unpause | ✅ | ❌ |
| `change_beneficiary` | ✅ | ❌ |
| `transfer_admin` | ✅ | ❌ |
| Best for | Single high-value grant | Employee/advisor batch grants |

Use `forge-vesting-factory` when you need to manage many vesting schedules (e.g. team token grants, investor lockups) without deploying a separate contract per beneficiary. Use `forge-vesting` when you need the full feature set including pause, beneficiary transfer, or admin rotation.

---

## Interface Summary

### `create_schedule(token, beneficiary, admin, total_amount, cliff_seconds, duration_seconds) -> u64`

Creates a new vesting schedule and returns its `schedule_id`. Transfers `total_amount` tokens from `admin` into the contract immediately. Requires authorization from `admin`.

### `claim(schedule_id) -> i128`

Withdraws all currently vested and unclaimed tokens for the schedule. Requires authorization from the schedule's `beneficiary`. Returns the amount transferred.

### `cancel(schedule_id)`

Cancels the schedule. Vested-but-unclaimed tokens are sent to the beneficiary; unvested tokens are returned to the admin. Requires authorization from the schedule's `admin`.

### `get_status(schedule_id) -> VestingStatus`

Returns a read-only snapshot of the schedule including `vested`, `claimed`, `claimable`, `cliff_reached`, `fully_vested`, and `cancelled`.

### `get_schedule_count() -> u64`

Returns the total number of schedules ever created. Schedule IDs are zero-indexed, so valid IDs range from `0` to `get_schedule_count() - 1`.

---

## Usage Example

```rust
// Deploy once
let factory = ForgeVestingFactoryClient::new(&env, &contract_id);

// Create schedule for Alice — 1M tokens, 100s cliff, 1000s duration
let alice_id = factory.create_schedule(
    &token,
    &alice,
    &admin,
    &1_000_000,
    &100,   // cliff_seconds
    &1000,  // duration_seconds
);

// Create schedule for Bob — 500k tokens, no cliff, 500s duration
let bob_id = factory.create_schedule(
    &token,
    &bob,
    &admin,
    &500_000,
    &0,
    &500,
);

// Advance time past cliff
env.ledger().with_mut(|l| l.timestamp += 200);

// Alice claims her vested tokens
let claimed = factory.claim(&alice_id);

// Check Bob's status — unaffected by Alice's claim
let status = factory.get_status(&bob_id);
assert_eq!(status.claimed, 0);

// Admin cancels Bob's schedule
factory.cancel(&bob_id);

// Total schedules created
assert_eq!(factory.get_schedule_count(), 2);
```

---

## Storage Strategy

`ScheduleCount` is stored in **persistent** storage (not instance storage). This is critical: if `ScheduleCount` were in instance storage and that entry expired, the counter would reset to 0 and new schedules would silently overwrite existing ones in persistent storage, permanently destroying beneficiary vesting data.

All per-schedule entries (`Schedule(id)`, `Claimed(id)`, `VestedAtCancel(id)`) are also stored in persistent storage and have their TTL extended on every write.

---

## Known Limitations

- **No pause/unpause** — schedules cannot be temporarily frozen. Use `forge-vesting` if pause support is required.
- **No `change_beneficiary`** — the beneficiary address is fixed at creation time.
- **No `transfer_admin`** — the admin address is fixed at creation time.
- **No `cancel_and_claim`** — atomic cancel-and-claim is not available; `cancel()` automatically pays out vested tokens to the beneficiary on cancellation.
- **No per-schedule events for admin changes** — since admin and beneficiary are immutable, no transfer events are emitted.

---

## See Also

- [`forge-vesting`](../forge-vesting/README.md) — single-schedule vesting with full feature set
- [Composability Guide](../../docs/composability.md) — combining factory with other StellarForge contracts
