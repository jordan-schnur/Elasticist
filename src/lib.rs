pub struct Node {
    pub position: Vector2,
    pub size: Vector2,
    pub options: Options,
}

impl Node {
    pub fn new(position: Vector2, size: Vector2, options: Options) -> Node {
        Node {
            position,
            size,
            options,
        }
    }
}

pub struct Options {
    // Container Options
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent, // TODO: Implement safe/unsafe
    pub align_items: AlignItems,
    pub column_gap: i64,
    pub row_gap: i64,
    pub order: i32,
    pub flex_grow: i32,
    pub flex_shrink: i32,
    pub flex_basis: FlexBasis,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Nowrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Normal,
            column_gap: 0,
            row_gap: 0,
            order: 0,
            flex_grow: 0,
            flex_shrink: 1,
            flex_basis: FlexBasis::Auto,
            align_self: AlignSelf::Auto,
            align_content: AlignContent::Normal,
        }
    }
}

impl Options {
    pub fn flew_flow(&mut self, flex_direction: FlexDirection, flex_wrap: FlexWrap) -> &Self {
        self.flex_direction = flex_direction;
        self.flex_wrap = flex_wrap;

        self
    }

    pub fn gap(&mut self, size: i64) -> &Self {
        self.column_gap = size;
        self.row_gap = size;

        self
    }

    pub fn flex(&mut self, flex_grow: i32, flex_shrink: i32, flex_basis: FlexBasis) -> &Self {
        self.flex_grow = flex_grow;
        self.flex_shrink = flex_shrink;
        self.flex_basis = flex_basis;

        self
    }
}

pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
    Normal,
}

pub enum FlexBasis {
    Int(i64),
    Auto,
}

pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

pub enum AlignContent {
    Normal,
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

pub struct Vector2 {
    pub x: i64,
    pub y: i64,
}

impl Vector2 {
    pub fn new(x: Option<i64>, y: Option<i64>) -> Vector2 {
        Vector2 {
            x: x.unwrap_or(0),
            y: y.unwrap_or(0),
        }
    }

    pub fn zero() -> Vector2 {
        Vector2 { x: 0, y: 0 }
    }

    pub fn test(&mut self) -> &Self {
        self.x = 10;

        self
    }
}
