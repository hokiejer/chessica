use crate::orchestrator::Orchestrator;
use crate::operator::message::OperatorMessage;
use crate::operator::message::OperatorInstruction::NewBoard;
use crate::operator::message::OperatorInstruction::MoveTaken;
use crate::operator::message::OperatorInstruction::PlayerStatusChange;
use crate::operator::message::OperatorInstruction::ExitProgram;
use crate::tree;
use tree::Tree;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use rayon::prelude::*;
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
                            let mut serialized_child = Arc::new(Mutex::new(child));
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
        self.tree_children.par_iter_mut().for_each(|child| {
            let (tx, rx) = mpsc::channel::<Arc<Mutex<Tree>>>();
            let rx = Arc::new(Mutex::new(rx));
            for _ in 0..3 {
                let rx = rx.clone();
                thread::spawn(move || {
                    while let Ok(tree) = rx.lock().unwrap().recv() {
                        /*
                        let mut move_count: u64 = 0;
                        let score = child.alpha_beta_promote_prune(
                            0,
                            6,
                            SCORE_MAX,
                            SCORE_MIN,
                            &mut move_count
                        );
                        println!("Score == {} ({})",score,move_count)
                        */
                    }
                });
            }
        });
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
}