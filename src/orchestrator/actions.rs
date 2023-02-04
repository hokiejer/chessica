use crate::orchestrator::Orchestrator;
use crate::operator::message::OperatorMessage;
use crate::operator::message::OperatorInstruction::NewBoard;
use crate::operator::message::OperatorInstruction::MoveTaken;
use crate::operator::message::OperatorInstruction::PlayerStatusChange;
use crate::operator::message::OperatorInstruction::ExitProgram;
use crate::tree;

impl Orchestrator {

    pub fn process_command(&mut self, received_message: OperatorMessage) -> bool {
            match received_message.instruction {
                MoveTaken => {
                },
                NewBoard => {
                    self.tree_root = tree::from_fen(received_message.data_string);
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

    #[test]
    fn test_something () {

    }
}