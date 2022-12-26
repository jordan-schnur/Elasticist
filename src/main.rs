use elasticist::{FlexWrap, Node, Options, Vector2};

fn main() {
    let options = Options {flex_wrap: FlexWrap::Wrap, ..Default::default()};
    let mut vec = Vector2::zero();
    println!("X: {:?}", vec.x);
    vec.test();
    println!("X2: {:?}", vec.x);
    let mut node = Node::new(Vector2::zero(), Vector2::new(None, None), options);
    println!("Before Method: Column gap {:?}. Row Gap {:?}", node.options.column_gap, node.options.row_gap);
    node.options.gap(10);
    println!("After Method: Column gap {:?}. Row Gap {:?}", node.options.column_gap, node.options.row_gap);
}
