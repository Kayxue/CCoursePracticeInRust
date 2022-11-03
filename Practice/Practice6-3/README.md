# 第六次第三題
## 題目
Write a function `int handRanking(hand[])` which returns the ranking of a poker hand stored in `hand[]`.  

The definitions of rankings are listed in the following table, the higher the stronger.  

|Ranking 順序|Name 牌型|Definition 定義|Possible Rule 可能判斷規則|
|-----|-----|-----|-----|
|8|Straight Flush 同花順|A straight flush is a poker hand which contains five cards in sequence, all of the same suit, such as ♣8 ♣9 ♣10 ♣J ♣Q.|The hand is flush and straight.|
|7|Four of a Kind 四條|Four of a kind is a poker hand such as ♣9 ♦9 ♥9 ♠9 ♣J which contains four cards of one rank, and an unmatched card of another rank.|Count the numbers of cards of each rank. Check if there are 4 cards in the same rank.|
|6|Full House 葫蘆|A full house is a hand such as ♣3 ♦3 ♠3 ♦6 ♥6
which contains three matching cards of one rank, and two matching cards of another rank.|Count the numbers of cards of each rank. Check if there are 3 cards in one rank and 2 cards in another.|
|5|Flush 同花|A flush is a poker hand such as ♦4 ♦6 ♦7 ♦10 ♦Q which contains five cards of the same suit, not in rank sequence.|Count the numbers of cards of each suit. Check if there are 5 cards with the same suit.|
|4|Straight 順|A straight is a poker hand such as ♠8 ♣9 ♦10 ♠J ♦Q which contains five cards of sequential rank but in more than one suit.|Count the numbers of cards of each rank. Check if there are 5 cards in sequential ranks.|
|3|Three of a Kind 三條|Three of a kind is a poker hand such as ♠2 ♣6 ♦6 ♠6 ♠K which contains three cards of the same rank, plus two unmatched cards.|Count the numbers of cards of each rank. Check if there are 3 cards in the same rank.|
|2|Two Pairs 兩對|A poker hand such as ♣4 ♦4 ♠9 ♦J ♠J which contains two cards of the same rank, plus two cards of another rank (that match each other but not the first pair), plus one unmatched card, is called two pair.|Count the numbers of cards of each rank. Check if there are 2 cards in one rank and 2 cards in another.|
|1|One Pair一對|One pair is a poker hand such as ♣4 ♦4 ♠5 ♦10 ♠K which contains two cards of the same rank, plus three other unmatched cards.|Count the numbers of cards of each rank. Check if there are 2 cards in the same rank.|
|0|High Card 無|A high-card or no-pair hand is a poker hand such as ♣3 ♦7 ♠8 ♦J ♠K which does not match any ranking.|Otherwise|

In your `main()`,
randomly shuffle the cards and print the first 5 cards and decides its poker hand ranking.  
Do it for 10 times.  
And then allow the user to input the 5 cards and decides its poker hand ranking.
## 範例
```
♣6 ♣K ♣A ♥4 ♥5 無
♣5 ♣8 ♦3 ♦8 ♥3 兩對
♣J ♦J ♥J ♠4 ♠J 四條
♥8 ♥9 ♥10 ♥J ♥Q 同花順
♣K ♣A ♦2 ♦3 ♦4 無
♣8 ♣10 ♦5 ♥10 ♠J 一對
♣4 ♣A ♦A ♥7 ♠A 三條
♦4 ♦5 ♦8 ♦10 ♦Q 同花
♣4 ♦5 ♥3 ♥7 ♠6 順
♦2 ♦10 ♥2 ♠2 ♠10 葫蘆

請輸入 5 張牌：0 1 2 3 12
♣2 ♣3 ♣4 ♣5 ♣A 同花順
```
