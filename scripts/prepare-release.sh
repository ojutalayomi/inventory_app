#!/bin/bash

# Script to prepare and create a new release
# Usage: ./scripts/prepare-release.sh <version>
# Example: ./scripts/prepare-release.sh 0.2.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number is required${NC}"
    echo "Usage: ./scripts/prepare-release.sh <version>"
    echo "Example: ./scripts/prepare-release.sh 0.2.0"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

# Validate version format (semantic versioning: x.y.z)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version must follow semantic versioning: MAJOR.MINOR.PATCH"
    echo "Example: 0.2.0"
    exit 1
fi

echo -e "${GREEN}Preparing release ${TAG}${NC}"

# Check if git repository is clean
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${YELLOW}Warning: Working directory is not clean${NC}"
    git status --short
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${YELLOW}Warning: Not on main branch (currently on: ${CURRENT_BRANCH})${NC}"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${RED}Error: Tag ${TAG} already exists${NC}"
    exit 1
fi

# Update version in Cargo.toml
echo -e "${GREEN}Updating version in Cargo.toml...${NC}"
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
else
    # Linux
    sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
fi

# Verify the change
if grep -q "version = \"${VERSION}\"" Cargo.toml; then
    echo -e "${GREEN}✓ Version updated successfully${NC}"
else
    echo -e "${RED}Error: Failed to update version in Cargo.toml${NC}"
    exit 1
fi

# Update Cargo.lock
echo -e "${GREEN}Updating Cargo.lock...${NC}"
cargo check --quiet

# Build to verify everything compiles
echo -e "${GREEN}Building project to verify...${NC}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}Error: Build failed${NC}"
    exit 1
fi

# Show summary
echo ""
echo -e "${GREEN}=== Release Summary ===${NC}"
echo "Version: ${VERSION}"
echo "Tag: ${TAG}"
echo ""
echo "Changes to be committed:"
git diff --stat Cargo.toml Cargo.lock

echo ""
read -p "Commit changes and create tag? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Release preparation cancelled${NC}"
    exit 0
fi

# Commit changes
echo -e "${GREEN}Committing changes...${NC}"
git add Cargo.toml Cargo.lock
git commit -m "Release version ${VERSION}"

# Create and push tag
echo -e "${GREEN}Creating and pushing tag ${TAG}...${NC}"
git tag -a "$TAG" -m "Release version ${VERSION}"

echo -e "${YELLOW}Ready to push. This will trigger the release workflow.${NC}"
read -p "Push to remote? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin main
    git push origin "$TAG"
    echo ""
    echo -e "${GREEN}✓ Release ${TAG} has been pushed!${NC}"
    echo -e "${GREEN}GitHub Actions will now build and create the release.${NC}"
    echo -e "Monitor progress at: https://github.com/ojutalayomi/inventory_app/actions"
else
    echo -e "${YELLOW}Tag created locally but not pushed.${NC}"
    echo "To push later, run:"
    echo "  git push origin main"
    echo "  git push origin ${TAG}"
fi

