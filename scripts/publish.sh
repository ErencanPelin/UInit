#!/usr/bin/env bash
set -euo pipefail

version=$(grep -m1 '^version' Cargo.toml | sed -E 's/version = "(.*)"/\1/')
tag="v${version}"

if [ -n "$(git status --porcelain)" ]; then
    echo "Working tree isn't clean - commit or stash changes before publishing."
    exit 1
fi

if git rev-parse "$tag" >/dev/null 2>&1; then
    echo "Tag $tag already exists locally - did you forget to bump the version in Cargo.toml?"
    exit 1
fi

if git rev-parse "$tag" >/dev/null 2>&1; then
    echo "Tag $tag already exists locally - did you forget to bump the version in Cargo.toml?"
    exit 1
fi

echo "Tagging and pushing $tag..."
git tag "$tag"
git push origin "$tag"
echo "Pushed $tag - release pipeline should be running now."