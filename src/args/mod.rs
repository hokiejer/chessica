pub mod profile;

#[derive(PartialEq,Eq,Debug)]
pub enum ProfileType {
    None,
    Reset,
    Tree,
}
    
pub struct ArgStruct {
    profile: bool,
    profile_type: ProfileType,
    error: bool,
}

pub fn process_args(args: Vec<String>) -> ArgStruct {
    use std::env;
    use crate::args::profile::args_profile;

    let mut arg_index = 1;
    let mut response = ArgStruct {
        profile: false,
        profile_type: ProfileType::None,
        error: false,
    };

    for (pos,val) in args.iter().enumerate() {
        println!("vec[{}] == {}",pos,val);
    }
    while arg_index < args.len() {
        let option: &str = &((&args[arg_index])[..]);
        match option {
            "--profile" => {
                response.profile = true;
                if arg_index + 1 < args.len() {
                    let parameter: &str = &((&args[arg_index+1])[..]);
                    args_profile(parameter,&mut response);
                    arg_index += 2;
                } else {
                    println!("No argument specifed for \"--profile\" option");
                    response.error = true;
                    arg_index += 1;
                }
            },
            _ => {
                println!("I don't know what to do with argument \"{}\"",option);
                response.error = true;
                arg_index = args.len();
            },
        }
    }
    response
}

impl ArgStruct {
    pub fn error(&self) -> bool {
        self.error
    }
}

#[cfg(test)]
mod tests {
    use crate::args::process_args;
    use crate::args::ArgStruct;
    use crate::args::ProfileType;

    fn convert_to_strings(arr: &[&str]) -> Vec<String> {
        let mut vec = Vec::new();
        for item in arr.into_iter().enumerate() {
            let (i, x): (usize, &&str) = item;
            println!("array[{i}] = {x}");
            vec.push(x.to_string());
        }
        vec
    }

    #[test]
    fn args_error() {
        let arr = ["chessica","whatever"];
        let vec = convert_to_strings(&arr);
        let mut a = process_args(vec);
        assert_eq!(a.error(),true);

        a.error = false;
        assert_eq!(a.error(),false);
    }

}
