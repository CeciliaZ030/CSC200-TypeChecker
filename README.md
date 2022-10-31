# CSC200-TypeChecker
Tokenizer -> AST

Group Members: Kevin Kao & Yixin Zhang
NetIDs: kkao4 & yzh223
Emails: kkao4@u.rochester.edu & yzh223@u.rochester.edu


Description:
We implemented tokenizer, parser, and type-checker for this project. All works are on our own. In the "tc200/src/files/" directory, there are five files that we used to test our program. Feel free to edit one of them and test it out.

Our implementation idea is
1. first, we read in command line argument [file_name], and then we find the file and read the content,
2. second, we take the content of the file as a string, and pass it to the tokenizer function (line 71 in lib.rs),
3. third, the tokenizer function outputs a vector of token, we then pass it to the parser function (line 250 in lib.rs), and it returns an AST,
4. next, we use this AST for type-checking in the type-checking function (line 923 in lib.rs),
5. and finally, the tc function returns a Type (if there is no panic! occur in the program).



Instruction:
To run the program, issue the following command:
```
$ cargo run [file_name]
```


To run the test cases, issue the following command:
```
$ cargo test
```
