use std::io;
use std::fs;
use std::path::Path;

fn main()
{
    //#TODO: get rid of unwrap and expect and do proper error handling

    let mut input: String = String::new();
    //.expect does some error handling for now us if it somehow fails
    io::stdin().read_line(&mut input).expect("\x1b[1;31mSomething went wrong\x1b[0m");
    command_processing(input);
}

//runs corresponding command from user input
fn command_processing(commands: String)
{
    //#Notes [delete later] for now use if statements maybe find a better way to check commands entered

    let commands_vec: Vec<&str> = commands.split_whitespace().collect();

    if commands_vec[0].to_lowercase() == "echo"
    {
        echo(commands_vec)
    }
    
    //TODO: handle case of directories with a space inbetween with ' or " around it like 'new fold' or "new fold"
    else if commands_vec[0].to_lowercase() == "ls"
    {
        if commands_vec.len() == 1
        {
            ls("".to_string(), "".to_string());
        }
        else if commands_vec.len() == 2
        {
            //only 1 option argument is passed in after ls command
            if commands_vec[1].starts_with("-")
            {
                ls(commands_vec[1].to_string(), "".to_string());
            }
            //only 1 directory is passed in after ls command
            else 
            {
                ls("".to_string(), commands_vec[1].to_string());
            }
        }
        else
        {
            let mut option_arg: &str = "";
            let mut path_vec: Vec<&str> = Vec::new();

            //find dominant option argument
            for i in 1..commands_vec.len()
            {
                if commands_vec[i].starts_with("-")
                {
                    option_arg = commands_vec[i];
                }
                else
                {
                    path_vec.push(commands_vec[i]);
                }
            }
            
            if path_vec.is_empty()
            {
                ls(option_arg.to_string(), "".to_string());
            }
            else if path_vec.len() == 1
            {
                ls(option_arg.to_string(), path_vec[0].to_string());
            }
            else 
            {
                //handles multiple paths entered
                for p in path_vec
                {
                    println!("{}:", p);
                    ls(option_arg.to_string(), p.to_string());
                    println!();
                }
            }
        }
    }
    else
    {
        //uses hexidecimal escape codes for color
        eprint!("\x1b[1;31m{0}: command not found\x1b[0m", commands_vec[0]);
    }
}

//#TODO: need to unit test echo command but I am pretty sure it is done

//windows powershell echo command
fn echo(message: Vec<&str>)
{
    //echo command is the only word typed
    if message.len() == 1
    {
        let enter_key: &str = "\r\n";
        let mut input = String::new();
        let mut text = String::new();
        let mut counter: i8 = 0;

        print!("\ncmdlet Write-Output at command pipeline position 1\nSupply values for the following parameters:\n");
        
        while input != enter_key
        {
            text += input.as_str();
            input = "".to_string();
            print!("InputObject[{}]:", counter);
            //write data from buffer before trying to get input so input isn't repeated
            io::Write::flush(&mut io::stdout()).expect("\x1b[1;31mflush failed\x1b[0m");
            io::stdin().read_line(&mut input).expect("\x1b[1;31mSomething went wrong\x1b[0m");
            counter += 1;
        }
        print!("{}", text);
    }
    else 
    {
        for i in 1..message.len()
        {
            println!("{}", message[i]);
        }
    }
}

//#Notes [delete later] features I want in ls command
//1. by default only list files and directories without . at beginning
//2. if using -A or -all list every kind of directory and all files
//3. sort by default alphabetically
//4. list all directories and files in a specified path (by including the path as the last argument)
//5. list only directories using -d -directory
//6. must have command options before specified path is listed, (use in command processing)

//for testing
//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

//handles errors when converting ReadDir into a string for printing
fn dir_to_string_err(file_dir: Option<&str>, path: Option<&str>) -> String
{
    //#TODO: fix strip error when multiple slashes are entered into a path input, should be able to run
    //even in case ./////// is entered or ./////test///// or test////////// as long as path is valid

    let file_dir: &str = match file_dir {
        None => "file dir error",
        Some(f) => f,
    };

    let path: &str = match path {
        None => "path error",
        Some(f) => f,
    };

    let mut added_str: String = path.to_string();
    //default path doesn't have \ but any other given path has it
    if added_str != "./"
    {
        added_str.push_str("\\");
    }

    let convert_string: &str = added_str.as_str();
    let file_dir: Option<&str> = file_dir.strip_prefix(convert_string);
    
    let file_dir: &str = match file_dir {
        None => "after strip prefix error",
        Some(f) => f,
    };
    
    return file_dir.to_string();
}

//#NOTES[delete later] For now, just print all directories in given valid path

//lists directories
fn ls(option_arg: String, mut path: String)
{
    // ./ is for the local directory
    //needed if empty path is given
    if path.len() == 0
    {
        path = "./".to_string();
    }

    let _path: &Path = Path::new(&path);
    let file_directories: Result<fs::ReadDir, io::Error> = fs::read_dir(_path);
    
    let file_directories = match file_directories
    {
        Ok(fd) => fd,
        Err(error) => 
        {
            println!("{}", error);
            return;
        },
    };
    
    for i in file_directories
    {
        //#TODO: store all directories into a container of some kind so the function can use arguments passed in for process
        match i
        {
            Ok(file) => println!("{}", dir_to_string_err(file.path().as_os_str().to_str(), _path.to_str())),
            Err(error) => println!("Problem with file: {}", error),
        };
    }

    //#TODO: do different things based on inputted arguments
    //for testing
    println!();
    println!("option argument: {}", option_arg);
}