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



#[cfg(test)]
#[test]
fn test_parser_godot_cpp(){
    let token = parse_clang_token(
        r##"
        // copy from godot cpp
	AnimationPlayer *animation_player = null;

AnimationPlayer *SpineAnimationTrack::find_animation_player() {
	AnimationPlayer *animation_player = nullptr;
	for (int i = 0; i < get_child_count(); i++) {
		animation_player = cast_to<AnimationPlayer>(get_child(i));
		if (animation_player) {
			break;
		}
	}
	return animation_player;
}

    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}


#[cfg(test)]
#[test]
fn test_parser_godot_cpp_1(){
    let token = parse_clang_token(
        r##"
        // copy from godot cpp
bool d = e | r;
string dsd = "dsd";
string empty = "";
void SpineAnimationTrack::_bind_methods() {
	ClassDB::bind_method();
	ClassDB::bind_method(D_METHOD("set_loop", "loop"), &SpineAnimationTrack::set_loop);
	ClassDB::bind_method(D_METHOD("get_loop"), &SpineAnimationTrack::get_loop);

	PropertyInfo("animation_name", PROPERTY_HINT_NONE, "");
	PropertyInfo(Variant::STRING, "animation_name", PROPERTY_HINT_NONE, "");
	PropertyInfo(Variant::STRING, "animation_name", PROPERTY_HINT_NONE, "", PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_INTERNAL | PROPERTY_USAGE_NOEDITOR);
	ADD_PROPERTY(PropertyInfo(Variant::STRING, "animation_name", PROPERTY_HINT_NONE, "", PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_INTERNAL | PROPERTY_USAGE_NOEDITOR), "set_animation_name", "get_animation_name");
	ADD_PROPERTY(PropertyInfo(Variant::BOOL, "loop", PROPERTY_HINT_NONE, "", PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_INTERNAL | PROPERTY_USAGE_NOEDITOR), "set_loop", "get_loop");
}

    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}
