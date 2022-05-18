# [WIP] Rusty CLI Application Tool : Resistance-Calculator

## 概要
CLI上で抵抗値の計算ができます。

## 機能
* 直列抵抗/並列抵抗の計算
* SI接頭辞の入力対応

## Install

### Cargo Install
```sh
$ cargo install reg-calc
```

### Souce Build
```sh
$ git clone hogehoge/reg-calc
$ cd reg-calc
$ cargo run
```


## Useage

```sh
$ reg-calc

Which calculation mode do you want to run, in Parallel(p) or Series(s)? > s
Enter Series calculation Mode...
R1 > 100
R2 > 200
R3 > 300
R4 > 
Do you want to run the calculations?(y/n) > y 
Execution calculating...
Combind Resitance(R1,R2,R3) = 600 (Ohm)
$
```

```sh
$ reg-calc -S
Enter Series calculation Mode...
R1 > 100k
R2 > 20k
R3 > 3000
R4 > 
Do you want to run the calculations?(y/n) > n 
R4 > 1k
R5 > 
Do you want to run the calculations?(y/n) > y 
Execution calculating...
Combind Resitance(R1,R2,R3,R4) = 124k (Ohm)
$
```

```sh
$ reg-calc -P
Enter Parallel calculation Mode...
R1 > 100M
R2 > 100M
R3 > 
Do you want to run the calculations?(y/n) > y 
Execution calculating...
Combind Resitance(R1,R2) = 50M (Ohm)
$
```
