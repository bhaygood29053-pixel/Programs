#!/usr/bin/env bash
set -euo pipefail

yarn install
anchor build
anchor test --skip-build
