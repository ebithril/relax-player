#!/bin/bash

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ $# -eq 0 ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION=$1

# Validate version format (semantic versioning: X.Y.Z)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version must be in semantic versioning format: X.Y.Z (e.g., 0.2.0)"
    exit 1
fi

# Check for uncommitted changes
if [[ -n $(git status -s) ]]; then
    echo -e "${RED}Error: Working directory is not clean${NC}"
    echo "Please commit or stash your changes before creating a release"
    git status -s
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
echo -e "${YELLOW}Current version: ${CURRENT_VERSION}${NC}"
echo -e "${YELLOW}New version: ${NEW_VERSION}${NC}"

# Confirm with user
read -p "Proceed with version bump? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Release cancelled"
    exit 0
fi

# Update version in Cargo.toml
echo -e "${GREEN}Updating Cargo.toml...${NC}"
sed -i "s/^version = \"${CURRENT_VERSION}\"/version = \"${NEW_VERSION}\"/" Cargo.toml

# Verify the change
NEW_CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
if [ "$NEW_CARGO_VERSION" != "$NEW_VERSION" ]; then
    echo -e "${RED}Error: Failed to update Cargo.toml${NC}"
    git checkout Cargo.toml
    exit 1
fi

# Run cargo build to update Cargo.lock and verify
echo -e "${GREEN}Running cargo build...${NC}"
if ! cargo build --release; then
    echo -e "${RED}Error: Build failed${NC}"
    git checkout Cargo.toml
    exit 1
fi

# Commit the version change
echo -e "${GREEN}Creating commit...${NC}"
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to ${NEW_VERSION}"

# Create git tag
echo -e "${GREEN}Creating tag v${NEW_VERSION}...${NC}"
git tag -a "v${NEW_VERSION}" -m "Release version ${NEW_VERSION}"

# Push to origin
echo -e "${GREEN}Pushing to origin...${NC}"
git push origin main
git push origin "v${NEW_VERSION}"

echo -e "${GREEN}✓ Release v${NEW_VERSION} created successfully!${NC}"
echo -e "${GREEN}✓ GitHub Actions will now build and publish the release${NC}"
