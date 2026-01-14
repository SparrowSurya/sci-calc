# Scientific Calculator

UPCOMING-FEATURES:
* [FUNC] base N
* [EXTRA] more functions and constants (combinatorics, angles, physics constants)
* [FEAT] factorial operator
* [REPL] override existing constant value
* [REPL] REPL commands
* [REPL,FEAT] variables
	* syntax: VAR = EXPR
	* stages:
		1. var can store pure values
		2. [RE-THINK] var can store partial computation (near to lambda functions with named arguments)
	* deletion command: PREFIX DEL VAR
* [REPL,FEAT] lambda functions
	* syntax: NAME(VAR1, VAR2) = sin(VAR1*VAR2)
	* syntax for positional based argument passing: NAME = sin($1) * $2
	* deletion command: PREFIX DEL NAME
* [REPL] magic shortcuts
	* CTRL+C to exit
	* CTRL+K to clear terminal
* [REPL,CLI] better error description and markers and help messages
* [REPL] coloring input expression in while typing

MAYBE:
* conditional values & comparison operators
* vectors
* matrices
* equation solving functions
* differentials
* integrals

