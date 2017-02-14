# Notes and ideas

## Change Token to TokenKind and make Token a struct?

```rust
struct Token {
    kind: TokenKind,
    line_number: i32,
}
```

## How to scan a real?
Should `real` literals always require 4 decimal values?

3 + 4 * 2 * 4 - 2

## How to parse expressions and generate an ast
the expression function is called.
didn't find a prefix op
found a 3 meaning it's a term
does a * / follow?
no
exit term
does a + - follow?
yes
parse next term.
is the next a term?
found 4, so far all good
does a */ follow?
yes
is it followed by a term?
yes, found 2
is it followed by a */?
yes
is it followed by a factor? yes
4
is it followed by */? no
return
we are back to the 3
we now have 3 + (4 * 2 * 4)
the expression is not over yet!
does a +- follow? yes
found -
is it followed by a term? yes
is it a factor? yes
2
is it followed by a */? no
return
is it followed by a +-?
no, found ; or something else
expression is done.
return AST
3 + (4 * 2 * 4) - 2

((float) 0.bbb) * 2^16 -> runde av til int