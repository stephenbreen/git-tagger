#!/bin/bash
# Builds a throwaway repo with staging/prod release tags for docs/demo.tape.
# Usage: docs/make-demo-repo.sh <dir>
set -e
rm -rf "$1" && mkdir -p "$1" && cd "$1"
git init -q -b main && git config user.name "Stephen Breen" && git config user.email demo@example.com
day=1
mk(){ d="2026-0${M}-$(printf %02d $day)T10:00:00"; export GIT_AUTHOR_DATE=$d GIT_COMMITTER_DATE=$d; echo "$1" >> CHANGELOG; git add -A; git commit -qm "$1"; day=$((day+1)); }
tg(){ git tag -a "$1" -m "$2"; }
M=6
mk "chore: scaffold telemetry ingest service"
mk "feat(ingest): accept batched GPS frames"
mk "feat(api): vehicle position endpoint"
tg v1.2.0-staging "Release 1.2.0 to staging"; tg v1.2.0-prod "Release 1.2.0 to production"
mk "fix(ingest): drop frames with future timestamps"
mk "feat(api): battery state-of-health endpoint"
mk "perf(db): partition positions table by day"
tg v1.3.0-staging "Release 1.3.0 to staging"
mk "fix(api): paginate fleet listing"
tg v1.3.0-prod "Release 1.3.0 to production (with pagination hotfix)"
M=7; day=2
mk "feat(alerts): low-battery threshold alerts"
mk "refactor(ingest): move parsing into codec crate"
git tag v1.4.0-rc.1
mk "fix(alerts): debounce repeated threshold crossings"
mk "feat(api): charging session history"
mk "chore(deps): bump tokio to 1.48"
tg v1.4.0-staging "Release 1.4.0 to staging"
cat > git-tagger.toml <<'TOML'
highlights = [
  { pattern = "-staging", color = "green" },
  { pattern = "-prod", color = "red" },
  { pattern = "-rc", color = "yellow" },
]
TOML
