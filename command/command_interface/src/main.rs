mod command;
use std::io;
fn main()
{
    let mut input: String = String::new();
    //.expect does some error handling for now us if it somehow fails
    io::stdin().read_line(&mut input).expect("\x1b[1;31mSomething went wrong\x1b[0m");
    command::command_processing(input);
}
