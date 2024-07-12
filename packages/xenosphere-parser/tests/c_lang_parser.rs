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
    $y = $x ** w;
    $y**;

    return;
}

    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}
