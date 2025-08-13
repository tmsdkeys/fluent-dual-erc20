# ERC20 Interop Project

This project demonstrates a multi-contract architecture using Fluent's client generation system for type-safe cross-contract interactions.

## Project Structure

```
erc20-interop/
├── contracts/
│   └── token-factory/            # Factory service for creating tokens
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # Factory contract entrypoint
│           ├── factory.rs        # Factory logic
│           └── registry.rs       # Token tracking
│
├── interfaces/                  # Shared trait definitions (lib crate)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── ierc20.rs            # ERC20 interface trait
│       ├── ifactory.rs          # Factory interface trait
│       └── igovernance.rs       # Governance interface trait
│
├── shared-types/                # Common data structures (lib crate)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── token_metadata.rs
│       └── governance_types.rs
│
├── src/
│   ├── MyToken.sol              # Original Solidity token
│   └── rust-token/              # Rust token implementation
│
└── Cargo.toml                   # Workspace configuration
```

## Architecture Overview

This project implements a **service-oriented architecture** where:

1. **Each contract is independently deployable** as a separate WASM module
2. **Cross-contract communication** happens through generated clients using trait interfaces
3. **Shared types and interfaces** are defined in separate library crates
4. **Type safety** is maintained through compile-time validation

## Key Components

### Shared Types (`shared-types/`)
- Common data structures used across contracts
- `TokenMetadata`: Token name, symbol, and decimals
- `Proposal`: Governance proposal structure

### Interfaces (`interfaces/`)
- Trait definitions with `#[client(mode = "solidity")]` attributes
- Auto-generates client implementations for cross-contract calls
- `IERC20`: Standard ERC20 token interface
- `IFactory`: Factory contract interface
- `IGovernance`: Governance contract interface

### Token Factory (`contracts/token-factory/`)
- Factory service for creating and managing tokens
- Uses generated clients to interact with deployed tokens
- Maintains a registry of created tokens

## Building

```bash
# Build all crates
cargo build

# Build specific contract
cargo build -p token-factory

# Build shared libraries
cargo build -p shared-types
cargo build -p token-interfaces
```

## Development Status

This is a **scaffolded structure** - the actual implementation logic is marked with TODO comments. The focus is on establishing the architectural patterns for:

- Multi-contract development
- Type-safe cross-contract interactions
- Shared interface definitions
- Service-oriented contract architecture

## Next Steps

1. Implement the actual token creation logic in `factory.rs`
2. Add proper error handling and validation
3. Implement the token registry functionality
4. Add tests for cross-contract interactions
5. Consider adding more sophisticated governance features
