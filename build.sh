#!/bin/bash
# SETUP:
#export GITHUB_TOKEN=your_github_token
#export GITHUB_USERNAME=your_github_username

# USAGE:
# ./build.sh linux/arm64 0.0.3 probate_api probate_api

if [ "$#" -ne 4 ]; then
    echo "Usage: $0 <platform> <version> <api_name> <path_to_dockerfile>"
    exit 1
fi

PLATFORM=$1
VERSION=$2
API_NAME=$3
DOCKERFILE_PATH=$4

TAG="ghcr.io/tyler-harpool/poc-ocfs/${API_NAME}:${VERSION}-$(echo $PLATFORM | tr '/' '-')"

# Ensure podman is logged in to GitHub Container Registry
echo ""
echo "Logging in to GitHub Container Registry..."
echo $GITHUB_TOKEN | podman login ghcr.io -u $GITHUB_USERNAME --password-stdin

if [ $? -ne 0 ]; then
    echo ""
    echo "Login failed!"
    exit 1
fi

echo ""
echo "Building image for $API_NAME on platform $PLATFORM..."

if podman build --platform "$PLATFORM" -t "$TAG" "$DOCKERFILE_PATH"; then
    echo ""
    echo "Build successful!"
    echo "Pushing image to registry..."

    if podman push "$TAG"; then
        echo ""
        echo "Push successful!"
    else
        echo ""
        echo "Push failed!"
        exit 2
    fi
else
    echo ""
    echo "Build failed!"
    exit 1
fi
