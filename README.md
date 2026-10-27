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

![alt text](https://github.com/masoumehhsn/ProcessTracker/blob/main/Screenshot%20from%202026-10-23%2021-59-32.png)


Requires a Linux system with `/proc` available.
