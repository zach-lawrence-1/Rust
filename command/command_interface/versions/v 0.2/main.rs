
//Make functions command processing which processes input to see if a specific command exists and
//outputs errors if needed. Make functions that run for each kind of command and does what is specified

mod command;
use std::io;
fn main()
{
    //#Notes [delete later]
    //echo either takes an unlimited amount of arguments after command or if entered by itself,
    //it will say InputObject[number]: text until empty argument is passed
    //also prints an endline between each instance of inputobject and each space of entered argument

    //#TODO: get rid of unwrap and expect and do proper error handling

    let mut input: String = String::new();
    //.expect does some error handling for now us if it somehow fails
    io::stdin().read_line(&mut input).expect("\x1b[1;31mSomething went wrong\x1b[0m");
    command::command_processing(input);
}
