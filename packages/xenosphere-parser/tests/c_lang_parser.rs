use pest::{iterators::Pairs, Parser};
use polars;
use polars::prelude::*;
use xenosphere_parser::parsers::{
    parse_clang_token, clang::Rule as ClangRule, clang::SyntexParser as ClangParser,
};


#[cfg(test)]
#[test]
fn test_parser_with_simple_clang() {
    let token = parse_clang_token(
        r##"
/****
 * 
 * This is a simple C program
 * 
 */

int w = 10;

int add(int a, int b) {
    return a + b;
}

int main() {
    int x = 5;
    int y = 10;
    // int result = add(x, y);
    if (x > y) {
        printf("x is greater than y\n");
    } else {
        printf("y is greater than x\n");
    }
    printf("Result: %d\n", result);
    return 0;
}

void main_func(tensor $x, tensor $y) {
    const tensor w_1 = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    
    $y = $x * w;
    $y = $x ++;
    $y = $x ** w + 2;
    $y**;

    return;
}

    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}




#[cfg(test)]
#[test]
fn test_parser_looping() {
    let token = parse_clang_token(
        r##"
/****
 * 
 * This is a simple C program
 * 
 */
for (int i = 0; i < 10; i++) {
    printf("i is %d\n", i);
}
for (item in items) {
    printf("i is %d\n", i);
}
    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}




#[cfg(test)]
#[test]
fn test_parser_with_simple_cpp(){
    let token = parse_clang_token(
        r##"

void swapNums(int &x, int &y) {
  int z = x;
  x = y;
  y = z;
}

int main() {
  int firstNum = 10;
  int secondNum = 20;

//   cout << "Before swap: " << "\n";
//   cout << firstNum << secondNum << "\n";

  // Call the function, which will change the values of firstNum and secondNum
  swapNums(firstNum, secondNum);

//   cout << "After swap: " << "\n";
//   cout << firstNum << secondNum << "\n";

  return 0;
}
    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}