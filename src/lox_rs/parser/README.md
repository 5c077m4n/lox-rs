# GRAMMAR

## Expressions

| name       | value                                                                   |
| ---------- | ----------------------------------------------------------------------- |
| expression | equality                                                                |
| assignment | IDENTIFIER `=` assignment \| equality                                   |
| equality   | comparison ( ( `!=` \| `==` ) comparison )\*                            |
| comparison | term ( ( `>` \| `>=` \| `<` \| `<=` ) term )\*                          |
| term       | factor ( ( `-` \| `+` ) factor )\*                                      |
| factor     | unary ( ( `/` \| `*` ) unary )\*                                        |
| unary      | ( `!` \| `-` \| `+` ) unary \| primary                                  |
| primary    | NUMBER \| STRING \| BOOLEAN \| NULL \| `(` expression `)` \| IDENTIFIER |

## Statements

| name        | value                                                   |
| ----------- | ------------------------------------------------------- |
| program     | ( statement )\* EOF                                     |
| declaration | varDecl \| statement                                    |
| statement   | exprStmt \| printStmt \| ifStmt \| block                |
| block       | `{` ( declaration )\* `}`                               |
| exprStmt    | expression `;`                                          |
| printStmt   | `print` expression `;`                                  |
| varDecl     | `var` IDENTIFIER ( `=` expression )? `;`                |
| ifStmt      | `if` `(` expression `)` statement ( `else` statement )? |