### Partizee bootstrapped project
This project was bootstrapped with [Partizee](https://github.com/raid-guild/partizee).

## Project Structure

```
project_name/
├── rust/                  # Smart contract template
│   ├── contracts/         # Contract source files
│   │   └── counter/
│   └── src/               # Rust main entry
│       └── main.rs
├── frontend/              # Frontend application template
│   ├── public/            # Static assets
│   └── src/               # Frontend source code
│       ├── app/
│       ├── context/
│       ├── hooks/
│       ├── types/
│       └── utils/
├── .gitignore.template
├── Cargo.template.toml
├── LICENSE.template
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
cd packages/frontend
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