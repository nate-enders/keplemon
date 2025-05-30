#!/bin/bash

set -e

IMAGE_NAME="citra-space/keplemon"
CONTAINER_NAME="temp-wheel"
OUTPUT_DIR="./target"

USER_ID=$(id -u)
GROUP_ID=$(id -g)

echo "Creating temporary container..."
docker create --name $CONTAINER_NAME $IMAGE_NAME > /dev/null

echo "Copying wheel files to $OUTPUT_DIR..."
mkdir -p "$OUTPUT_DIR"
docker cp $CONTAINER_NAME:/io/target/wheels "$OUTPUT_DIR"

echo "Fixing ownership to UID=$USER_ID GID=$GROUP_ID..."
chown -R "$USER_ID:$GROUP_ID" "$OUTPUT_DIR"

echo "Removing temporary container..."
docker rm $CONTAINER_NAME > /dev/null

echo "Done. Wheels available in $OUTPUT_DIR/wheels"
