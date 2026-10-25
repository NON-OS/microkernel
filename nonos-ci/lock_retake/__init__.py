# The twice-taken lock check (check-lock-retake.py), in parts:
#   text.py   the source with comments and strings blanked, and its braces;
#   holds.py  where each take of a lock is held, by Rust 2021's drop rules;
#   graph.py  which functions take which locks, and the findings.
