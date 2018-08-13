# shiplift fork [WIP]

> a rust interface for maneuvering [docker](https://www.docker.com/) containers

In this fork there are two main branches:
- refactor, this is continuation of refactor started by matthiasbeyer/shiplift/refactor - there had been many issues that were fixed here.
- async, massive asynchronous refactor of shiplift library

## install

Add the following to your `Cargo.toml` file

```toml
[dependencies]
shiplift = { git = "https://github.com/destruktiw/shiplift", branch = "<name>" }
```

\<name\> should be `async` or `refactor`.
