CLI sudoku solver and generator

This exposes functionality of the rust sudoku library for testing purposes.
The solver is reasonably fast, capable of solving sudokus in 5-200μs depending on difficulty on a 15W laptop cpu (i5 4210u).

Generation happens in ~25μs (full grid) or ~400μs (uniquely solvable, minimal sudoku) on the same hardware.

The best solvers known to me (ZSolve, fsss2) are ~4x as fast.

```bash
$ echo '.816...9............4.376..6..4..5...3.....7...7..2..4..521.3............7...481.' \
| sudoku
281645793763928145594137628629473581438591276157862934845219367312786459976354812

$ sudoku generate 2
.....25..2...13....1.75....158...4....3.....8.....9.7...7.....152....7.....4.8.3.
45.......7.94.......27.65...2...3.6.....6...5.....53.4876..2.....4...2.....9...1.

$ sudoku generate 2 --solved
371684592592713684846925137785461923423859761169372458658247319237196845914538276
647395128952718643138426957379862514516934872824571396265187439791643285483259761
```