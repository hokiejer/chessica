
pub const B_FOUR_CORNERS: u64 = 0x8100000000000081;
pub const B_LOWER_RIGHT_CORNER: u64 = 0x0000000000000001;
pub const B_LOWER_LEFT_CORNER: u64 = 0x0000000000000080;
pub const B_UPPER_RIGHT_CORNER: u64 = 0x0100000000000000;
pub const B_UPPER_LEFT_CORNER: u64 = 0x8000000000000000;

pub const B_SE_CORNER: u64 = 0x0000000000000001;
pub const B_SW_CORNER: u64 = 0x0000000000000080;
pub const B_NE_CORNER: u64 = 0x0100000000000000;
pub const B_NW_CORNER: u64 = 0x8000000000000000;

pub const B_NOT_UL_EDGE: u64 = 0x007f7f7f7f7f7f7f;
pub const B_NOT_UR_EDGE: u64 = 0x00fefefefefefefe;
pub const B_NOT_DL_EDGE: u64 = 0x7f7f7f7f7f7f7f00;
pub const B_NOT_DR_EDGE: u64 = 0xfefefefefefefe00;

pub const B_NOT_TOP_EDGE: u64 = 0x00ffffffffffffff;
pub const B_NOT_RIGHT_EDGE: u64 = 0xfefefefefefefefe;
pub const B_NOT_LEFT_EDGE: u64 = 0x7f7f7f7f7f7f7f7f;
pub const B_NOT_BOTTOM_EDGE: u64 = 0xffffffffffffff00;

pub const B_NOT_NW_EDGE: u64 = 0x007f7f7f7f7f7f7f;
pub const B_NOT_NE_EDGE: u64 = 0x00fefefefefefefe;
pub const B_NOT_SW_EDGE: u64 = 0x7f7f7f7f7f7f7f00;
pub const B_NOT_SE_EDGE: u64 = 0xfefefefefefefe00;

pub const B_NOT_N_EDGE: u64 = 0x00ffffffffffffff;
pub const B_NOT_E_EDGE: u64 = 0xfefefefefefefefe;
pub const B_NOT_W_EDGE: u64 = 0x7f7f7f7f7f7f7f7f;
pub const B_NOT_S_EDGE: u64 = 0xffffffffffffff00;

pub const B_RANK_1: u64 = 0x00000000000000ff;
pub const B_RANK_2: u64 = 0x000000000000ff00;
pub const B_RANK_3: u64 = 0x0000000000ff0000;
pub const B_RANK_4: u64 = 0x00000000ff000000;
pub const B_RANK_5: u64 = 0x000000ff00000000;
pub const B_RANK_6: u64 = 0x0000ff0000000000;
pub const B_RANK_7: u64 = 0x00ff000000000000;
pub const B_RANK_8: u64 = 0xff00000000000000;

pub const B_KNIGHT_CAN_MOVE_0100: u64 = 0x0000fefefefefefe;
pub const B_KNIGHT_CAN_MOVE_0200: u64 = 0x00fcfcfcfcfcfcfc;
pub const B_KNIGHT_CAN_MOVE_0400: u64 = 0xfcfcfcfcfcfcfc00;
pub const B_KNIGHT_CAN_MOVE_0500: u64 = 0xfefefefefefe0000;
pub const B_KNIGHT_CAN_MOVE_0700: u64 = 0x7f7f7f7f7f7f0000;
pub const B_KNIGHT_CAN_MOVE_0800: u64 = 0x3f3f3f3f3f3f3f00;
pub const B_KNIGHT_CAN_MOVE_1000: u64 = 0x003f3f3f3f3f3f3f;
pub const B_KNIGHT_CAN_MOVE_1100: u64 = 0x00007f7f7f7f7f7f;

pub const B_WHITE_CASTLEK_SAFETY: u64 = 0x000000000000000e;
pub const B_WHITE_CASTLEQ_SAFETY: u64 = 0x0000000000000038;
pub const B_BLACK_CASTLEK_SAFETY: u64 = 0x0e00000000000000;
pub const B_BLACK_CASTLEQ_SAFETY: u64 = 0x3800000000000000;
pub const B_WHITE_CASTLEK_EMPTY: u64 = 0x0000000000000006;
pub const B_WHITE_CASTLEQ_EMPTY: u64 = 0x0000000000000030;
pub const B_BLACK_CASTLEK_EMPTY: u64 = 0x0600000000000000;
pub const B_BLACK_CASTLEQ_EMPTY: u64 = 0x3000000000000000;

pub const B_WHITE_CASTLEK_DESTINATION: u64 = 0x0000000000000002;
pub const B_WHITE_CASTLEQ_DESTINATION: u64 = 0x0000000000000020;
pub const B_BLACK_CASTLEK_DESTINATION: u64 = 0x0200000000000000;
pub const B_BLACK_CASTLEQ_DESTINATION: u64 = 0x2000000000000000;
