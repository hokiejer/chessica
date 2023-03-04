use std::sync::{Arc, Barrier, Mutex};
use std::sync::atomic::{AtomicI32,AtomicBool,Ordering};
use crate::tree;
use tree::Tree;
use crate::reset::r#const::SCORE_MIN;
use crate::reset::r#const::SCORE_MAX;


/// Data necessary for the Cogitator functionality to run successfully
///
/// The Cogitator will be launched by the Orchestrator and will build out the game tree and
/// identify the best possible move.  The chess engine itself runs within this thread.
/// Cogitators are built to be scaled horizontally.
///
pub struct Cogitator {
    pub id: u8,
    pub barrier: Arc<Barrier>,
    pub global_min: Arc<AtomicI32>,
    pub global_max: Arc<AtomicI32>,
    pub white_move: bool,
    pub children: Vec<Arc<Mutex<Tree>>>,
    pub red_light: Arc<AtomicBool>,
    pub exit_signal: Arc<AtomicBool>,
}


/// Constructs a new Cogitator
///
/// # Examples
///
/// ```
/// use chessica::cogitator::Cogitator;

/// //let mut my_cogitator = chessica::cogitator::new();
/// ```
pub fn new(
    id: u8,
    barrier: Arc<Barrier>,
    global_min: Arc<AtomicI32>,
    global_max: Arc<AtomicI32>,
    white_move: bool,
    red_light: Arc<AtomicBool>,
    exit_signal: Arc<AtomicBool>,
) -> Cogitator {
    Cogitator {
        id,
        barrier,
        global_min,
        global_max,
        white_move,
        children: vec![],
        red_light,
        exit_signal,
    }
}


impl Cogitator {

    /// Set the child list for the Cogitators to go after
    pub fn set_child_list(&mut self, child_list: Vec<Arc<Mutex<Tree>>>) {
        self.children = child_list;
    }

    /// Run Chessica's Cogitator
    pub fn run(&mut self) {
        for i in 3..9 {
            self.search(i);
            self.barrier.wait();
            if self.id == 0 && self.search_got_far_enough() {
                self.sort_children();
                self.children[0].lock().unwrap().print();
                self.prep_for_next_iteration();
            }
            self.barrier.wait();
        }
    }

    pub fn search(&mut self, depth: u8) {

        let mut locked_trees = Vec::new();
        for tree in &mut self.children {
            if let Ok(mut tree) = tree.try_lock() {
                let mut move_count: u64 = 0;
                let (success, score) = tree.alpha_beta_promote_prune_parallel(
                    0,
                    depth,
                    &(self.global_min),
                    &(self.global_max),
                    &(self.red_light),
                    &mut move_count
                );
                if success {
                    tree.score = Some(score);
                    if self.white_move {
                        while score > self.global_max.load(Ordering::SeqCst) {
                            let temp = self.global_max.load(Ordering::SeqCst);
                            let _r = self.global_max.compare_exchange(temp,score,Ordering::SeqCst,Ordering::SeqCst);
                        }
                    } else {
                        while score < self.global_min.load(Ordering::SeqCst) {
                            let temp = self.global_min.load(Ordering::SeqCst);
                            let _r = self.global_min.compare_exchange(temp,score,Ordering::SeqCst,Ordering::SeqCst);
                        }
                    }
                }
                println!("# Move = {}, Depth = {}, Thread = {}, Score == {} [{}] {}",tree.reset.move_text(),depth,self.id,score,move_count,success);
                locked_trees.push(tree);
            }
        }
        self.barrier.wait();
    }

    pub fn pre_sort_children(&mut self) -> usize {
        //let mut locked_trees = Vec::new();
        let mut i = 0;
        let mut j = self.children.len() - 1;
        while i <= j {
            if self.children[i].lock().unwrap().score.is_some() {
                i += 1;
                continue;
            }
            if self.children[j].lock().unwrap().score.is_none() {
                j -= 1;
                continue;
            }
            self.children.swap(i,j);
            i += 1;
            j -= 1;
        }
        if self.children[i-1].lock().unwrap().score.is_none() {
            i-2
        } else {
            i-1
        }
    }

    pub fn search_got_far_enough(&mut self) -> bool {
        !self.red_light.load(Ordering::SeqCst)
    }

    pub fn sort_children(&mut self) {
        // If the previous top score didn't get a chance at this depth, don't do anything
        let i = self.pre_sort_children();
        if self.white_move {
            self.children[0..=i].sort_by(|a, b| {
                b.lock().unwrap().score.cmp(&a.lock().unwrap().score)
            });
        } else {
            self.children[0..=i].sort_by(|a, b| {
                a.lock().unwrap().score.cmp(&b.lock().unwrap().score)
            });
        }
    }

