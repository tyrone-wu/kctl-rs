use crate::command::options;

mod command;
mod subcommand;

fn main() {
    println!("{:?}", options().run())
}
