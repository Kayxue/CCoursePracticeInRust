# 第六次第一題
## 題目
**(1-1)** Please write a function to print a poker card, whose prototype is:

`void printCard(int id);`
Let the numbers 0 ~ 51 represent 52 poker cards as follows:
|編號 ID|花色 Suit|ASCII Code|數字 Rank|
|-----|-----|-----|-----|
|0 ~ 12|梅花 clover ♣|x05|2 3 4 5 6 7 8 9 10 J Q K A|
|13 ~ 25|方塊 diamond ♦|x04|2 3 4 5 6 7 8 9 10 J Q K A|
|26 ~ 38|紅心 heart ♥|x03|2 3 4 5 6 7 8 9 10 J Q K A|
|39 ~ 51|黑桃 spade ♠|x06|2 3 4 5 6 7 8 9 10 J Q K A|

You need the following array to print suit symbols:

`char* suitSymbol[4] = {"♣", "♦", "♥", "♠"};`
You can call, for example, `printf("%s", suitSymbol[2])` to print out a heart ♥.
**NOTE** that these strings are in **UTF-8**, so you need to change the charset of the Command Prompt window (命令提示字元) by
`system("chcp 65001");`
To print a card's rank, you can declare another array

`char* highRank[4] = {"J", "Q", "K", "A"};`
to treat J ~ A as special cases and 2 ~ 10 as ordinary cases.
Use the following main() to test your function.
```cpp
int main() {
    int id = 0;
    while (1) {
        printf("Input a card ID: ");
        scanf("%d", &id); if (id == -1) break;
        printCard(id);
        putchar('\n');
    }
    return 0;
}
```
**(1-2)** Write a function to generate a random permutation (亂排).

`void shuffling(int a[], int N);`
which will create a random permutation of the numbers from 0 to N − 1 in the array `a[]`.
For example, calling shuffling(a, 7) may get a random permutation like

`a[] = {5, 1, 3, 0, 4, 6, 2}`
## 範例
```
Player 1: ♦3 ♣4 ♣7 ♥10 ♠9 ♠6 ♥Q ♠A ♣9 ♣2 ♦9 ♦4 ♦Q 
Player 2: ♦K ♥9 ♥A ♦5 ♦10 ♥5 ♥6 ♣10 ♠7 ♥8 ♠3 ♣A ♠K 
Player 3: ♠Q ♦J ♠10 ♥3 ♠8 ♠5 ♦A ♥K ♦2 ♦6 ♠4 ♣3 ♦7 
Player 4: ♦8 ♥4 ♣8 ♥J ♠J ♥2 ♥7 ♣K ♣J ♣Q ♣6 ♣5 ♠2 

Input a card ID: 35
♥J
Input a card ID: 18
♦7
Input a card ID: 2
♣4
Input a card ID: 49
♠Q
Input a card ID: -1
```
