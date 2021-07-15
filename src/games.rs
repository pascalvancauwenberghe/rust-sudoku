// Examples from https://sandiway.arizona.edu/sudoku/examples.html

pub fn easy_sudoku() -> &'static str {
    r#"
...26.7.1
68..7..9.
19...45..
82.1...4.
..46.29..
.5...3.28
..93...74
.4..5..36
7.3.18...
"#
}

pub fn easy_sudoku_solution() -> &'static str {
    r#"
435269781
682571493
197834562
826195347
374682915
951743628
519326874
248957136
763418259
"#
}

pub fn easy_sudoku2() -> &'static str {
    r#"
1..489..6
73.....4.
.....1295
..712.6..
5..7.3..8
..6.957..
9146.....
.2.....37
8..512..4
"#
}

pub fn easy_sudoku2_solution() -> &'static str {
    r#"
152489376
739256841
468371295
387124659
591763428
246895713
914637582
625948137
873512964
"#
}
