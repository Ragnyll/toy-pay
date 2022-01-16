# toy-pay
A simple toy payment system representing transactions occurring off of a block chain

## Build instructions
**Minimum Supported Rust Version:**: 1.58

```
cargo run -- transactions.csv > output.csv
```

## Assumptions
- An invalid transaction should not halt the program. It should be ignored and continue.
- Overdrafts (withdrawing past balance to go negative) is allowed [[1]].


## Testing Methodology

[1]: https://overdraftapps.com/can-i-withdraw-money-if-my-account-is-overdrawn/
