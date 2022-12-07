# 第十次作業第二題
## 題目
Write a function

```c++
void keywordMasking(char * sentence, char * keyword);
```

which masks keywords in the given sentence with # signs.


Use the following `main()` function to test your function.  
You CANNOT modify this `main()` function except the ??? part.

```c++
void duplicateStr(char dest[], char *src, int n) { ??? }

int main() {
    char newStr[???], inputStr[???];
    int ti, repeatTimes, rpTimes;
    ??? // get repeatTimes from keyboard
    for (ti = 0; ti < repeatTimes; ti++) {
        printf("\nInput a string: ");
        scanf(???); // or fgets(???)
        printf("Duplicate how many times: ");
        scanf(???);
        duplicateStr(newStr, inputStr, rpTimes);
        printf("The new string is [%s]\n", ???);
    }
    return 0;
}
```

Programs not using `keywordMasking()` will receive 0 points.
## 範例
```
How many sets of test data: 2

Input a sentence: Oh my gosh! Can you say gosh?
Input a keyword: gosh
The masked sentence is [Oh my ####! Can you say ####?]

Input a sentence: I don't want to see any "an" in the sentence.
Input a keyword: an
The masked sentence is [I don't w##t to see ##y "##" in the sentence.]
```
