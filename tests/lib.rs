#[cfg(test)]
mod tests {
	use elasticist::{FlexDirection, Node, Options, Vector2};

	fn setup() -> Node {
		let mut root_node = Node::new();

		root_node.size = Vector2::new(400, 200);

		root_node.nodes.push(Node::from(Vector2::zero(), Vector2::new(50, 50), Options {..Default::default()}));
		root_node.nodes.push(Node::from(Vector2::zero(), Vector2::new(50, 50), Options {..Default::default()}));
		root_node.nodes.push(Node::from(Vector2::zero(), Vector2::new(50, 50), Options {..Default::default()}));

		root_node
	}

	// http://test.csswg.org/suites/css-flexbox-1_dev/nightly-unstable/html/flex-direction.htm
	#[test]
	fn flex_direction_row() {
		let root = setup();

		root.calculate();

		assert_eq!(root.nodes.get(0).unwrap().position, Vector2::new(0, 0));
		assert_eq!(root.nodes.get(1).unwrap().position, Vector2::new(50, 0));
		assert_eq!(root.nodes.get(2).unwrap().position, Vector2::new(100, 0));
	}

	#[test]
	fn flex_direction_reverse_row() {
		let mut root = setup();
		root.options.flex_direction = FlexDirection::RowReverse;

		root.calculate();

		assert_eq!(root.nodes.get(2).unwrap().position, Vector2::new(250, 0));
		assert_eq!(root.nodes.get(1).unwrap().position, Vector2::new(300, 0));
		assert_eq!(root.nodes.get(0).unwrap().position, Vector2::new(350, 0));
	}

	#[test]
	fn flex_direction_column() {
		let mut root = setup();
		root.options.flex_direction = FlexDirection::Column;

		root.calculate();

		assert_eq!(root.nodes.get(0).unwrap().position, Vector2::new(0, 0));
		assert_eq!(root.nodes.get(1).unwrap().position, Vector2::new(0, 50));
		assert_eq!(root.nodes.get(2).unwrap().position, Vector2::new(0, 100));
	}

	#[test]
	fn flex_direction_reverse_column() {
		let mut root = setup();
		root.options.flex_direction = FlexDirection::ColumnReverse;

		root.calculate();

		assert_eq!(root.nodes.get(2).unwrap().position, Vector2::new(0, 50));
		assert_eq!(root.nodes.get(1).unwrap().position, Vector2::new(0, 100));
		assert_eq!(root.nodes.get(0).unwrap().position, Vector2::new(0, 150));
	}
}
