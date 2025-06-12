# Upgradable v3

Upgradable example WASM smart contract with hash-based governance. This
contract demonstrates that contracts can be terminators: They can allow upgrade
to itself, but not upgrades from itself.

Contract can upgrade from `counterV2`, but cannot be upgraded further.

## About upgrade governance

This contract is an example, and does not reflect what good upgrade logic for a
contract should look like. Please read documentation page for [upgradable smart
contracts](https://partisiablockchain.gitlab.io/documentation/smart-contracts/upgradable-smart-contracts.html)
for suggestion of how to implement the upgrade governance.
