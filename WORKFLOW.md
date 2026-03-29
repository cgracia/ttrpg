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

## Issue Tracking — GitHub Issues

**Tasks and findings are tracked as GitHub Issues.**
Repo: https://github.com/cgracia/ttrpg

### Useful queries (via `gh` CLI)

```bash
# All open tasks
gh issue list --label task

# Open tasks for a specific skill
gh issue list --label "task,skill:architect"

# Current sprint tasks
gh issue list --label "task,sprint:current"

# All open findings
gh issue list --label finding

# Open findings by type
gh issue list --label "finding,bug"
gh issue list --label "finding,balance"

# Full history (open + closed)
gh issue list --label task --state all
gh issue list --label finding --state all

# View a specific issue
gh issue view <number>
```

### Labels

**Type**: `task`, `finding`
**Finding subtype**: `bug`, `balance`, `ux`, `design`
**Skill assignment**: `skill:architect`, `skill:worldbuild`, `skill:balance`, `skill:designer`, `skill:playtest`, `skill:narrative`, `skill:art`
**Priority**: `priority:high`, `priority:medium`, `priority:low`
**Severity**: `severity:critical`, `severity:major`, `severity:minor`
**Sprint**: `sprint:current`

### Creating a new finding

```bash
gh issue create \
  --title "[BUG-NNN] Short description" \
  --label "finding,bug,severity:major" \
  --body "## Observed\n...\n## Impact\n...\n## Fix\n..."
```

Use `BUG-`, `BAL-`, `UX-`, or `DESIGN-` prefix in the title. Number sequentially from the last issue of that type.

### Creating a new task

```bash
gh issue create \
  --title "[TASK-NNN] Short description" \
  --label "task,skill:architect,priority:medium" \
  --body "## What\n...\n## Why\n...\n## Acceptance Criteria\n..."
```

### Closing a finding (fixed) or task (done)

```bash
gh issue close <number> --comment "Done. [brief note on what was done]"
```

### Adding to current sprint

```bash
gh issue edit <number> --add-label "sprint:current"
```

---

## Session Log

`findings/SESSIONS.md` — append-only log of every skill session. This stays local.
Format: date, skill, tasks completed, key observations.

The individual `findings/BUG-NNN-*.md` and `tasks/TASK-NNN-*.md` files are historical
artifacts — kept for reference but GitHub Issues are the source of truth.

---

## Typical Flows

### Bug found in playtest
1. `playtest` creates a GitHub Issue: `gh issue create --label "finding,bug,severity:major"`
2. `playtest` or Carlos creates a task issue: `gh issue create --label "task,skill:architect,priority:high"`
3. `architect` works on it, closes the task: `gh issue close <number>`
4. `playtest` verifies, closes the finding: `gh issue close <number> --comment "Verified fixed."`

### Designer sets direction
1. `designer` runs, checks open issues: `gh issue list --label finding`
2. Creates task issues for follow-up skills
3. Skills pick up tasks: `gh issue list --label "task,skill:worldbuild"`

### Worldbuild discovers a content gap
1. `worldbuild` creates a finding issue: `gh issue create --label "finding,design,severity:minor"`
2. Optionally creates a task issue for itself or designer

---

## Environment Notes (Sway/Wayland)

The game requires `nix-shell shell.nix` to launch (X11 libs needed for Bevy).

**The dev machine runs Sway (Wayland)**. Bevy launches as a native Wayland window.

- **State dump (F11)**: Primary AI observation tool. Always reliable.
  1. Start ydotool daemon: `nix-shell -p ydotool --run "ydotoold" &`
  2. Focus game: `SWAYSOCK=/run/user/1000/sway-ipc.*.sock swaymsg '[title="Ashenveil"]' focus`
  3. Send F11: `YDOTOOL_SOCKET=/run/user/1000/.ydotool_socket nix-shell -p ydotool --run "ydotool key 87:1 87:0"`

- **Screenshots (F12)**: Currently broken on Wayland (BUG-002 / issue #2). `scrot` captures
  XWayland (`:0`) which has no content from the Wayland-native Bevy window, but exits 0 so
  the fallback is never reached. Fix pending (TASK-006 / issue #12).
  **Workaround**: use `grim` if available: `grim screenshots/latest.png`

---

## Review Convention

Carlos does periodic reviews. At review time:
```bash
gh issue list --label finding          # open findings
gh issue list --label "sprint:current" # current sprint status
```

Skills should never close a finding `wont-fix` or a task `wont-do` without a comment
explaining why — Carlos may disagree.
