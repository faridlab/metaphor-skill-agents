---
name: orchestrate-agents
description: Run a task as a multi-agent fleet via the Workflow tool — a project-manager agent sequencing priority, concurrent worker tracks (spec→impl chains, verification legs, read-only recons), and a steelman→skeptic→chair council at the end. Use when the user asks to orchestrate this with multiple agents, use a workflow, fan out agents, run a multi-agent workflow, spawn a fleet, wants one agent as project manager to prioritize the rest, names an agent count ("10 agents"), or invokes /orchestrate-agents.
---

# Orchestrate agents — the fleet pattern

Invoking this skill IS the user's explicit opt-in to multi-agent orchestration (workflows are never
inferred from a task that merely looks parallelizable). Build and launch a Workflow; never run the
tracks inline in the main conversation.

## 1. Fix the shape before writing the script

Confirm (or default) three things — ask only if the prompt left them genuinely ambiguous; otherwise
state the chosen shape and proceed:

- **Size** — the user's number wins ("10 agents may be" → ~10). Default small: 4–6 for ordinary
  tasks; 8–12 only when the user names a number or the task spans several independent tracks.
- **Structure** — default is the proven shape: **PM first, then concurrent tracks, then council**.
  When the user names a seat ("one as project manager which tasks need to be prioritized"), that
  seat runs FIRST and its output gates the rest.
- **Budget** — a "+500k"-style directive is a hard ceiling the script must respect; without one,
  size from track count.

## 2. The proven fleet shape (adapt, don't inflate)

```
phase Prioritize   PM: priority order across tracks + GO/NO-GO gate per track + guidance strings
phase Tracks       pipeline(): independent tracks run concurrently;
                   spec→impl chains are ONE pipeline per track (impl consumes spec's output —
                   no barrier between tracks; live verification legs and read-only recons are single agents)
phase Council      steelman → skeptic → chair, sequential, each seat sees the prior seat's verdict
```

Skeleton to adapt:

```js
export const meta = {
  name: 'task-fleet',
  description: 'PM-sequenced worker tracks with a closing council',
  phases: [{ title: 'Prioritize' }, { title: 'Tracks' }, { title: 'Council' }],
}
const REPORT = { type: 'object', properties: { report: { type: 'string' } }, required: ['report'] }
const PM_SCHEMA = { /* order: string[], gates: {<track>: boolean}, guidance: {<track>: string} */ }

phase('Prioritize')
const pm = await agent(PM_PROMPT, { label: 'pm:priorities', phase: 'Prioritize', schema: PM_SCHEMA })

phase('Tracks')
const results = await pipeline(
  TRACKS,
  t => t.implementable === false ? { report: `${t.key} gated off` } : null,
  // …per-track: single agent, or a spec→impl chain as nested awaits in one stage callback
)

phase('Council')
const steelman = await agent(STEELMAN_PROMPT(results), { label: 'council:steelman', phase: 'Council', schema: COUNCIL_SCHEMA })
const skeptic  = await agent(SKEPTIC_PROMPT(results, steelman.verdict), { label: 'council:skeptic', phase: 'Council', schema: COUNCIL_SCHEMA })
const chair    = await agent(CHAIR_PROMPT(results, steelman.verdict, skeptic.verdict), { label: 'council:chair', phase: 'Council', schema: COUNCIL_SCHEMA })
return { pm, results, council: { steelman, skeptic, chair } }
```

Rules of construction:

- **Structured output schemas on every agent** — a `report: string` field at minimum; schemas for
  gates (`implementable: boolean`), PM ordering, and council verdicts. No free-text parsing.
- **Gates are data**: a recon agent returns `implementable: false` → the impl stage is skipped by
  the script, not by judgment.
- **pipeline() over parallel()** — a barrier is only right when a later stage needs ALL prior
  results (the council does; the tracks don't).
- The council sees the agents' actual reports, not summaries-of-summaries. The chair adjudicates —
  it does not average.
- If this pack's council agents are installed, the three seats can run as
  `agentType: 'council-steelman' | 'council-skeptic' | 'council-chair'` instead of inline prompts.

## 3. House rules — pack into EVERY agent prompt

Build one `HOUSE_RULES` string and interpolate it into each prompt, covering at minimum:

- Read the repo's own CLAUDE.md first; the more local one wins on conflict.
- Repo discipline as it applies here (read-only trees, where module work belongs, build entry
  points). Fill these from the repo's own rules before launching.
- **NO commits, NO tags, NO pushes** — workers leave changes uncommitted; the orchestrator
  serializes review → tests → user-in-the-loop for anything outbound.
- Comments, docs, and commit-style discipline in generic self-contained language — nothing that
  only the current session can decode.
- Scope fidelity: do only your track; report findings, don't fix outside your lane.

## 4. Orchestration discipline (the main conversation's job)

- Launch in the **background**; do NOT poll — the completion notification re-invokes you. While it
  runs, do your own non-overlapping work (plan-doc recording, reviews outside the fleet's trees).
- If the user asks mid-run: a non-blocking TaskOutput status check plus a peek at
  `<transcriptDir>/journal.jsonl` (agent keys started vs returned) is fine. **Never fabricate a
  pending agent's results.**
- If the returned result looks empty or wrong: read `journal.jsonl` — it carries each agent's real
  return value — before concluding anything.
- On completion: relay the PM ordering, per-track outcomes, and council verdict — then the
  serialized after-fleet sequence: review the uncommitted diff, run the test suites, and bring
  every commit/tag/ship decision back to the user before executing.

## 5. When NOT to build a fleet

One agent (or plain inline work) is right for: single-file fixes, one-question lookups, tasks with
one lane and no verification layer. Don't spend ten agents on a todo item — the user asked for
orchestration of *this* task, and scale should track the task, not the vocabulary.
