#!/bin/bash
set -e

# Usage: ./release.sh [version]
# Example: ./release.sh 0.1.1

if [ -z "$1" ]; then
    echo "Error: Please provide a version number (e.g., 0.1.1)"
    exit 1
fi

VERSION=$1
TAG="v$VERSION"

# Ensure working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo "Error: Working directory is not clean. Commit changes first."
    exit 1
fi

echo "Creating tag $TAG..."
git tag -a "$TAG" -m "Release $TAG"

echo "Pushing tag to origin..."
git push origin "$TAG"

echo "Done! You can now draft a release on GitHub: https://github.com/kowshik24/configsync/releases/new?tag=$TAG"
