#!/bin/bash

# Fix typo 1: overral -> overall
sed -i 's/overral/overall/g' /workspace/crates/chat-cli/src/cli/chat/mod.rs

# Fix typo 2: accidently -> accidentally  
sed -i 's/accidently/accidentally/g' /workspace/crates/chat-cli/src/telemetry/util.rs

# Fix typo 3: accidently -> accidentally
sed -i 's/accidently/accidentally/g' /workspace/crates/fig_telemetry/src/util.rs

echo "Typos fixed!"