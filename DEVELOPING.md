# Developing
## Dependencies
1. You need rust installed

## Testing
To run a test without building:
```sh
cargo run -- <UINIT COMMANDS>
# e.g.
cargo run -- project init --template package MyTestProject
```

To test the project by building:
1. Build the project `cargo build`
2. Install the project locally `cargo install --path .`
3. Run the `create_test_env.sh` script to create your test project
4. `cd` into the test project and run `uinit` commands from within it


## Publishing a new version
First update the version in [Cargo.toml](./Cargo.toml). If you don't do this, the plan step will fail on the pipeline.

Then, simply push a new tag, this will kick off the release Github workflow to build and publish a new release.
```sh
git tag v<MAJOR>.<MINOR>.<PATCH>
git push origin <TAG>

# e.g.
git tag v1.0.0
git push origin v1.0.0

# deleting tags (if you make a mistake)
git tag -d v0.0.2
git push origin --delete v0.0.2
```