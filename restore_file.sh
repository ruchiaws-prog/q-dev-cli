#!/bin/bash
cd /workspace
git status
git checkout HEAD -- crates/chat-cli/src/cli/chat/mod.rs
echo "File restoration completed"