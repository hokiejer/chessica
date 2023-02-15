use crate::tree::Tree;
use crate::reset::r#const::SCORE_STALEMATE;
use crate::reset::r#const::SCORE_BLACK_CHECKMATE;
use crate::reset::r#const::SCORE_WHITE_CHECKMATE;
use crate::tree::r#const::MAX_CHILDREN_KEPT;

#[allow(clippy::never_loop)]
impl Tree {

    /// Use Alpha-Beta search to that promotes the best move found and prunes when done searching
    ///
    pub fn alpha_beta_promote_prune_parallel(&mut self, depth: u8, max_depth: u8, mut min: i32, mut max: i32, move_count: &mut u64) -> i32 {
        let mut moves_generated: bool = false;
        let mut boards_seen: Vec<u32> = Vec::new();
        if depth == max_depth {
            *move_count += 1;
            self.reset.score()
        } else {
            'outer: loop {
                for c in 0..self.children.len() {
                    let child = &mut self.children[c];
                    moves_generated = true;
                    boards_seen.push(child.reset.child_hash());
                    let temp_score: i32 = child.alpha_beta_promote_prune_parallel(depth+1,max_depth,min,max,move_count);
                    if self.reset.white_to_move() {
                        if temp_score > max {
                            self.promote_last_child_to_first(c);
                            max = temp_score;
                        }
                    } else if temp_score < min {
                        self.promote_last_child_to_first(c);
                        min = temp_score;
                    }
                    if min <= max {
                        break 'outer;
                    }
                }
                self.reset.initialize_move_generation();
                self.reset.complete_move_initialization();
                while self.add_next_child() {
                    let child = self.children.last_mut().unwrap();
                    if boards_seen.contains(&child.reset.child_hash()) {
                        self.children.truncate(MAX_CHILDREN_KEPT);
                        continue;
                    } else {
                        moves_generated = true;
                    }
                    let temp_score: i32 = child.alpha_beta_promote_prune_parallel(depth+1,max_depth,min,max,move_count);
                    if self.reset.white_to_move() {
                        if temp_score > max {
                            self.promote_last_child_to_first(self.children.len()-1);
                            max = temp_score;
                        }
                    } else if temp_score < min {
                        self.promote_last_child_to_first(self.children.len()-1);
                        min = temp_score;
                    }
                    self.children.truncate(MAX_CHILDREN_KEPT);
                    if min <= max {
                        break 'outer;
                    }
                }
                break 'outer;
            }
            if moves_generated {
                if self.reset.white_to_move() {
                    max
                } else {
                    min
                }
            } else if self.reset.in_check() {
                if self.reset.white_to_move() {
                    SCORE_BLACK_CHECKMATE
                } else {
                    SCORE_WHITE_CHECKMATE
                }
            } else {
                SCORE_STALEMATE
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
    fn ab_keep_depth_stalemate_test() {
        let fen = String::from("8/8/8/8/8/3K4/3B4/3k4 b - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_promote_prune_parallel(0, 8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_STALEMATE);

        let fen = String::from("7K/5k2/p4n2/Pp2b3/1P6/8/8/8 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_promote_prune_parallel(0, 8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_STALEMATE);
    }

    #[test]
    fn ab_keep_depth_checkmate_test() {
        let fen = String::from("r1bqkbnr/pppp1Qpp/8/4p3/2BnP3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_promote_prune_parallel(0, 8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_WHITE_CHECKMATE);

        let fen = String::from("8/7P/5n2/1P6/2P2p2/4k3/8/r3K3 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let score = t.alpha_beta_promote_prune_parallel(0, 8, SCORE_MAX, SCORE_MIN, &mut move_count);
        assert_eq!(score,SCORE_BLACK_CHECKMATE);
    }
}


