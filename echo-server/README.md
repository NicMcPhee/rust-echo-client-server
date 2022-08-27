# Echo server (no threads)

This is a simple implementation of an _echo server_ without threads.
Here the server listens (via a socket) on some port,
and sends back whatever content it receives.

We'll start with receiving and sending back text, but we'll want
to get up to receiving and sending back binary data, i.e., a
stream of uninterpreted bytes.
