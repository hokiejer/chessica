use crate::orchestrator::Orchestrator;
use crate::operator::message::OperatorMessage;
use crate::operator::message::OperatorInstruction::NewBoard;
use crate::operator::message::OperatorInstruction::MoveTaken;
use crate::operator::message::OperatorInstruction::PlayerStatusChange;
use crate::operator::message::OperatorInstruction::ExitProgram;
use crate::tree;
use std::sync::{Arc, Mutex, mpsc};

impl Orchestrator {

    pub fn process_command(&mut self, received_message: OperatorMessage) -> bool {
            match received_message.instruction {
                MoveTaken => {
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
                ExitProgram => {
                    return true;
                },

                _ => {},
            }
            false
    }
}

#[cfg(test)]
mod tests {
    use crate::orchestrator;
    use crate::operator::message;

    #[test]
    fn exit_program() {
        use std::sync::mpsc;
        let (_t,r) = mpsc::channel();
        let mut o = orchestrator::new(r);
        let mut message = message::new();
        message.exit_program();
        assert!(o.process_command(message)); //returns `true` indicating "quit"
    }

    #[test]
    fn initialize_new_board() {
        use std::sync::mpsc;
        let (_t,r) = mpsc::channel();
        let mut o = orchestrator::new(r);
        let mut message = message::new();
        let fen = String::from("k7/p7/P7/8/8/6Bp/7P/7K w - - 0 1");
        message.new_board(fen);
        assert!(!o.process_command(message)); //returns `false` to go on
        assert_eq!(o.tree_children.len(),9);
    }
}