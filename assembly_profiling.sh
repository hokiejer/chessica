#!/bin/bash

echo "TIER 1 (every consideration)"
echo "================================================================"

VALUE=`cargo asm 'chessica::reset::moves::<impl chessica::reset::Reset>::generate_next_move' | wc -l`
echo "chessica::reset::moves::generate_next_move $VALUE"

echo
echo "TIER 2 (proposed moves)"
echo "================================================================"

VALUE=`cargo asm 'chessica::reset::moves::<impl chessica::reset::Reset>::add_move_unconditional' | wc -l`
echo "chessica::reset::moves::add_move_unconditional $VALUE"

VALUE=`cargo asm 'chessica::reset::moves::<impl chessica::reset::Reset>::add_move_if_valid' | wc -l`
echo "chessica::reset::moves::add_move_if_valid $VALUE"

VALUE=`cargo asm 'chessica::reset::child::<impl chessica::reset::Reset>::init_child' | wc -l`
echo "chessica::reset::init_child $VALUE"

VALUE=`cargo asm 'chessica::reset::safe_revealed::<impl chessica::reset::Reset>::is_safe_from_revealed_check' | wc -l`
echo "chessica::reset::safe_revealed::is_safe_from_revealed_check $VALUE"

echo
echo "TIER 3 (only valid moves)"
echo "================================================================"

VALUE=`cargo asm 'chessica::reset::moves::<impl chessica::reset::Reset>::initialize_move_generation' | wc -l`
echo "chessica::reset::moves::initialize_move_generation $VALUE"

VALUE=`cargo asm 'chessica::reset::moves::<impl chessica::reset::Reset>::valid_child_post_processing' | wc -l`
echo "chessica::reset::moves::valid_child_post_processing $VALUE"

echo
echo "TIER 4 (every piece considered, only some moves)"
echo "================================================================"

VALUE=`cargo asm 'chessica::reset::moves::<impl chessica::reset::Reset>::add_move_full_safety_check' | wc -l`
echo "chessica::reset::moves::add_move_full_safety_check $VALUE"

VALUE=`cargo asm 'chessica::reset::moves::<impl chessica::reset::Reset>::consider_next_moveable_piece' | wc -l`
echo "chessica::reset::moves::consider_next_moveable_piece $VALUE"

echo
echo "TIER 5 (other)"
echo "================================================================"

VALUE=`cargo asm 'chessica::reset::new' | wc -l`
echo "chessica::reset::new $VALUE"

VALUE=`cargo asm 'chessica::reset::clone::<impl chessica::reset::Reset>::clone_to' | wc -l`
echo "chessica::reset::clone_to $VALUE"

VALUE=`cargo asm 'chessica::reset::clone::clone_from' | wc -l`
echo "chessica::reset::clone::clone_from $VALUE"

VALUE=`cargo asm 'chessica::reset::score::<impl chessica::reset::Reset>::score' | wc -l`
echo "chessica::reset::score::score $VALUE"
