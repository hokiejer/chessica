use crate::reset::Reset;

impl Reset {

    pub fn score(&mut self) -> i32 {
        self.score = self.material as i32 * 1000000;
        let randomfactor: i32 = ((self.b_all % 1997) - 998).try_into().unwrap();
        self.score += randomfactor;
        #[cfg(debug_assertions)]
        {
            println!("IN DEBUG");
            return self.score;
        }
        println!("NOT IN DEBUG");
        self.score
    }

}


#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;


    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn score_test() {
        //This is a crappy test
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let score = r.score();
        let testvalue = score / 1000;
        assert_eq!(testvalue,0);
    }
}
