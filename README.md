# syntactic_monoid
compute minimum dfa,syntactic monoid, and starfree expression

### How to use 
```
$ cargo run --release "<regular expression>"
```

### Example
```
$ cargo run --release "(a|ba)*"
regular expression:
(a|ba)*
minimized dfa:
digraph DFA {
  rankdir="LR"
  0 [ shape=doublecircle ];
  1 [ shape=circle ];
  start [ shape=plaintext ];
 0 -> 0 [ label = "a" ];
 0 -> 1 [ label = "b" ];
 start -> 0
 1 -> 0 [ label = "a" ];
}
dfa size:2
mat(1) = 
 0 0
 0 0

mat(5) = 
 1 0
 0 0

mat(0) = 
 1 0
 0 1

mat(4) = 
 0 1
 0 1

mat(2) = 
 1 0
 1 0

mat(3) = 
 0 1
 0 0

starfree expression:
@||!(!(a!@)|!(!@a)|!@(![ab]|bb)!@)|!(!(b!@)|!(!@a)|!@(![ab]|bb)!@)
```
