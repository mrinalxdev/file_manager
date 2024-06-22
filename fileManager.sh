#!/bin/bash

# Function to display the menu
show_menu() {
    clear
    echo "============================"
    echo " Enhanced Bash File Manager"
    echo "============================"
    echo "1. List files"
    echo "2. Copy file"
    echo "3. Move file"
    echo "4. Delete file"
    echo "5. View file"
    echo "6. Create directory"
    echo "7. Rename file"
    echo "8. Display disk usage"
    echo "9. Search for a file"
    echo "10. Quit"
    echo "============================"
    echo -n "Please choose an option [1 - 10]: "
}

# Function to list files
list_files() {
    echo "Listing files in the current directory:"
    ls -lh
    echo "Press Enter to continue..."
    read
}

# Function to copy files
copy_file() {
    echo -n "Enter the source file: "
    read source
    echo -n "Enter the destination: "
    read destination
    cp "$source" "$destination" && echo "File copied successfully." || echo "Failed to copy file."
    echo "Press Enter to continue..."
    read
}

# Function to move files
move_file() {
    echo -n "Enter the source file: "
    read source
    echo -n "Enter the destination: "
    read destination
    mv "$source" "$destination" && echo "File moved successfully." || echo "Failed to move file."
    echo "Press Enter to continue..."
    read
}

# Function to delete files
delete_file() {
    echo -n "Enter the file to delete: "
    read file
    rm "$file" && echo "File deleted successfully." || echo "Failed to delete file."
    echo "Press Enter to continue..."
    read
}

# Function to view a file
view_file() {
    echo -n "Enter the file to view: "
    read file
    if [ -f "$file" ]; then
        less "$file"
    else
        echo "File does not exist."
    fi
    echo "Press Enter to continue..."
    read
}

# Function to create a directory
create_directory() {
    echo -n "Enter the name of the directory to create: "
    read dir
    mkdir -p "$dir" && echo "Directory created successfully." || echo "Failed to create directory."
    echo "Press Enter to continue..."
    read
}

# Function to rename a file
rename_file() {
    echo -n "Enter the current file name: "
    read old_name
    echo -n "Enter the new file name: "
    read new_name
    mv "$old_name" "$new_name" && echo "File renamed successfully." || echo "Failed to rename file."
    echo "Press Enter to continue..."
    read
}

# Function to display disk usage
display_disk_usage() {
    echo "Displaying disk usage:"
    df -h
    echo "Press Enter to continue..."
    read
}

# Function to search for a file
search_file() {
    echo -n "Enter the name of the file to search for: "
    read file_name
    echo "Searching for $file_name:"
    find . -name "$file_name"
    echo "Press Enter to continue..."
    read
}

# Main loop
while true; do
    show_menu
    read choice
    case $choice in
        1)
            list_files
            ;;
        2)
            copy_file
            ;;
        3)
            move_file
            ;;
        4)
            delete_file
            ;;
        5)
            view_file
            ;;
        6)
            create_directory
            ;;
        7)
            rename_file
            ;;
        8)
            display_disk_usage
            ;;
        9)
            search_file
            ;;
        10)
            echo "Exiting the file manager. Goodbye!"
            exit 0
            ;;
        *)
            echo "Invalid option, please try again."
            echo "Press Enter to continue..."
            read
            ;;
    esac
done
