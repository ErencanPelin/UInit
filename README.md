# UInit
Small project to learn Rust and making it faster to start new Unity projects.

Starting a new Unity project means spending a couple hours just organising folders, creating necessary files and fetching dependencies.
The aim of this project is to initialise Unity projects with the core folder structures, assemblies and files I use frequently.

## Features
- Create all necessary folders in one go
- Create package manifest for unity packages quickly - uses Jinja2 templating.
- Create LICENSE files automatically using Jinja2 templating. Currently just works for the BSD 3-clause license.
- Initialise Steamworks dependencies and steam-appid.txt in one command
- Create whole feature domains with runtime, editor, test assemblies with a single command

## Installation, Setup & Customisation
Nothing here yet. Still in development

## Geting Started
Run `uinit --help` in your terminal to get started.

### To init a new project 
```sh
uinit new --name <PROJECT_NAME> --template <GAME | PACKAGE>
```

### To init steam
```sh
uinit steam init --app-id <APP_ID>
```