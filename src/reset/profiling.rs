use crate::reset::Reset;

pub fn count_possible_games(fen: &str, depth: u8) -> u64 {
    let mut move_count: u64 = 0;
    let mut r: Reset = crate::reset::new();
    let fen = String::from(fen);
    r.init_from_fen(fen);
    r.in_place_move_tree(depth, &mut move_count);
    move_count
}

//#[ignore]
//#[test]
pub fn burn() {
    // No pawns (A285877), takes about 8 minutes to do 3-6
    let fen = String::from("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,3),96062,"no pawns, ply=3");
    assert_eq!(count_possible_games(&fen,4),4200525,"no pawns, ply=4");
    assert_eq!(count_possible_games(&fen,5),191462298,"no pawns, ply=5");
    assert_eq!(count_possible_games(&fen,6),8509434855,"no pawns, ply=6"); //wrong - short on moves
    // Old Safety: 8509434052
    // New Safety: 8509434052
return;
    // No queens (A285873), takes about 3 minutes to do 4-7
    let fen = String::from("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,4),220447,"no queens, ply=4"); 
    assert_eq!(count_possible_games(&fen,5),5247292,"no queens, ply=5");
    assert_eq!(count_possible_games(&fen,6),124278971,"no queens, ply=6");
    assert_eq!(count_possible_games(&fen,7),3113440755,"no queens, ply=7");

    // Just pawns and king (A285873), takes about 4 minutes to do 4-8
    let fen = String::from("4k3/pppppppp/8/8/8/8/PPPPPPPP/4K3 w - - 0 1");
    assert_eq!(count_possible_games(&fen,4),98766,"pawns and kings, ply=4");
    assert_eq!(count_possible_games(&fen,5),1683597,"pawns and kings, ply=5");
    assert_eq!(count_possible_games(&fen,6),28677387,"pawns and kings, ply=6");
    assert_eq!(count_possible_games(&fen,7),479763588,"pawns and kings, ply=7");
    assert_eq!(count_possible_games(&fen,8),8014917042,"pawns and kings, ply=8");
    //assert_eq!(count_possible_games(&fen,9),132060434889,"pawns and kings, ply=9"); //haven't run - probably takes 5-6 hours
return;
    // No rooks (A285874), takes about 67 minutes to do 4-8
    let fen = String::from("1nbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/1NBQKBN1 w - - 0 1");
    assert_eq!(count_possible_games(&fen,4),188473,"no rooks, ply=4");
    assert_eq!(count_possible_games(&fen,5),4505624,"no rooks, ply=5");
    assert_eq!(count_possible_games(&fen,6),106770421,"no rooks, ply=6");
    assert_eq!(count_possible_games(&fen,7),2770746488,"no rooks, ply=7");
    assert_eq!(count_possible_games(&fen,8),71151220765,"no rooks, ply=8");

    // No knights (A285875)
    let fen = String::from("r1bqkb1r/pppppppp/8/8/8/8/PPPPPPPP/R1BQKB1R w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,3),6572,"no knights, ply=3");
    assert_eq!(count_possible_games(&fen,4),132640,"no knights, ply=4");
    assert_eq!(count_possible_games(&fen,5),3030492,"no knights, ply=5");
    assert_eq!(count_possible_games(&fen,6),68633066,"no knights, ply=6");
    assert_eq!(count_possible_games(&fen,7),1733220521,"no knights, ply=7");

    // No bishops (A285876)
    let fen = String::from("rn1qk1nr/pppppppp/8/8/8/8/PPPPPPPP/RN1QK1NR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,4),260904,"no bishops, ply=4");
    assert_eq!(count_possible_games(&fen,5),6434922,"no bishops, ply=5");
    assert_eq!(count_possible_games(&fen,6),158069690,"no bishops, ply=6");
    assert_eq!(count_possible_games(&fen,7),4126252938,"no bishops, ply=7");
    //assert_eq!(count_possible_games(&fen,8),107097735673,"no bishops, ply=8"); //haven't rerun - probably takes two hours or so

    // Starting position (A048987), takes ~79 minutes to do 1-8
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,1),20,"starting position, ply=1");
    assert_eq!(count_possible_games(&fen,2),400,"starting position, ply=2");
    assert_eq!(count_possible_games(&fen,3),8902,"starting position, ply=3");
    assert_eq!(count_possible_games(&fen,4),197281,"starting position, ply=4");
    assert_eq!(count_possible_games(&fen,5),4865609,"starting position, ply=5");
    assert_eq!(count_possible_games(&fen,6),119060324,"starting position, ply=6");
    assert_eq!(count_possible_games(&fen,7),3195901860,"starting position, ply=7");
    assert_eq!(count_possible_games(&fen,8),84998978956,"starting position, ply=8");

}

impl Reset {


    pub fn in_place_move_tree(&mut self, depth: u8, move_count: &mut u64) {
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
    //use crate::reset;
    //use crate::utils;
    //use crate::reset::Reset;
}
