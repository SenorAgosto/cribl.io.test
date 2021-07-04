# Cribl.io Engineering Test

The log_collection service is implemented using Rust and the Rocket web api framework. Some very basic test scenarios for the log_collection microservice were implemented using Behave BDD Testing Framework.

### Assumptions

- the retrieved logs are text and each line ends in a "\n"
- "reverse time order" means newest events are listed first
- git revision control tool is already install

## Comments on the Bonus Challenges

For the first bonus challenge, I would use [Cap'n Proto](https://capnproto) to define an RPC endpoint. The server and clients could be implemented in C++, C#, Erlang, Go, Haskell, JavaScript (Node.JS), OCaml, Python, or Rust.

For the second bonus challenge, I would likely use a C# Windows Forms application to interact with the Cap'n Proto RPC service.

## Environment & Dependencies

The following OS and tools were used to implement & test the code:

- macOS Big Sur 11.4
- Homebrew 3.2.0 (howebrew-core 5bf8e35d52, howebrew-cask d256c74ff0)
- rustup 1.24.3
- rustc 1.55.0-nightly
- pip3 21.1.3

### Rust Crates

The following Rust crates were used to implement the microservice:

- rocket 0.4.10
- circular-queue 0.2.6

    $ cargo update

### Python3 Modules

The following Python modules were installed with pip3:

- behave 1.2.6
- requests 2.25.1

    $ pip3 install --user behave requests

Add `behave` to your path, on macOS this is done by setting:

    $ export PATH=$PATH:/Users/<user>/Library/Python/<python_version>/bin

Where <user> is your username, and <python_version> is your python version, example `3.8`

### Install Homebrew

Using a user with administrative permissions, install [brew](https://brew.sh).

	$ /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

### Install Rustup

	$ brew install rustup

Initialize rustup by running `rustup-init`

	$ rustup-init

During the init script, choose `(2) custom installation`, change the channel to "nightly". There are some features used by Rocket that aren't available in the stable build of the rustc compiler. 

### Install rustc

Use `rustup` to install the nightly build of the rust compiler.

	$ rustup toolchain install nightly

## log_collection microservice

### Building the Code

From the root of the project directory, use `cargo` to build the code:

    $ cargo build

This will download and build all the external dependencies needed by Rocket.

### Running the Code for Manual Testing

From the root of the project directory, use `cargo` to run the microservice:

    $ cargo run

The microservice should be listening on `http://localhost:8888`

### Requesting Log Events

You can request a log using the URL format:

    http://localhost:8888/log/<log_name>/<num_lines>

Where `<log_name>` is the name of the log you want to retrieve and `<num_lines>` is the number of lines to get.

You can request a log filtered by simple keywords using the URL format:

    http://localhost:8888/filtered_log/<log_name>/<filter>/<num_lines>

Where `<log_name>` is the name of the log you want to retrive, `<filter>` is the keyword to filter by, and `<num_lines>` is the number of lines to get.

### Running BDD Tests

Ensure the microservice is _not_ running, then from the root of the project directory, use `behave` to run the microservice test cases:

    $ behave

