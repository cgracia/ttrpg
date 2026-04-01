---
id: BUG-003
type: bug
title: faction_tension_thresholds not registered in simulate.rs
severity: major
status: open
sprint: current
opened: 2026-03-31
---

## Finding

**Observed**: Tension threshold events (TASK-014) never appear in any headless simulation dump, across all 100 turns and all faction threshold crossings (Shadows 75→100, Merchant Guild 75→100+, Order 75+).

**Expected**: When a faction's tension first crosses 75 or 100, a narrative event should appear in the event log (e.g., "A Merchant Guild runner is found beaten near the River Docks. No witnesses." when Shadows cross 75).

**Root cause identified**: The `faction_tension_thresholds` system is registered in `src/main.rs` (the windowed game) but is **absent from `src/simulate.rs`** (the headless simulator). The commit implementing TASK-014 updated `main.rs` but not `simulate.rs`.

**Steps to reproduce**:
1. `cargo run --bin simulate -- 100 25`
2. Check any dump for Shadows tension (crosses 75 around T17, 100 around T31)
3. Grep for "Guild runner" or "Sable's crew" — no results

**Affected file**: `src/simulate.rs` — missing import and chain entry for `faction_tension_thresholds`

**Fix**: Add to `src/simulate.rs`:
```rust
// import line:
use systems::fronts::{advance_fronts, faction_tension_effects, faction_tension_thresholds};

// chain:
advance_fronts,
faction_tension_thresholds,   // ← add here
faction_tension_effects,
```

**Severity**: Major — TASK-014 cannot be verified via the primary testing tool. Tension threshold events may or may not work in the windowed game (unverified without a windowed playtest).
