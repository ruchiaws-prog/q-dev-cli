#!/bin/bash
cd /workspace

# First restore the deleted file
echo "Restoring deleted file..."
git checkout HEAD -- crates/chat-cli/src/cli/chat/mod.rs

# Check if file was restored
if [ -f "crates/chat-cli/src/cli/chat/mod.rs" ]; then
    echo "File restored successfully"
    
    # Fix the typos using sed
    echo "Fixing typos..."
    
    # Fix "overral" to "overall" in mod.rs
    sed -i 's/overral/overall/g' crates/chat-cli/src/cli/chat/mod.rs
    
    # Fix "accidently" to "accidentally" in both util.rs files
    sed -i 's/accidently/accidentally/g' crates/chat-cli/src/telemetry/util.rs
    sed -i 's/accidently/accidentally/g' crates/fig_telemetry/src/util.rs
    
    echo "Typos fixed successfully"
    
    # Verify the changes
    echo "Verifying changes..."
    grep -n "overall" crates/chat-cli/src/cli/chat/mod.rs | head -5
    grep -n "accidentally" crates/chat-cli/src/telemetry/util.rs | head -5
    grep -n "accidentally" crates/fig_telemetry/src/util.rs | head -5
    
else
    echo "Failed to restore file"
fi