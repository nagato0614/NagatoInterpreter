# 自作プログラミング言語実行環境
Rust 勉強のため自作言語用のインタプリタを作成する

## 自作プログラムの使用 

```
    1 + 1
    a = 1
    b = 2
    c = a + b
    c
```

型はすべて32bit int とする.
一行ごとに解析する.
変数だけを記述すると, 標準出力に値を表示する.
四則演算に対応するが, 0除算はエラーで処理を途中終了する
同じ変数が使われたときは値が更新される
スペースで意味ごとに区切る

## Backus Naur Form
- Equation : 式
- Variable : 変数
- Value : 値
- Arithmetic Equation (AE) : 算術式 
- Arithmetic operand (AO) : 算術演算オペランド (+, -, *, /, %)
```
  Equation := Value | Variable | AE
  AE := (Value | Variable) AO (Value | Variable)
  AO := (+|-|*|/|%)
  Variable := (a-z)+
```