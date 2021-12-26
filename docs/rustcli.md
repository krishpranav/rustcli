## RustCli

- a basic rust cli app:
```rust
use rustcli::{App};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description("hey this is a cli app build using rust cli")
        .author("Authorone")
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args));

    app.run(args);
}
```

- command application
```rust
use rustcli::{App};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description("hey this is a cli app build using rust cli")
        .author("Authorone")
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .command(add_command())
        .command(sub_command());

    app.run(args);
}

fn default_action(c: &Context) {
    println!("Hello, {:?}", c.args);
}

fn add_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum();
    println!("{}", sum);
}

fn add_command() -> Command {
    Command::new("add")
        .description("add command")
        .alias("a")
        .usage("cli add(a) [nums...]")
        .action(add_action)
}

fn sub_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum();
    println!("{}", sum);
}

fn sub_command() -> Command {
    Command::new("sub")
        .description("sub command")
        .alias("s")
        .usage("cli sub(s) [nums...]")
        .action(sub_action)
}
```