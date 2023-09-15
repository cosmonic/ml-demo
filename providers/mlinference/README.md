# mlinference capability provider

Build with 'make'. Test with 'cargo test'.

## Test procedure

1. Start nats, e.g. `nats-server --jetstream`
2. from `providers/mlinference` do `cargo test`
3. modify a few log lines in code
4. kill orphaned processes, e.g. by `pkill -f mlinference`
5. from `providers/mlinference` do `cargo test`
6. observe that changes are NOT taken into account
