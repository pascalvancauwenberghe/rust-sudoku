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

pub fn intermediate_sudoku1() -> &'static str {
    r#"
.2.6.8...
58...97..
....4....
37....5..
6.......4
..8....13
....2....
..98...36
...3.6.9.
"#
}

pub fn intermediate_sudoku1_solution() -> &'static str {
    r#"
123678945
584239761
967145328
372461589
691583274
458792613
836924157
219857436
745316892
"#
}

pub fn difficult_sudoku1() -> &'static str {
    r#"
...6..4..
7....36..
....91.8.
.........
.5.18...3
...3.6.45
.4.2...6.
9.3......
.2....1..
"#
}

pub fn difficult_sudoku1_solution() -> &'static str {
    r#"
581672439
792843651
364591782
438957216
256184973
179326845
845219367
913768524
627435198
"#
}

pub fn not_fun_sudoku1() -> &'static str {
    r#"
.2.......
...6....3
.74.8....
.....3..2
.8..4..1.
6..5.....
....1.78.
5....9...
.......4.
"#
}

pub fn not_fun_sudoku1_solution() -> &'static str {
    r#"
126437958
895621473
374985126
457193862
983246517
612578394
269314785
548769231
731852649
"#
}

// https://www.youtube.com/watch?v=zR-ngVP0kVg

pub fn computer_freaks_out() -> &'static str {
    r#"
...1.2...
..8.6.7.5
.9.....8.
4...1....
.8.3.4.6.
3...2.8..
.6...547.
..5.7.6.9
.7.....5.
"#
}

pub fn computer_freaks_out_solution() -> &'static str {
    r#"
547182396
218963745
693547281
429618537
781354962
356729814
162895473
835471629
974236158
"#
}

// https://www.youtube.com/watch?v=SDTtcipqw7M

pub fn given_36_digits() -> &'static str {
    r#"
.23.65.89
9....4..5
5..9.....
6..3...18
38.59...2
....863..
23......6
8.7.2...3
.96.5382.
"#
}

pub fn given_36_digits_solution() -> &'static str {
    r#"
723165489
961874235
548932167
652347918
384591672
179286354
235418796
817629543
496753821
"#
}

// https://www.youtube.com/watch?v=TQ0lso4fJzk

pub fn tatooine_sunset() -> &'static str {
    r#"
.........
..98....7
.8..6..5.
.5..4..3.
..79....2
.........
..27....9
.4..5..6.
3....62..
"#
}

pub fn tatooine_sunset_solution() -> &'static str {
    r#"
124573896
569814327
783269154
251647938
437985612
896321475
612738549
948152763
375496281
"#
}

// https://www.youtube.com/watch?v=z3IAgDi6Ves

pub fn kingda_ka() -> &'static str {
    r#"
...4.6...
.12.3.5..
.......7.
8....4..7
.6.7...5.
5...8...9
..3....2.
....9..1.
...3.5...
"#
}

pub fn kingda_ka_solution() -> &'static str {
    r#"
935476281
712938546
486152973
829564137
364719852
571283469
693841725
258697314
147325698
"#
}

// https://www.youtube.com/watch?v=z3IAgDi6Ves

pub fn jovial_negative() -> &'static str {
    r#"
9...63...
..12..5..
7.2....3.
...3.6.5.
..34.76..
.7.5.8...
.8....1.5
..7..12..
...84...9
"#
}

pub fn jovial_negative_solution() -> &'static str {
    r#"
954163872
831274596
762985431
148326957
523497618
679518324
286739145
497651283
315842769
"#
}
