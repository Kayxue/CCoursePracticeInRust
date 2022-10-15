# 第五次第二題
## 題目
The hidden answer is a distinct-4-digit number. (0 can be its leading digit.)
The program repeatedly asks the user to guess a distinct-4-digit number and gives a hint.
m-A means m digits in the guessing number are correct and in right positions.
n-B means n digits in the guessing number are correct but in wrong positions.
## 範例
```
請猜一個 4 位數不重覆的數字：7522
不能有重覆數字。
請猜一個 4 位數不重覆的數字：75263
只能猜 4 位數的數字。
請猜一個 4 位數不重覆的數字：-7523
只能猜正整數。
請猜一個 4 位數不重覆的數字：7523
2A1B
請猜一個 4 位數不重覆的數字：3725
1A2B
請猜一個 4 位數不重覆的數字：3625
1A1B
請猜一個 4 位數不重覆的數字：9528
3A0B
請猜一個 4 位數不重覆的數字：1468
0A0B
請猜一個 4 位數不重覆的數字：7901
0A2B
請猜一個 4 位數不重覆的數字：7952
0A4B
請猜一個 4 位數不重覆的數字：9527
4A0B
猜對了！你猜了 8 次。
```
## 備註
### [Basic]
Finish the definition of separateDigit(), checkA(), and checkB().

Test these functions as follows.
```
#define HiddenDigits 4
int main() {
    int hidden[10], guess[10];
    // setHiddenAnswer(hidden);
    separateDigit(9527, hidden);
    // guessing(guess);
    separateDigit(7523, guess);
    printf("%d A %d B\n", checkA(...), checkB(...));
    return 0;
}
```

### [Advanced]
Finish the complete system.
The complete system (1) creates hidden answers randomly; (2) checks invalid guessing from the user; (3) loops until the user guesses the hidden answer.