# 自作プログラミング言語実行環境
Rust 勉強のため自作言語用のインタプリタを作成する

## 自作プログラムの使用 

```
    a = 1
    b = 2
    c = a + b
    d = c
    d
```

 - 型はすべて32bit int とする.
 - 一行ごとに解析する.
 - 変数だけを記述すると, 標準出力に値を表示する. 
 - 四則演算に対応するが, 0除算はエラーで処理を途中終了する 
 - 同じ変数が使われたときは値が更新される
 - 変数を再定義した場合は上書きされる

## Backus Naur Form
- Equation : 式
- Variable : 変数
- Value : 値
- Arithmetic Equation (AE) : 算術式 
- Arithmetic operand (AO) : 算術演算オペランド (+, -, *, /, %)
```
  Equation ::= Variable | ArithmeticEquation
  ArithmeticEquation ::= Term | {ArithmeticOperandHead Term}
  Term ::= Factor | {ArithmeticOperandTail Factor}
  Factor ::= Value | Variable
  ArithmeticOperandHead ::= (+|-)
  ArithmeticOperandTail ::= (*|/|%)  
  Variable ::= (a-z)+
  Value ::= [0-9]+
```