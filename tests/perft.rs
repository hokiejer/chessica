use chessica::reset::profiling::count_possible_games;

#[test]
fn test_perft_numpty2_short() {
    // Numpty 2 - https://sites.google.com/site/numptychess/perft/position-2
    let fen = String::from("8/p7/8/1P6/K1k3p1/6P1/7P/8 w - - 0 1");
    assert_eq!(count_possible_games(&fen,1),5,"Numpty 2, ply=1");
    assert_eq!(count_possible_games(&fen,2),39,"Numpty 2, ply=2");
    assert_eq!(count_possible_games(&fen,3),237,"Numpty 2, ply=3");
    assert_eq!(count_possible_games(&fen,4),2002,"Numpty 2, ply=4");
    assert_eq!(count_possible_games(&fen,5),14062,"Numpty 2, ply=5");
    assert_eq!(count_possible_games(&fen,6),120995,"Numpty 2, ply=6");
    assert_eq!(count_possible_games(&fen,7),966152,"Numpty 2, ply=7");
    assert_eq!(count_possible_games(&fen,8),8103790,"Numpty 2, ply=8");
}

/**************************************************************************/

#[test]
fn test_perft_rook_king_rotation() {
    // Rook-King rotational check - takes about 16.75 minutes to do 1-8
    // Rook-King rotational check - takes about 42 seconds to do 1-7
    let fen1 = String::from("8/kr6/r7/8/8/8/6RK/7R w - - 0 1");
    let fen2 = String::from("5rk1/6r1/8/8/8/8/1R6/RK6 w - - 0 1");
    let fen3 = String::from("R7/KR6/8/8/8/7r/6rk/8 w - - 0 1");
    let fen4 = String::from("6KR/6R1/8/8/8/8/1r6/1kr5 w - - 0 1");

    for depth in 1..5 {
        let count1 = count_possible_games(&fen1,depth);
        let count2 = count_possible_games(&fen2,depth);
        let count3 = count_possible_games(&fen3,depth);
        let count4 = count_possible_games(&fen4,depth);
        assert_eq!(count1,count2,"rook-king rotation 1/2 d={}",depth);
        assert_eq!(count3,count4,"rook-king rotation 3/4 d={}",depth);
        assert_eq!(count1,count3,"rook-king rotation 1/3 d={}",depth);
    }
}

/**************************************************************************/

#[test]
fn test_perft_pos3_short() {
    // Position 3 - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    assert_eq!(count_possible_games(&fen,1),14,"Position 3, ply=1");
    assert_eq!(count_possible_games(&fen,2),191,"Position 3, ply=2");
    assert_eq!(count_possible_games(&fen,3),2812,"Position 3, ply=3");
    assert_eq!(count_possible_games(&fen,4),43238,"Position 3, ply=4");
    assert_eq!(count_possible_games(&fen,5),674624,"Position 3, ply=5");
    assert_eq!(count_possible_games(&fen,6),11030083,"Position 3, ply=6");
}

#[test]
#[ignore]
fn test_perft_pos3_long() {
    // Position 3 - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    assert_eq!(count_possible_games(&fen,7),178633661,"Position 3, ply=7");
    assert_eq!(count_possible_games(&fen,8),3009794393,"Position 3, ply=8");
}

/**************************************************************************/

