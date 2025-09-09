#!/usr/bin/env python3
import subprocess
import os

# Change to workspace directory
os.chdir('/workspace')

# Restore the deleted file
try:
    result = subprocess.run(['git', 'checkout', 'HEAD', '--', 'crates/chat-cli/src/cli/chat/mod.rs'], 
                          capture_output=True, text=True)
    print(f"Git restore result: {result.returncode}")
    print(f"Stdout: {result.stdout}")
    print(f"Stderr: {result.stderr}")
    
    # Check if file exists now
    if os.path.exists('crates/chat-cli/src/cli/chat/mod.rs'):
        print("File restored successfully!")
        
        # Fix typos using sed
        subprocess.run(['sed', '-i', 's/overral/overall/g', 'crates/chat-cli/src/cli/chat/mod.rs'])
        subprocess.run(['sed', '-i', 's/accidently/accidentally/g', 'crates/chat-cli/src/telemetry/util.rs'])
        subprocess.run(['sed', '-i', 's/accidently/accidentally/g', 'crates/fig_telemetry/src/util.rs'])
        
        print("Typos fixed!")
    else:
        print("File restoration failed")
        
except Exception as e:
    print(f"Error: {e}")