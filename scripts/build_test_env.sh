#!/usr/bin/env bash

set -euo pipefail
test_env_dir="${1:-.test-env}"

if [ -d "$test_env_dir" ]; then
    echo "$test_env_dir already exists - run 'just clean-test-env' first if you want a fresh one."
    exit 0
fi
mkdir -p "$test_env_dir/Assets"
mkdir -p "$test_env_dir/ProjectSettings"
mkdir -p "$test_env_dir/Packages"
cat > "$test_env_dir/ProjectSettings/ProjectSettings.asset" <<'EOF'
%YAML 1.1
%TAG !u! tag:unity3d.com,2011:
--- !u!129 &1
PlayerSettings:
  companyName: DefaultCompany
  productName: MyProject
EOF
cat > "$test_env_dir/Packages/manifest.json" <<'EOF'
{
  "dependencies": {}
}
EOF
echo "Created fake Unity project at $test_env_dir/"
echo "Try it: just doctor-check"