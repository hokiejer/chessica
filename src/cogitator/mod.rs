use std::thread;
use std::sync::{Arc, Barrier, Mutex};
use std::sync::atomic::{AtomicI32,AtomicBool,Ordering};
use crate::tree;
use tree::Tree;


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

/// let mut my_cogitator = chessica::cogitator::new();
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
        id: id,
        barrier: barrier,
        global_min: global_min,
        global_max: global_max,
        white_move: white_move,
        children: vec![],
        red_light: red_light,
        exit_signal: exit_signal,
    }
}


impl Cogitator {

    /// Set the child list for the Cogitators to go after
    pub fn set_child_list(&mut self, child_list: Vec<Arc<Mutex<Tree>>>) {
        self.children = child_list;
    }

    /// Run Chessica's Cogitator
    pub fn run(&mut self) {
        self.search(6);
        self.barrier.wait();
        println!("Mind the gap!");
        self.barrier.wait();
        self.pre_sort_children();
        self.barrier.wait();

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
                            let _r = self.global_max.compare_exchange(temp,score,Ordering::Acquire,Ordering::SeqCst);
                        }
                    } else {
                        while score < self.global_min.load(Ordering::SeqCst) {
                            let temp = self.global_min.load(Ordering::SeqCst);
                            let _r = self.global_min.compare_exchange(temp,score,Ordering::Acquire,Ordering::SeqCst);
                        }
                    }
                }
                println!("Move = {}, Thread = {}, Score == {} [{}] {}",tree.reset.move_text(),self.id,score,move_count,success);
                locked_trees.push(tree);
            }
        }
        self.barrier.wait();
    }

    pub fn pre_sort_children(&mut self) {
        //let mut locked_trees = Vec::new();
        if self.id == 0 {
            println!("Leader has to sort here!");
            let mut i = 0;
            let mut j = self.children.len() - 1;
            while i <= j {
                if self.children[i].lock().unwrap().score != None {
                    i += 1;
                    continue;
                }
                if self.children[j].lock().unwrap().score == None {
                    j -= 1;
                    continue;
                }
                self.children.swap(i,j);
                i += 1;
                j -= 1;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::cogitator;
    use std::sync::{Arc,Mutex,Barrier};
    use crate::tree;
    use tree::Tree;
    use std::sync::atomic::{AtomicBool,AtomicI32};

    #[test]
    fn new_cogitator() {
        //let o = cogitator::new();
        //assert_eq!(o.x,0);
    }

    #[test]
    fn sorting() {
        let barrier = Arc::new(Barrier::new(1));
        let min = Arc::new(AtomicI32::new(1));
        let max = Arc::new(AtomicI32::new(2));
        let red_light = Arc::new(AtomicBool::new(false));
        let exit_signal = Arc::new(AtomicBool::new(false));
        let mut c = cogitator::new(0, barrier, min, max, true, red_light, exit_signal);

        let mut tree_list: Vec<Arc<Mutex<Tree>>> = Vec::new();
        let scores = vec![None, Some(123), None, Some(-123), Some(12), Some(100), None];
        for score in scores {
            let mut t = tree::new();
            t.score = score;
            tree_list.push(Arc::new(Mutex::new(t)));
        }
        c.set_child_list(tree_list.clone());
        c.pre_sort_children();
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
        assert!(c.children[0].lock().unwrap().score.is_some());
        assert!(c.children[1].lock().unwrap().score.is_some());
        assert!(c.children[2].lock().unwrap().score.is_some());
        assert!(c.children[3].lock().unwrap().score.is_some());
        assert_eq!(c.children[4].lock().unwrap().score,None);
        assert_eq!(c.children[5].lock().unwrap().score,None);
        assert_eq!(c.children[6].lock().unwrap().score,None);
    }
}
