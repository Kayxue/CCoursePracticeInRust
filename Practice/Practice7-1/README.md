# 替七次第一題
## 題目
Do text encryption by the following rules:
|Case|To Do|
|-----|-----|
|Upper-case English letters  'A', 'B', ..., 'Z'|Change the letter into its second successor letter, I.e. 'A'→'C', 'B'→'D', etc.  Note that 'Y'→'A', and 'Z'→'B'.|
|Lower-case English letters  'a', 'b', ..., 'z'|Change the letter into its second successor letter, I.e. 'a'→'c', 'b'→'d', etc.  Note that 'y'→'a', and 'z'→'b'.|
|Digits ('0', '1', ... '9')|Change the digit into the digit preceding it. E.g. '9'→'8', '8'→'7', etc. Note that '0'→'9'.|
|Otherwise|Leave it as the same character.|


## 範例
1.
```
Input a line  : A funny guy. @@
Encrypted line: C hwppa iwa. @@
```
2.
```
Input a line  : It is 10:15 now.
Encrypted line: Kv ku 09:04 pqy.
```
3.
```
Input a line  : ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz
Encrypted line: CDEFGHIJKLMNOPQRSTUVWXYZAB cdefghijklmnopqrstuvwxyzab
```
4.
```
Input a line  : !"#$%&'()*+,-./ 0123456789 :;<=>?@ [\]^_` {|}~
Encrypted line: !"#$%&'()*+,-./ 9012345678 :;<=>?@ [\]^_` {|}~
```
