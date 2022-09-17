# Echo client and server with threads

This is a simple implementation of an [_echo server_ 
and _echo client_](https://en.wikipedia.org/wiki/Echo_Protocol).

An _echo server_ listens for (TCP) connections, and
echos back whatever it receives on any connection until
the client closes the connection. The server uses
threads so that it can concurrently handle an arbitrary
number of connections.

The _echo client_ connects to an echo server, and
sends anything from `stdio` to the server, and writing
anything it receives from the server to `stdout`.

## Building and running

To build the client and server, clone this repository
and run

```text
cargo build
```

in the the `rust-echo-client-server` directory.

Then you can run the server with either:

```text
cargo run --bin echo_server
```

or

```text
target/debug/echo_server
```

Similar commands will start `echo_client`.

Because the server is threaded, you should be able to start
multiple clients in different shells and connect to the same
server at the same time. Those clients can also be on a variety
of machines as long as those machines have network access to
the machine running the server.

## Command line arguments

Both the client and server take command line arguments to specify
either the IP address or the port (or both) that you're connecting
listening on/connecting to. These default to `127.0.0.1`
(localhost) and `7707`. (The "standard" port for the echo
protocol is `7`, but many systems prevent user code from
listening on ports less than 1,000 without being root.)

The command line flags are:

* -i, --ip-address <IP_ADDRESS>    The IP address to connect to [default: 127.0.0.1]
* -p, --port <PORT>                The port number to connect to [default: 7707]

To specify these when using `cargo run` you need to add `--`
at the end of the command before providing any command line
flags, e.g.,

```text
cargo run --bin echo_server -- --port 60606
```

If you're running the binary directly (e.g., `target/debug/echo_server`), then you don't want the `--`, e.g.,

```text
target/debug/echo_server --port 60606
```

## Logging

The server provides some simple logging of connections. For
most shells, prefixing the command that starts the server
with `RUST_LOG=info `, e.g.,

```text
RUST_LOG=info cargo run --bin echo_server
```

will enable that logging.
