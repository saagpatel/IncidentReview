#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$branch" == "HEAD" && -n "${GITHUB_HEAD_REF:-}" ]]; then
  branch="$GITHUB_HEAD_REF"
elif [[ "$branch" == "HEAD" && "${GITHUB_REF_TYPE:-}" == "branch" && -n "${GITHUB_REF_NAME:-}" ]]; then
  branch="$GITHUB_REF_NAME"
fi
pattern='^codex/(feat|fix|chore|refactor|docs|test|perf|ci|spike|hotfix)/[a-z0-9]+(-[a-z0-9]+)*$'

if [[ "$branch" == "main" || "$branch" == "master" ]]; then
  echo "Direct work on $branch is blocked."
  exit 1
fi

if ! [[ "$branch" =~ $pattern ]]; then
  echo "Invalid branch: $branch"
  echo "Expected: codex/<type>/<slug>"
  exit 1
fi
