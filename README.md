# 自作プログラミング言語実行環境
Rust 勉強のため自作言語用のインタプリタを作成する

## 自作プログラムの使用 

```
    a = 1
    b = 2
    c = a + b
    d = c
    d
    e = (1 + 1) / 2
    e
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

## 関数の例
関数は以下のように定義することができる
```
    func(a) {
        a + 1
    }
    a = 1
    b = func(a)
    b
```

引数としてaを受取り, a+1 の結果を返す.
関数内の変数は関数内でのみ有効で,外部変数は取り扱わない

## Backus Naur Form
- Equation : 式
- Variable : 変数
- Value : 値
- Arithmetic Equation (AE) : 算術式 
- Arithmetic operand (AO) : 算術演算オペランド (+, -, *, /, %)
```
  Equation ::= Variable | Assignment
  Function ::= "func" '(' Equation ')' '{' Equation '}'
  Comparison ::= ArithmeticEquation ComparisonOperator ArithmeticEquation
  Assignment ::= Variable '=' ArithmeticEquation | Variable '=' Function | Variable '=' Comparison
  ArithmeticEquation ::= Term | Term ArithmeticOperandHead Term
  Term ::= Factor | Factor ArithmeticOperandTail Factor
  Factor ::= Value | Variable | '(' ArithmeticEquation ')'
  ArithmeticOperandHead ::= + | -
  ArithmeticOperandTail ::= * | / | %  
  ArithmeticOperandParen ::= ( | )
  ComparisonOperator ::= < | > | == | <= | >= | !=
  Variable ::= (a-z)+
  Value ::= Integer | Float
  Integer ::= [0-9]+
  Float ::= [0-9]+ '.' [0-9]+
```