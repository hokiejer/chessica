use crate::args::ArgStruct;
use crate::args::ProfileType;

pub fn args_profile(arg: &str, response: &mut ArgStruct) {
    println!("arg == {}",arg);
    match arg {
        "reset" => {
            response.profile_type = ProfileType::Reset;
        },
        "tree" => {
            response.profile_type = ProfileType::Tree;
        },
        "" => {
            response.error = true;
            println!("The \"--profile\" option must be specified with one of the following: [reset,tree]");
        },
        _ => {
            response.error = true;
            println!("I don't know how to profile \"{}\"",arg);
        },
    }
}

impl ArgStruct {
    pub fn profile_reset(&self) -> bool {
        self.profile & (self.profile_type == ProfileType::Reset)
    }

    pub fn profile_tree(&self) -> bool {
        self.profile & (self.profile_type == ProfileType::Tree)
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
    fn args_profile() {
        let arr = ["chessica","--profile","reset"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.profile_type,ProfileType::Reset);
        assert_eq!(a.error,false);
        assert!(a.profile_reset());
        assert!(!a.profile_tree());

        let arr = ["chessica","--profile","tree"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.profile_type,ProfileType::Tree);
        assert_eq!(a.error,false);
        assert!(a.profile_tree());
        assert!(!a.profile_reset());

        let arr = ["chessica","--profile","noclue"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.error,true);

        let arr = ["chessica","--profile"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.error,true);
    }

}
