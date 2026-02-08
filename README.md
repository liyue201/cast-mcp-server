# Cast MCP Server

A Model Context Protocol (MCP) server that provides AI assistants with access to [cast](https://book.getfoundry.sh/cast/) tools.

cast is a command-line tool from the Foundry toolkit for interacting with Ethereum blockchain. This MCP server encapsulates cast's functionality as tools that can be called by AI models.

## Features

### Blockchain Tools
- `chain`: Get the symbolic name of the current chain
- `chain_id`: Get the chain ID of the current chain
- `client`: Get the current client version
- `block`: Block-related test tool

### Utility Tools
- `ping`: Simple connectivity test
- `max_int`: Get maximum value for integer types
- `min_int`: Get minimum value for integer types
- `max_uint`: Get maximum value for unsigned integer types
- `address_zero`: Get the zero address
- `hash_zero`: Get the zero hash

## Quick Start

### Prerequisites

- Rust 1.91.0 or higher
- AI client that supports MCP protocol (such as Claude Desktop)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd cast-mcp-server

# Build the project
make build
# Or use cargo directly
cargo build --release
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
# Set log level (default: DEBUG)
RUST_LOG=info

# RPC endpoint configuration (default: http://localhost:8545)
RPC_URL=https://mainnet.infura.io/v3/YOUR_PROJECT_ID
```

### Usage Example

Configure this server in an MCP-enabled AI client:

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

## Development

### Code Formatting

```bash
# Format code (requires nightly version)
make fmt
# Or
cargo +nightly fmt
```

### Clean Build

```bash
make clean
# Or
cargo clean
```

## Project Structure

```
cast-mcp-server/
├── src/
│   ├── common/
│   │   ├── block_tools.rs    # Block-related tools
│   │   ├── chain_tools.rs    # Chain-related tools
│   │   ├── mod.rs           # Module declarations
│   │   ├── server.rs        # Main server logic
│   │   └── utility_tools.rs # Utility tools
│   └── main.rs              # Program entry point
├── Cargo.toml               # Project dependencies
├── Makefile                 # Build scripts
├── README.md                # This document
├── rust-toolchain.toml      # Rust toolchain configuration
└── rustfmt.toml             # Code formatting configuration
```

## API Documentation

### Chain Tools

#### `chain`
Gets the symbolic name of the currently connected blockchain network.

**Parameters:**
- `rpc` (optional): RPC endpoint URL, defaults to `http://localhost:8545`

**Returns:**
Blockchain network name (e.g., "ethereum", "polygon", etc.)

#### `chain_id`
Gets the chain ID of the current chain.

**Parameters:**
- `rpc` (optional): RPC endpoint URL, defaults to `http://localhost:8545`

**Returns:**
Chain ID number

#### `client`
Gets RPC client version information.

**Parameters:**
- `rpc` (optional): RPC endpoint URL, defaults to `http://localhost:8545`

**Returns:**
Client version string

### Utility Tools

#### `max_int`
Gets the maximum value for a specified integer type.

**Parameters:**
- `type`: Integer type, possible values: `int8`, `int16`, `int32`, `int64`, `int256`

**Returns:**
Maximum value for the corresponding type

#### `min_int`
Gets the minimum value for a specified integer type.

**Parameters:**
- `type`: Integer type, possible values: `int8`, `int16`, `int32`, `int64`, `int256`

**Returns:**
Minimum value for the corresponding type

#### `max_uint`
Gets the maximum value for a specified unsigned integer type.

**Parameters:**
- `type`: Unsigned integer type, possible values: `uint8`, `uint16`, `uint32`, `uint64`, `uint256`

**Returns:**
Maximum value for the corresponding type

## Contributing

Issues and Pull Requests are welcome!

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Links

- [MCP Official Documentation](https://modelcontextprotocol.io/)
- [Foundry Documentation](https://book.getfoundry.sh/)
- [Cast Tool Documentation](https://book.getfoundry.sh/cast/)
