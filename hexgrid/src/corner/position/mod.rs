use std::marker::PhantomData;

pub mod op_add;
pub mod op_mul;
pub mod op_sub;

pub struct Low;
pub struct High;
pub struct Center;

pub trait Height {
    fn is_low() -> bool;
    fn is_high() -> bool;
}

impl Height for Low {
    fn is_low() -> bool {
        true
    }

    fn is_high() -> bool {
        false
    }
}
impl Height for High {
    fn is_low() -> bool {
        false
    }

    fn is_high() -> bool {
        true
    }
}

pub struct CornerPosition<H> {
    rights: i32,
    downs: i32,
    height: PhantomData<H>,
}

impl CornerPosition<High> {
    pub const UP_LEFT: CornerPosition<High> = CornerPosition {
        rights: -1,
        downs: -1,
        height: PhantomData::<High>,
    };

    pub const UP_RIGHT: CornerPosition<High> = CornerPosition {
        rights: 1,
        downs: -1,
        height: PhantomData::<High>,
    };

    pub const DOWN: CornerPosition<High> = CornerPosition {
        rights: 0,
        downs: 2,
        height: PhantomData::<High>,
    };

    pub const BOTTOM_LEFT: CornerPosition<High> = CornerPosition {
        rights: 0,
        downs: 2,
        height: PhantomData::<High>,
    };

    pub const BOTTOM_RIGHT: CornerPosition<High> = CornerPosition {
        rights: 2,
        downs: 2,
        height: PhantomData::<High>,
    };

    pub const TOP: CornerPosition<High> = CornerPosition {
        rights: 1,
        downs: -1,
        height: PhantomData::<High>,
    };

    pub fn go_right(self) -> CornerPosition<Low> {
        self + CornerPosition::DOWN_RIGHT
    }

    pub fn go_left(self) -> CornerPosition<Low> {
        self + CornerPosition::DOWN_LEFT
    }

    pub fn go_up(self) -> CornerPosition<Low> {
        self + CornerPosition::UP
    }

    pub fn as_generic<H: Height>(&self) -> CornerPosition<H> {
        CornerPosition {
            rights: self.rights,
            downs: self.downs,
            height: PhantomData::<H>,
        }
    }
}

impl CornerPosition<Low> {
    pub const TOP_LEFT: CornerPosition<Low> = CornerPosition {
        rights: 0,
        downs: 0,
        height: PhantomData::<Low>,
    };

    pub const TOP_RIGHT: CornerPosition<Low> = CornerPosition {
        rights: 2,
        downs: 0,
        height: PhantomData::<Low>,
    };

    pub const BOTTOM: CornerPosition<Low> = CornerPosition {
        rights: 1,
        downs: 3,
        height: PhantomData::<Low>,
    };

    pub fn go_right(self) -> CornerPosition<High> {
        self + CornerPosition::UP_RIGHT
    }

    pub fn go_left(self) -> CornerPosition<High> {
        self + CornerPosition::UP_LEFT
    }

    pub fn go_down(self) -> CornerPosition<High> {
        self + CornerPosition::DOWN
    }

    pub fn as_generic<H: Height>(&self) -> CornerPosition<H> {
        CornerPosition {
            rights: self.rights,
            downs: self.downs,
            height: PhantomData::<H>,
        }
    }
}

impl CornerPosition<Center> {
    pub const DOWN_LEFT: CornerPosition<Center> = CornerPosition {
        rights: -1,
        downs: 1,
        height: PhantomData::<Center>,
    };

    pub const DOWN_RIGHT: CornerPosition<Center> = CornerPosition {
        rights: 1,
        downs: 1,
        height: PhantomData::<Center>,
    };

    pub const UP: CornerPosition<Center> = CornerPosition {
        rights: 0,
        downs: -2,
        height: PhantomData::<Center>,
    };

    pub fn as_generic<H: Height>(&self) -> CornerPosition<H> {
        CornerPosition {
            rights: self.rights,
            downs: self.downs,
            height: PhantomData::<H>,
        }
    }
}

impl<H> CornerPosition<H> {
    pub fn horizontal_distance<T>(&self, other: CornerPosition<T>) -> i32 {
        self.rights - other.rights
    }

    pub fn vertical_distance<T>(&self, other: CornerPosition<T>) -> i32 {
        self.downs - other.downs
    }
}

impl<H: Height> CornerPosition<H> {
    pub fn is_low(&self) -> bool {
        H::is_low()
    }

    pub fn is_high(&self) -> bool {
        H::is_high()
    }

    pub fn as_low(&self) -> Option<CornerPosition<Low>> {
        if self.is_low() {
            Some(CornerPosition {
                rights: self.rights,
                downs: self.downs,
                height: PhantomData::<Low>,
            })
        } else {
            None
        }
    }

    pub fn as_high(&self) -> Option<CornerPosition<High>> {
        if self.is_high() {
            Some(CornerPosition {
                rights: self.rights,
                downs: self.downs,
                height: PhantomData::<High>,
            })
        } else {
            None
        }
    }
}

impl<H> Clone for CornerPosition<H> {
    fn clone(&self) -> Self {
        CornerPosition::<H> {
            rights: self.rights,
            downs: self.downs,
            height: PhantomData::<H>,
        }
    }
}

impl<H> Copy for CornerPosition<H> {}

impl<H> PartialEq for CornerPosition<H> {
    fn eq(&self, other: &Self) -> bool {
        self.rights == other.rights && self.downs == other.downs && self.height == other.height
    }
}
