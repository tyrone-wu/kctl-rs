use crate::cmd::options;

mod cmd;
mod subcommand;

fn main() {
    println!("{:?}", options().run())
}
