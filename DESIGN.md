# Dotsy Design Document

## Architecture Overview

Dotsy is a modular dotfile manager built with a focus on extensibility, maintainability, and clean architecture.

## Module Structure

### Core Modules

#### `configs`
Handles configuration file structures and serialization/deserialization.
- `DotsyConfig`: Global configuration
- `ProfileConfig`: Profile-specific configurations
- `ConfigConfig`: Individual config specifications
- `Link`: Symlink configuration

#### `error`
Centralized error handling using the Snafu library.
- All errors are well-defined with meaningful messages
- Errors include context about what went wrong and how to fix it

#### `handlers`
Operations handlers for different types of tasks:
- `link`: Symlink creation and removal
- `files`: Directory creation
- `package`: Package installation/uninstallation
- `script`: Shell script execution

### Utility Modules

#### `utils::path`
Path manipulation utilities:
- `absolute()`: Convert paths to absolute, expanding tildes
- `is_symlink()`: Check if a path is a symlink
- `link_exists()`: Check if a symlink exists

#### `utils::logger`
Centralized logging with consistent formatting:
- `Logger::error()`: Error messages in red
- `Logger::warning()`: Warning messages in yellow
- `Logger::info()`: Info messages in blue
- `Logger::success()`: Success messages in green

#### `utils::file_manager`
`FileManager` struct for all file system operations:
- `create_directory()`: Create directories with parents
- `create_symlink()`: Create symlinks with validation
- `remove_symlink()`: Remove symlinks safely
- Helper methods for checking file types

#### `utils::config_manager`
`ConfigManager` for configuration file operations:
- `load<T>()`: Load and deserialize configurations
- `save<T>()`: Serialize and save configurations
- `validate_path()`: Validate configuration file paths

#### `utils::builders`
Builder patterns for constructing configuration objects:
- `LinkBuilder`: Build Link configurations
- `ProfileConfigBuilder`: Build ProfileConfig objects
- `ConfigConfigBuilder`: Build ConfigConfig objects

### Profile Management

#### `profiles::profile_manager`
Encapsulates all profile-related operations:
- `install()`: Install a profile and its dependencies
- `uninstall()`: Uninstall a profile and clean up

### Plugin System

#### `plugins::plugin_trait`
Extensibility framework for adding custom functionality:
- `Plugin` trait: Core plugin interface
- `HandlerPlugin` trait: For custom handlers
- `ExamplePlugin`: Reference implementation

## Design Patterns

### Builder Pattern
Used for constructing complex configuration objects:
```rust
let link = LinkBuilder::new()
    .from("/source")
    .to("/target")
    .glob(true)
    .build();
```

### Manager Pattern
Centralized managers for related operations:
- `FileManager`: File operations
- `ConfigManager`: Configuration management
- `ProfileManager`: Profile operations

### Trait-Based Plugin System
Allows for dynamic extension without modifying core code:
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String]) -> DotsyResult<()>;
}
```

## Thread Safety

### Current State
The current implementation is primarily single-threaded. However, the following considerations are in place:

- Plugin traits require `Send + Sync` for future parallel execution
- No global mutable state exists
- All operations are isolated and can be made concurrent in the future

### Future Enhancements
For true thread-safety in concurrent scenarios:
- Use `Arc<Mutex<T>>` for shared mutable state
- Use `RwLock<T>` for read-heavy, write-light scenarios
- Consider using channels for message passing between components

## Error Handling

All errors use the Snafu library for structured error handling:
- Errors are enum variants with context
- Error messages are user-friendly
- Errors include actionable information

Example:
```rust
#[snafu(display("Failed to create symlink from {from} to {to}: {reason}"))]
CouldntCreateSymLink {
    from: PathBuf,
    to: PathBuf,
    reason: String,
}
```

## Testing Strategy

### Unit Tests
- Each utility module has its own test suite
- Builders are tested for correctness
- Plugin system is validated with example implementations

### Integration Tests
- Test full workflow scenarios
- Validate interactions between modules
- Test error handling paths

## Future Enhancements

### Planned Features
1. **Configuration Validation**: JSON schema validation for config files
2. **Batch Operations**: Optimize glob operations with parallel processing
3. **Plugin Loader**: Dynamic plugin loading from external crates
4. **Rollback Support**: Transaction-like operations with automatic rollback
5. **Dry Run Mode**: Preview operations without executing them
6. **Verbose Logging**: Configurable log levels
7. **Config Templates**: Predefined templates for common setups

### Performance Optimizations
- Lazy evaluation of glob patterns
- Parallel file operations where safe
- Caching of configuration files
- Incremental updates instead of full reinstalls

## Code Quality

### Tools in Use
- `rustfmt`: Code formatting (configured in rustfmt.toml)
- `clippy`: Linting and code quality checks
- Unit tests for all new modules

### Standards
- All public APIs are documented
- Error handling is comprehensive
- No unwrap() calls in production code paths (except where failure is truly impossible)
- Consistent naming conventions
