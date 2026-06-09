#!/bin/bash

# Grab the arguments
action=$1
input_filename=$2

# A single, reusable function for all MP4 conversions
convert_to_mp4() {
    # Check if the input file actually exists
    if [ ! -f "$input_filename" ]; then
        echo "Error: File '$input_filename' not found."
        exit 1
    fi

    # Strip the original extension to get just the base name
    local base_name="${input_filename%.*}"
    
    echo "Converting '$input_filename' to '${base_name}.mp4'..."
    ffmpeg -i "$input_filename" -c:v libx264 -crf 23 -c:a aac -b:a 192k "${base_name}.mp4"
}

# Ensure the user actually provided a second argument (the file)
check_file_arg() {
    if [ -z "$input_filename" ]; then
        echo "Error: Please provide an input filename."
        echo "Usage: $0 $action <filename>"
        exit 1
    fi
}

# The case statement now handles avi, wmv, mpg, and mpeg
case "$action" in
    mp4|*mp4)
        check_file_arg
        convert_to_mp4
        ;;
    *)
        echo "Usage: $0 {command} <filename>"
        echo "Commands:"
        echo "  to_mp4"
        ;;
esac
