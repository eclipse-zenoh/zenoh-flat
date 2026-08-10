#!/usr/bin/env bash

set -xeo pipefail

# Release number
readonly version=${VERSION:?input VERSION is required}
# Dependencies' pattern
readonly bump_deps_pattern=${BUMP_DEPS_PATTERN:-''}
# Dependencies' version
readonly bump_deps_version=${BUMP_DEPS_VERSION:-''}
# Dependencies' git branch
readonly bump_deps_branch=${BUMP_DEPS_BRANCH:-''}
# Git actor name
readonly git_user_name=${GIT_USER_NAME:?input GIT_USER_NAME is required}
# Git actor email
readonly git_user_email=${GIT_USER_EMAIL:?input GIT_USER_EMAIL is required}

# Install toml-cli if not present
ensure_toml_cli() {
    if ! command -v toml &> /dev/null; then
        echo "Installing toml-cli2..."
        cargo +stable install toml-cli2
    fi
}

# toml-cli doesn't support in-place modification
# See: https://github.com/gnprice/toml-cli?tab=readme-ov-file#writing-ish-toml-set
toml_set_in_place() {
    local file="$1"
    local key="$2"
    local value="$3"
    local tmp
    tmp=$(mktemp)
    toml set "$file" "$key" "$value" > "$tmp"
    mv "$tmp" "$file"
}

ensure_toml_cli

export GIT_AUTHOR_NAME=$git_user_name
export GIT_AUTHOR_EMAIL=$git_user_email
export GIT_COMMITTER_NAME=$git_user_name
export GIT_COMMITTER_EMAIL=$git_user_email

# Bump Cargo version
toml_set_in_place Cargo.toml "package.version" "$version"

# Cargo.lock records the crate's *own* version, so it goes stale the instant the
# manifest is bumped, and any later `--locked` build refuses to run. `cargo
# metadata` re-resolves and rewrites the lockfile minimally without compiling.
cargo metadata --format-version 1 > /dev/null

# Show the changes to be committed
git diff Cargo.toml Cargo.lock
git commit Cargo.toml Cargo.lock -m "chore: Bump version to \`$version\`"

# Select all dependencies that match $bump_deps_pattern and bump them to $bump_deps_version.
#
# Both tables are walked, as zenoh-c does: a binding whose generator is a
# build-dependency must move the two copies together. Nothing matches
# `zenoh.*` under `build-dependencies` here today, but keeping the loop makes
# this script a sibling of zenoh-flat-jni's rather than a variant of it.
if [[ "$bump_deps_pattern" != '' ]]; then
  for deps_key in "dependencies" "build-dependencies"; do
    deps=$(toml get Cargo.toml "$deps_key" | jq -r "keys[] | select(test(\"$bump_deps_pattern\"))")
    for dep in $deps; do
      if [[ -n $bump_deps_version ]]; then
        toml_set_in_place Cargo.toml "$deps_key.$dep.version" "$bump_deps_version"
      fi

      if [[ -n $bump_deps_branch ]]; then
        toml_set_in_place Cargo.toml "$deps_key.$dep.branch" "$bump_deps_branch"
      fi
    done
  done

  # Update the lockfile.
  #
  # NOTE: deliberately `cargo check` and not zenoh-c's `cargo generate-lockfile`.
  # `generate-lockfile` re-resolves the whole graph to the newest semver-
  # compatible versions, which would silently undo the Cargo.lock sync with
  # zenoh — the ABI alignment this lockfile exists for. `cargo check` updates
  # the lock minimally, keeping every pin the sync established.
  cargo check

  if [[ -n $bump_deps_version || -n $bump_deps_branch ]]; then
    # Show the changes to be committed
    git diff Cargo.toml Cargo.lock
    git commit Cargo.toml Cargo.lock -m "chore: Bump \`$bump_deps_pattern\` dependencies to \`$bump_deps_version\`"
  else
    echo "warn: no changes have been made to any dependencies matching $bump_deps_pattern"
  fi
fi

git log -10
git push origin

# This script does not tag. The release workflow tags the commit it leaves here
# only once that commit has passed validation, so a release tag never points at
# something that was not checked.
