use crate::tree::Tree;
use crate::reset::r#const::SCORE_STALEMATE;
use crate::reset::r#const::SCORE_BLACK_CHECKMATE;
use crate::reset::r#const::SCORE_WHITE_CHECKMATE;

impl Tree {

    pub fn alpha_beta_in_place(&mut self, depth: u8, mut min: i32, mut max: i32, move_count: &mut u64) -> i32 {
        let mut moves_generated: bool = false;
        if depth == 0 {
            *move_count += 1;
            self.reset.score()
        } else {
            let mut child: Tree = crate::tree::new();
            self.reset.conditionally_complete_move_initialization();
            while self.reset.generate_next_move(&mut child.reset) {
                moves_generated = true;
                let temp_score: i32 = child.alpha_beta_in_place(depth-1,min,max,move_count);
                if self.reset.white_to_move() {
                    if temp_score > max {
                        max = temp_score;
                    }
                } else {
                    if temp_score < min {
                        min = temp_score;
                    }
                }
                if min <= max {
                    break;
                }
            }
            if moves_generated {
                if self.reset.white_to_move() {
                    max
                } else {
                    min
                }
            } else {
                if self.reset.in_check() {
                    if self.reset.white_to_move() {
                        //println!("Found Black Checkmate???");
                        SCORE_BLACK_CHECKMATE
                    } else {
                        //println!("Found White Checkmate???");
                        SCORE_WHITE_CHECKMATE
                    }
                } else {
                        //println!("Found Stalemate???");
                    SCORE_STALEMATE
                }
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    use crate::reset::r#const::SCORE_MIN;
    use crate::reset::r#const::SCORE_MAX;
    use crate::reset::r#const::SCORE_STALEMATE;
    use crate::reset::r#const::SCORE_WHITE_CHECKMATE;
    use crate::reset::r#const::SCORE_BLACK_CHECKMATE;

    #[test]
    fn ab_in_place_stalemate_test() {
        let fen = String::from("8/8/8/8/8/3K4/3B4/3k4 b - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_STALEMATE);

        let fen = String::from("7K/5k2/p4n2/Pp2b3/1P6/8/8/8 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_STALEMATE);
    }

    #[test]
    fn ab_in_place_checkmate_test() {
        let fen = String::from("r1bqkbnr/pppp1Qpp/8/4p3/2BnP3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_WHITE_CHECKMATE);

        let fen = String::from("8/7P/5n2/1P6/2P2p2/4k3/8/r3K3 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_BLACK_CHECKMATE);
    }
}

