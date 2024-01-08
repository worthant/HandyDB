# Архитектура HandyDB

```rust
HandyDB/
│
├── Cargo.toml
├── Cargo.lock
│
└── src/
    ├── main.rs                   // + Entry point, handles CLI arguments, inits components
    │
    ├── cli/                      // + CLI-related modules
    │   ├── command_manager.rs    // + Manages CLI commands
    │   ├── commands/             // + Individual command implementations
    │   │   ├── mod.rs            // + Command trait and common utilities
    │   │   ├── close.rs
    │   │   ├── help.rs
    │   │   ├── info.rs
    │   │   ├── lookup.rs         // New: Command to discover other shards
    │   │   ... other commands
    │   ├── mod.rs                // + CLI module
    │   └── utils.rs              // + CLI utility functions
    │
    ├── db/                       // Core database functionalities
    │   ├── mod.rs                // + Database module
    │   ├── storage.rs            // Handles disk storage, page management
    │   ├── kv_store.rs           // + Key-Value store logic
    │   ├── shard_manager.rs      // Manages sharding logic
    │   └── network_discovery.rs  // New: Discovers and communicates with other shards
    │
    ├── network/                  // + Networking capabilities
    │   ├── mod.rs                // + Networking module
    │   ├── server.rs             // + Request handling
    │   └── services/             // + HTTP services   
    │       ├── mod.rs
    │       ├── set_service.rs
    │       └── get_service.rs     
    │
    ├── utils/                    // Utility functions
    │   ├── mod.rs                // Utils module
    │   └── logger.rs             // Logging utilities
    │
    ├── models/                   // Data models
    │   ├── mod.rs                // + Models module
    │   ├── page.rs               // Page model for storage
    │   ├── kv_pair.rs            // + Key-Value pair model
    │   ├── shard.rs              // Shard data model
    │   └── shard_info.rs         // New: Model to store information about each shard
    │
    └── tests/                    // Unit tests
        ├── mod.rs                // + Tests module
        ├── storage_tests.rs
        ├── web_server_tests.rs   // + Sync-async tests for concurrent 'set' requests
        ├── async_tokio_tests.rs  // + Async tests for concurrent 'set' and 'get' requests
        ├── kv_store_tests.rs     // + Concurrent and Multithread tests for kv_store
        └── shard_tests.rs        // Tests for sharding
```
