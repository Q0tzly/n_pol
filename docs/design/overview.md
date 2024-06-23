# Nano Pol Lang
## Purpose
言語を実装することに重きをおいたテスト実装の言語

## Language Features
### Summary
- 拡張子は`.np`
- ポーランド記法
- 式指向
- 全てがグローバル
- 引数が存在しないバインド
- 型推論

- 条件分岐が存在しない!(今後実装)
- ループが存在しない!(今後実装)

### Data Types
- int 整数型
- str 文字列型

### Syntax
#### Entory Point
- エントリーポイントは`main`

#### Bind
- `A := B` BをAにbind可能

#### Call
- `A` bindしたBをcall可能

#### Operator(int型のみに適用可能)
- `+ A B` AとBを足した値を返す
- `- A B` AからBを引いた値を返す

#### IO
- `in` 標準入力で得た文字列を返す
- `out A` 文字列を標準出力に出力

#### Type Change
- `int A` 文字列型を数値に変換(数字の文字のみ)して返す
- `str A` 数値型を文字列に変換して返す
