# Cribl.io Engineering Test

I'm implementing a solution to the problem using the Rust language and the Rocket web api framework on macOS.

## Installing Environment

Using a user with administrative permissions, install [brew](https://brew.sh), `git`, and `rustup`.

	$ /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
	$ brew install git rustup
	$ rustup-init

Choose (2) custom installation, change the channel to "nightly". There are some features used by Rocket that aren't available in the stable build of the rustc compiler. 

Use `rustup` to install the nightly build of the rust compiler.

	$ rustup toolchain install nightly

Use `pip3` to install `behave` BDD testing framework and the 'requests' module.

    $ pip3 install --user behave requests

Add `behave` to your path, on macOS this is done by setting:

    $ export PATH=$PATH:/Users/<user>/Library/Python/<python_version>/bin

Where <user> is your username, and <python_version> is your python version, example `3.8`

## Building the Code

From the root of the project directory, use `cargo` to build the code:

    $ cargo build

This will download and build all the external dependencies needed by Rocket.

## Running the Code

From the root of the project directory, use `cargo` to run the microservice:

    $ cargo run

The microservice should be listening on `http://localhost:8888`

You can request a log using the URL format:

    http://localhost:8888/log/<log_name>/<num_lines>

Where `<log_name>` is the name of the log you want to retrieve and `<num_lines>` is the number of lines to get.

You can request a log filtered by simple keywords using the URL format:

    http://localhost:8888/filtered_log/<log_name>/<filter>/<num_lines>

Where `<log_name>` is the name of the log you want to retrive, `<filter>` is the keyword to filter by, and `<num_lines>` is the number of lines to get.

## Running Tests

From the root of the project directory, use `behave` to run the microservice test cases:

    $ behave

