# CLI sudoku solver and generator

This program exposes functionality of the Rust sudoku library for testing purposes.
The solver is very fast, capable of solving sudokus in 2-200μs depending on difficulty on a 15W laptop cpu (i5 4210u).
It uses an implementation of one of the fastest sudoku solver algorithms (JCZsolve) with a few adaptions.
Sudoku generation takes ~25μs (full grid) or ~150μs (uniquely solvable, minimal sudoku) on the same hardware.
The program is completely parallelised.

```bash
# solve sudoku
$ echo '.816...9............4.376..6..4..5...3.....7...7..2..4..521.3............7...481.' \
| rudoku solve
281645793763928145594137628629473581438591276157862934845219367312786459976354812

# check unique solvability and bench solver speed
$ rudoku solve --stat top50k.txt sudoku17.txt top1465.txt 500x_hard20.txt 10x_hard375.txt
    total    unique nonunique   invalid   time [s]  sudokus/s
    50000     50000         0         0      0.125     400222 top50k.txt
    49151     49151         0         0      0.078     632572 sudoku17.txt
     1465      1465         0         0      0.011     134262 top1465.txt
    10000     10000         0         0      0.574      17417 500x_hard20.txt
     3750      3750         0         0      0.295      12720 10x_hard375.txt

# generate random, minimal sudokus (mostly easy)
$ rudoku generate 2
.....25..2...13....1.75....158...4....3.....8.....9.7...7.....152....7.....4.8.3.
45.......7.94.......27.65...2...3.6.....6...5.....53.4876..2.....4...2.....9...1.

$ rudoku generate 2 --solved
371684592592713684846925137785461923423859761169372458658247319237196845914538276
647395128952718643138426957379862514516934872824571396265187439791643285483259761

# perform random symmetry transformations that do not change solvability or difficulty
$ head -n 1 hard20.txt | rudoku shuffle
..4.5.3.....2............1..9.....7.....4.....1.....62.....1....6...7.....3...4.5
$ head -n 1 hard20.txt | rudoku shuffle
9.3...1...............2..5..2..4..........6.9...8.....1.6..9.......5..2.8......4.

# find the minlex canonical form of the sudoku
$ rudoku generate 3 --solved | tee random_sudokus.txt
354987261718236954962145738687493512245718396193562487826354179571629843439871625
375241986846937521129856734294513678583769412617428359462175893738692145951384267
194768253587213946623945817975821634836594172412637598249176385368459721751382469

# the number behind the minlex form is the number of automorphisms
# https://en.wikipedia.org/wiki/Mathematics_of_Sudoku#Automorphic_Sudokus
$ rudoku canonicalize random_sudokus.txt
123456789457189236689273541236795418875641923941328657314962875562837194798514362 1
123456789457189236869732145281564397395827614746913852514678923638295471972341568 1
123456789457189263896327514281563497574918632639742851368294175712635948945871326 1

$ MOST_CANONICAL_SUDOKU="123456789456789123789123456231564897564897231897231564312645978645978312978312645"
$ echo $MOST_CANONICAL_SUDOKU | rudoku canonicalize
123456789456789123789123456231564897564897231897231564312645978645978312978312645 648
```
