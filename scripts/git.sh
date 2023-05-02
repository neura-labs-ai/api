#!/usr/bin/bash

DEFAULT_COMMIT_MSG="updated"

# Ask for the commit message
read -p "Enter commit message (press enter to use default message '$DEFAULT_COMMIT_MSG'): " COMMIT_MSG

# Use the default commit message if none is provided
if [[ -z "$COMMIT_MSG" ]]; then
  COMMIT_MSG="$DEFAULT_COMMIT_MSG"
fi

# Add all files to the staging area
if ! git add .; then
  echo "Error: Failed to add files to staging area."
  exit 1
fi

# Commit changes with the commit message
if ! git commit -m "$COMMIT_MSG"; then
  echo "Error: Failed to commit changes."
  exit 1
fi

# Push changes to the default branch of the origin remote
if ! git push origin HEAD; then
  echo "Error: Failed to push changes to Git repository."
  exit 1
fi

echo "Code successfully added, committed, and pushed to Git repository."