#[test]
fn test_perft_starting_no_pawns_short() {
    // No pawns (A285877), takes about 8 minutes to do 3-6
    let fen = String::from("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,3),96062,"no pawns, ply=3");
    assert_eq!(count_possible_games(&fen,4),4200525,"no pawns, ply=4");
}

#[test]
#[ignore]
fn test_perft_starting_no_pawns_long() {
    // No pawns (A285877), takes about 8 minutes to do 3-6
    let fen = String::from("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,5),191462298,"no pawns, ply=5");
    assert_eq!(count_possible_games(&fen,6),8509434052,"no pawns, ply=6"); //wrong - short on moves
    // Stockfish: 8509434052
    // Stockfish ply=7: 390020558283
}

/**************************************************************************/

#[test]
fn test_perft_starting_no_queens_short() {
    // No queens (A285873), takes about 3 minutes to do 4-7
    let fen = String::from("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,4),220447,"no queens, ply=4"); 
    assert_eq!(count_possible_games(&fen,5),5247292,"no queens, ply=5");
}

#[test]
#[ignore]
fn test_perft_starting_no_queens_long() {
    // No queens (A285873), takes about 3 minutes to do 4-7
    let fen = String::from("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,6),124278971,"no queens, ply=6");
    assert_eq!(count_possible_games(&fen,7),3113440755,"no queens, ply=7");
}

/**************************************************************************/

#[test]
fn test_perft_starting_pawns_and_king_short() {
    // Just pawns and king (A285873), takes about 4 minutes to do 4-8
    let fen = String::from("4k3/pppppppp/8/8/8/8/PPPPPPPP/4K3 w - - 0 1");
    assert_eq!(count_possible_games(&fen,4),98766,"pawns and kings, ply=4");
    assert_eq!(count_possible_games(&fen,5),1683597,"pawns and kings, ply=5");
    assert_eq!(count_possible_games(&fen,6),28677387,"pawns and kings, ply=6");
}

#[test]
#[ignore]
fn test_perft_starting_pawns_and_king_long() {
    // Just pawns and king (A285873), takes about 4 minutes to do 4-8
    let fen = String::from("4k3/pppppppp/8/8/8/8/PPPPPPPP/4K3 w - - 0 1");
    assert_eq!(count_possible_games(&fen,7),479763588,"pawns and kings, ply=7");
    assert_eq!(count_possible_games(&fen,8),8014917042,"pawns and kings, ply=8");
    //assert_eq!(count_possible_games(&fen,9),132060434889,"pawns and kings, ply=9"); //haven't run - probably takes 5-6 hours
}

/**************************************************************************/

#[test]
fn test_perft_starting_no_rooks_short() {
    // No rooks (A285874), takes about 67 minutes to do 4-8
    let fen = String::from("1nbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/1NBQKBN1 w - - 0 1");
    assert_eq!(count_possible_games(&fen,4),188473,"no rooks, ply=4");
    assert_eq!(count_possible_games(&fen,5),4505624,"no rooks, ply=5");
}

#[test]
#[ignore]
fn test_perft_starting_no_rooks_long() {
    // No rooks (A285874), takes about 67 minutes to do 4-8
    let fen = String::from("1nbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/1NBQKBN1 w - - 0 1");
    assert_eq!(count_possible_games(&fen,6),106770421,"no rooks, ply=6");
    assert_eq!(count_possible_games(&fen,7),2770746488,"no rooks, ply=7");
    //assert_eq!(count_possible_games(&fen,8),71151220765,"no rooks, ply=8");
}

/**************************************************************************/

#[test]
fn test_perft_starting_no_knights_short() {
    // No knights (A285875)
    let fen = String::from("r1bqkb1r/pppppppp/8/8/8/8/PPPPPPPP/R1BQKB1R w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,3),6572,"no knights, ply=3");
    assert_eq!(count_possible_games(&fen,4),132640,"no knights, ply=4");
    assert_eq!(count_possible_games(&fen,5),3030492,"no knights, ply=5");
}

#[test]
#[ignore]
fn test_perft_starting_no_knights_long() {
    // No knights (A285875)
    let fen = String::from("r1bqkb1r/pppppppp/8/8/8/8/PPPPPPPP/R1BQKB1R w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,6),68633066,"no knights, ply=6");
    assert_eq!(count_possible_games(&fen,7),1733220521,"no knights, ply=7");
}

/**************************************************************************/

#[test]
fn test_perft_starting_no_bishops_short() {
    // No bishops (A285876)
    let fen = String::from("rn1qk1nr/pppppppp/8/8/8/8/PPPPPPPP/RN1QK1NR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,4),260904,"no bishops, ply=4");
    assert_eq!(count_possible_games(&fen,5),6434922,"no bishops, ply=5");

}

#[test]
#[ignore]
fn test_perft_starting_no_bishops_long() {
    // No bishops (A285876)
    let fen = String::from("rn1qk1nr/pppppppp/8/8/8/8/PPPPPPPP/RN1QK1NR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,6),158069690,"no bishops, ply=6");
    assert_eq!(count_possible_games(&fen,7),4126252938,"no bishops, ply=7");
    //assert_eq!(count_possible_games(&fen,8),107097735673,"no bishops, ply=8"); //haven't rerun - probably takes two hours or so

}

/**************************************************************************/

#[test]
fn test_perft_starting_position_short() {
    // Starting position (A048987), takes ~79 minutes to do 1-8
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,1),20,"starting position, ply=1");
    assert_eq!(count_possible_games(&fen,2),400,"starting position, ply=2");
    assert_eq!(count_possible_games(&fen,3),8902,"starting position, ply=3");
    assert_eq!(count_possible_games(&fen,4),197281,"starting position, ply=4");
    assert_eq!(count_possible_games(&fen,5),4865609,"starting position, ply=5");

}

#[test]
#[ignore]
fn test_perft_starting_position_long() {
    // Starting position (A048987), takes ~79 minutes to do 1-8
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,6),119060324,"starting position, ply=6");
    assert_eq!(count_possible_games(&fen,7),3195901860,"starting position, ply=7");
    assert_eq!(count_possible_games(&fen,8),84998978956,"starting position, ply=8");

}

/**************************************************************************/

#[test]
fn test_perft_starting_pos2_short() {
    // Position 2 (Kiwipete) - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,1),48,"Position 2 - Kiwipete, ply=1");
    assert_eq!(count_possible_games(&fen,2),2039,"Position 2 - Kiwipete, ply=2");
    assert_eq!(count_possible_games(&fen,3),97862,"Position 2 - Kiwipete, ply=3");
    assert_eq!(count_possible_games(&fen,4),4085603,"Position 2 - Kiwipete, ply=4");
}

#[test]
#[ignore]
fn test_perft_starting_pos2_long() {
    // Position 2 (Kiwipete) - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,5),193690690,"Position 2 - Kiwipete, ply=5");
    assert_eq!(count_possible_games(&fen,6),8031647685,"Position 2 - Kiwipete, ply=6");

}

/**************************************************************************/

#[test]
fn test_perft_pos4_black_short() {
    // Position 4 (Black) - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
    assert_eq!(count_possible_games(&fen,1),6,"Position 4b, ply=1");
    assert_eq!(count_possible_games(&fen,2),264,"Position 4b, ply=2");
    assert_eq!(count_possible_games(&fen,3),9467,"Position 4b, ply=3");
    assert_eq!(count_possible_games(&fen,4),422333,"Position 4b, ply=4");
    assert_eq!(count_possible_games(&fen,5),15833292,"Position 4b, ply=5");
}

#[test]
#[ignore]
fn test_perft_pos4_black_long() {
    // Position 4 (Black) - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
    assert_eq!(count_possible_games(&fen,6),706045033,"Position 4b, ply=6");
}

/**************************************************************************/

#[test]
fn test_perft_pos4_white_short() {
    // Position 4 (White) - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    assert_eq!(count_possible_games(&fen,1),6,"Position 4w, ply=1");
    assert_eq!(count_possible_games(&fen,2),264,"Position 4w, ply=2");
    assert_eq!(count_possible_games(&fen,3),9467,"Position 4w, ply=3");
    assert_eq!(count_possible_games(&fen,4),422333,"Position 4w, ply=4");
    assert_eq!(count_possible_games(&fen,5),15833292,"Position 4w, ply=5");
}

#[test]
#[ignore]
fn test_perft_pos4_white_long() {
    // Position 4 (White) - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    assert_eq!(count_possible_games(&fen,6),706045033,"Position 4w, ply=6");
}

/**************************************************************************/

#[test]
fn test_perft_starting_pos5_short() {
    // Position 5 - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    assert_eq!(count_possible_games(&fen,1),44,"Position 5, ply=1");
    assert_eq!(count_possible_games(&fen,2),1486,"Position 5, ply=2");
    assert_eq!(count_possible_games(&fen,3),62379,"Position 5, ply=3");
    assert_eq!(count_possible_games(&fen,4),2103487,"Position 5, ply=4");
}

#[test]
#[ignore]
fn test_perft_starting_pos5_long() {
    // Position 5 - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    assert_eq!(count_possible_games(&fen,5),89941194,"Position 5, ply=5");
}

/**************************************************************************/

#[test]
fn test_perft_starting_pos6_short() {
    // Position 6 - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    assert_eq!(count_possible_games(&fen,1),46,"Position 6, ply=1");
    assert_eq!(count_possible_games(&fen,2),2079,"Position 6, ply=2");
    assert_eq!(count_possible_games(&fen,3),89890,"Position 6, ply=3");
    assert_eq!(count_possible_games(&fen,4),3894594,"Position 6, ply=4");
}

#[test]
#[ignore]
fn test_perft_starting_pos6_long() {
    // Position 6 - https://www.chessprogramming.org/Perft_Results
    let fen = String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    assert_eq!(count_possible_games(&fen,5),164075551,"Position 6, ply=5");
    assert_eq!(count_possible_games(&fen,6),6923051137,"Position 6, ply=6");
    //assert_eq!(count_possible_games(&fen,7),287188994746,"Position 6, ply=7"); //haven't run -
    //probably takes 10+ hours
}

/**************************************************************************/

#[test]
fn test_perft_castling_short() {
    let fen = String::from("r3k2r/2p2p2/2q5/4N3/8/4Q1n1/8/R3K2R b KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,1),53,"Castling, ply=1");
    assert_eq!(count_possible_games(&fen,2),2289,"Castling, ply=2");
    assert_eq!(count_possible_games(&fen,3),94286,"Castling, ply=3");
    assert_eq!(count_possible_games(&fen,4),3903493,"Castling, ply=4");
}

#[test]
#[ignore]
fn test_perft_castling_long() {
    let fen = String::from("r3k2r/2p2p2/2q5/4N3/8/4Q1n1/8/R3K2R b KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,5),160927133,"Castling, ply=5");
    assert_eq!(count_possible_games(&fen,6),6456074424,"Castling, ply=6");
}

/**************************************************************************/




