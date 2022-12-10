use crate::reset::Reset;
use crate::tree::Tree;
use crate::reset::r#const::SCORE_STALEMATE;
use crate::reset::r#const::SCORE_BLACK_CHECKMATE;
use crate::reset::r#const::SCORE_WHITE_CHECKMATE;

impl Tree {

    pub fn iterative_alpha_beta_keep_depth_swap(&mut self, keep_depth: u8, depth: u8, mut min: i32, mut max: i32) -> i32 {
        let mut temp_score: i32 = 0;
        self.reset.conditionally_complete_move_initialization();
        for i in 1..(depth+1) {
            let mut move_count: u64 = 0;
            println!("i == {}",i);
            temp_score = self.alpha_beta_keep_depth_swap(keep_depth,i,min,max,&mut move_count);
            println!("Score == {}",temp_score);
            println!("Move count == {}",move_count);
            self.print_diagnostics();
        }
        temp_score
    }

}


#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::tree::Tree;
    use crate::utils;
    use crate::reset::r#const::SCORE_MIN;
    use crate::reset::r#const::SCORE_MAX;
    use crate::reset::r#const::SCORE_STALEMATE;
    use crate::reset::r#const::SCORE_WHITE_CHECKMATE;
    use crate::reset::r#const::SCORE_BLACK_CHECKMATE;

    #[test]
    fn ab_iterative_keep_depth_stalemate_test() {
        let fen = String::from("8/8/8/8/8/3K4/3B4/3k4 b - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let score = t.iterative_alpha_beta_keep_depth_swap(4, 8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_STALEMATE);

        let fen = String::from("7K/5k2/p4n2/Pp2b3/1P6/8/8/8 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let score = t.iterative_alpha_beta_keep_depth_swap(4, 8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_STALEMATE);
    }

    #[test]
    fn ab_iterative_keep_depth_checkmate_test() {
        let fen = String::from("r1bqkbnr/pppp1Qpp/8/4p3/2BnP3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let score = t.iterative_alpha_beta_keep_depth_swap(4, 8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_WHITE_CHECKMATE);

        let fen = String::from("8/7P/5n2/1P6/2P2p2/4k3/8/r3K3 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let score = t.iterative_alpha_beta_keep_depth_swap(4, 8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_BLACK_CHECKMATE);
    }
}




