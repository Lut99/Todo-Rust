# SEND.sh
#   by Lut99
#
# Created:
#   19 Mar 2022, 10:47:42
# Last edited:
#   19 Mar 2022, 17:15:27
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
port=5000
if [[ "$#" -eq 2 ]]; then
    port="$2"
elif [[ "$#" -ne 1 ]]; then
    echo "Usage: $0 <registry> [<registry_port>]"
    exit 1
fi
registry="$1"



# Run build first
exec_cmd ./build.sh

# Send it over using docker's push
version=$(awk -F ' = ' '$1 ~ /version/ { gsub(/["]/, "", $2); printf("%s",$2) }' todo-auth/Cargo.toml)
exec_cmd docker tag "todo-auth:$version" "$registry:$port/todo-auth:$version"
exec_cmd docker push "$registry:$port/todo-auth:$version"

# Also send the auxillary files
exec_cmd scp ./run.sh "$registry:Rust/Todo-Rust/"
exec_cmd scp ./todo-server.yml "$registry:Rust/Todo-Rust/"

# Done
echo ""
echo "Done; don't forget to force refresh the image on the docker daemon in question."
echo ""
