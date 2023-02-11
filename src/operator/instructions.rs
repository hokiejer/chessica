use crate::operator::Operator;
use crate::operator::OperatorMessage;
use crate::operator::message;

impl Operator {

    // Process an instruction, communicating with the Orchestrator as appropriate.
    pub fn process_instruction(&mut self, instruction: &String) {
        println!("Board Input == \"{}\"",instruction);
        let mut message = message::new();
        match instruction.as_str() {
            "e2e4" => {
                message.move_taken();
                self.send(&message);
            }
            "quit" => {
                message.exit_program();
                self.send(&message);
            },
            "force" => {
                message.player_status_change(false, false);
                self.send(&message);
            },
            "new" => {
                message.new_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
                self.send(&message);
                message.player_status_change(false, true);
                self.send(&message);
            },
            _ => {

            },
        }
     }

     pub fn send(&mut self, message: &OperatorMessage) {
        let new_message = message.clone();
        self.orchestrator_transmit_channel.as_ref().unwrap().send(new_message).unwrap();
     }
}
