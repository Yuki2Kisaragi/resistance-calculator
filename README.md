# reg-calc

## 概要
抵抗値の計算をCLI上で簡単に行えるツールです。直列抵抗と並列抵抗の計算に対応しており、SI接頭辞を使用した入力が可能です。

## 機能
* 直列抵抗/並列抵抗の計算
* SI接頭辞対応（p, n, u, m, k, M, G, T）
* 日本語/英語の言語切り替え

## インストール方法

### ソースからのビルド
```sh
git clone https://github.com/Yuki2Kisaragi/resistance-calculator.git
cd resistance-calculator
cargo build --release
```

ビルドが完了すると、`target/release/reg-calc`が生成されます。

## 使い方

### 基本的な使い方

1. 直列抵抗の計算:
```sh
$ reg-calc s
モード: 直列計算
抵抗1> 1k      # 1kΩ
抵抗2> 500     # 500Ω
抵抗3>         # Enter で計算へ
計算を実行しますか？(y/n) > y
計算を実行中...
直列合成抵抗: 1.5kΩ
```

2. 並列抵抗の計算:
```sh
$ reg-calc p
モード: 並列計算
抵抗1> 1k      # 1kΩ
抵抗2> 1k      # 1kΩ
抵抗3>         # Enter で計算へ
計算を実行しますか？(y/n) > y
計算を実行中...
並列合成抵抗: 500Ω
```

### SI接頭辞の使用方法
以下のSI接頭辞に対応しています：

| 接頭辞 | 記号 | 倍率 |
|--------|------|------|
| ピコ   | p    | 1e-12|
| ナノ   | n    | 1e-9 |
| マイクロ| u    | 1e-6 |
| ミリ   | m    | 1e-3 |
| (なし) | -    | 1    |
| キロ   | k    | 1e3  |
| メガ   | M    | 1e6  |
| ギガ   | G    | 1e9  |
| テラ   | T    | 1e12 |

使用例：
- `1k` → 1kΩ (1000Ω)
- `1M` → 1MΩ (1000000Ω)
- `100m` → 100mΩ (0.1Ω)

### 言語設定
英語表示に切り替える場合は、`--lang en`オプションを使用します：

```sh
$ reg-calc s --lang en
Mode: Series Calculation
R1 > 100
R2 > 200
R3 > 
Do you want to run the calculations?(y/n) > y
Execution calculating...
Combined Series resistance: 300.0Ω
```

### コマンドラインオプション
```sh
Usage: reg-calc [OPTIONS] [MODE]

Options:
  --lang <LANG>    言語設定 (ja/en)
  -h, --help       ヘルプメッセージを表示
  -V, --version    バージョン情報を表示

Arguments:
  MODE             計算モード (s: 直列, p: 並列)
```

## 注意事項
- SI接頭辞は1つの値に対して1つだけ使用できます（例：`1kk`は無効）
- 負の値も入力可能です（例：`-1k`）
- 小数点を含む値も入力可能です（例：`1.5k`）
