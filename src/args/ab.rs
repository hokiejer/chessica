use crate::args::ArgStruct;

pub fn args_ab(arg: &str, option: &str, response: &mut ArgStruct) {
    match option {
        "--ab-search-depth" => args_ab_search_depth(arg, response),
        "--ab-keep-depth" => args_ab_keep_depth(arg, response),
        _ => {
            // Will not get here
        },
    }
}

pub fn args_ab_search_depth(arg: &str, response: &mut ArgStruct) {
    match arg.parse::<u8>() {
        Ok(n) => {
            response.ab_search_depth = n;
        },
        Err(e) => {
            response.error = true;
            println!("Unexpected ab-search-depth: \"{}\"!",arg);
        },
    }
}

pub fn args_ab_keep_depth(arg: &str, response: &mut ArgStruct) {
    match arg.parse::<u8>() {
        Ok(n) => {
            response.ab_keep_depth = n;
        },
        Err(e) => {
            response.error = true;
            println!("Unexpected ab-keep-depth: \"{}\"!",arg);
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::args::process_args;
    use crate::args::ArgStruct;

    fn convert_to_strings(arr: &[&str]) -> Vec<String> {
        let mut vec = Vec::new();
        for item in arr.into_iter().enumerate() {
            let (i, x): (usize, &&str) = item;
            vec.push(x.to_string());
        }
        vec
    }

    #[test]
    fn args_ab_search_depth_valid_1() {
        let arr = ["chessica","--ab-search-depth","117"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.error,false);
        assert_eq!(a.ab_search_depth,117);
    }

    #[test]
    fn args_ab_search_depth_invalid_1() {
        let arr = ["chessica","--ab-search-depth","moose"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.error,true);
    }

    #[test]
    fn args_ab_keep_depth_valid_1() {
        let arr = ["chessica","--ab-keep-depth","2"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.error,false);
        assert_eq!(a.ab_keep_depth,2);
    }

    #[test]
    fn args_ab_keep_depth_valid_2() {
        let arr = ["chessica","--ab-keep-depth","200"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.error,false);
        assert_eq!(a.ab_keep_depth,200);
    }

    #[test]
    fn args_ab_keep_depth_invalid_1() {
        let arr = ["chessica","--ab-keep-depth","dog"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.error,true);
    }

    #[test]
    fn args_ab_keep_depth_invalid_2() {
        let arr = ["chessica","--ab-keep-depth","-1"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.error,true);
    }

    #[test]
    fn args_ab_keep_depth_invalid_3() {
        let arr = ["chessica","--ab-keep-depth","2000"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.error,true);
    }

}
