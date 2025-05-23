# {{project_name}}

This project was bootstrapped with [Partisi](https://github.com/MrDeadCe11/partizee).

## Project Structure

```
{{project_name}}/
├── packages/
│   ├── rust/           # Smart Contract
│   │   ├── contracts/  # Contract source files
│   │   ├── src/        # Contract tests
│   │   └── test/       # Contract tests
│   └── nodejs/         # Frontend application
```

## Getting Started

### Smart Contract Development

Navigate to the contract directory:
```bash
cd packages/rust
```

Build the contract:
```bash
partizee compile
```

Run contract tests:
```bash
cargo test
```

### Frontend Development

Navigate to the frontend directory:
```bash
cd packages/nodejs
```

Install dependencies:
```bash
npm install
# or
pnpm install
```

Start the development server:
```bash
npm run dev
# or
pnpm run dev
```

## Deployment

To deploy your contract to the Partisia blockchain:
```bash
partizee deploy
```

## Documentation

- [Partisia Blockchain Documentation](https://partisiablockchain.gitlab.io/documentation/index.html)
- [Smart Contract SDK](https://gitlab.com/partisiablockchain/language/contract-sdk)
- [Frontend SDK](https://gitlab.com/partisiablockchain/language/pbc-client)

## License

MIT