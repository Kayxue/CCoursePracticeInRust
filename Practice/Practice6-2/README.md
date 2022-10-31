# 第六次第二題
## 題目
In card games, numbers of cards in the same suits or ranks is often important.  
Please write functions to count numbers of cards in the same suits or ranks as follows.

* `suitCounting(card[], suit[])` to count the number of cards in each suit, i.e. ♣♦♥♠
* `rankCounting(card[], rank[])` to count the number of cards in each rank, i.e. for A, 2, 3, ... , J, Q, K, and A again 
 
You need the following arrays in your main() function.  

* `card[52]` to save the IDs of cards
* `suitCount[4]` to save the number of cards in each suit
* `rankCount[14]` to save the number of cards in each rank

## 範例
```
Input cards: 4 11 12 28 29
♣6 ♣K ♣A ♥4 ♥5 have
♣(3) ♦(0) ♥(2) ♠(0)
2(0) 3(0) 4(1) 5(1) 6(1) 7(0) 8(0) 9(0) 10(0) J(0) Q(0) K(1) A(1)

Input cards: 3 6 14 19 27
♣5 ♣8 ♦3 ♦8 ♥3 have
♣(2) ♦(2) ♥(1) ♠(0)
2(0) 3(2) 4(0) 5(1) 6(0) 7(0) 8(2) 9(0) 10(0) J(0) Q(0) K(0) A(0)

Input cards: 9 22 35 41 48
♣J ♦J ♥J ♠4 ♠J have
♣(1) ♦(1) ♥(1) ♠(2)
2(0) 3(0) 4(1) 5(0) 6(0) 7(0) 8(0) 9(0) 10(0) J(4) Q(0) K(0) A(0)

Input cards: 32 33 34 35 36
♥8 ♥9 ♥10 ♥J ♥Q have
♣(0) ♦(0) ♥(5) ♠(0)
2(0) 3(0) 4(0) 5(0) 6(0) 7(0) 8(1) 9(1) 10(1) J(1) Q(1) K(0) A(0)

Input cards: 11 12 13 14 15
♣K ♣A ♦2 ♦3 ♦4 have
♣(2) ♦(3) ♥(0) ♠(0)
2(1) 3(1) 4(1) 5(0) 6(0) 7(0) 8(0) 9(0) 10(0) J(0) Q(0) K(1) A(1)

Input cards: 6 8 15 34 48
♣8 ♣10 ♦4 ♥10 ♠J have
♣(2) ♦(1) ♥(1) ♠(1)
2(0) 3(0) 4(1) 5(0) 6(0) 7(0) 8(1) 9(0) 10(2) J(1) Q(0) K(0) A(0)

Input cards: 2 12 25 31 51
♣4 ♣A ♦A ♥7 ♠A have
♣(2) ♦(1) ♥(1) ♠(1)
2(0) 3(0) 4(1) 5(0) 6(0) 7(1) 8(0) 9(0) 10(0) J(0) Q(0) K(0) A(3)

Input cards: 15 16 19 21 23
♦4 ♦5 ♦8 ♦10 ♦Q have
♣(0) ♦(5) ♥(0) ♠(0)
2(0) 3(0) 4(1) 5(1) 6(0) 7(0) 8(1) 9(0) 10(1) J(0) Q(1) K(0) A(0)

Input cards: 2 16 27 31 43
♣4 ♦5 ♥3 ♥7 ♠6 have
♣(1) ♦(1) ♥(2) ♠(1)
2(0) 3(1) 4(1) 5(1) 6(1) 7(1) 8(0) 9(0) 10(0) J(0) Q(0) K(0) A(0)

Input cards: 13 21 26 39 47
♦2 ♦10 ♥2 ♠2 ♠10 have
♣(0) ♦(2) ♥(1) ♠(2)
2(3) 3(0) 4(0) 5(0) 6(0) 7(0) 8(0) 9(0) 10(2) J(0) Q(0) K(0) A(0)
```