# GRAMMAR

| name        | value                                                                   |
| ----------- | ----------------------------------------------------------------------- |
| expression  | equality                                                                |
| equality    | comparison ( ( `!=` \| `==` ) comparison )\*                            |
| comparison  | term ( ( `>` \| `>=` \| `<` \| `<=` ) term )\*                          |
| term        | factor ( ( `-` \| `+` ) factor )\*                                      |
| factor      | unary ( ( `/` \| `*` ) unary )\*                                        |
| unary       | ( `!` \| `-` \| `+` ) unary \| primary                                  |
| primary     | NUMBER \| STRING \| BOOLEAN \| NULL \| `(` expression `)` \| IDENTIFIER |
| ----------  | ---------------------------------------------------------               |
| program     | ( statement )\* EOF                                                     |
| declaration | varDecl \| statement                                                    |
| statement   | exprStmt \| printStmt                                                   |
| exprStmt    | expression `;`                                                          |
| printStmt   | `print` expression `;`                                                  |
| varDecl     | `var` IDENTIFIER ( `=` expression )? `;`                                |
