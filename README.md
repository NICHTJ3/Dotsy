# Dotsy

## TODO

- [X] Bootstrapping?
  - [X] Create a config
  - [X] Create a profile
  - [X] dotsy init?
- [x] CLI
  - [x] Allow for commands
  - [x] Allow for command options
- [ ] Parsing config
  - [x] YAML or json?
    - JSON parsing is inbuilt
  - [ ] config validation (global,config and profiles )
- [X] Installing
  - [X] Link files
  - [X] Create directories
  - [X] Install programs with built in pm
  - [X] Run shell commands
  - [X] Install a profile/config? (I don't know how the command structure or file
        structure will work with this)
- [X] Uninstall?
  - [X] Unlink files
  - [X] Uninstall programs with built in pm
  - [X] Allow for custom uninstall files to run the reverse of shell commands if
        neccisary
  - [X] Uninstall a profile/config? (I don't know how the command structure or file
        structure will work with this)

## Planning

### Command structure?

- `dotsy profile -i <profile-name/'s'>`
- `dotsy profile --uninstall <profile-name/'s'>`
- `dotsy config -i <config-name/'s>`
- `dotsy config --uninstall <config-name/'s>`
- `dotsy config ls`
- `dotsy profile ls`

### File structure

- dotsy/
- configs/
  - config/
    - config-files/
    - config.json/.yml
- profiles/
  - profile.json/.yml
- .dotsyrc ( This will hold global options such as the package manager install
  and uninstall command etc...)

### test-config.json/.yml

Everything will be optional

```json
{
  "description": "Test config",
  "links": [{ "from": "./test", "to": "~/test" }],
  "directories": ["~/Test-Dir"],
  "packages": ["npm"],
  "shell": ["npm i test"],
  "revert-shell": ["npm uninstall test"]
}
```

### profile.json/.yml example

Everything but configs will be optional

```json
{
  "description": "Test profile",
  "configs": ["test-config"],
  "directories": ["~/Documents"],
  "packages": ["nvim"],
  "shell": ["npm i test"],
  "revert-shell": ["npm uninstall test"]
}
```
