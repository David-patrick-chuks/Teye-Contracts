#!/bin/bash
# Pre-commit check script to ensure code quality before pushing
# Run this script before committing or pushing code.

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🚀 Starting Pre-Commit Quality Checks...${NC}"
echo "================================================="

# 1. Format Check
echo ""
echo -e "${YELLOW}1️⃣  Checking Code Formatting...${NC}"
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Formatting looks good!${NC}"
else
    echo -e "${RED}✗ Formatting issues found!${NC}"
    echo "Run 'cargo fmt --all' to fix them."
    exit 1
fi

# 2. Cargo Check (Strict)
echo ""
echo -e "${YELLOW}2️⃣  Running Strict Compile Check...${NC}"
export RUSTFLAGS="-D warnings"
if cargo check --all --all-targets; then
    echo -e "${GREEN}✓ Code compiles without warnings!${NC}"
else
    echo -e "${RED}✗ Compilation failed or has warnings!${NC}"
    exit 1
fi

# 3. Clippy Linting
echo ""
echo -e "${YELLOW}3️⃣  Running Clippy Lints...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}✓ Clippy is happy!${NC}"
else
    echo -e "${RED}✗ Clippy found issues!${NC}"
    exit 1
fi

# 4. Tests
echo ""
echo -e "${YELLOW}4️⃣  Running Tests...${NC}"
if cargo test --all; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
else
    echo -e "${RED}✗ Tests failed!${NC}"
    exit 1
fi

echo ""
echo "================================================="
echo -e "${GREEN}🎉 CONGRATULATIONS! ALL CHECKS PASSED.${NC}"
echo "You are ready to push your professional grade code."
