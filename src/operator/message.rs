#[derive(PartialEq,Eq,Copy,Clone,Hash,Debug)]
pub enum OperatorInstruction {
    Instruction1,
    Instruction2,
}

#[derive(PartialEq,Eq,Copy,Clone,Hash,Debug)]
pub struct OperatorMessage {
    pub instruction: OperatorInstruction,
}

