# ProcessTracker

A small Rust CLI for watching Linux processes in real time.

It reads process data from `/proc` and shows:
- PID
- process name
- state
- memory usage
- CPU usage

## Run

```bash
cargo run
```

Requires a Linux system with `/proc` available.
