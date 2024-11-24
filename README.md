# 自作プログラミング言語実行環境

Rust 勉強のため自作言語用のインタプリタを作成する.
c言語をベースとして一部仕様を切り取っている.

## 仕様

基本的にはC言語の仕様をベースにしているが, 以下の点が異なる.

- 文字列は取り扱わない
- ポインタは取り扱わない
- 構造体は取り扱わない
- マクロは取り扱わない
- 配列の取り扱いはしない
- プリプロセッサは取り扱わない
- ヘッダファイルは取り扱わない
- switch_case文は取り扱わない
- assignment は = のみで, += などは取り扱わない
- 3項演算子は取り扱わない
- ビット演算は取り扱わない
- ループは while 文のみ
## BNF

```

// トークンの定義
{
    tokens=[
         space='regexp:\s+'
         identifier='regexp:[a_zA_Z][a_zA_Z0_9_]*'

         integer_constant='regexp:\d+'
         floating_constant='regexp:[+_]?([0_9]*[.])?[0_9]+f'
    ]
}

translation_unit ::= {external_declaration}*

external_declaration ::= function_definition
                         | declaration

// 関数周りの定義
function_definition ::= type_specifier direct_declarator {declaration}* compound_statement

type_specifier ::= void
                   | int
                   | float

// ブロック内の処理
compound_statement ::= '{' {declaration}* {statement}* '}'
statement ::= expression_statement
              | compound_statement
              | selection_statement
              | iteration_statement

// 変数の初期化
expression_statement ::= {expression}? ';'
expression ::= assignment_expression    
               | expression ',' assignment_expression   // 関数呼び出しやif文の中で複数の式を書く場合に使用
assignment_expression ::= logical_or_expression
                          | postfix_expression assignment_operator assignment_expression

// if文
selection_statement ::= if '(' expression ')' compound_statement
                        | if '(' expression ')' compound_statement else compound_statement
                        
// while文
iteration_statement ::= while '(' expression ')' statement

                          
// 演算子周りの優先順位
// OR 演算子
logical_or_expression ::= logical_and_expression
                          | logical_or_expression '||' logical_and_expression
// AND 演算子
logical_and_expression ::= equality_expression
                           | logical_and_expression '&&' equality_expression
equality_expression ::= relational_expression
                        | equality_expression '==' relational_expression
                        | equality_expression '!=' relational_expression
relational_expression ::= additive_expression
                          | relational_expression '<' additive_expression
                          | relational_expression '>' additive_expression
                          | relational_expression '<=' additive_expression
                          | relational_expression '>=' additive_expression

additive_expression ::= multiplicative_expression
                        | additive_expression '+' multiplicative_expression
                        | additive_expression '_' multiplicative_expression

multiplicative_expression ::= postfix_expression
                              | multiplicative_expression '*' postfix_expression
                              | multiplicative_expression '/' postfix_expression
                              | multiplicative_expression '%' postfix_expression
postfix_expression ::= primary_expression                                       // 単項演算子
                       | postfix_expression '[' expression ']'                  // 配列アクセス
                       | postfix_expression '(' {assignment_expression}* ')'    // 関数呼び出し

primary_expression ::= identifier
                       | constant
                       | '(' expression ')'

assignment_operator ::= '='

constant ::= integer_constant
             | floating_constant

// 宣言周りの定義
declaration ::=  type_specifier {init_declarator}*
init_declarator ::= direct_declarator                      // 宣言だけ
                    | direct_declarator '=' initializer    // 初期化付きの宣言
direct_declarator ::= identifier                           // 変数宣言 
                      | identifier '(' parameter_list ')'  // 関数宣言 : 定義時に使用する
                      | identifier '(' {identifier}* ')'   // 関数宣言 : 呼び出し時に使用する
                      
parameter_list ::= parameter_declaration                        // 1つのパラメータ
                   | parameter_list ',' parameter_declaration   // 複数のパラメータ

parameter_declaration ::= {type_specifier}+ direct_declarator
                          | {type_specifierr}+               





```

### 参考 : C言語のBNF

多分 c89 の仕様に基づいていると思われる.

[C言語のBNF](https://gist.githubusercontent.com/arslancharyev31/c48d18d8f917ffe217a0e23eb3535957/raw/45c6f49d927adf288aa3ac9fb0b88d2d569ed691/C_v1.bnf)

出現する要素とそれぞれの日本語訳

| 要素                         | 日本語訳         |
|____________________________|______________|
| translation_unit           | 翻訳単位         |
| external_declaration       | 外部宣言         |
| function_definition        | 関数定義         |
| declaration                | 宣言           |
| declaration_specifier      | 宣言指定子        |
| storage_class_specifier    | 記憶クラス指定子     |
| type_specifier             | 型指定子         |
| type_qualifier             | 型修飾子         |
| struct_or_union_specifier  | 構造体または共用体指定子 |
| struct_or_union            | 構造体または共用体    |
| struct_declaration         | 構造体宣言        |
| specifier_qualifier        | 指定子または修飾子    |
| struct_declarator_list     | 構造体宣言子リスト    |
| struct_declarator          | 構造体宣言子       |
| declarator                 | 宣言子          |
| pointer                    | ポインタ         |
| type_qualifier             | 型修飾子         |
| direct_declarator          | 直接宣言子        |
| constant_expression        | 定数式          |
| conditional_expression     | 条件式          |
| logical_or_expression      | 論理和式         |
| logical_and_expression     | 論理積式         |
| inclusive_or_expression    | 包含的論理和式      |
| exclusive_or_expression    | 排他的論理和式      |
| and_expression             | AND式         |
| equality_expression        | 等価式          |
| relational_expression      | 関係式          |
| shift_expression           | シフト式         |
| additive_expression        | 加減式          |
| multiplicative_expression  | 乗除式          |
| cast_expression            | キャスト式        |
| unary_expression           | 単項式          |
| unary_operator             | 単項演算子        |
| postfix_expression         | 後置式          |
| primary_expression         | 主式           |
| constant                   | 定数           |
| expression                 | 式            |
| assignment_expression      | 代入式          |
| assignment_operator        | 代入演算子        |
| type_name                  | 型名           |
| parameter_type_list        | パラメータ型リスト    |
| parameter_list             | パラメータリスト     |
| parameter_declaration      | パラメータ宣言      |
| abstract_declarator        | 抽象宣言子        |
| direct_abstract_declarator | 直接抽象宣言子      |
| enum_specifier             | 列挙型指定子       |
| enumerator_list            | 列挙子リスト       |
| enumerator                 | 列挙子          |
| typedef_name               | 型定義名         |
| init_declarator            | 初期化宣言子       |
| initializer                | 初期化子         |
| initializer_list           | 初期化子リスト      |
| compound_statement         | 複合文          |
| statement                  | 文            |
| labeled_statement          | ラベル付き文       |
| expression_statement       | 式文           |
| selection_statement        | 選択文          |
| iteration_statement        | 繰り返し文        |
| jump_statement             | ジャンプ文        |
