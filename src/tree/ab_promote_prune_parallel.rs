use crate::tree::Tree;
use crate::reset::r#const::SCORE_STALEMATE;
use crate::reset::r#const::SCORE_BLACK_CHECKMATE;
use crate::reset::r#const::SCORE_WHITE_CHECKMATE;
use crate::tree::r#const::MAX_CHILDREN_KEPT;
use std::sync::atomic::{AtomicI32, AtomicBool, Ordering};
use std::cmp;

#[allow(clippy::never_loop)]
impl Tree {

    /// Use Alpha-Beta search to that promotes the best move found and prunes when done searching
    ///
    pub fn alpha_beta_promote_prune_parallel(
        &mut self,
        depth: u8,
        max_depth: u8,
        min: &AtomicI32,
        max: &AtomicI32,
        red_light: &AtomicBool,
        move_count: &mut u64) -> (bool,i32)
    {
        let mut local_min = min.load(Ordering::SeqCst);
        let mut local_max = max.load(Ordering::SeqCst);
        //println!("=====ABPPP was called with [{},{}]",local_min,local_max);
        let mut moves_generated: bool = false;
        let mut boards_seen: Vec<u32> = Vec::new();
        let mut successful_search: bool = true;
        if depth == max_depth {
            *move_count += 1;
            (true, self.reset.score())
        } else {
            'outer: loop {
                for c in 0..self.children.len() {
                    let child = &mut self.children[c];
                    moves_generated = true;
                    boards_seen.push(child.reset.child_hash());

                    let temp_score: i32 = child.alpha_beta_promote_prune(depth+1,max_depth,local_min,local_max,move_count);
                    if self.reset.white_to_move() {
                        local_max = max.load(Ordering::SeqCst);
                        if temp_score > local_max {
                            self.promote_last_child_to_first(c);
                            local_max = temp_score;
                        }
                    } else {
                        local_min = min.load(Ordering::SeqCst);
                        if temp_score < local_min {
                            self.promote_last_child_to_first(c);
                            local_min = temp_score;
                        }
                    }
                    if local_min <= local_max {
                        successful_search = false;
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
                    let temp_score: i32 = child.alpha_beta_promote_prune(depth+1,max_depth,local_min,local_max,move_count);
                    if self.reset.white_to_move() {
                        local_max = max.load(Ordering::SeqCst);
                        if temp_score > local_max {
                            self.promote_last_child_to_first(self.children.len()-1);
                            local_max = temp_score;
                        }
                    } else {
                        local_min = min.load(Ordering::SeqCst);
                        if temp_score < local_min {
                            self.promote_last_child_to_first(self.children.len()-1);
                            local_min = temp_score;
                        }
                    }
                    self.children.truncate(MAX_CHILDREN_KEPT);
                    //println!("Temp Score == {} [{},{}]",temp_score,local_min,local_max);
                    if local_min <= local_max {
                        successful_search = false;
                        break 'outer;
                    }
                }
                break 'outer;
            }
            if moves_generated {
                if self.reset.white_to_move() {
                    (successful_search, local_max)
                } else {
                    (successful_search, local_min)
                }
            } else if self.reset.in_check() {
                if self.reset.white_to_move() {
                    (true, SCORE_BLACK_CHECKMATE)
                } else {
                    (true, SCORE_WHITE_CHECKMATE)
                }
            } else {
                (true, SCORE_STALEMATE)
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool,AtomicI32};


    #[test]
    fn ab_keep_depth_stalemate_test() {
        let search_min = Arc::new(AtomicI32::new(SCORE_MAX));
        let search_max = Arc::new(AtomicI32::new(SCORE_MIN));
        let red_light = Arc::new(AtomicBool::new(false));

        let fen = String::from("8/8/8/8/8/3K4/3B4/3k4 b - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let (success, score) = t.alpha_beta_promote_prune_parallel(0, 8, &search_min, &search_max, &red_light, &mut move_count);
        assert!(success);
        assert_eq!(score,SCORE_STALEMATE);

        let fen = String::from("7K/5k2/p4n2/Pp2b3/1P6/8/8/8 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let (success, score) = t.alpha_beta_promote_prune_parallel(0, 8, &search_min, &search_max, &red_light, &mut move_count);
        assert!(success);
        assert_eq!(score,SCORE_STALEMATE);
    }

    #[test]
    fn ab_keep_depth_checkmate_test() {
        let search_min = Arc::new(AtomicI32::new(SCORE_MAX));
        let search_max = Arc::new(AtomicI32::new(SCORE_MIN));
        let red_light = Arc::new(AtomicBool::new(false));

        let fen = String::from("r1bqkbnr/pppp1Qpp/8/4p3/2BnP3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let (success, score) = t.alpha_beta_promote_prune_parallel(0, 8, &search_min, &search_max, &red_light, &mut move_count);
        assert!(success);
        assert_eq!(score,SCORE_WHITE_CHECKMATE);

        let fen = String::from("8/7P/5n2/1P6/2P2p2/4k3/8/r3K3 w - - 0 1");
        let mut t: Tree = crate::tree::from_fen(fen);
        let mut move_count: u64 = 0;
        let (success, score) = t.alpha_beta_promote_prune_parallel(0, 8, &search_min, &search_max, &red_light, &mut move_count);
        assert!(success);
        assert_eq!(score,SCORE_BLACK_CHECKMATE);
    }
}


