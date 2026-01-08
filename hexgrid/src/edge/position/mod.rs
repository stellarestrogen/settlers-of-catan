use std::marker::PhantomData;

pub mod op_add;
pub mod op_mul;
pub mod op_sub;
pub mod r#type;

#[derive(Clone, Copy)]
pub struct Even;

#[derive(Clone, Copy)]
pub struct Odd;

#[derive(Clone, Copy)]
pub struct Positive;

#[derive(Clone, Copy)]
pub struct Negative;

pub trait Valid {
    fn is_even() -> bool;
    fn is_odd() -> bool;
    fn is_positive() -> bool;
}

impl Valid for Even {
    fn is_even() -> bool {
        true
    }

    fn is_odd() -> bool {
        false
    }

    fn is_positive() -> bool {
        false
    }
}
impl Valid for Odd {
    fn is_even() -> bool {
        false
    }

    fn is_odd() -> bool {
        true
    }

    fn is_positive() -> bool {
        false
    }
}
impl Valid for Positive {
    fn is_even() -> bool {
        false
    }

    fn is_odd() -> bool {
        false
    }

    fn is_positive() -> bool {
        true
    }
}

pub struct EdgePosition<Type> {
    rights: i32,
    downs: i32,
    r#type: PhantomData<Type>,
}

impl EdgePosition<Even> {
    pub const TOP_LEFT: EdgePosition<Even> = EdgePosition {
        rights: 0,
        downs: 0,
        r#type: PhantomData::<Even>,
    };

    pub const BOTTOM_RIGHT: EdgePosition<Even> = EdgePosition {
        rights: 2,
        downs: 2,
        r#type: PhantomData::<Even>,
    };
}

impl EdgePosition<Odd> {
    pub const GO_RIGHT: EdgePosition<Odd> = EdgePosition {
        rights: 2,
        downs: 0,
        r#type: PhantomData::<Odd>,
    };

    pub const GO_LEFT: EdgePosition<Odd> = EdgePosition {
        rights: -2,
        downs: 0,
        r#type: PhantomData::<Odd>,
    };

    pub const BOTTOM_LEFT: EdgePosition<Odd> = EdgePosition {
        rights: -1,
        downs: 1,
        r#type: PhantomData::<Odd>,
    };

    pub const TOP_RIGHT: EdgePosition<Odd> = EdgePosition {
        rights: 1,
        downs: -1,
        r#type: PhantomData::<Odd>,
    };
}

impl EdgePosition<Positive> {
    pub const DOWN_LEFT: EdgePosition<Positive> = EdgePosition {
        rights: -1,
        downs: 1,
        r#type: PhantomData::<Positive>,
    };

    pub const UP_RIGHT: EdgePosition<Positive> = EdgePosition {
        rights: 1,
        downs: -1,
        r#type: PhantomData::<Positive>,
    };

    pub const LEFT: EdgePosition<Positive> = EdgePosition {
        rights: -1,
        downs: 1,
        r#type: PhantomData::<Positive>,
    };

    pub const RIGHT: EdgePosition<Positive> = EdgePosition {
        rights: 3,
        downs: 1,
        r#type: PhantomData::<Positive>,
    };
}

impl EdgePosition<Negative> {
    pub const DOWN_RIGHT: EdgePosition<Negative> = EdgePosition {
        rights: 1,
        downs: 1,
        r#type: PhantomData::<Negative>,
    };

    pub const UP_LEFT: EdgePosition<Negative> = EdgePosition {
        rights: -1,
        downs: -1,
        r#type: PhantomData::<Negative>,
    };
}

impl<Type> EdgePosition<Type> {
    pub fn horizontal_distance<T>(&self, other: EdgePosition<T>) -> i32 {
        self.rights - other.rights
    }

    pub fn vertical_distance<T>(&self, other: EdgePosition<T>) -> i32 {
        self.downs - other.downs
    }
}

impl<Type: Valid> EdgePosition<Type> {
    pub fn is_even(&self) -> bool {
        Type::is_even()
    }

    pub fn is_odd(&self) -> bool {
        Type::is_odd()
    }

    pub fn is_positive(&self) -> bool {
        Type::is_positive()
    }

    pub fn as_even(&self) -> Option<EdgePosition<Even>> {
        if self.is_even() {
            Some(EdgePosition {
                rights: self.horizontal_distance(EdgePosition::TOP_LEFT),
                downs: self.vertical_distance(EdgePosition::TOP_LEFT),
                r#type: PhantomData::default(),
            })
        } else {
            None
        }
    }

    pub fn as_odd(&self) -> Option<EdgePosition<Odd>> {
        if self.is_odd() {
            Some(EdgePosition {
                rights: self.horizontal_distance(EdgePosition::TOP_LEFT),
                downs: self.vertical_distance(EdgePosition::TOP_LEFT),
                r#type: PhantomData::default(),
            })
        } else {
            None
        }
    }

    pub fn as_positive(&self) -> Option<EdgePosition<Positive>> {
        if self.is_positive() {
            Some(EdgePosition {
                rights: self.horizontal_distance(EdgePosition::TOP_LEFT),
                downs: self.vertical_distance(EdgePosition::TOP_LEFT),
                r#type: PhantomData::default(),
            })
        } else {
            None
        }
    }
}

impl<Type> Clone for EdgePosition<Type> {
    fn clone(&self) -> Self {
        EdgePosition::<Type> {
            rights: self.rights,
            downs: self.downs,
            r#type: self.r#type,
        }
    }
}

impl<Type> Copy for EdgePosition<Type> {}

impl<Type> PartialEq for EdgePosition<Type> {
    fn eq(&self, other: &Self) -> bool {
        self.rights == other.rights && self.downs == other.downs && self.r#type == other.r#type
    }
}
