# MemoLens

A tool for analyzing LLM chat conversations stored in SQLite databases.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Building the Project

Clone the repository and build using Cargo:

```sh
git clone <repository-url>
cd memo_lens
cargo build --release
```

The compiled binaries will be available in `target/release/`.

## Running the Tools

### Main Application (Conversation Analysis)

This tool displays all conversations and responses from your LLM chat database:

```sh
# Using cargo
cargo run -- path/to/your/database.sqlite

# Or using the binary directly
./target/release/memo_lens path/to/your/database.sqlite
```

### SQLite Schema Discovery Tool

This tool helps you inspect the schema of any SQLite database:

```sh
# Using cargo with the specific binary
cargo run --bin sqlite_schema_discovery -- path/to/your/database.sqlite

# Or using the binary directly
./target/release/sqlite_schema_discovery path/to/your/database.sqlite
```

## Example Output

### Main Application
```
Reading conversations...

Conversation: Conversation { id: "conv_123", title: "Sample Chat" }
Response ID: 1
DateTime: 2024-01-01T12:00:00Z
Model: gpt-4
Prompt: What is Rust?
Response: Rust is a systems programming language...
Tokens: in=10, out=50
```

### Schema Discovery Tool
```
Schema for database: chat.sqlite
----------------------------
-- Schema for table 'conversations':
CREATE TABLE conversations (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL
)
```

## License

[Your chosen license]