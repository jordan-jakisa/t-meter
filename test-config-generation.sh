#!/bin/bash
# Test script to verify config generation

echo "=== Testing Config Auto-Generation ==="
echo

# Remove existing config
echo "1. Removing existing config file..."
rm -f ~/.config/t-meter/config.json
echo "   ✓ Config removed"
echo

# Run t-meter briefly to trigger config generation
echo "2. Running t-meter to trigger config generation..."
cd /Users/jordan/projects/rust/t-meter/t-meter
cargo run 2>&1 | grep -E "(Generated|Loaded|Warning)" &
PID=$!
sleep 2
pkill -P $$ t-meter 2>/dev/null || true
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
echo

# Check if config was created
echo "3. Checking if config file was created..."
if [ -f ~/.config/t-meter/config.json ]; then
    echo "   ✓ Config file created successfully!"
    echo
    echo "4. Config file contents:"
    cat ~/.config/t-meter/config.json
else
    echo "   ✗ Config file was NOT created"
    echo "   Checking directory..."
    ls -la ~/.config/t-meter/ 2>&1
fi
