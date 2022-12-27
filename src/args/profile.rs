use crate::args::ArgStruct;

#[derive(PartialEq,Eq,Debug)]
pub enum ProfileType {
    None,
    Reset,
    Tree,
    InPlaceAB,
    KeepDepthAB,
    IterativeKeepDepthAB,
    IterativeKeepDepthABSwap,
}
    
pub fn args_profile(arg: &str, response: &mut ArgStruct) {
    match arg {
        "reset" => {
            response.profile_type = ProfileType::Reset;
        },
        "tree" => {
            response.profile_type = ProfileType::Tree;
        },
        "in-place-ab" => {
            response.profile_type = ProfileType::InPlaceAB;
        },
        "keep-depth-ab" => {
            response.profile_type = ProfileType::KeepDepthAB;
        },
        "iterative-keep-depth-ab" => {
            response.profile_type = ProfileType::IterativeKeepDepthAB;
        },
        "iterative-keep-depth-ab-promote" => {
            response.profile_type = ProfileType::IterativeKeepDepthABSwap;
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

    pub fn profile_in_place_ab(&self) -> bool {
        self.profile & (self.profile_type == ProfileType::InPlaceAB)
    }

    pub fn profile_keep_depth_ab(&self) -> bool {
        self.profile & (self.profile_type == ProfileType::KeepDepthAB)
    }

    pub fn profile_iterative_keep_depth_ab(&self) -> bool {
        self.profile & (self.profile_type == ProfileType::IterativeKeepDepthAB)
    }

    pub fn profile_iterative_keep_depth_ab_promote(&self) -> bool {
        self.profile & (self.profile_type == ProfileType::IterativeKeepDepthABSwap)
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
        assert!(!a.profile_in_place_ab());
        assert!(!a.profile_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab_promote());

        let arr = ["chessica","--profile","tree"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.profile_type,ProfileType::Tree);
        assert_eq!(a.error,false);
        assert!(a.profile_tree());
        assert!(!a.profile_reset());
        assert!(!a.profile_in_place_ab());
        assert!(!a.profile_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab_promote());

        let arr = ["chessica","--profile","in-place-ab"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.profile_type,ProfileType::InPlaceAB);
        assert_eq!(a.error,false);
        assert!(!a.profile_tree());
        assert!(!a.profile_reset());
        assert!(a.profile_in_place_ab());
        assert!(!a.profile_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab_promote());

        let arr = ["chessica","--profile","keep-depth-ab"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.profile_type,ProfileType::KeepDepthAB);
        assert_eq!(a.error,false);
        assert!(!a.profile_tree());
        assert!(!a.profile_reset());
        assert!(!a.profile_in_place_ab());
        assert!(a.profile_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab_promote());

        let arr = ["chessica","--profile","iterative-keep-depth-ab"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.profile_type,ProfileType::IterativeKeepDepthAB);
        assert_eq!(a.error,false);
        assert!(!a.profile_tree());
        assert!(!a.profile_reset());
        assert!(!a.profile_in_place_ab());
        assert!(!a.profile_keep_depth_ab());
        assert!(a.profile_iterative_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab_promote());

        let arr = ["chessica","--profile","iterative-keep-depth-ab-promote"];
        let vec = convert_to_strings(&arr);
        let a = process_args(vec);
        assert_eq!(a.profile,true);
        assert_eq!(a.profile_type,ProfileType::IterativeKeepDepthABSwap);
        assert_eq!(a.error,false);
        assert!(!a.profile_tree());
        assert!(!a.profile_reset());
        assert!(!a.profile_in_place_ab());
        assert!(!a.profile_keep_depth_ab());
        assert!(!a.profile_iterative_keep_depth_ab());
        assert!(a.profile_iterative_keep_depth_ab_promote());

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
