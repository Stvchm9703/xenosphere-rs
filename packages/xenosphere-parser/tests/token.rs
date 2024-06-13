use xenosphere_parser::tokens::tensor::{merge_shape};

#[cfg(test)]
#[test]
fn test_merge_shape() {
    let a = vec![2, 2, 3];
    let b = vec![1, 2, 4, 5];
    let result = vec![2, 2, 4, 5];
    let c = merge_shape(&a, &b);
    assert_eq!(c.len(), result.len());
    for (i, item) in c.iter().enumerate() {
        println!("{}: {} ,, {}", i, item, &result[i]);
        assert_eq!(item, &result[i]);
    }
}
