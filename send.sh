# SEND.sh
#   by Lut99
#
# Created:
#   19 Mar 2022, 10:47:42
# Last edited:
#   19 Mar 2022, 11:41:44
# Auto updated?
#   Yes
#
# Description:
#   Simple script that sends the relevant binaries off to a given server.
#


# Runs a command while saying what its running
exec_cmd() {
    # Print the command that is run
    cmd=""
    for arg in "$@"; do
        if [[ "$arg" =~ ( ) ]]; then
            arg="\"$arg\""
        fi
        cmd="$cmd $arg"
    done
    echo " >$cmd"

    # Run the command
    "$@" || exit $?
}





# Parse the CLI
if [[ "$#" -ne 1 ]]; then
    echo "Usage: $0 <registry>"
    exit 1
fi
registry="$1"



# Build the project for the Docker kind
exec_cmd rustup target add x86_64-unknown-linux-musl
exec_cmd cargo build --release --target x86_64-unknown-linux-musl --package todo-auth

# Build the images we'll be using
version=$(awk -F ' = ' '$1 ~ /version/ { gsub(/["]/, "", $2); printf("%s",$2) }' todo-auth/Cargo.toml)
exec_cmd docker build --load -t "$registry/todo-auth:$version" -f Dockerfile.auth .

# Send it over using docker's push
exec_cmd docker push "$registry/todo-auth:$version"

# Done
echo ""
echo "Done; don't forget to force refresh the image on the docker daemon in question."
echo ""
