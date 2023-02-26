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
use std::sync::atomic::{AtomicI32,AtomicBool,Ordering};
use crate::cogitator;
use crate::cogitator::Cogitator;

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
                    self.close_cogitators();
                    return true;
                },

                _ => {},
            }
            false
    }

    pub fn launch_cogitators(&mut self) {
        let mut cogitators: Vec<Cogitator> = vec![];

        //Shared variables
        let barrier = Arc::new(Barrier::new(self.cogitator_thread_count.into()));
        let search_min = Arc::new(AtomicI32::new(SCORE_MAX));
        let search_max = Arc::new(AtomicI32::new(SCORE_MIN));
        let white_move: bool = self.tree_root.reset.white_to_move();

        for thread_id in 0..self.cogitator_thread_count {

            let mut cogitator = cogitator::new(
                thread_id,
                Arc::clone(&barrier),
                Arc::clone(&search_min),
                Arc::clone(&search_max),
                white_move,
                Arc::clone(&self.red_light),
                Arc::clone(&self.exit_signal),
            );

            cogitator.set_child_list(self.tree_children.clone());

            let handle = thread::spawn(move || {
                cogitator.run();
            });
            self.cogitator_handles.push(handle);
        }

        self.close_cogitators();
    }

    pub fn start_cogitators(&mut self) {
        //TODO Clear tree scores
        //TODO Green Light
        //TODO Set Finish Timer
    }

    pub fn close_cogitators(&mut self) {
        self.exit_signal.store(true,Ordering::Relaxed);
        while self.cogitator_handles.len() > 0 {
            let handle = self.cogitator_handles.pop().unwrap();
            handle.join().unwrap();
        }
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