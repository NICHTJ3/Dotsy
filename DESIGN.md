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
- `plugin_handlers`: Plugin-based implementations of all handlers

**Plugin Integration**: All handlers are now available as plugins through `plugin_handlers`:
- `PackageHandlerPlugin`: Package installation/uninstallation
- `ScriptHandlerPlugin`: Shell script execution
- `LinkHandlerPlugin`: Symlink operations
- `DirectoryHandlerPlugin`: Directory creation

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

**Built-in Plugin Handlers**: All existing handlers implement the plugin interface:
```rust
// Use package handler as a plugin
let plugin = PackageHandlerPlugin::new(
    "brew install {}".to_string(),
    "brew uninstall {}".to_string(),
);
plugin.install("neovim")?;

// Use script handler as a plugin
let script_plugin = ScriptHandlerPlugin::new();
script_plugin.execute(&["echo 'Hello World'".to_string()])?;

// Use directory handler as a plugin
let dir_plugin = DirectoryHandlerPlugin::new();
dir_plugin.install("/path/to/directory")?;
```

This unified interface allows all handlers to be used interchangeably through the plugin system.

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

## Handler-Plugin Integration

All existing handlers have been migrated to implement the plugin trait interface, providing a unified way to interact with different operations:

### Available Handler Plugins

1. **PackageHandlerPlugin**
   - Manages package installation/uninstallation
   - Implements both `Plugin` and `HandlerPlugin` traits
   - Configurable install/uninstall commands

2. **ScriptHandlerPlugin**
   - Executes shell scripts
   - Implements `Plugin` trait
   - Cross-platform script execution

3. **LinkHandlerPlugin**
   - Creates and manages symlinks
   - Implements both `Plugin` and `HandlerPlugin` traits
   - Supports glob patterns

4. **DirectoryHandlerPlugin**
   - Creates directories
   - Implements both `Plugin` and `HandlerPlugin` traits
   - Preserves existing directories

### Usage Examples

```rust
use dotsy::handlers::plugin_handlers::*;
use dotsy::plugins::plugin_trait::{Plugin, HandlerPlugin};

// Package management
let pkg_plugin = PackageHandlerPlugin::new(
    "apt install {}".to_string(),
    "apt remove {}".to_string()
);
pkg_plugin.install("vim")?;
pkg_plugin.uninstall("vim")?;

// Script execution
let script_plugin = ScriptHandlerPlugin::new();
script_plugin.execute(&["ls -la".to_string()])?;

// Directory creation
let dir_plugin = DirectoryHandlerPlugin::new();
dir_plugin.install("/home/user/.config")?;
```

### Benefits

1. **Unified Interface**: All handlers follow the same pattern
2. **Extensibility**: Easy to add new handlers that integrate seamlessly
3. **Testability**: Plugin interface makes testing more straightforward
4. **Modularity**: Handlers can be composed and swapped dynamically
5. **Documentation**: Single interface to document and understand

## Plugin Registry and CLI Integration

The plugin system is fully integrated with the CLI through a centralized registry system.

### Plugin Registry

The `PluginRegistry` provides:
- **Centralized Management**: All plugins are registered in a single location
- **Thread-Safe Access**: Uses `Arc<RwLock<>>` for safe concurrent access
- **Discovery**: Plugins can be listed and queried dynamically
- **Help System**: Each plugin provides its own help information

### CLI Integration

The `plugin` subcommand provides three operations:

1. **List Plugins**
   ```bash
   dotsy plugin list
   ```
   Shows all registered plugins with their versions and descriptions.

2. **Plugin Help**
   ```bash
   dotsy plugin help                    # All plugins
   dotsy plugin help package-handler    # Specific plugin
   ```
   Displays modular help information for plugins.

3. **Execute Plugin**
   ```bash
   dotsy plugin execute script-handler -- "echo 'Hello'"
   dotsy plugin execute directory-handler -- "/path/to/dir"
   ```
   Executes a plugin with the provided arguments.

### Built-in Plugins

The registry automatically initializes with these built-in plugins:
- `package-handler` - Package installation/uninstallation
- `script-handler` - Shell script execution
- `directory-handler` - Directory creation

### Modular Help

Each plugin provides its own help information through the `Plugin` trait:
- `name()` - Plugin identifier
- `version()` - Version string
- `description()` - What the plugin does

This creates a modular, discoverable system where new plugins can be added without modifying CLI code.

### Usage Example

```bash
# Discover available plugins
$ dotsy plugin list
Registered Plugins:

  package-handler (v1.0.0)
    Handles package installation and uninstallation

  script-handler (v1.0.0)
    Executes shell scripts

  directory-handler (v1.0.0)
    Creates directories

# Get help for a specific plugin
$ dotsy plugin help script-handler
script-handler v1.0.0

Executes shell scripts

# Execute a plugin
$ dotsy plugin execute script-handler -- "ls -la"
```

This design allows for easy extension and provides a unified interface for all plugin operations.

## Internal Plugin Usage

The plugin system is used internally by existing commands rather than exposing a separate `plugin` CLI subcommand. This keeps the CLI interface consistent while leveraging the plugin architecture behind the scenes.

### How Commands Use Plugins

The existing `profile` and `config` commands use plugin handlers internally:

**Profile Installation** uses:
- `DirectoryHandlerPlugin` - For creating directories
- `PackageHandlerPlugin` - For installing packages
- `ScriptHandlerPlugin` - For running shell scripts
- Direct handlers for complex link operations

**Profile Uninstallation** uses:
- `PackageHandlerPlugin` - For uninstalling packages
- `ScriptHandlerPlugin` - For running cleanup scripts
- Direct handlers for unlinking files

### Benefits

1. **Consistent CLI**: The user-facing CLI remains unchanged
2. **Internal Modularity**: Handlers are implemented as plugins
3. **Easy Testing**: Each plugin can be tested independently
4. **Future Extensibility**: New handlers can be added as plugins
5. **Clean Architecture**: Plugin system provides standardized interfaces

### Example Internal Usage

```rust
// In profile_manager.rs
let package_handler = PackageHandlerPlugin::new(
    config.package_add_command.clone(),
    config.package_remove_command.clone(),
);

for package in packages {
    package_handler.install(&package)?;
}
```

The plugin system provides the infrastructure, but the existing CLI commands control how and when plugins are used.
