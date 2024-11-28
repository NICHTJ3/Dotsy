# Dotsy

A Huge WIP for managing dotfiles 🧪

<https://github.com/user-attachments/assets/2937ebfe-6d4b-4ea3-935f-1b215e806766>

## Usage

### Instalation

```bash
cargo install dotsy
```

### Creating a dotfiles directory

Initialising dotsy

```bash
mkdir dotfiles

cd dotfiles
dotsy init

# this creates a .dotsyrc file containing the following defaults view `dotsy --help` to view dotsy config resolution order
# {
#   "dotfiles": "~/Dotfiles", // The location you intend on storing your dotfiles in. This is needed to link things
#   "profiles_dir": "profiles", // The directory you intend on storing your <profile>.profile.json files in
#   "configs_dir": "configs", // The directory you intend on storing your <config>.config.json files in
#   "package_add_command": "brew add {}", // The command that will be used to install packages listed in configs or profiles
#   "package_remove_command": "brew remove {}" // The command that will  be used to uninstall packages listed in configs or profiles
# }
```

Creating your first config

```bash
# Creating your first config
echo "My test file" > test_file
mkdir config
cd config

dotsy init --config test # This command will create a test.config.json file in the current directory

# Update the file to contain the following
# {
#    "links": [
#        {
#            "from": "test_file", # Link the test_file created earlier
#            "to": "~/test_file" # To ~/test_file
#        }
#    ]
# }
```

Listing your configs

```bash
cd ~/dotfiles

dotsy config list # List available configs. You should see your new "test" config listed
```

Installing your new config

```bash
# This will symlink your test_file to ~/test_file
dotsy install test

cat ~/test_file # My test file
```

Go take a look at my dotfiles for more of a "real world" example https://github.com/nichtj3/dotfiles or run `dotsy --help`

## The thinking behind Dotsy

The main idea behind this is to be able to manage and reinstall a "config" or "profile"
from where ever you were in the file tree.

For example

```sh
ls ~/
> ~/Dotfiles
> ~/dev
pwd
> ~/dev/supercool-project/

dotsy config install neovim
# ^ This would install your neovim config without moving you
# out of the directory or context you're currently in (I've felt this is useful
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
dotsy profile install <name of profile to install>
```

## Configuration

### File structure

- configs/
  - config/
    - \<name\>/
    - \<name\>.config.json
- profiles/
  - \<name\>.profile.json
- .dotsyrc ( This will hold global options such as the package manager install
  and uninstall command etc...)

### test.config.json

Everything will be optional

```json
{
  "description": "Test config",
  "links": [
    { "from": "./test", "to": "~/test" },
    { "from": "./test_glob/*", "to": "~/test_glob", "glob": true }
  ],
  "directories": ["~/Test-Dir"],
  "packages": ["npm"],
  "shell": ["npm i test"],
  "revert-shell": ["npm uninstall test"]
}
```

### test.profile.json example

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
