use crate::operator::Operator;
use crate::operator::message;

impl Operator {

    // Process an instruction, communicating with the Orchestrator as appropriate.
    pub fn process_instruction(&mut self, instruction: &String) {
        println!("Board Input == \"{}\"",instruction);
        let mut message = message::new();
        match instruction.as_str() {
            "quit" => {
                message.exit_program();
            },
            "force" => {
                message.player_status_change(false, false);
            },
            _ => {

            },
        }
        if message.sendable() {
            self.orchestrator_transmit_channel.as_ref().unwrap().send(message).unwrap();
        }
     }

}
