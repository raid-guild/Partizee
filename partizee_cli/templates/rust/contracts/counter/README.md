# Counter Contract

A simple counter smart contract for Partisia Blockchain that demonstrates basic state management.

## Overview

This contract maintains a counter that can be incremented by a specified amount.

## State

The contract stores two values:
- `counter`: The current count value (starts at 0)
- `increment_amount`: How much to increase the counter by each time (default: 5)

## Actions

- **Initialize**: Sets up the contract with initial values
- **Increment Counter**: Increases the counter by the increment amount

## Example Usage

To increment the counter:
```rust
// Call the increment action from your client
client.call_contract(contract_address, "increment_counter", {});
```

After calling this action, the counter will increase by the increment amount (default 5).