    pub fn prep_for_next_iteration(&mut self) {
        if self.id == 0 {
            for child in &self.children {
                child.lock().unwrap().score = None;
            }
            self.global_min.store(SCORE_MAX, Ordering::SeqCst);
            self.global_max.store(SCORE_MIN, Ordering::SeqCst);
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::cogitator;
    use cogitator::Cogitator;
    use std::sync::{Arc,Mutex,Barrier};
    use crate::tree;
    use tree::Tree;
    use std::sync::atomic::{AtomicBool,AtomicI32};

    fn prep_cogitator() -> Cogitator {
        let barrier = Arc::new(Barrier::new(1));
        let min = Arc::new(AtomicI32::new(1));
        let max = Arc::new(AtomicI32::new(2));
        let red_light = Arc::new(AtomicBool::new(false));
        let exit_signal = Arc::new(AtomicBool::new(false));
        cogitator::new(0, barrier, min, max, true, red_light, exit_signal)
    }

    fn pre_sort(c: &mut Cogitator, scores: Vec<Option<i32>>) -> usize {
        let mut tree_list: Vec<Arc<Mutex<Tree>>> = Vec::new();
        for score in scores {
            let mut t = tree::new();
            t.score = score;
            tree_list.push(Arc::new(Mutex::new(t)));
        }
        c.set_child_list(tree_list.clone());
        let index = c.pre_sort_children();
        for tree in &c.children {
            match tree.lock().unwrap().score {
                Some(this_score) => {
                    println!("Score == {}",this_score);
                },
                None => {
                    println!("Found a node with no score");
                },
            }
        }
        index
    }

    fn sort(c: &mut Cogitator, scores: Vec<Option<i32>>) {
        let mut tree_list: Vec<Arc<Mutex<Tree>>> = Vec::new();
        for score in scores {
            let mut t = tree::new();
            t.score = score;
            tree_list.push(Arc::new(Mutex::new(t)));
        }
        c.set_child_list(tree_list.clone());
        c.sort_children();
        for tree in &c.children {
            match tree.lock().unwrap().score {
                Some(this_score) => {
                    println!("Score == {}",this_score);
                },
                None => {
                    println!("Found a node with no score");
                },
            }
        }
    }

    #[test]
    fn new_cogitator() {
        //let o = cogitator::new();
        //assert_eq!(o.x,0);
    }

    #[test]
    fn pre_sort_1() {
        let mut c = prep_cogitator();
        let scores = vec![None, Some(123), None, Some(-123), Some(12), Some(100), None];
        let index = pre_sort(&mut c,scores);
        assert!(c.children[index].lock().unwrap().score.is_some());
        assert!(c.children[index+1].lock().unwrap().score.is_none());
        assert!(c.children[0].lock().unwrap().score.is_some());
        assert!(c.children[1].lock().unwrap().score.is_some());
        assert!(c.children[2].lock().unwrap().score.is_some());
        assert!(c.children[3].lock().unwrap().score.is_some());
        assert!(c.children[4].lock().unwrap().score.is_none());
        assert!(c.children[5].lock().unwrap().score.is_none());
        assert!(c.children[6].lock().unwrap().score.is_none());
    }

    #[test]
    fn pre_sort_2() {
        let mut c = prep_cogitator();
        let scores = vec![Some(5), None, None, None, None, None];
        let index = pre_sort(&mut c,scores);
        assert!(c.children[index].lock().unwrap().score.is_some());
        assert!(c.children[index+1].lock().unwrap().score.is_none());
        assert!(c.children[0].lock().unwrap().score.is_some());
        assert!(c.children[1].lock().unwrap().score.is_none());
        assert!(c.children[2].lock().unwrap().score.is_none());
        assert!(c.children[3].lock().unwrap().score.is_none());
        assert!(c.children[4].lock().unwrap().score.is_none());
        assert!(c.children[5].lock().unwrap().score.is_none());
    }

    #[test]
    fn pre_sort_3() {
        let mut c = prep_cogitator();
        let scores = vec![None, None, Some(6), None];
        let index = pre_sort(&mut c,scores);
        assert!(c.children[index].lock().unwrap().score.is_some());
        assert!(c.children[index+1].lock().unwrap().score.is_none());
        assert!(c.children[0].lock().unwrap().score.is_some());
        assert!(c.children[1].lock().unwrap().score.is_none());
        assert!(c.children[2].lock().unwrap().score.is_none());
        assert!(c.children[3].lock().unwrap().score.is_none());
    }

    #[test]
    fn pre_sort_4() {
        let mut c = prep_cogitator();
        let scores = vec![None, Some(-2), Some(6), None];
        let index = pre_sort(&mut c,scores);
        assert!(c.children[index].lock().unwrap().score.is_some());
        assert!(c.children[index+1].lock().unwrap().score.is_none());
        assert!(c.children[0].lock().unwrap().score.is_some());
        assert!(c.children[1].lock().unwrap().score.is_some());
        assert!(c.children[2].lock().unwrap().score.is_none());
        assert!(c.children[3].lock().unwrap().score.is_none());
    }

    #[test]
    fn sort_1() {
        let mut c = prep_cogitator();
        let scores = vec![None, Some(123), None, Some(-123), Some(12), Some(100), None];
        sort(&mut c,scores);
        assert_eq!(c.children[0].lock().unwrap().score,Some(123));
        assert_eq!(c.children[1].lock().unwrap().score,Some(100));
        assert_eq!(c.children[2].lock().unwrap().score,Some(12));
        assert_eq!(c.children[3].lock().unwrap().score,Some(-123));
        assert_eq!(c.children[4].lock().unwrap().score,None);
        assert_eq!(c.children[5].lock().unwrap().score,None);
        assert_eq!(c.children[6].lock().unwrap().score,None);
    }

    #[test]
    fn sort_2() {
        let mut c = prep_cogitator();
        c.white_move = false;
        let scores = vec![None, Some(-2), Some(6), None];
        sort(&mut c,scores);
        assert_eq!(c.children[0].lock().unwrap().score,Some(-2));
        assert_eq!(c.children[1].lock().unwrap().score,Some(6));
        assert_eq!(c.children[2].lock().unwrap().score,None);
        assert_eq!(c.children[3].lock().unwrap().score,None);
    }

    #[test]
    fn sort_3() {
        let mut c = prep_cogitator();
        let scores = vec![None, Some(-2), Some(6), None];
        sort(&mut c,scores);
        assert_eq!(c.children[0].lock().unwrap().score,Some(6));
        assert_eq!(c.children[1].lock().unwrap().score,Some(-2));
        assert_eq!(c.children[2].lock().unwrap().score,None);
        assert_eq!(c.children[3].lock().unwrap().score,None);
    }

}
