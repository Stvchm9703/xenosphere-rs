use pest::{Parser, iterators::Pairs};
// use polars;
// use polars::prelude::*;
use xenosphere_parser::parsers::{
    makeup_lang::Rule as MarkupRule, makeup_lang::SyntexParser as MarkupParser, parse_makeup_token,
};

use serde_json::to_string_pretty as to_string;

use bincode;

#[cfg(test)]
#[test]
fn test_parser_with_layer_stack() {
    use std::{fs, io::Write, ptr::write_bytes};

    let token = parse_makeup_token(
        r##"
#[use("Tensor")]
#[import("Tensor")]
#[import(name="Conv", path="../path/to/file")]
#[export("Tensor")]
    layer Conv {
        property {
            in      int y = 3 ;
            in      int e ;
            static  int x = 3;
            static  float z = 3.0;
            static  string a = "3";
            static  array b  = [3, 4, 5];
            static  func c;
            static  func c1 = LEUKOCYTE;
            static  func c2 = LEUKOCYTE($x, $y, b=12);
            static  tensor<(3,3)> mask;
        }
        stack
        [
            Conv2D($y, $z, 3, 3, activation="relu"),
            Flatten(),
            Dense(128),
            Dense(10),
            Classification(label=10),
            test.run(label=10),
            Resu/Resu101(label=10),
            #branch
            {
                stack [
                    Conv2D($y, $z, 3, 3, activation="relu"),
                    Flatten(),
                    Dense(128),
                    Dense(10),
                ],
                stack [
                    Conv2D($y, $z, 3, 3, activation="relu"),
                    Flatten(),
                    Dense(128),
                    Dense(10),
                ],
            }
        ]
    }
    "##,
    )
    .unwrap();
    // println!("{:#?}", token);
    println!("{:?}", to_string(&token));

    let s = bincode::serialize(&token).unwrap();
    let mut bin_file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("test_parser_with_layer_stack.bincode")
        .unwrap();

    bin_file.write_all(&s).unwrap();

    assert_eq!(4, 4);
}

#[cfg(test)]
#[test]
fn test_parser_with_layer_pass() {
    let token = parse_makeup_token(
        r##"
#[use("Tensor")]
#[export="Tensor"]

    layer Conv {
        property {
            in int y = 3 ;
            static int x = 3 ;
            static float z = 3.0 ;
            static string a = "3" ;
            static array b = [3, 4, 5];
            static tensor<(3,3)> mask = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        }
        pass {
            #[target="x86", syntex="gcc"]{
                int x = 5;
                int y = 10;
                // code block
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
            }#
        }
        preview {

        }
    }
    "##,
    )
    .unwrap();
    println!("{:#?}", token);
    println!("{:?}", to_string(&token));

    assert_eq!(4, 4);
}

pub type TestCase<'a> = (&'a str, Pairs<'static, MarkupRule>);

#[cfg(test)]
#[test]
fn test_property_define<'a>() {
    let test_cases = vec![
        ("int y = 3", MarkupRule::int_def_block),
        ("float y = 3.0", MarkupRule::float_def_block),
        ("string y = \"3\"", MarkupRule::string_def_block),
        ("array y = [3, 4, 5]", MarkupRule::array_def_block),
        (
            "tensor<(3,3)> y = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]",
            MarkupRule::tensor_def_block,
        ),
        ("tensor<(3,3)> y ", MarkupRule::tensor_def_block),
        ("func y", MarkupRule::func_def_block),
        ("func y = LEUKOCYTE", MarkupRule::func_def_block),
        ("func y = LEUKOCYTE()", MarkupRule::func_def_block),
        ("func y = LEUKOCYTE(3, b=4)", MarkupRule::func_def_block),
        ("func y = LEUKOCYTE($z)", MarkupRule::func_def_block),
    ];

    // let mut test_cases_result: Vec<bool> = vec![];

    // let input = r#"int y = 3"#;
    // let token = MarkupParser::parse(MarkupRule::var_def_block, input);
    // println!("{:?}", token);

    for (input, rule) in test_cases {
        let token = MarkupParser::parse(MarkupRule::var_def_block, input);

        println!("{:?}", input);
        println!("ok ? : {:?}", token.is_ok());

        if token.is_ok() {
            let result = token.clone().unwrap().next().unwrap();
            assert!(result.as_rule() == rule);
            println!("{:#?}", result);
        } else {
            println!("{:?}", token.err().unwrap());
        }
    }
    // assert_eq!(token.is_ok(), true);
}

#[cfg(test)]
#[test]
fn test_parser_with_layer_stack2() {
    let token = parse_makeup_token(
        r##"
#[use("Tensor")]
#[import("Tensor")]
#[import(name="Conv", path="../path/to/file")]
#[export("Tensor")]
    layer Conv {
        property {
            in      int y = 3 ;
            in      int e ;
            static  int x = 3;
            static  float z = 3.0;
            static  string a = "3";
            static  array b  = [3, 4, 5];
            static  func c;
            static  func c1 = LEUKOCYTE;
            static  func c2 = LEUKOCYTE($x, $y, b=12);
            static  tensor<(3,3)> mask;
        }
        stack
        [
             Conv2D(
                 input_tensor = $input_tensor,
                 output_tensor = $output_tensor,
                 kernel_size = 1,
                 stride_size = 1,
             ),
             MaxPool2D(
                 input_tensor = $output_tensor,
                 output_tensor = $output_tensor,
                 kernel_size = kernel_size,
             ),
             CELU(
                 input_tensor = $output_tensor,
                 output_tensor = $output_tensor,
             ),
        ]
    }
    "##,
    )
    .unwrap();
    // println!("{:#?}", token);
    println!("{:?}", to_string(&token));
    assert!(true);
}
