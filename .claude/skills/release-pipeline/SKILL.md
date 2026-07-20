---
name: release-pipeline
description: How to cut and publish a new curate-draw release — CI builds, signs, notarizes, and publishes to GitHub Releases; the website picks it up automatically. Use when asked to release, ship, publish, or bump the version of curate-draw.
---

# Curate Draw release pipeline

Releases build entirely in GitHub Actions. Nothing runs locally.

## Cutting a release

1. Bump `"version"` in `package.json`.
2. Commit + push to `main`.
3. Tag and push:
   ```bash
   git tag v0.1.11 && git push origin v0.1.11
   ```
4. CI (`.github/workflows/release.yml`) builds a universal `.dmg`, signs,
   notarizes, publishes a public GitHub Release, then pings the
   `CF_DEPLOY_HOOK_URL` Cloudflare deploy hook so
   jan-hendrik-mueller.de rebuilds and shows the new version automatically.

## Required repo secrets

`APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_ID`,
`APPLE_PASSWORD`, `APPLE_TEAM_ID`, `CF_DEPLOY_HOOK_URL` — already set via `gh secret set <NAME>`.

