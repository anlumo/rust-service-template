# HTTP Service Template

This is a template for writing an HTTP service in Rust using [axum](https://github.com/tokio-rs/axum). It also contains an example on how to make a Docker image out of it and it shows how to use [cargo-make](https://sagiegurari.github.io/cargo-make/) to simplify building and running.

## Features

- [axum](https://github.com/tokio-rs/axum) for building the HTTP service
- [cargo-chef](https://github.com/lukemathwalker/cargo-chef) for creating a Docker image
- [cargo-make](https://sagiegurari.github.io/cargo-make/) for building and running the application
- [log4rs](https://github.com/estk/log4rs) for logging
- [serde](https://github.com/serde-rs/serde) for configuration
- [serde_yaml](https://github.com/dtolnay/serde-yaml) for YAML configuration
- [tokio](https://github.com/tokio-rs/tokio) for asynchronous programming

## Building and Running

First, install docker and [cargo-make](https://sagiegurari.github.io/cargo-make/). Then, use the `cargo make build` command to build and run the application.

Use `cargo make run` to run the built docker file.

You can also use the regular `cargo build`/`cargo run` commands to use this project without Docker (during development for example).

## Configuration

The project uses a three-step approach for configuration (for each configuration item separately):

* If there's an environment variable for configuration, use that (using the `env` feature of clap).
* If there's a command line parameter, use that (using regular clap).
* If there's a line in the configuration file, use that (yaml parser).
* Otherwise, use a hardcoded (sane) default value.

This ensures optimal flexibilty for devops during deployment in production.

To get started, copy the file `config.example.yaml` to `config.yaml` and adjust its settings. This is important because the file might contain passwords and API keys that should not be included in the repository. `config.yaml` is ignored by git (via `.gitignore`). `config.example.yaml` is purely there as an example for the devops people, which might not be able to read the config.rs file to find out how to structure the file. It's also recommended to add comments there to improve the workflow.

## LICENSE

This code is dual-licensed MIT/ISC.

### MIT

Copyright 2024 Andreas Monitzer

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

### ISC

Copyright 2024 Andreas Monitzer

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED “AS IS” AND ISC DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL ISC BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.