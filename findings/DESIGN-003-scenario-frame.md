---
name: DESIGN-003 — Scenario Frame: The Ashenveil Brief
description: Prototype scenario frame — player identity, goal, the hidden conspiracy, NPC roles, and ending states. Prerequisite for TASK-016 (player action design).
type: project
---

# DESIGN-003: The Ashenveil Brief — Prototype Scenario Frame

**Status**: Design complete — feeds TASK-015 (#25), TASK-016 (#26)
**GitHub issue**: #25

---

## Logline

*You are a Crown investigator working undercover in a town that is being quietly consumed from within. You have until the pieces fall into place — or until they find you.*

---

## Player Identity

**Role**: The Envoy — a Crown investigator operating under deep cover in Ashenveil.

**Cover**: A traveling scribe from the capital, seeking to update trade records for the Crown's assessor office. Plausible cover for asking questions, visiting multiple locations, and talking to merchants.

**Starting resources**:
- Wealth: 20 coin (enough to survive; not enough to buy everyone)
- Cover: intact
- Knowledge: one contact name, one mission objective (see below)

---

## The Mission (what the Crown told you)

Three months ago, Ashenveil's trade revenue to the Crown fell sharply. A Crown assessor sent to investigate never returned. Last week, an anonymous report arrived by courier: something criminal is happening in Ashenveil, centered on the Merchant Guild.

Your orders:
1. Make contact with **Lena Marsh** at the River Docks. She sent the report.
2. Identify who is responsible and gather evidence.
3. Extract safely before the situation resolves.

You do not know who the conspirators are. You do not know what happened to the assessor. You do not know about the mine.

---

## The Hidden Truth (GM layer — not visible to player at start)

**The conspiracy**:

Aldric Voss (Merchant Guild) funds Sable (The Shadows) directly. In return, Sable provides enforcement: intimidating merchants who resist Guild tribute, silencing people who ask questions, and removing anyone who threatens the arrangement.

**The evidence trail**:
- Tomas Reed (Guild clerk) witnessed an unauthorized payment in the ledger — the entry was altered the next day. He's terrified.
- Lena Marsh has seen mismatched cargo manifests for three weeks running — Guild shipments that bypass Crown tolls.
- The old mine was sealed by Guild order after something was found inside: fragments of a Crown assessor's travel papers, and remains of miners who refused to comply with the tribute system.
- Vex arrived independently, chasing the same mine story. They have documents. Sable wants them dead.

**Mira Dent** (Guild broker) knows about the arrangement and facilitates the payments. She's cautious and greedy — she'll prioritize her own survival if pressed.

**Finn Crowe** (Shadow runner) knows enough to be useful but is primarily self-interested. Every time the player buys information from Finn, there's a small chance Finn mentions the exchange to Sable.

---

## Key NPCs as Player Resources

| NPC | Role for Player | Risk |
|-----|----------------|------|
| Lena Marsh | Initial contact; knows manifest fraud | Target of Iron Ledger front — if she's removed, player loses anchor |
| Tomas Reed | Best single evidence source (altered ledger) | Cowardly — won't talk unless he feels safe |
| Vex | Independent investigator; knows the mine | Doesn't trust easily; Sable wants them dead — proximity = exposure risk |
| Canon Thess | Credible testimony; knows Order is being dismantled | Limited knowledge; useful for legitimizing evidence |
| Finn Crowe | Information broker; knows a lot, sells to anyone | Each purchase risks a mention to Sable |
| Brega Halm | Trustworthy; can provide protection | Low cunning — limited intelligence value |
| Mira Dent | Knows the arrangement; potentially breakable | Will not talk unless the player has enough leverage |
| Aldric Voss | Primary antagonist | Rarely accessible; approaching him directly burns cover fast |
| Sable | Secondary antagonist | Unseen; becomes active when exposure rises |

---

## The Two Core Mechanics

These are design-level descriptions. Implementation is scoped to TASK-016 and beyond.

### 1. Evidence

Accumulated through specific player actions. Three tiers:

- **Rumor** (low credibility): Overheard, bought from Finn, passed through the rumor network. Useful but not Crown-admissible alone.
- **Testimony** (medium): An NPC who speaks directly to the player about what they witnessed. Requires trust-building.
- **Document/Artifact** (high): Physical proof — the altered ledger entry, cargo manifests, the mine contents, Vex's papers.

At T65, the player's evidence total (weighted by tier) determines the outcome bracket.

### 2. Exposure

The player's cover integrity. Starts intact (0). Certain actions raise it:
- Visiting the Back Alleys
- Pushing NPCs with high cunning too aggressively
- Being seen in the same location as Vex repeatedly
- Buying information from Finn Crowe
- Getting caught with evidence

**Threshold events**:
- Exposure ≥ medium: Finn Crowe mentions the player to Sable. Sable starts watching.
- Exposure ≥ high: A Shadow operative follows the player. Certain locations become dangerous.
- Exposure = max: Sable makes a direct move. The player must flee or fight.

Exposure can be reduced: maintaining cover through mundane interactions, avoiding Shadow-affiliated locations for several turns, bribing Finn to stay quiet.

---

## The Clock

The fronts provide the pressure. Key inflection points that affect player options:

| ~Turn | Event | Player Impact |
|-------|-------|---------------|
| T28 | Whispers final stage: Vex ambushed in the alleys | Vex is at risk — player may need to act |
| T45 | Guild's Gambit final stage: open conflict | Town becomes dangerous; certain locations restricted |
| T50 | Iron Ledger activates: tribute extraction begins | Lena faces removal; Guild locks down the market |
| T65 | Scenario end | Outcome determined |

---

## Ending States at T65

Determined by **evidence total** and **exposure level**:

| State | Condition | Description |
|-------|-----------|-------------|
| **Crown Justice** | High evidence + low exposure | Conspiracy documented, player extracts safely. Aldric faces Crown judgment. Ashenveil survives. |
| **Partial Case** | Medium evidence + any exposure | The Crown has something real but incomplete. Aldric is weakened but not finished. The player got out. |
| **Empty Hands** | Low evidence + low exposure | Nothing actionable found. Guild wins by default. The conspiracy succeeds. The player failed. |
| **Burned** | High exposure + any evidence | Sable identified the player. Forced to run. Evidence may or may not reach the Crown depending on how the player played it. |
| **Untimely End** | Max exposure + caught | The Crown's envoy disappeared in Ashenveil. Nobody knows what happened to them. |

---

## What This Scenario Validates

If the prototype loop works, a player should be able to:
1. Arrive not knowing who the bad guys are
2. Gather information through dialogue and observation
3. Make meaningful decisions about who to trust, what to pursue, when to act
4. Reach T65 with a different outcome based on those decisions
5. Look back at their choices and see the causal chain

That's the core fantasy. If that experience exists in this scenario, the game's thesis is proven.

---

## Open Questions (deferred)

- Does exposure reduce automatically over time (slow decay) or only through player action?
- Can the player "side with" the Guild deliberately? (Interesting long-term design, out of scope for prototype)
- What does "extracting safely" look like mechanically? (A location? A specific action? Automatic at T65?)
- Should Vex be a potential ally the player can actively protect, or remain passive?
