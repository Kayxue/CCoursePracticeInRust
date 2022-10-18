# 第二次第二題
## 題目
Ask the user to input the number of the first prize and the number on his/her uniform-invoice (統一發票).
Tell the user that he/she does or does not win any prize according to the the following rules:
|獎項|條件|範例|
|-----|-----|-----|
|頭獎|Same as the first prize winning number|**81674526**|
|二獎|Same last 7 digits as the first prize winning number|5**1674526**|
|三獎|Same last 6 digits as the first prize winning number|53**674526**|
|四獎|Same last 5 digits as the first prize winning number|531**74526**|
|五獎|Same last 4 digits as the first prize winning number|5319**4526**|
|六獎|Same last 3 digits as the first prize winning number|53192**526**|
## 範例
```
請輸入頭獎號碼：91274553
請輸入你的發票號碼：00374553
恭喜！你中了四獎！
```
```
請輸入頭獎號碼：91274553
請輸入你的發票號碼：00374554
抱歉沒有中獎。
```