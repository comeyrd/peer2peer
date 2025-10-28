# Build Instructions

## Build All Components

```bash
cargo build
```

Builds the entire workspace (client, server, and shared library).

## Build Individual Components

```bash
# Build only the client
cargo build -p client

# Build only the server
cargo build -p server
```

## Run Components

```bash
# Run the client
cargo run -p client

# Run the server
cargo run -p server
```

## Build for Release

```bash
# Build all components optimized
cargo build --release

# Build specific component optimized
cargo build -p client --release
cargo build -p server --release
```

## Additional Commands

```bash
# Check code without building
cargo check

# Run tests
cargo test

# Clean build artifacts
cargo clean
```

## Output Locations

Compiled binaries are located in:
- Debug: `target/debug/client` and `target/debug/server`
- Release: `target/release/client` and `target/release/server`