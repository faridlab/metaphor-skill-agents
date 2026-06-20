# 7 · Deployment

**Goal:** ship the change safely and repeatably — a sound release process, automated pipelines, and
the right infrastructure to run on.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `shipping-and-launch` | skill | Ship-readiness, rollout plans, launch checks | You're preparing a release and need a go/no-go checklist | `metaphor agent install shipping-and-launch` |
| `ci-cd-and-automation` | skill | CI/CD pipelines and automation patterns | You're building or fixing the pipeline that builds/tests/deploys | `metaphor agent install ci-cd-and-automation` |
| `cloud-infrastructure-architect` | skill | Multi-cloud design, cost optimization, security | You're provisioning or changing where it runs | `metaphor agent install cloud-infrastructure-architect` |

Cross-cutting: `git-workflow-and-versioning` for tagging/release branches, and `metaphor-cli-master`
for the deploy/compose commands. See the [index](README.md#cross-cutting-tools).

## Recommended flow

1. **Check readiness.** Walk `shipping-and-launch`'s ship-readiness and rollout checks before
   cutting a release.
2. **Automate the path.** Use `ci-cd-and-automation` to build the pipeline; in CI prefer
   `metaphor test --affected --base=main` and `metaphor build --all` so you only rebuild/retest
   what changed.
3. **Generate deployment artifacts.** Run `metaphor compose generate --write` to produce the
   cross-project docker-compose, and lean on `cloud-infrastructure-architect` for the target
   topology, cost, and security posture.
4. **Version and tag.** Use `git-workflow-and-versioning` to cut the release cleanly.

## Hand-off

Shipped → move to [8 · Maintenance](8-maintenance.md) to keep it healthy.
