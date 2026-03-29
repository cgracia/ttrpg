# Ashenveil — AI Team Workflow

## Skills and Their Roles

| Skill | Role | Can Create | Can Close |
|-------|------|------------|-----------|
| **designer** | Creative direction, player experience | Tasks (for any skill), Findings (design) | Design findings |
| **architect** | ECS design, technical decisions | Tasks (technical), Findings (code/architecture) | Technical findings |
| **worldbuild** | NPCs, locations, factions, fronts, content | Tasks (content), Findings (content gaps) | Content findings |
| **playtest** | Build, run, observe, report | Tasks (from observations), Findings (bugs/balance/ux) | Any finding after verifying the fix |
| **planner** | Backlog review, sprint planning, task assignment | Tasks (reorganize/create), — | — |
| **narrative** | Dialogue, rumors, event text, world voice | Tasks (narrative work), Findings (design/narrative gaps) | Narrative findings |
| **balance** | Simulation tuning, pacing, emergent dynamics | Tasks (for architect), Findings (balance) | Balance findings |
| **art** | Visual direction, AI asset prompts, style consistency | Tasks (art sprints, UI work), Findings (ux/visual) | UX/visual findings |

The **designer** acts as the top of the creative hierarchy — it can create tasks for
any skill. Other skills can also create tasks for each other based on what they
discover. **planner** coordinates across all skills but does not implement anything.

Carlos reviews findings and tasks at any point. He doesn't need to approve individual
closures, but nothing is truly done until he's seen it.

---

## Two Artifact Types

### findings/ — Passive discoveries
Things observed to be wrong, missing, or worth questioning. Created during any skill
session. Closed by **playtest** after verifying the fix is in place, or by the
originating skill if it's a design/content issue.

```
findings/
├── INDEX.md              ← status table (one row per finding)
├── SESSIONS.md           ← append-only log of every skill session
├── BUG-NNN-title.md
├── BAL-NNN-title.md
├── UX-NNN-title.md
└── DESIGN-NNN-title.md
```

**Frontmatter fields**: `id`, `type` (bug/balance/ux/design), `severity`
(critical/major/minor/cosmetic), `status`, `session`, `found_by`

**Status flow**: `open` → `needs-verify` → `fixed` | `wont-fix`

---

### tasks/ — Active work items
Things that need to be built, written, or decided. Created by any skill or by Carlos.
Assigned to a specific skill. Worked on and closed by that skill (or reassigned).

```
tasks/
├── INDEX.md              ← status table (one row per task)
├── TASK-NNN-title.md
└── ...
```

**Frontmatter fields**: `id`, `assigned_to` (skill name), `priority`
(high/medium/low), `status`, `created_by`, `created_date`

**Status flow**: `open` → `in-progress` → `done` | `blocked` | `wont-do`

**Body structure**:
- What needs to be done
- Why (which finding or design goal it addresses)
- Acceptance criteria (how we know it's done)

---

## Typical Flows

### Bug found in playtest
1. `playtest` creates `findings/BUG-NNN.md` (status: open)
2. `playtest` or Carlos creates `tasks/TASK-NNN.md` assigned to `architect`
3. `architect` works on it, updates task to `done`
4. `playtest` runs again, verifies, updates finding to `fixed`

### Designer sets direction
1. `designer` runs, reviews current state
2. Creates `tasks/TASK-NNN.md` assigned to `worldbuild` ("add a third front")
3. Creates `tasks/TASK-NNN.md` assigned to `architect` ("implement front chaining")
4. Skills pick up tasks in subsequent sessions

### Worldbuild discovers a content gap
1. `worldbuild` creates `findings/DESIGN-NNN.md` (status: open)
2. Optionally creates a task for itself or designer to address it

---

## Review Convention

Carlos does periodic reviews. At review time:
- Check `findings/INDEX.md` for anything needing attention
- Check `tasks/INDEX.md` for blocked or stale items
- Update status on anything that needs human judgment

Skills should never mark something `wont-fix` or `wont-do` without a note explaining
why — Carlos may disagree.
