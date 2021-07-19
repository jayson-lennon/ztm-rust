# About
This repository is part of the Rust programming language course available at [zerotomastery.io](https://zerotomastery.io/).

## Activities
To work on activities, open up your editor/IDE to the `activities` directory. The activities can can be ran by executing `cargo run --bin a1` in your terminal, where `a1` is the name of the activity file you are working on.

## ClipStash
To work on the ClipStash project, open up your editor/IDE to the `clipstash` directory. You will then be able to run `cargo check` to check your code, `cargo test` to test your code, or `cargo run --bin name` where `name` is the name of the binary you wish to run.

## Solutions
To view solutions for the activities, and for the ClipStash project, checkout the `solutions` branch by running `git checkout solutions` after cloning this repo.

### ClipStash
The ClipStash project requires additional steps in order to properly build. You will need the `sqlx-cli` tool which can be installed by running

```
cargo install sqlx-cli
```

After installing the tool, you can configure the database for the project by running

```
sqlx database setup
```
