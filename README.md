# Dotsy

## TODO

- [ ] Bootstrapping?
  - [ ] Create a config
  - [ ] Create a profile
  - [ ] dotsy init?
- [x] CLI
  - [x] Allow for commands
  - [x] Allow for command options
- [ ] Parsing config
  - [x] YAML or json?
    - JSON parsing is inbuilt
  - [ ] config validation (global,config and profiles )
- [ ] Installing
  - [ ] Link files
    - [ ] Handle linking files
    - [ ] Handle linking files to directories that don't exist
    - [ ] Handle globs
    - [ ] Handle forcing
    - [ ] Handle re-linking
    - [ ] Handle relative?
  - [ ] Create directories
  - [ ] Install programs with built in pm
  - [ ] Run shell commands
  - [ ] Install a profile/config? (I don't know how the command structure or file
        structure will work with this)
- [ ] Uninstall
  - [ ] Unlink files
  - [ ] Remove directories
  - [ ] Uninstall programs with built in pm
  - [ ] Allow for custom uninstall files to run the reverse of shell commands if
        neccisary
  - [ ] Uninstall a profile/config? (I don't know how the command structure or file
        structure will work with this)

## Planning

### Command structure?

- `npx dotsy profile -i <profile-name/'s'>`
- `npx dotsy profile --uninstall <profile-name/'s'>`
- `npx dotsy config -i <config-name/'s>`
- `npx dotsy config --uninstall <config-name/'s>`
- `npx dotsy config ls`
- `npx dotsy profile ls`

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
