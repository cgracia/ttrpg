# Project Vision - *Working Title TBD*

**Status: Work in Progress / Aspirational Draft**

---

## 1. High-Level Vision

This project explores the idea of a **simulation-driven RPG**, where the player inhabits a living world filled with autonomous characters, evolving situations, and hidden information.

The core fantasy:

> *You are one person in a world that moves without you — but where your choices can meaningfully shape outcomes.*

The game blends:

* **TTRPG-inspired systems** (stats, traits, equipment, partial knowledge)
* **Systemic simulation** (NPC goals, relationships, factions, time progression)
* **Narrative intrigue** (dialogue, rumors, secrets, evolving situations)
* **Map-based exploration** (locations, travel, discovery)

---

## 2. Design Pillars

### 🧠 A Living World

The world evolves independently of the player:

* NPCs have goals, relationships, and routines
* Factions pursue their own interests
* Situations (“fronts”) progress over time

The player is not the center — but can become influential.

---

### 🔍 Partial Information & Discovery

Information is:

* incomplete
* biased
* contextual

The player learns through:

* dialogue
* observation
* rumor
* consequence

There is no “omniscient UI.”

---

### ⚖️ Meaningful Choices

Choices are:

* grounded in incomplete knowledge
* influenced by relationships and context
* capable of altering outcomes

Decisions should feel like:

* bets
* risks
* commitments

---

### 🧩 Simple Systems, Deep Interactions

Each system is intentionally simple:

* stats
* traits
* relationships
* locations
* time

Depth emerges from their interaction, not complexity.

---

### 🧱 Modular World Design

The world is built from:

* small, reusable modules
* scenarios (“fronts”)
* location sets
* NPC groups

This enables:

* easy expansion
* procedural variation
* experimentation

---

## 3. Player Experience

### Core Loop

> **Travel → Interact → Learn → Decide → Consequences → Repeat**

1. Move between locations on a map
2. Encounter NPCs or situations
3. Gather information (explicit or implicit)
4. Make decisions
5. Observe consequences over time

---

### Player Role

The player:

* controls a single character
* has stats, traits, and equipment
* builds relationships
* gathers knowledge
* navigates social and systemic dynamics

The player is:

* not omnipotent
* not predetermined
* not guaranteed success

---

## 4. World Structure

### Locations

* Represented as nodes on a map
* Contain NPCs, events, and interactions
* Examples:

  * tavern
  * market
  * manor
  * shrine
  * outskirts

---

### NPCs

Each NPC has:

* stats and traits
* goals and motivations
* relationships with others
* current location
* limited knowledge of the world

NPCs act over time.

---

### Factions

Groups with:

* shared interests
* internal/external relationships
* influence in locations

Factions create tension and context.

---

### Fronts (Core Concept)

A “front” is an evolving situation:

* progresses over time
* involves NPCs and factions
* has stages and outcomes

Examples:

* smuggling operation growing
* political power struggle
* hidden cult activity

Fronts drive the world forward.

---

## 5. Systems Overview (Conceptual)

### Time System

* Advances in discrete steps
* Triggers:

  * NPC actions
  * front progression
  * world updates

---

### NPC Behavior

* Simple decision-making
* Driven by:

  * goals
  * traits
  * context

No need for full simulation — only enough to feel alive.

---

### Knowledge & Rumors

* Information spreads imperfectly
* NPCs know different things
* Player must interpret signals

---

### Interaction System

* Dialogue (initially simple)
* Actions like:

  * talk
  * observe
  * investigate

Outcomes influenced by:

* stats
* relationships
* hidden state

---

### Equipment & Stats

* Minimal but meaningful
* Influence interactions and outcomes
* Not the primary focus (especially early)

---

## 6. Visual & UX Direction

### Presentation Style

* Minimalist 2D
* Map-based navigation
* UI-driven interactions

Focus on:

* clarity
* readability
* feedback

Not on:

* animation
* realism
* graphical fidelity

---

### UI Elements

* map view
* location panel
* NPC list
* interaction panel
* event/rumor log
* player sheet

---

## 7. Vertical Slice Goal

### Scope

A small, complete scenario:

* 1 town
* 5–8 locations
* 8–12 NPCs
* 2–3 factions
* 1 main front
* 1–2 side situations

---

### Success Criteria

The prototype succeeds if:

* The world evolves without player input
* NPCs behave differently from each other
* A situation progresses over time
* The player can discover and influence outcomes
* The experience is **interesting**, even if simple

---

## 8. Technical Direction (High-Level)

* Engine: **Bevy (Rust)**
* Architecture: ECS-based simulation
* Data-driven content (external files)
* Clear separation:

  * simulation
  * data
  * presentation

Focus on:

* simplicity
* inspectability
* iteration speed

---

## 9. Non-Goals (Important)

This project does NOT aim to:

* simulate everything realistically
* build a full open world
* create deep combat systems (initially)
* produce high-end visuals
* match AAA or grand strategy complexity

---

## 10. Risks & Unknowns

* Will the simulation produce interesting situations?
* Will the player understand what is happening?
* Will hidden information feel engaging or frustrating?
* Can simple systems generate meaningful depth?
* How much content is needed before it feels alive?

---

## 11. Guiding Principle

> Build something small that **feels alive**,
> then expand — not the other way around.

---

## 12. Next Step

Build a **minimal vertical slice** that proves:

* time progression
* NPC activity
* one evolving front
* player interaction with consequences

Everything else comes later.
