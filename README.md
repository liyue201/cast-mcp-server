# Cast MCP Server

A Model Context Protocol (MCP) server that provides AI assistants with access to [cast](https://book.getfoundry.sh/reference/cast/) tools from the Foundry toolkit.

Cast is a powerful command-line tool for interacting with Ethereum blockchain networks. This MCP server exposes cast's functionality as callable tools that can be used by AI models through the MCP protocol.

## Features

### Blockchain Tools
- `chain`: Get the symbolic name of the current chain
- `chain_id`: Get the chain ID of the current chain
- `client`: Get the current client version
- `age`: Get the timestamp of a block

### Utility Tools
- `ping`: Test tool that returns "pong"
- `max_int`: Get maximum value for signed integer types (int8, int16, int32, int64, int256)
- `min_int`: Get minimum value for signed integer types (int8, int16, int32, int64, int256)
- `max_uint`: Get maximum value for unsigned integer types (uint8, uint16, uint32, uint64, uint256)
- `address_zero`: Get the zero Ethereum address (0x0000000000000000000000000000000000000000)
- `hash_zero`: Get the zero hash (0x0000000000000000000000000000000000000000000000000000000000000000)

## Quick Start

### Prerequisites

- Rust 1.91.0 (managed automatically via rust-toolchain.toml)
- AI client that supports MCP protocol (such as Claude Desktop, Cursor, or other MCP-compatible clients)
- Access to an Ethereum RPC endpoint (default: http://localhost:8545)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd cast-mcp-server

# Build the project
make build
# Or use cargo directly
cargo build --release

# The executable will be located at:
# ./target/release/cast-mcp-server
```

### Running

```bash
# Run in development mode
cargo run

# Run in production mode
./target/release/cast-mcp-server
```

## Configuration

### Environment Variables

The server supports the following environment variable configurations:

```bash
# Set log level (options: trace, debug, info, warn, error)
# Default: debug
RUST_LOG=info

# Note: RPC endpoints are configured per-tool via parameters
# Default RPC endpoint: http://localhost:8545
```

### Usage Example

Configure this server in an MCP-enabled AI client:

#### Claude Desktop Configuration

Add to your Claude desktop configuration file:

```json
{
  "mcpServers": {
    "cast-mcp-server": {
      "command": "/path/to/cast-mcp-server/target/release/cast-mcp-server",
      "args": []
    }
  }
}
```

#### General MCP Client Configuration

For other MCP-compatible clients, configure the server with:
- **Command**: Path to the compiled binary
- **Arguments**: None required (empty array)
- **Working Directory**: Project root directory

## Development

### Code Formatting

```bash
# Format code (requires nightly toolchain)
make fmt
# Or
cargo +nightly fmt
```

### Linting

```bash
# Run clippy linter
cargo clippy

# Run clippy with all features and stricter warnings
cargo clippy --all-targets --all-features -- -D warnings
```

### Clean Build

```bash
make clean
# Or
cargo clean
```

### Testing

```bash
# Run tests (if available)
cargo test

# Run tests with verbose output
cargo test -- --nocapture
```

## Contributing

Contributions are welcome! Here's how you can help:

### Development Setup

1. Fork this repository
2. Clone your fork: `git clone https://github.com/your-username/cast-mcp-server.git`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes
5. Format your code: `make fmt`
6. Test your changes: `cargo test`
7. Commit your changes: `git commit -am 'Add amazing feature'`
8. Push to the branch: `git push origin feature/amazing-feature`
9. Open a Pull Request

### Code Standards

- Follow the existing code style (enforced by rustfmt)
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass before submitting PR

### Reporting Issues

Please include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment information (OS, Rust version, etc.)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Foundry](https://github.com/foundry-rs/foundry) - Ethereum development toolkit
- [MCP Specification](https://modelcontextprotocol.io/) - Model Context Protocol
- [Rust](https://www.rust-lang.org/) - Systems programming language

## Related Links

- [MCP Official Documentation](https://modelcontextprotocol.io/)
- [Foundry Documentation](https://book.getfoundry.sh/)
- [Cast Tool Documentation](https://book.getfoundry.sh/cast/)
