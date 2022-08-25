use crate::reset::Reset;

pub fn burn() {
    let mut move_count: u64 = 0;
    let mut r: Reset = crate::reset::new();

    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    r.init_from_fen(fen);
    r.in_place_move_tree(6, &mut move_count);
    assert_eq!(move_count,119060324)
}

impl Reset {


    pub fn in_place_move_tree(&mut self, depth: i32, move_count: &mut u64) {
        if depth == 0 {
            *move_count += 1;
            return
        }
        let mut child = crate::reset::new();

        while self.generate_next_move(&mut child) {
            child.in_place_move_tree(depth - 1, move_count);
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::utils;
    use crate::reset::Reset;
}
