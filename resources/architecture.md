# Архитектура HandyDB

```rust
HandyDB/
│
├── Cargo.toml
├── Cargo.lock
│
└── src/
    ├── main.rs                   // Entry point, inits and runs the command manager
    │
    ├── cli/                      // CLI-related modules
    │   ├── command_manager.rs    // Manages CLI commands
    │   ├── commands/             // Individual command implementations
    │   │   ├── mod.rs            // Command trait and common utilities
    │   │   ├── close.rs
    │   │   ├── help.rs
    │   │   ├── info.rs
    │   │   ... other commands
    │   ├── mod.rs              // CLI module
    │   └── utils.rs              // CLI utility functions
    │
    ├── db/                       // Core database functionalities
    │   ├── mod.rs
    │   ├── storage.rs            // Handles disk storage, page management
    │   ├── btree.rs              // B-Tree implementation for indexing
    │   ├── kv_store.rs           // Key-Value store logic
    │   ├── shard_manager.rs      // Manages sharding logic
    │   └── partition_manager.rs  // Manages partitioning logic
    │
    ├── network/                  // Networking capabilities
    │   ├── mod.rs
    │   └── server.rs             // HTTP server and request handling
    │
    ├── utils/                    // Utility functions
    │   ├── mod.rs
    │   ├── serializer.rs         // Serialization and deserialization utilities
    │   └── logger.rs             // Logging utilities
    │
    ├── models/                   // Data models
    │   ├── mod.rs
    │   ├── page.rs               // Page model for storage
    │   ├── btree_node.rs         // B-Tree node model
    │   ├── kv_pair.rs            // Key-Value pair model
    │   ├── shard.rs              // Shard data model
    │   └── partition.rs          // Partition data model
    │
    └── tests/                    // Unit tests
        ├── storage_tests.rs
        ├── btree_tests.rs
        ├── kv_store_tests.rs
        ├── shard_tests.rs        // Tests for sharding
        └── partition_tests.rs    // Tests for partitioning
```
