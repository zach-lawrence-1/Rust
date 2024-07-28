use std::collections::HashMap;
//use std::env::temp_dir;
//use std::fs::DirBuilder;
//use std::fs::DirEntry;
//use std::fs::ReadDir;
//std::io is for input output so we can process input
use std::io;
//std::fs is for filesystem manipulation
use std::fs;
use std::path::Path;
use std::fs::metadata;

//runs corresponding command from user input
pub fn command_processing(mut commands: String) -> (Vec<String>, String)
{
    let mut indices: Vec<u8> = Vec::new();
    let mut quote: Vec<String> = Vec::new();
    let mut commands_vec: Vec<&str> = Vec::new();

    //for processing paths with "" or ''
    if !(commands.chars().filter(|c| *c == '"').count() % 2 == 0) || !(commands.chars().filter(|c| *c == '\'').count() % 2 == 0)
    {
        commands_vec.push("echo");
        commands_vec.push("\x1b[1;31mError: invalid path entered\x1b[0m");
    }
    else if commands.chars().filter(|c| *c == '"').count() == 0 && commands.chars().filter(|c| *c == '\'').count() == 0
    {
        commands_vec = commands.split_whitespace().collect();
    }
    else
    {
        //even quotation amount
        let mut indx: u8 = 0;
        for ch in commands.chars()
        {
            if ch == '\"' || ch == '\''
            {
                indices.push(indx);
            }
            indx += 1;
        }

        let mut offset = 0;
        for i in (0..indices.len()).step_by(2)
        {
            let j = i + 1;
            quote.push(commands[((indices[i] - offset + 1) as usize)..((indices[j] - offset) as usize)].to_string());
            commands.replace_range(((indices[i] - offset) as usize)..((indices[j] + 1 - offset) as usize), "");
            offset += (indices[j] + 1) - indices[i];
        }

        commands_vec = commands.split_whitespace().collect();
        for q in &quote
        {
            commands_vec.push(q.as_str());
        }
    }
    
    let mut option_arg: &str = "";
    let mut path_vec: Vec<&str> = Vec::new();
    
    if commands_vec[0].to_lowercase() == "echo"
    {
        echo(commands_vec)
    }
    else if commands_vec[0].to_lowercase() == "ls"
    {
        if commands_vec.len() == 1
        {
            print_vec(ls("".to_string(), ""));
        }
        else if commands_vec.len() == 2
        {
            //only 1 option argument is passed in after ls command
            if commands_vec[1].starts_with('-')
            {
                print_vec(ls(commands_vec[1].to_string(), ""));
            }
            else 
            {
                //only 1 directory is passed in after ls command
                path_vec.push(commands_vec[1]);
                print_vec(ls("".to_string(), path_vec[0]));
            }
        }
        else
        {
            //find dominant option argument
            for i in 1..commands_vec.len()
            {
                if commands_vec[i].starts_with('-')
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
                print_vec(ls(option_arg.to_string(), ""));
            }
            else 
            {
                //handles multiple paths entered
                for p in 0..path_vec.len()
                {
                    println!("{}:", path_vec[p]);
                    print_vec(ls(option_arg.to_string(), path_vec[p]));
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
    let pv: Vec<String> = path_vec.into_iter().map(|v| v.to_string()).collect();

    return (pv, option_arg.to_string());
}

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
        for mess in message
        {
            println!("{mess}");
        }
    }
}

//quick way to print vector
fn print_vec(vec: Vec<String>)
{
    for v in vec
    {
        println!("{v}");
    }
}

//handles errors when converting ReadDir into string for printing and trims off path entered after ls command is ran
fn dir_to_string_err_trim(file_dir: Option<&str>, path: &str) -> String
{
    let file_dir: &str = match file_dir
    {
        None => "\x1b[file dir error\x1b[0m",
        Some(f) => f,
    };

    //this beginning part of the path needs to be trimmed off of file_dir taken from ReadDir option in ls()
    let mut added_str: String = path.to_string();
    //default path doesn't have \ but any other given path has it
    if added_str.len() >= 2
    {
        //check for end of string containing "/" since those paths do not need to have \ removed at the end
        if added_str[added_str.len() - 1..] != *"/".to_string()
        {
            added_str.push('\\');
        }
    }

    let file_dir_op: Option<&str> = file_dir.strip_prefix(added_str.as_str());
    
    let file_dir: &str = match file_dir_op
    {
        None => file_dir,
        Some(f) => f,
    };
    
    return file_dir.to_string();
}

//lists directories
fn ls(option_arg: String, mut path: &str) -> Vec<String>
{
    // ./ is for the local directory
    //needed if empty path is given
    if path.is_empty()
    {
        path = "./";
    }

    //#Note[] I was unable to get this idea to work
    //idea: make a hash map where is_dir is the key and the value is a vector holding each string corresponding
    //let mut file_dirs: HashMap<bool, Vec<String>> = HashMap::new();
    //let mut file_dirs_vec: Vec<String> = Vec::new();

    let _path: &Path = Path::new(&path);
    let file_directories: Result<fs::ReadDir, io::Error> = fs::read_dir(_path);
    let mut file_dirs: Vec<String> = Vec::new();
    let mut is_dir_true: Vec<bool> = Vec::new();
    
    let file_directories = match file_directories
    {
        Ok(fd) => fd,
        Err(error) => 
        {
            println!("{}", error);
            return file_dirs;
        },
    };
    
    for entry in file_directories
    {
        match entry
        {
            Ok(file) => 
            {
                //determine if file path is a directory or file for option argument filtering later
                let dir_md_res: Result<fs::Metadata, io::Error> = metadata(file.path());
                let mut trimmed_path_string: String = dir_to_string_err_trim(file.path().as_os_str().to_str(), path);
                match dir_md_res
                {
                    Ok(met) =>
                    {
                        if !met.is_dir()
                        {
                            is_dir_true.push(false);
                            //file_dirs.insert(false, vec![dir_to_string_err_trim(file.path().as_os_str().to_str(), path)]);
                        }
                        else
                        {
                            trimmed_path_string = "\x1b[1;34m".to_string() + trimmed_path_string.as_str() + "\x1b[0m";
                            is_dir_true.push(true);
                            //file_dirs.insert(true, vec![dir_to_string_err_trim(file.path().as_os_str().to_str(), path)]);
                        }
                    }
                    Err(error) => print!("\x1b[metadata read ERROR: {}\x1b[0m", error),
                }
                file_dirs.push(trimmed_path_string);
            },
            Err(error) => println!("\x1b[Problem with file: {}\x1b[0m", error),
        };
    }

    //print all files and directories
    if option_arg == "-a" || option_arg == "-all"
    {
        //file_dirs_vec.append(&mut file_dirs[&false].clone());
        //file_dirs_vec.append(&mut file_dirs[&true].clone());
        return file_dirs;
    }
    //prints only directories
    else if option_arg == "-d" || option_arg == "-directory"
    {
        for i in (0..is_dir_true.len()).rev()
        {
            if !is_dir_true[i]
            {
                file_dirs.remove(i);
            }
        }
        return file_dirs;
    }
    //prints only files
    else if option_arg == "-f" || option_arg == "-file"
    {
        for i in (0..is_dir_true.len()).rev()
        {
            if is_dir_true[i]
            {
                file_dirs.remove(i);
            }
        }
        return file_dirs;
    }
    else
    {
        //only keeps all vector elements based on condition of each element
        //file_dirs[&false].clone().retain(|element| !(element).starts_with('.'));
        //file_dirs[&true].clone().retain(|element| !(element).starts_with('.'));
        file_dirs.retain(|element| !(element).starts_with('.') && !(element.starts_with("\x1b[1;34m.")));
    }

    //file_dirs_vec.append(&mut file_dirs[&false].clone());
    //file_dirs_vec.append(&mut file_dirs[&true].clone());
    return file_dirs;
}

#[cfg(test)]
mod test
{
   use super::*;
 
   #[test]
   fn test_dir_string_err()
   {
        let path: &str = "./";
        let dir: Option<&str> = Some("./test");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "test");

        let path: &str = "///////////";
        let dir: Option<&str> = Some("///////////test");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "test");

        let path: &str = ".///////////";
        let dir: Option<&str> = Some(".///////////test");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "test");

        let path: &str = ".///////////test//////";
        let dir: Option<&str> = Some(".///////////test//////bean");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "bean");

        let path: &str = ".///////////test//////";
        let dir: Option<&str> = Some(".///////////test//////");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "");

        let path: &str = "";
        let dir: Option<&str> = Some("");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "");

        let path: &str = "test/";
        let dir: Option<&str> = Some("test/src");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "src");

        let path: &str = "test/new fold/";
        let dir: Option<&str> = Some("test/new fold/src");
        let raw_dir = dir_to_string_err_trim(dir, path);
        assert_eq!(raw_dir, "src");
   }

   #[test]
   fn test_command_processing_echo()
   {
        let command: String = "echo h".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "".to_string());
        assert_eq!(vec.is_empty(), true);

        let command: String = "echo hi hi hi".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "".to_string());
        assert_eq!(vec.is_empty(), true);
   }

   #[test]
   fn test_command_processing_option_plus_path() 
   {
        let command: String = "ls".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "".to_string());
        assert_eq!(vec.is_empty(), true);

        let command: String = "ls test".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "".to_string());
        assert_eq!(vec[0], "test".to_string());

        let command: String = "ls -a -f -d".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "-d".to_string());
        assert_eq!(vec.is_empty(), true);

        let command: String = "ls test -d \"test\" -a \"test/new fold\" '' 'test\" \"test' 'test/new fold' \"\"".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "-a".to_string());
        let vec2 = vec!["test", "test", "test/new fold", "", "test", "test", "test/new fold", ""];
        assert_eq!(vec, vec2);

        //since there is odd " and ' there should be no option arg or path since
        //code stops processing commands
        let command: String = "ls -a -f -d \" \'".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "".to_string());
        assert_eq!(vec.is_empty(), true);

        let command: String = "ls -a -f -d \" \" \'".to_string();
        let (vec, option) = command_processing(command);
        assert_eq!(option, "".to_string());
        assert_eq!(vec.is_empty(), true);
   }

   #[test]
   fn test_ls()
   {
        let option: String = "".to_string();
        let path: &str = "";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec!["Cargo.lock", "Cargo.toml", "file.txt", "main.exe", "main.pdb", "\x1b[1;34msrc\x1b[0m", "\x1b[1;34mtarget\x1b[0m", "\x1b[1;34mtest\x1b[0m", "\x1b[1;34mv 0.1\x1b[0m"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "-a".to_string();
        let path: &str = "";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec!["\x1b[1;34m.git\x1b[0m", ".gitignore", "Cargo.lock", "Cargo.toml", "file.txt", "main.exe", "main.pdb", "\x1b[1;34msrc\x1b[0m", "\x1b[1;34mtarget\x1b[0m", "\x1b[1;34mtest\x1b[0m", "\x1b[1;34mv 0.1\x1b[0m"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "-all".to_string();
        let path: &str = "";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec!["\x1b[1;34m.git\x1b[0m", ".gitignore", "Cargo.lock", "Cargo.toml", "file.txt", "main.exe", "main.pdb", "\x1b[1;34msrc\x1b[0m", "\x1b[1;34mtarget\x1b[0m", "\x1b[1;34mtest\x1b[0m", "\x1b[1;34mv 0.1\x1b[0m"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "-d".to_string();
        let path: &str = "";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec!["\x1b[1;34m.git\x1b[0m", "\x1b[1;34msrc\x1b[0m", "\x1b[1;34mtarget\x1b[0m", "\x1b[1;34mtest\x1b[0m", "\x1b[1;34mv 0.1\x1b[0m"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "-directory".to_string();
        let path: &str = "";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec!["\x1b[1;34m.git\x1b[0m", "\x1b[1;34msrc\x1b[0m", "\x1b[1;34mtarget\x1b[0m", "\x1b[1;34mtest\x1b[0m", "\x1b[1;34mv 0.1\x1b[0m"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "-f".to_string();
        let path: &str = "";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec![".gitignore", "Cargo.lock", "Cargo.toml", "file.txt", "main.exe", "main.pdb"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "-file".to_string();
        let path: &str = "";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec![".gitignore", "Cargo.lock", "Cargo.toml", "file.txt", "main.exe", "main.pdb"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "".to_string();
        let path: &str = "test";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec!["\x1b[1;34mnew fold\x1b[0m", "test.txt"];
        assert_eq!(path_vec, comp_vec);

        let option: String = "".to_string();
        let path: &str = "test/new fold";
        let path_vec: Vec<String> = ls(option, path);
        let comp_vec = vec!["z.zip"];
        assert_eq!(path_vec, comp_vec);
   }
}