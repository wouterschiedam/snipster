#!/bin/bash

# Check if the command is provided
if [ -z "$1" ]; then
    echo "No command provided."
    exit 1
fi

# Print the command to the terminal and place it in the input buffer
echo -n "$1" # Print without a newline
# Use ANSI escape code to move to the next line
echo -e "\033[K" # Clear to the end of the line

# Prompt the user
echo "Press Enter to execute this command in your terminal."
read -r
