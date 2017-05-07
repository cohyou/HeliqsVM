use std::collections::HashMap;

struct Unit {}

impl Unit {
    fn new() -> Unit {
        Unit {}
    }
}

/*
 いわゆるUnit
 */
const N: Unit = Unit {};

#[derive(Copy, Clone)]
pub struct Bool {}

impl Bool {
    fn new() -> Bool {
        Bool {}
    }
}

pub const TRUE: Bool = Bool {};
pub const FALSE: Bool = Bool {};


/*
 圏
 */
pub struct Category<Obj: Copy> {
    arrows: Vec<(Obj, Obj)>
}

impl<Obj: Copy> Category<Obj> {
    pub fn new() -> Category<Obj> {
        Category { arrows: vec![] }
    }

    pub fn addObj(&mut self, o: Obj) {
        self.arrows.push((o, o))
    }
}
