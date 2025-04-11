# Lunchr

Lunchr is a command-line tool that helps you manage your local development environment easily.

## What is it, exactly?

Well, if you're like me, you probably have a lot of services you need to run, all at once. Before **lunchr**, I used to manually start each service one by one, which was time-consuming. Legend has it that I once forgot how to run all the local servers to do my work after an OS update.

Lunchr simplifies this process by providing a single command to start all the necessary services at once.

All you have to do is create a `.lunchr.toml` file and put it in your `$HOME` directory.

## Installation

Clone this repository:

```bash
$ git clone https://github.com/huytd/lunchr.git
```

Compile and install the binary:

```bash
$ cargo install --path .
```

## Configuration

Create a `.lunchr.toml` file in your `$HOME` directory with the following content:

```toml
[[commands]]
name = "the name to display on the screen"
cwd = "the absolute path to the directory where the command is expected to run"
start_command = "the command to start the service"
kill_command = "the command to kill the service (optional)"
health_check_pattern = "the grep pattern to check the status of the service"

...
```

You can leave the `kill_command` empty, and `pkill` will be used to kill the service.

Refer to the `lunchr.example.toml` file as a template for your configuration file.

## Usage

Start **Lunchr** with the following command:

```bash
$ lunchr
```

Press the number keys on your keyboard to toggle the services on or off.

You can safely exit **Lunchr** by pressing `q` and open it again anytime you want.
