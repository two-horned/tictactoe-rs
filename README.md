# tictactoe-rs
This is Rust implementation of my tictactoe game evaluater.

For further information see https://github.com/two-horned/tictactoe

## Not mentioned in the python project
My Evaluater is highly efficient due to implementing an alpha-beta-pruning algorithm.
Branches are cut away, that won't change the outcome of the game, when using a minimax algorithm.

Flawed is that we are not further able to analyse which decision is the most likely to
lead to a win because we'd need to compute every outcome.

When comparing to other algorithms my algorithm makes use of dictionaries and
early pruning of symmetric alike decisions.

The dictionary is used to lookup an already known position and fetches
results accordingly.

Analysing symmetry allows us to reduce how many decisions are relevant for our problem.
Redundant decisions can be pruned away immediately.

## Benchmark compared to python
Rust can be compiled with the release build which will lead to better peformance in most cases,
due to certain cpu related optimiziations.

|                   | Rust (recursive)  | Rust w/ release build (recursive)  | Python (recursive) | Python (iterative)  |
|-------------------|-------------------|------------------------------------|---------- ---------|---------------------|
| Starting position |  2.56 ms          | 456µs                              | 34.80 ms           | 60.41 ms            |

