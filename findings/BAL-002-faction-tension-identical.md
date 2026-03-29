---
id: BAL-002
type: balance
severity: minor
status: open
session: 2026-03-29
found_by: playtest
---

# All factions start at identical tension (60/60/60)

## Observed
Turn 29 state: Merchant Guild tension=60, Order tension=60, Shadows tension=60.
All three factions have the same tension value.

## Root Cause
All factions share the same initial tension value in `data.rs`, and `advance_fronts`
applies `+10` to ALL factions equally on every stage advance. There is no per-faction
tension differentiation.

## Impact
Minor — faction power IS diverging correctly (Guild 69, Shadows 54, Order 34).
Tension being uniform just means the "instability" coin-flip applies equally to all.

## Fix Options
- Set different starting tension values in data (e.g. Guild 40, Order 20, Shadows 70)
- Make front escalation target specific factions based on which faction the front
  involves, rather than all factions globally
