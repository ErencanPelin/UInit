# UInit
Small project to learn Rust and making it faster to start new Unity projects.

Starting a new Unity project means spending a couple hours just organising folders, creating necessary files and fetching dependencies.
The aim of this project is to initialise Unity projects with the core folder structures, assemblies and files I use frequently.

![](./docs/project_init.png)

## Features
- Create all necessary folders in one go
- Create package manifest for unity packages quickly - uses Jinja2 templating.
- Create LICENSE files automatically using Jinja2 templating. Currently just works for the BSD 3-clause license.
- Initialise Steamworks dependencies and steam-appid.txt in one command
- Create whole feature domains with runtime, editor, test assemblies with a single command
- Create CI workflows from templates
- Validate project structure against defined templates
- Import default or custom modules into your project with a single command. Imports modules as part of your project, not just a package

## Developing
See [DEVELOPING.md](/DEVELOPING.md)

## Getting Started
1. Install the latest version using
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ErencanPelin/UInit/releases/download/v0.1.1/uinit-installer.sh | sh
```
2. Run `uinit --help` in your terminal to get started.
3. Update your current version with `uinit-update`

### To setup UInit in your Unity project
```sh
uinit init --template <GAME | PACKAGE> <PROJECT_NAME>
# e.g. with all optional fields
uinit init --template package --company ErencanPelin --email myemail@mailserver.com MyNewPackage
```

### To init steam
```sh
uinit setup steam --app-id <APP_ID>
# e.g. 480 = Spacewar
uinit setup steam --app-id 480
```

### To init a CI/CD workflow
Currently only Github is supported.
```sh
uinit setup ci <CI_HOST> <WORKFLOW_TYPE>
# e.g.
uinit setup ci github editor-tests
```

### To create a new feature domain
A feature domain lives inside /Scripts. This command creates sub folders for the feature (runtime, editor, tests) as well as the necessary assembly definition files for those sub folders.
```sh
uinit gen <FEATURE_NAME> [--no-editor] [--no-tests]
# e.g.
uinit gen MyNewFeature
```

### To import predefined tool scripts, utils or feature modules
```sh
# list all aliases for remotes
uinit remote list

# import a remote tool/feature/util by its alias
uinit import <ALIAS>
# e.g.
uinit import statemachines
```

### To add or customise your own aliases and point them to your own code
```sh
# add a custom alias or alias override for a remote
uinit remote add --repo <REPO_HTTP_URL> --path <PATH_TO_MODULE_FROM_REPO_ROOT> --category <UTIL | TOOL | MODULE> <ALIAS_NAME>
# e.g.
uinit remote add --repo https://github.com/ErencanPelin/Unity-Utils --path /Utils/Core --category util core-utils

# remove a custom alias or alias override for a remote
uinit remote rm <ALIAS_NAME>
# e.g.
uinit remote rm core-utils

# list available remote aliases
uinit remote list
```

### Defining custom dependency bundles in your uinit.toml
You can add custom dependencies by adding to your `custom_bundles` in your uinit.toml in your project's root.
```toml
# define a block like this:
[custom_aliases.bundles.<ALIAS>]
dependencies = [
    { name = "<PACKAGE_NAME>", version = "<VERSION>" },
    { name = "<PACKAGE_NAME>", version = "<VERSION>" }
]

# for example:
[custom_aliases.bundles.mybundle]
dependencies = [
    { name = "com.unity.textmeshpro", version = "3.0.6" },
    { name = "com.unity.ui", version = "1.0.0" }
]
```

### Check general health of Unity Project
```sh
uinit doctor
```