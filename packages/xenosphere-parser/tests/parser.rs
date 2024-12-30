use pest::{iterators::Pairs, Parser};
use polars;
use polars::prelude::*;
use xenosphere_parser::parsers::{
    makeup_lang::Rule as MarkupRule, makeup_lang::SyntexParser as MarkupParser, parse_makeup_token,
};

#[cfg(test)]
#[test]
fn test_parser_with_layer_stack() {
    let token = parse_makeup_token(
        r##"
#[use("Tensor")]
#[import("Tensor")]
#[import(path="../path/to/file")]
#[export="Tensor"]
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
            static  tensor<(3,3)> mask ;

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
    );
    println!("{:#?}", token);
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
            #[target="x86", syntex("gcc")]{
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
    } 
    "##,
    );
    // println!("{:#?}", token);
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
fn test_parse_tensor_dt<'a>() {
    let test_string = vec![0f32; 27];
    let mut series = Series::new("test", test_string);
    series = series.reshape(&[3i64; 3]).unwrap();
    println!("{:?}", series);
    assert!(true);
}
