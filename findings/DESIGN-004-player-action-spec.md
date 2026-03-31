---
name: DESIGN-004 — Player Action Spec: Three World-State Actions
description: Spec for three player actions that mutate simulation state (front countdowns, faction power/tension, player wealth). Feeds architect implementation.
type: project
---

# DESIGN-004: Player Action Spec — Three World-State Actions

**Status**: Spec complete — feeds architect implementation
**GitHub task**: #26 (TASK-016)

---

## Design Intent

The current player actions are observation-only (AskRumor, Scout, AskAboutFaction). The player watches the world but cannot push back against it. These three actions are the minimum needed to prove the gameplay loop: *travel → interact → learn → decide → consequences*.

Each action costs something real, changes named simulation state, and gives the player legible feedback. The player should feel like they are maneuvering in a world that could tip either way.

---

## Action 1: Warn Lena

**Name in UI**: "Warn her about the Guild's plans"
**Location**: River Docks
**NPC**: Lena Marsh only (`npc_id = "lena"`)
**Availability**: Always available when in conversation with Lena

### Cost
None in coin. Costs 1 time step (the interaction itself). Player is revealing they've been paying attention — in a future exposure system, this would carry weight.

### World-State Effect
Find the active front named `"The Guild's Gambit"` or (if it has resolved) `"The Iron Ledger"`. Add **+4** to its current `countdown`.

This represents Lena getting wind of the timeline and taking countermeasures — moving cargo early, stalling on paperwork, refusing to sign dock transfer documents. She doesn't stop the Guild; she slows them.

### Event Log Entry
> *You warn Lena about what you've heard. She listens carefully, then nods once. "I'll be careful." The Guild's timeline slips — for now.*

### Legibility
The player won't see the countdown number directly, but they will notice front stage events firing later than expected. A future state dump (F11) shows countdown values explicitly. This is enough for prototype validation.

### One-Time Use
After the first use, the option should not reappear in conversation with Lena during the same game. (Simple boolean flag on the action or check for the log entry — architect's call on mechanism.)

---

## Action 2: Pay for Information

**Name in UI**: "Pay for information (10 coin)"
**Location**: Tavern, Back Alleys, Docks, Market (wherever Finn is)
**NPC**: Finn Crowe only (`npc_id = "finn"`)
**Availability**: Available when player Wealth ≥ 10

### Cost
- Player Wealth −10
- **33% chance**: Shadows faction Tension +8

The 33% risk is Finn mentioning the interaction to Sable. He doesn't mean anything by it — he just talks to everyone. The player never knows for sure if it happened.

### World-State Effect
- Player gains a rumor from Finn's Knowledge (same pool as AskRumor, but prefer a rumor the player doesn't already have — fall back to Finn's pool if all known)
- Player Wealth mutated: `wealth.0 -= 10`
- Random risk: Shadows `FactionTension.0 += 8`

### Event Log Entries
Primary: `Finn pockets the coin and leans in close. "[rumor text]"`
If tension fires: `Finn's eyes drift toward the door as you leave. Someone noticed you were asking.`

### Design Note
AskRumor already exists and is free. This action is for players who want to exhaust Finn's knowledge faster or who are willing to pay for a more targeted answer. The real value is the economy: spending 10 coin is a meaningful choice at Wealth 20, and becomes strategic once Wealth is low. The Shadows tension risk is the price of expedience.

---

## Action 3: Share Evidence with Canon Thess

**Name in UI**: "Tell her what you know"
**Location**: Temple of Accord
**NPC**: Canon Thess only (`npc_id = "canon_thess"`)
**Availability**: Player's Knowledge contains ≥ 2 rumors

### Cost
No coin cost. Player is revealing their investigative interest to Thess — a risk in the world, even if not yet simulated mechanically. The minimum 2-rumor gate ensures the player has earned this moment through observation first.

### World-State Effect
Three simultaneous changes:

1. Order faction `FactionPower.0 += 8`
   *The Order is energized. They now have specific grievances to act on.*

2. Guild faction `FactionTension.0 += 5`
   *Word reaches Aldric that the Order is stirring. He takes notice.*

3. Find active front named `"Whispers from the Mine"` (if still active) and subtract **−4** from its current `countdown`.
   *The Order will now begin asking questions about the mine. The secret surfaces faster.*

### Event Log Entry
> *You share what you've learned with Canon Thess. She listens without speaking, then: "Then the Accord must speak." The Order stirs. The mine's secret draws closer to daylight.*

### Design Note
This is the highest-leverage action of the three. It doesn't just change one number — it shifts the political balance and accelerates a front simultaneously. The player is making a bet: the Order becomes a useful counterforce, but they're also exposing their investigation to a faction they don't fully control. Canon Thess is trustworthy, but Thess taking public action makes the player's presence in Ashenveil more visible.

One-time use only (same reasoning as Action 1 — after sharing, there's nothing left to tell).

---

## Implementation Notes for Architect

### New PlayerAction variants needed
```
PlayerAction::WarnLena
PlayerAction::PayForInfo
PlayerAction::ShareWithThess
```

### Query mutations required in `execute_player_action`
The function currently takes `faction_query` as immutable. Two of these actions need mutable faction access:

- `Query<(&ActorName, &mut FactionPower, &mut FactionTension), With<FactionMarker>>` for Action 2 (Shadows tension) and Action 3 (Order power, Guild tension)

A `Query<&mut Front>` is needed for Actions 1 and 3 to modify front countdowns. This query doesn't currently exist in the function signature.

### Availability guards
- Action 1 (WarnLena): only when `interaction.selected_npc` maps to NPC id `"lena"`
- Action 2 (PayForInfo): only when selected NPC is `"finn"` AND player `Wealth.0 >= 10`
- Action 3 (ShareWithThess): only when selected NPC is `"canon_thess"` AND player `Knowledge.0.len() >= 2`

These guards live in `build_interaction` (where options are constructed), not in `execute_player_action`.

### Finding the right front by name
```rust
// Example lookup pattern
for mut front in fronts.iter_mut() {
    if front.name == "The Guild's Gambit" && front.active {
        front.countdown = front.countdown.saturating_add(4);
        break;
    }
}
```

### Random for Finn's risk
Use `rand::thread_rng().gen_bool(0.33)` — consistent with existing pattern in `execute_player_action`.

---

## What This Validates

After these three actions are implemented, a playthrough can demonstrate:
1. Player learns about the Guild (rumor from Finn or Otto or Lena)
2. Player chooses to warn Lena → Guild's Gambit front slows
3. Player chooses to inform Canon Thess → Order power rises, mine front accelerates
4. Player can observe both effects via F11 state dump

That's the core loop proven. The player made two decisions under partial information, and the simulation visibly responded to both.
