# RUN.sh
#   by Lut99
#
# Created:
#   19 Mar 2022, 11:52:00
# Last edited:
#   19 Mar 2022, 17:20:34
# Auto updated?
#   Yes
#
# Description:
#   Simply script that runs the server-side of the Todo-Rust project.
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





# Determine whether to start or stop
mode="start"
version=""
if [[ "$#" -eq 1 ]]; then
    if [[ "$1" == "start" ]]; then
        echo "Usage: $0 start <version>"
        exit 1
    elif [[ "$1" == "stop" ]]; then
        mode="$1"
    else
        # We assume it's the version
        version="$1"
    fi
elif [[ "$#" -eq 2 ]]; then
    if [[ "$1" != "start" ]]; then
        echo "Only 'start' can have a version"
        exit 1
    fi
    version="$2"
elif [[ "$#" -ne 0 ]]; then
    echo "Usage: $0 [<version>|start <version>|stop]"
    exit 1
fi



# Switch on the mode
if [[ "$mode" == "start" ]]; then
    # Pull new ones
    exec_cmd docker pull "server.timinc.nl:5000/todo-auth:$version"
    exec_cmd docker tag "server.timinc.nl:5000/todo-auth:$version" "todo-auth:run"

    # Start everything
    exec_cmd docker-compose -p todo -f ./todo-server.yml up -d || exit $?
else
    # Stop everything
    exec_cmd docker-compose -p todo -f ./todo-server.yml down || exit $?
fi
