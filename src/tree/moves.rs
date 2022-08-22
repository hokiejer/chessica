use crate::reset::Reset;
use crate::tree::Tree;

impl Tree<Reset> {

    pub fn add_next_child(&mut self) -> bool {
        let mut child = crate::tree::new();
        if self.reset.generate_next_move(&mut child.reset) {
            self.add_child_last(child);
            true
        } else {
            false
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::reset::Reset;
    use crate::tree::Tree;

    #[test]
    fn add_next_child_starting_position() {
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree<Reset> = crate::tree::from_fen(starting_fen);
        let mut count = 0;
        while t.add_next_child() == true {
            count += 1;
        }
        assert_eq!(count,20);
    }

}

