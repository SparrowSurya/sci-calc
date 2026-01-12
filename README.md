# Scientific Calculator

UPCOMING-FEATURES:
* [CLI] allowed/disabled functions or constants
* base N
* [CLI] external constants values
* [CLI/REPL] override existing constant value
* factorial operator
* case based constants (NOTE: Required for physics constants)
* more functions and constants (combinatorics, angles, physics constants)
* [REPL] REPL commands
* [UNCLEAR: In REPL do i want up key to access last computed value or last expression] value history
* [REPL] variables
	* syntax: VAR = EXPR
	* stages:
		1. var can store pure values
		2. [RE-THINK] var can store partial computation (near to lambda functions with named arguments)
	* deletion command: PREFIX DEL VAR
* [REPL] lambda functions
	* syntax: NAME(VAR1, VAR2) = sin(VAR1*VAR2)
	* syntax for positional based argument passing: NAME = sin($1) * $2
	* deletion command: PREFIX DEL NAME
* [REPL] magic shortcuts
	* CTRL+C to exit
	* CTRL+K to clear terminal
* [REPL] better error description and markers
* [REPL] coloring input expression in while typing

MAYBE:
* conditional values & comparison operators
* vectors
* matrices
* equation solving functions
* differentials
* integrals

