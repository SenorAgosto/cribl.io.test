# Cribl.io Engineering Test

I'm implementing a solution to the problem using the Rust language and the Rocket web api framework on macOS.

## Installing Environment

Using a user with administrative permissions, install [brew](https://brew.sh), `git`, and `rustup`.

	$ /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
	$ brew install git
    $ brew install rustup
	$ rustup-init

Choose (2) custom installation, change the channel to "nightly". There are some features used by Rocket that aren't available in the stable build of the rustc compiler. 

Use `rustup` to install the nightly build of the rust compiler.

	$ rustup toolchain install nightly

## Building the Code

From the root of the project directory, use `cargo` to build the code:

    $ cargo build

This will download and build all the external dependencies needed by Rocket.

## Running the Code

From the root of the project directory, use `cargo` to run the microservice:

    $ cargo run

The microservice should be listening on `http://localhost:8000`

You can request a log using the URL format:

    http://localhost:8000/log/<log_name>/<num_lines>

Where `<log_name>` is the name of the log you want to retrieve and `<num_lines>` is the number of lines to get.
