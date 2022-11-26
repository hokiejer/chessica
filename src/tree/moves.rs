use crate::reset::Reset;
use crate::reset::PieceType;
use crate::tree::Tree;

impl Tree {

    pub fn add_next_child(&mut self) -> bool {
        let mut child = crate::tree::new();
        if self.reset.generate_next_move(&mut child.reset) {
            self.add_child_last(child);
            true
        } else {
            false
        }
    }

    pub fn simple_move_tree(&mut self, depth: i32, move_count: &mut u64) {
        if depth == 0 {
            *move_count += 1;
            return
        }
        let mut i = 0;

        self.reset.conditionally_complete_move_initialization();
        while self.add_next_child() {
            let mut child = &mut self.children[i];
            //child.print();
            child.simple_move_tree(depth - 1, move_count);
            i += 1;
        }
        if i == 0 {
            //println!("Found checkmate!");
            //self.print();
        }
        //println!("Has {} children",i);
    }
}

#[cfg(test)]
mod tests {
    use crate::reset::Reset;
    use crate::tree::Tree;

    #[test]
    fn add_next_child_starting_position() {
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = crate::tree::from_fen(starting_fen);
        let mut count = 0;
        while t.add_next_child() == true {
            count += 1;
        }
        assert_eq!(count,20);
    }

}

