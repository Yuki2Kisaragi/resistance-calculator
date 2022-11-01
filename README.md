# [WIP] Rusty CLI Application Tool : Resistance-Calculator

## 概要
CLI上で抵抗値の計算ができます。

## 機能
* 直列抵抗/並列抵抗の計算
* SI接頭辞の入力対応 [WIP]

## Install
いずれCargoでインストールできるようにする予定です。

### Souce Build

```sh
git clone https://github.com/Yuki2Kisaragi/resistance-calculator.git
cd reg-calc
cargo build --release
cp ./target/release/reg-calc .
alias reg-calc=./reg-calc
reg-calc
```


## Useage

```sh
$ reg-calc
Which calculation mode do you want to run, in Parallel(p) or Series(s)? > p
Mode : Parallel Calculation
R1>100

R2>100

R3>
Combind Parallel resistance[ohm]: 50
$
```

```sh
$ reg-calc
Which calculation mode do you want to run, in Parallel(p) or Series(s)? > s
Mode : Serial Calculation
R1>100

R2>100

R3>
Combind Serial resistance[ohm]: 200
```

```sh
$ reg-calc p
Mode : Parallel Calculation
R1>100

R2>100

R3>
Combind Parallel resistance[ohm]: 50
```

```sh
$ reg-calc s
Mode : Parallel Calculation
R1>100

R2>100

R3>
Combind Parallel resistance[ohm]: 200
```
