use crate::operator::Operator;
use crate::operator::OperatorMessage;
use crate::operator::message::OperatorInstruction::ExitProgram;
use crate::operator::message::OperatorInstruction::TestInstruction;

impl Operator {

    // Process an instruction, communicating with the Orchestrator as appropriate.
    pub fn process_instruction(&mut self, instruction: &String) {
        eprintln!("Input == {}",instruction);
        if instruction == "quit" {
            let mut message = OperatorMessage {
                instruction: ExitProgram,
                data_string: "".to_string(),
            };
            self.orchestrator_transmit_channel.as_ref().unwrap().send(message).unwrap();
        } else if instruction == "protover 2" {
            println!("feature done=1")
        } else {
            let mut message = OperatorMessage {
                instruction: TestInstruction,
                data_string: "".to_string(),
            };
            self.orchestrator_transmit_channel.as_ref().unwrap().send(message).unwrap();
        }
    }

}
