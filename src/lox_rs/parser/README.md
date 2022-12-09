# GRAMMAR

## Expressions

| name       | value                                                                   |
| ---------- | ----------------------------------------------------------------------- |
| expression | equality                                                                |
| assignment | IDENTIFIER `=` assignment \| equality \| logicOr                        |
| logicOr    | logicAnd ( `or` logicAnd )\*                                            |
| logicAnd   | equality ( `and` equality )\*                                           |
| equality   | comparison ( ( `!=` \| `==` ) comparison )\*                            |
| comparison | term ( ( `>` \| `>=` \| `<` \| `<=` ) term )\*                          |
| term       | factor ( ( `-` \| `+` ) factor )\*                                      |
| factor     | unary ( ( `/` \| `*` ) unary )\*                                        |
| unary      | ( `!` \| `-` \| `+` ) unary \| call                                     |
| call       | primary ( `(` ( arguments )? `)` )\*                                    |
| arguments  | expression ( `,` expression )\*                                         |
| primary    | NUMBER \| STRING \| BOOLEAN \| NULL \| `(` expression `)` \| IDENTIFIER |

## Statements

| name        | value                                                                                     |
| ----------- | ----------------------------------------------------------------------------------------- |
| program     | ( statement )\* EOF                                                                       |
| declaration | fnDecl \| varDecl \| statement                                                            |
| fnDecl      | `fn` function                                                                             |
| function    | IDENTIFIER `(` parameters ? `)` block                                                     |
| parameters  | INDTIFIER ( `,` IDENTIFIER )\*                                                            |
| statement   | exprStmt \| printStmt \| ifStmt \| block \| whileStmt \| forStmt                          |
| whileStmt   | `while` `(` expression `)` statment                                                       |
| forStmt     | `for` `(` ( varDecl \| exprStmt \| `;` ) ( expression )? `;` ( expression )? `)` statment |
| block       | `{` ( declaration )\* `}`                                                                 |
| exprStmt    | expression `;`                                                                            |
| printStmt   | `print` expression `;`                                                                    |
| varDecl     | `var` IDENTIFIER ( `=` expression )? `;`                                                  |
| ifStmt      | `if` `(` expression `)` statement ( `else` statement )?                                   |
