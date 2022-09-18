# Dotsy

A Huge WIP... Infact most of it doesn't work so don't bother downloading yet 😂
It'll get there eventually 🤦

The idea behind this was to be able to manage and reinstall a config or profile
from where ever you were in the file tree. For example

```sh
ls ~/
> ~/Dotfiles
> ~/dev
pwd
> ~/dev/supercool-project/

dotsy config -i neovim
# ^ This would install your neovim config without moving you
# out of the directory or context you're currently in (i've felt this is useful 
# when I break things)
```

Getting started on a new machine would theoretically be as simple as running
this after cloning your dotfiles

```sh
cargo install dotsy
dotsy init
# ^ After this you would have to fill in the config file with the location
# of your dotfiles but this step could be skipped if you have a .dotsyrc in your
# dotfiles repo
dotsy profile -i <name of profile to install>
```

## Planning

### Command structure?

- `dotsy profile install <profile-name/'s'>`
- `dotsy profile uninstall <profile-name/'s'>`
- `dotsy config install <config-name/'s>`
- `dotsy config uninstall <config-name/'s>`
- `dotsy config list`
- `dotsy profile list`

### File structure

- configs/
  - config/
    - \<name\>/
    - \<name\>.json
- profiles/
  - \<name\>.json
- .dotsyrc ( This will hold global options such as the package manager install
  and uninstall command etc...)

### test-config.json

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

### profile.json example

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
