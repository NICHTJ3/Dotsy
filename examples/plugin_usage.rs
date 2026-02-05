/// Example demonstrating plugin handler usage
/// 
/// This example shows how to use the plugin-based handler implementations
/// for common operations like package management, script execution, and file operations.

use dotsy::{
    configs::DotsyConfig,
    handlers::plugin_handlers::{
        DirectoryHandlerPlugin, PackageHandlerPlugin, ScriptHandlerPlugin,
    },
    plugins::plugin_trait::{HandlerPlugin, Plugin},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Dotsy Plugin Handler Examples ===\n");

    // Example 1: Package Handler Plugin
    println!("1. Package Handler Plugin");
    let package_plugin = PackageHandlerPlugin::new(
        "echo 'Installing: {}'".to_string(),
        "echo 'Uninstalling: {}'".to_string(),
    );
    
    println!("   Plugin: {} v{}", package_plugin.name(), package_plugin.version());
    println!("   Description: {}", package_plugin.description());
    println!("   Installing package 'vim'...");
    package_plugin.install("vim")?;
    println!();

    // Example 2: Script Handler Plugin
    println!("2. Script Handler Plugin");
    let script_plugin = ScriptHandlerPlugin::new();
    
    println!("   Plugin: {} v{}", script_plugin.name(), script_plugin.version());
    println!("   Description: {}", script_plugin.description());
    println!("   Executing script...");
    script_plugin.execute(&["echo 'Hello from script plugin!'".to_string()])?;
    println!();

    // Example 3: Directory Handler Plugin
    println!("3. Directory Handler Plugin");
    let dir_plugin = DirectoryHandlerPlugin::new();
    
    println!("   Plugin: {} v{}", dir_plugin.name(), dir_plugin.version());
    println!("   Description: {}", dir_plugin.description());
    println!("   Creating directory '/tmp/dotsy-example'...");
    dir_plugin.install("/tmp/dotsy-example")?;
    println!();

    println!("=== All examples completed successfully! ===");
    
    Ok(())
}
