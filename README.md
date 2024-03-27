# Cavity: The suckless Rust Project Template

Cavity is a Rust project template which comes with [direnv](https://github.com/casey/just) & [just](https://github.com/casey/just) integration for an effortless Developer Experience :sparkle:

## Dependencies

1. [just](https://github.com/casey/just): To simplify repeatedly run commands.
2. [wactchexec](https://github.com/watchexec/watchexec): Watches files in current dir & re-builds project on any changes.
3. [direnv](https://github.com/direnv/direnv): To create an isolated development environment
4. [nix-direnv](https://github.com/nix-community/nix-direnv): For nix/flakes users.

## Usage

* **Create file** `scratch.md`: We recommend creating this file for local planning purposes. It is not tracked by Git.

* **Create file** `.envrc`: This file is used by direnv. Not tracked by Git. Add the following contents to it:
```
# Flakes users: uncomment the line below
# use flake

# Load project specific env variables this way.
export FOO=foo

# Run `direnv allow` to start using direnv. 
```

* **Building your project**: Use the `just` command to run your project. To test your project run `just test`.

* **Testing your project**: Use the `just test` command to test your project. Check the `justfile` file for more.

* **Nix/Flakes**: "flake.nix" & "flake.lock" are flake files. Remove them if you're not using nix/flakes.

* **Edit Cargo.toml**: Change the project name in the `Cargo.toml` file to your project name.

* **README.md**: Replace this `README.md` file with your project's `README.md`.

## Directory Structure

* **src/**: Contains project source code.
* **tests/**: Houses integration tests.
* **target/**: Contains compiled files & executables.
* **ui/**: Use it for your frontend. Remove if unnecessary.

<!-- git remote set-url origin https://<github-key>@github.com/txtyash/cavity -->
