# 8 · Maintenance

**Goal:** keep a live system healthy and evolving — migrate and deprecate safely, fix what breaks,
keep it fast and secure, and help new people understand it.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `deprecation-and-migration` | skill | Safely deprecate APIs and migrate callers | You're retiring or changing a surface other code depends on | `metaphor agent install deprecation-and-migration` |
| `database-migration-specialist` | skill | PostgreSQL migrations via Backbone tooling | You need a schema/data migration in production | `metaphor agent install database-migration-specialist` |
| `crate-maintainer` | skill | Shared crate versioning and integration | You're releasing a new crate version or bumping deps | `metaphor agent install crate-maintainer` |
| `apps-maintainer` | skill | App lifecycle, module integration, health monitoring | You're keeping a running app healthy over time | `metaphor agent install apps-maintainer` |
| `debugging-and-error-recovery` | skill | Root-cause analysis, reproduction, recovery | A production incident needs diagnosing | `metaphor agent install debugging-and-error-recovery` |
| `debugger` | agent | Root-causes failures with reproduction, proof, and a verified fix | You want an incident fixed end-to-end | `metaphor agent install debugger` |
| `performance-optimization` | skill | Measure-first perf improvements | A regression or growth has made something slow | `metaphor agent install performance-optimization` |
| `perf-analyzer` | agent | Hot-path and allocation analysis, impact-ranked fixes | You need the new slow paths found and ranked | `metaphor agent install perf-analyzer` |
| `security-and-hardening` | skill | Secure coding, input validation, hardening | A vulnerability or audit finding needs addressing | `metaphor agent install security-and-hardening` |
| `security-auditor` | agent | OWASP-style audit, exploit-focused findings | You're auditing live code for weaknesses | `metaphor agent install security-auditor` |
| `onboarding-explainer` | agent | Explains a module/service to a new engineer, outside-in | A new maintainer needs to understand the system | `metaphor agent install onboarding-explainer` |
| `refactorer` | agent | Targeted, behavior-preserving refactors | Accumulated cruft needs cleaning without behavior change | `metaphor agent install refactorer` |

Cross-cutting: commit each maintenance change with `grouped-commits` / `commit-generator`. See the
[index](README.md#cross-cutting-tools).

## Recommended flow

1. **Stay diagnostic.** Run `metaphor doctor` for tooling/health and
   `metaphor test --affected --base=main` to catch regressions as code changes.
2. **Fix incidents** with the `debugger` agent — reproduce, prove the root cause, verify the fix.
3. **Evolve safely.** Use `deprecation-and-migration` to retire surfaces without breaking callers,
   and `database-migration-specialist` for schema/data changes.
4. **Keep it fast and secure** with `perf-analyzer` and `security-auditor` on a recurring basis.
5. **Version and onboard.** Use `crate-maintainer` / `apps-maintainer` for releases and lifecycle,
   `refactorer` for safe cleanup, and `onboarding-explainer` to bring new maintainers up to speed.

## Hand-off

New work discovered while maintaining → loop back to [1 · Ideation](1-ideation.md) or
[2 · Planning & Design](2-planning-and-design.md).
