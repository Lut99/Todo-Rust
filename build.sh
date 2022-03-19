# BUILD.sh
#   by Lut99
#
# Created:
#   19 Mar 2022, 16:43:59
# Last edited:
#   19 Mar 2022, 16:48:44
# Auto updated?
#   Yes
#
# Description:
#   Builds the server part of the Todo project.
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



# If it does not exist, build OpenSSL
if [[ ! -f "./target/openssl/lib/libssl.a" ]]; then
    # Build the image
    exec_cmd docker build --load -t openssl-build -f Dockerfile.ssl .

    # Compile the OpenSSL library
    exec_cmd docker run --attach STDIN --attach STDOUT --attach STDERR --rm -v "$(pwd):/build" openssl-build

    # Restore the permissions
    exec_cmd sudo chown -R "$USER":"$USER" ./target

    # Done with the OpenSSL build
fi

# Build the project for the Docker kind
exec_cmd rustup target add x86_64-unknown-linux-musl
exec_cmd export OPENSSL_DIR="$(pwd)/target/openssl"
exec_cmd export OPENSSL_LIB_DIR="$OPENSSL_DIR/lib"
exec_cmd cargo build --release --target x86_64-unknown-linux-musl --package todo-auth

# Build the images we'll be using
version=$(awk -F ' = ' '$1 ~ /version/ { gsub(/["]/, "", $2); printf("%s",$2) }' todo-auth/Cargo.toml)
exec_cmd docker build --load -t "todo-auth:$version" -f Dockerfile.auth .
