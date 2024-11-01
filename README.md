# 自作プログラミング言語実行環境
Rust 勉強のため自作言語用のインタプリタを作成する

## 自作プログラムの使用 

```
    a = 1;
    b = 2;
    c = a + b;
    d = c;
    d;
    e = (1 + 1) / 2;
    e;
    return e;
```

 - 型はすべて32bit int とする.
 - 一行ごとに解析する.
 - 式は基本的にassign式となり, 例外として変数だけの場合は標準出力に値を表示する.
 - 変数だけを記述すると, 標準出力に値を表示する. 
 - 四則演算に対応するが, 0除算はエラーで処理を途中終了する 
 - 同じ変数が使われたときは値が更新される
 - 変数を再定義した場合は上書きされる
 - 比較演算は真の場合は1, 偽の場合は0を返す
 - 型は浮動小数点と整数のみ対応する
 - '#' 以降はコメントとして無視される
 - 固定長の引数を取る関数を定義することができる
 - 式の最後には必ずセミコロン ';' をつけること
 - return を実行すると, その時点で処理を終了し, return の後ろの値を返す
 - return は関数内では最後に一つ書く必要がある. それ以外の場所に書いた場合はそこで処理を終了する.

## 関数の例
関数は以下のように定義することができる
```
    func f(a) {
        b = a + 1;
        return b;
    }
    a = 1;
    b = f(a);
    b;
```

引数としてaを受取り, a+1 の結果を返す.
関数内には必ずreturn文を記述すること.
関数内の変数は関数内でのみ有効で,外部変数は取り扱わない

波括弧は省略できず, 以下のような処理はできない
```:エラー
    func add(a) return a + 1
```

関数の引数は複数指定することができる
```
    func add(a, b) {
     c = a + b
     return c;
    }
    a = 1
    b = 2
    c = add(a, b)
    c
```

一つの引数内に関数呼び出しは不可能
```エラー
    func(a) { return a + 1; }
    a = 1;
    b = func(func(a));
    b;
```

## Backus Naur Form
- Equation : 式
- Variable : 変数
- Value : 値
- Arithmetic Equation (AE) : 算術式 
- Arithmetic operand (AO) : 算術演算オペランド (+, -, *, /, %)
```
  Statement ::= Equation | Function
  Equation ::= Identifier ';' | Assignment ';' | ReturnStatement ';'
  Assignment ::= Identifier '=' Expression
  Expression ::= ArithmeticEquation | Comparison | FunctionCall
  Comparison ::= ArithmeticEquation ComparisonOperator ArithmeticEquation
 
  Function ::= "func" Identifier '(' Arguments ')' Block
  Block ::= '{' (Statement ';')* ReturnStatement ';'}'
  Statement ::= Identifier ';' | Assignment ';'
  FunctionCall ::= Identifier '(' CallArgument ')'
  Arguments ::= [Variable] (',' [Variable])*
　CallArguments ::= [CallArgument] (',' [CallArgument])*
  ReturnStatement ::= "return" Expression
  
  ArithmeticEquation ::= Term | Term ArithmeticOperandHead Term
  Term ::= Factor | Factor ArithmeticOperandTail Factor
  Factor ::= Value | Identifier | '(' ArithmeticEquation ')' | FunctionCall
  ArithmeticOperandHead ::= + | -
  ArithmeticOperandTail ::= * | / | %  
  ArithmeticOperandParen ::= ( | )
  ComparisonOperator ::= < | > | == | <= | >= | !=
  BlockParen ::= { | }
  Identifier ::= (a-z)+
  Value ::= Integer | Float
  Integer ::= [0-9]+
  Float ::= [0-9]+ '.' [0-9]+
```