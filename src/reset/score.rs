use crate::reset::Reset;
use crate::reset::r#const::SCORE_STALEMATE;
use crate::reset::r#const::SCORE_BLACK_CHECKMATE;
use crate::reset::r#const::SCORE_WHITE_CHECKMATE;

impl Reset {

    pub fn score(&mut self) -> i32 {
        let mut child = crate::reset::new();
        let mut clone = crate::reset::clone::clone_from(self);
        clone.initialize_move_generation();
        clone.complete_move_initialization();
        if !clone.generate_next_move(&mut child) {
            if clone.in_check() {
                if clone.white_to_move() {
                    //println!("Found Black Checkmate???");
                    self.score = SCORE_BLACK_CHECKMATE;
                } else {
                    //println!("Found White Checkmate???");
                    self.score = SCORE_WHITE_CHECKMATE;
                }
            } else {
                //println!("Found Stalemate???");
                self.score = SCORE_STALEMATE;
            }
            return self.score;
        }
        self.score = self.material as i32 * 1000000;
        let randomfactor: i32 = (self.b_all % 1997) as i32 - 998;
        self.score += randomfactor;
        self.score
    }

}


#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::reset::r#const::SCORE_STALEMATE;
    use crate::reset::r#const::SCORE_WHITE_CHECKMATE;
    use crate::reset::r#const::SCORE_BLACK_CHECKMATE;

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

    #[test]
    fn score_stalemate_test() {
        let mut r = prep_board("8/8/8/8/8/3K4/3B4/3k4 b - - 0 1");
        let score = r.score();
        assert_eq!(score,SCORE_STALEMATE);

        let mut r = prep_board("7K/5k2/p4n2/Pp2b3/1P6/8/8/8 w - - 0 1");
        let score = r.score();
        assert_eq!(score,SCORE_STALEMATE);
    }

    #[test]
    fn score_checkmate_test() {
        let mut r = prep_board("r1bqkbnr/pppp1Qpp/8/4p3/2BnP3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1");
        let score = r.score();
        assert_eq!(score,SCORE_WHITE_CHECKMATE);

        let mut r = prep_board("8/7P/5n2/1P6/2P2p2/4k3/8/r3K3 w - - 0 1");
        let score = r.score();
        assert_eq!(score,SCORE_BLACK_CHECKMATE);
    }
}
