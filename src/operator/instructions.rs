use crate::operator::Operator;

impl Operator {

    // Process an instruction, communicating with the Orchestrator as appropriate.
    pub fn process_instruction(&mut self, instruction: &String) {
        println!("Input == {}",instruction);
    }

}
