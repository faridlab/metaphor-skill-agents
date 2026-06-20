# 5 · Review

**Goal:** catch defects, security holes, and needless complexity before the change merges — an
independent second opinion on correctness, quality, and risk.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `code-review-and-quality` | skill | Five-dimension review: correctness, readability, architecture, security, perf | You want a structured, consistent review standard | `metaphor agent install code-review-and-quality` |
| `code-reviewer` | agent | Independent PR/diff reviewer; flags real issues with `file:line` | You want a diff reviewed and concrete issues surfaced | `metaphor agent install code-reviewer` |
| `security-and-hardening` | skill | Secure coding, input validation, hardening | The change touches auth, input handling, or sensitive data | `metaphor agent install security-and-hardening` |
| `security-auditor` | agent | OWASP-style diff audit, exploit-focused findings | You want the diff checked for exploitable weaknesses | `metaphor agent install security-auditor` |
| `code-simplification` | skill | Reduce complexity, remove dead code, collapse abstractions | The review surfaces accidental complexity to cut | `metaphor agent install code-simplification` |
| `perf-analyzer` | agent | Hot-path and allocation analysis, impact-ranked fixes | The change might regress performance | `metaphor agent install perf-analyzer` |

## Recommended flow

1. **Run the gate.** `metaphor lint check` runs clippy + rustfmt + cargo-audit first — clear the
   automated findings before human-style review.
2. **Review the diff.** Use the `code-reviewer` agent (backed by `code-review-and-quality`) for an
   independent pass across correctness, readability, architecture, security, and performance.
3. **Audit security.** For anything touching auth, input, or sensitive data, run `security-auditor`
   (backed by `security-and-hardening`) for an exploit-focused look.
4. **Cut complexity.** Apply `code-simplification` to anything the review flags as needlessly
   complex; run `perf-analyzer` if the change could regress hot paths.

## Hand-off

Reviewed and clean → move to [6 · Documentation](6-documentation.md), then
[7 · Deployment](7-deployment.md).
