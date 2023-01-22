#[cfg(test)]
mod tests {
    use crate::reset::Reset;
    use crate::utils;

    impl Reset {
        pub fn current_piece_init(&mut self, square: &str) {
            self.b_current_piece = utils::convert_square_to_bitstring(square.to_string());
            self.set_current_piece_type();
        }
    }
}
