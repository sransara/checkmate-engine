# Checkmate Engine

Building a chess engine for 

- the [Kaggle Chess AI challenge](https://www.kaggle.com/competitions/fide-google-efficiency-chess-ai-challenge/)
- learn how chess engines work
- Chess Engines are known to use lot of tips and tricks to be highly efficient in analyzing a large search space. Personally I am interested in figuring out whether a compiler can be smart enough to infer these tricks just by static analysis of a high level language constructs.

## General plan by considering components of a chess engine:

- Board representation
	- be able to read from some serialization (like FEN) into this board representation
- Move generation
	- Move validation
	- Enpassant moves
	- Castling validation
- State recognition
	- Checkmate
	- Check
	- 3-fold repetitions
	- Stalemate
	- King only moves
	 Insufficient materials
- Move search tree
	- Monte Carlo Tree Search
	- Alpha-beta pruning 
		- Positional evaluation
			- Neural network based evaluation (NNUE)
			- Coded heuristic evaluation

## Resources

1. Chess Programming Wiki: https://www.chessprogramming.org/

    Great resource to find rabbit holes.

2. Sunfish: https://github.com/thomasahle/sunfish/tree/master

    Simple but strong chess engine written in Python.