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
- Negative Deposits are not allowed (you can't insert debt into an ATM).
- Negative Withdraws are not allowed (you can't add money to an ATM by taking out debt).
- Any transaction made to a locked account is denied and therefore not tracked
- Disputing a currently disputed transaction is an invalid partner action and should be ignored
- The reversing a transaction part of a dispute is ignored in interest of the clause "this means the available funds should decrease by the amount disputed..."
- If there is an error in the input (ie: a dispute contains an amount) than the program should continue.

## Testing Methodology
- Unit tests for all individual functions
- test file for io validation

[1]: https://overdraftapps.com/can-i-withdraw-money-if-my-account-is-overdrawn/
