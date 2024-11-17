# journald-otel-collector
This project is an attempt to implement an otel collector gathering log entries from journald entries implemented in Rust.

## Prerequisites
The crate used for reading the journal entries relies on the following package:
````shell
sudo apt install libsystemd-dev
````