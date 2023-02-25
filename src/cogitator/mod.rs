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
}


/// Constructs a new Cogitator
///
/// # Examples
///
/// ```
/// use chessica::cogitator::Cogitator;

/// let mut my_cogitator = chessica::cogitator::new();
/// ```
pub fn new(id: u8, barrier: Arc<Barrier>, global_min: Arc<AtomicI32>, global_max: Arc<AtomicI32>, white_move: bool, red_light: Arc<AtomicBool>) -> Cogitator {
    Cogitator {
        id: id,
        barrier: barrier,
        global_min: global_min,
        global_max: global_max,
        white_move: white_move,
        children: vec![],
        red_light: red_light,
    }
}


impl Cogitator {

    /// Set the child list for the Cogitators to go after
    pub fn set_child_list(&mut self, child_list: Vec<Arc<Mutex<Tree>>>) {
        self.children = child_list;
    }

    /// Run Chessica's Cogitator
    pub fn run(&self) {
        self.search();
    }

    pub fn search(&self) {

        let mut locked_trees = Vec::new();
        for tree in &(self.children) {
            if let Ok(mut tree) = tree.try_lock() {
                let mut move_count: u64 = 0;
                let (success, score) = tree.alpha_beta_promote_prune_parallel(
                    0,
                    6,
                    &(self.global_min),
                    &(self.global_max),
                    &(self.red_light),
                    &mut move_count
                );
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
                println!("Move = {}, Thread = {}, Score == {} [{}] {}",tree.reset.move_text(),self.id,score,move_count,success);
                locked_trees.push(tree);
            }
        }
        self.barrier.wait();

    }


}

#[cfg(test)]
mod tests {
    use crate::cogitator;

    #[test]
    fn new_cogitator() {
        //let o = cogitator::new();
        //assert_eq!(o.x,0);
    }

}
