
use xenosphere_parser::view_parser;


#[cfg(test)]
fn test_view() {
    view_parser::view! {
      Hello {
        world: 2,
        foo: 3,
        {}
      }
    };
    assert_eq!(4, 4);
}