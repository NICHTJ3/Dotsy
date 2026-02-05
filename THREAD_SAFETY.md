# Thread Safety Documentation

## Current State

Dotsy is currently designed for single-threaded execution. However, the architecture has been designed with future concurrent execution in mind.

## Thread-Safe Components

### Plugin System
- All plugins must implement `Send + Sync`
- This ensures plugins can be safely shared across threads
- Plugins can be executed in parallel in the future

### Immutable Operations
Most operations in Dotsy are read-only or create new data:
- Configuration loading is read-only
- Path operations are stateless
- File operations are isolated

## Non-Thread-Safe Components

### Global Configuration
- `DotsyConfig` is passed by reference and not shared across threads
- Each operation receives its own copy or reference

### File System Operations
- File system operations are inherently not thread-safe at the OS level
- Creating symlinks, directories should not be done concurrently on the same paths

## Future Enhancements for Concurrency

### Recommended Patterns

1. **Shared Configuration**
   ```rust
   use std::sync::Arc;
   let config = Arc::new(load_rcfile()?);
   ```

2. **Mutable State with Mutex**
   ```rust
   use std::sync::Mutex;
   let state = Arc::new(Mutex::new(State::new()));
   ```

3. **Read-Heavy Scenarios with RwLock**
   ```rust
   use std::sync::RwLock;
   let cache = Arc::new(RwLock::new(HashMap::new()));
   ```

### Parallel Operations

For operations that can be parallelized:

```rust
use rayon::prelude::*;

links.par_iter()
    .for_each(|link| {
        // Process links in parallel
        link_file(link, config);
    });
```

## Audit Checklist

- [x] No global mutable state
- [x] Plugin traits require Send + Sync
- [x] File operations are isolated
- [ ] Add Mutex/RwLock for shared mutable state (when needed)
- [ ] Implement parallel glob processing
- [ ] Add concurrent package installation

## Best Practices

1. **Minimize Shared State**: Keep operations independent
2. **Use Message Passing**: Consider channels for inter-component communication
3. **Avoid Unwrap**: Always handle errors properly
4. **Test Concurrency**: Add tests for concurrent scenarios when implemented
