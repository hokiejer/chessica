pub mod profile;
pub mod ab;
use crate::args::profile::ProfileType;

pub struct ArgStruct {
    pub profile: bool,
    pub profile_type: ProfileType,
    pub ab_search_depth: u8,
    pub ab_keep_depth: u8,
    pub error: bool,
}

pub fn usage() {
    println!("USAGE:");
}

pub fn process_args(args: Vec<String>) -> ArgStruct {
    use crate::args::profile::args_profile;
    use crate::args::ab::args_ab;

    let mut arg_index = 1;
    let mut response = ArgStruct {
        profile: false,
        profile_type: ProfileType::None,
        ab_search_depth: 8, // <= This should be a constant somewhere
        ab_keep_depth: 4, // <= This should be a constant somewhere
        error: false,
    };

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
            "--ab-search-depth" | "--ab-keep-depth" => {
                response.profile = true;
                if arg_index + 1 < args.len() {
                    let parameter: &str = &((&args[arg_index+1])[..]);
                    args_ab(parameter,option,&mut response);
                    arg_index += 2;
                } else {
                    println!("No argument specifed for \"{}\" option",option);
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

    fn convert_to_strings(arr: &[&str]) -> Vec<String> {
        let mut vec = Vec::new();
        for item in arr.iter().enumerate() {
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
        assert!(a.error());

        a.error = false;
        assert!(!a.error());
    }

}
