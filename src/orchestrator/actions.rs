use crate::orchestrator::Orchestrator;
use crate::operator::message::OperatorMessage;
use crate::operator::message::OperatorInstruction::NewBoard;
use crate::operator::message::OperatorInstruction::MoveTaken;
use crate::operator::message::OperatorInstruction::PlayerStatusChange;
use crate::operator::message::OperatorInstruction::ExitProgram;
use crate::tree;
//use tree::Tree;
use std::thread;
use std::sync::{Arc, Barrier, Mutex};
//use crossbeam_channel::unbounded;
use crate::reset::r#const::SCORE_MIN;
use crate::reset::r#const::SCORE_MAX;

impl Orchestrator {

    pub fn process_command(&mut self, received_message: OperatorMessage) -> bool {
            match received_message.instruction {
                MoveTaken => {
                    self.launch_cogitators();
                },
                NewBoard => {
                    self.tree_root = tree::from_fen(received_message.data_string);
                    //Do I need to initialize move generation??
                    loop {
                        let mut child = crate::tree::new();
                        if self.tree_root.get_next_child(&mut child) {
                            let serialized_child = Arc::new(Mutex::new(child));
                            self.tree_children.push(serialized_child);
                        } else {
                            break;
                        }
                    }
                    let mut m: u64 = 0;

                    self.tree_root.simple_move_tree(1,&mut m);
                },
                PlayerStatusChange => {

                },
                ExitProgram => {
                    return true;
                },

                _ => {},
            }
            false
    }

    pub fn launch_cogitators(&mut self) {
        //self.tree_children.iter_mut().for_each(|child| {
            //let (s, r) = unbounded(); //mpsc::channel::<Arc<Mutex<Tree>>>();
            let mut handles = vec![];
            let barrier = Arc::new(Barrier::new(self.cogitator_thread_count.into()));
            for thread_id in 0..self.cogitator_thread_count {
                let b = Arc::clone(&barrier);
                let children = self.tree_children.clone();
                let handle = thread::spawn(move || {
                    let mut locked_trees = Vec::new();
                    for tree in &children {
                        if let Ok(mut tree) = tree.try_lock() {
                            let mut move_count: u64 = 0;
                            let score = tree.alpha_beta_promote_prune(
                                0,
                                6,
                                SCORE_MAX,
                                SCORE_MIN,
                                &mut move_count
                            );
                            println!("Move = {}, Thread = {}]",tree.reset.move_text(),thread_id);
                            locked_trees.push(tree);
                        }
                    }
                    println!("Thread {} waiting on barrier",thread_id);
                    b.wait();
                    println!("Thread {} beyond barrier",thread_id);
                 });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
            //s.send(child.clone()).unwrap();
        //});
    }

}

#[cfg(test)]
mod tests {
    use crate::orchestrator;
    use crate::operator::message;

    #[test]
    fn exit_program() {
        let mut message = message::new();
        message.exit_program();
        let mut o = orchestrator::new();
        assert!(o.process_command(message)); //returns `true` indicating "quit"
    }

    #[test]
    fn initialize_new_board() {
        let mut message = message::new();
        let fen = String::from("k7/p7/P7/8/8/6Bp/7P/7K w - - 0 1");
        message.new_board(fen);
        let mut o = orchestrator::new();
        assert!(!o.process_command(message)); //returns `false` to go on
        assert_eq!(o.tree_children.len(),9);
    }

    #[test]
    fn launch_cogitators() {
        let mut message = message::new();
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        message.new_board(fen);
        let mut o = orchestrator::new();
        assert!(!o.process_command(message)); //returns `false` to go on
        let mut message = message::new();
        message.move_taken();
        assert!(!o.process_command(message)); //returns `false` to go on
        assert!(false);
    }

}