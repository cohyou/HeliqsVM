use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Bead {
    Dbr1(i8),
    Lmbd{ p: usize, q: usize },
    Appl{ p: usize, q: usize },

    Int1(i8),
    Text(String),

    None,
}

#[derive(Default)]
pub struct VM {
    target_tube: i16,
    tubes: HashMap<i16, Vec<Bead>>,
}

impl VM {
    pub fn new() -> VM {
        let mut res = VM { ..Default::default() };
        res.tubes.insert(0, vec![]);
        res
    }

    pub fn make_dbr1(&mut self, index: i8) -> usize {
        let new_bead = Bead::Dbr1(index);
        self.make(new_bead)
    }

    pub fn edit_dbr1(&mut self, dntr: usize, index: i8) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        tube.remove(dntr);
        tube.insert(dntr, Bead::Dbr1(index));
    }

    pub fn make_lmbd(&mut self, p: usize, q: usize) -> usize {
        self.make(Bead::Lmbd{ p: p, q: q })
    }

    pub fn edit_lmbd_p(&mut self, dntr: usize, new_p: usize) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        let lambda_q = match tube.get_mut(dntr) {
            Some(&mut Bead::Lmbd { p: _, q }) => q,
            _ => panic!("could not get bead"),
        };

        tube.remove(dntr);
        tube.insert(dntr, Bead::Lmbd{ p: new_p, q: lambda_q });
    }

    pub fn edit_lmbd_q(&mut self, dntr: usize, new_q: usize) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        let lambda_p = match tube.get_mut(dntr) {
            Some(&mut Bead::Lmbd { p, q: _ }) => p,
            _ => panic!("could not get bead"),
        };

        tube.remove(dntr);
        tube.insert(dntr, Bead::Lmbd{ p: lambda_p, q: new_q });
    }

    pub fn make_appl(&mut self, p: usize, q: usize) -> usize {
        self.make(Bead::Appl{ p: p, q: q })
    }

    pub fn edit_appl_p(&mut self, dntr: usize, new_p: usize) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        let lambda_q = match tube.get_mut(dntr) {
            Some(&mut Bead::Appl { p: _, q }) => q,
            _ => panic!("could not get bead"),
        };

        tube.remove(dntr);
        tube.insert(dntr, Bead::Appl{ p: new_p, q: lambda_q });
    }

    pub fn edit_appl_q(&mut self, dntr: usize, new_q: usize) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        let lambda_p = match tube.get_mut(dntr) {
            Some(&mut Bead::Appl { p, q: _ }) => p,
            _ => panic!("could not get bead"),
        };

        tube.remove(dntr);
        tube.insert(dntr, Bead::Appl{ p: lambda_p, q: new_q });
    }

    pub fn make_int1(&mut self, i: i8) -> usize {
        self.make(Bead::Int1(i))
    }

    pub fn edit_int1(&mut self, dntr: usize, new_i: i8) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        tube.remove(dntr);
        tube.insert(dntr, Bead::Int1(new_i));
    }

    pub fn make_text(&mut self, t: &str) -> usize {
        self.make(Bead::Text(t.to_string()))
    }

    pub fn edit_text(&mut self, dntr: usize, new_t: &str) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        tube.remove(dntr);
        tube.insert(dntr, Bead::Text(new_t.to_string()));
    }

    pub fn invalidate(&mut self, dntr: usize) {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        tube.remove(dntr);
        tube.insert(dntr, Bead::None);
    }

    pub fn show_target_tube(&self) {
        println!("{:?}", self.get_tube().expect("could not get tube"));
    }

    fn get_tube(&self) -> Option<&Vec<Bead>> {
        self.tubes.get(&self.target_tube)
    }

    fn get_mut_tube(&mut self) -> Option<&mut Vec<Bead>> {
        self.tubes.get_mut(&self.target_tube)
    }

    fn make(&mut self, bead: Bead) -> usize {
        let r = self.reuse(&bead);
        if r == usize::max_value() {
            self.push(&bead)
        } else {
            r
        }
    }

    fn reuse(&mut self, bead: &Bead) -> usize {
        let mut tube = self.get_mut_tube().expect("could not get tube");
        match tube.iter().position(|b| *b == Bead::None) {
            Some(none_index) => {
                tube.remove(none_index);
                tube.insert(none_index, bead.clone());
                none_index
            },
            None => usize::max_value(),
        }
    }

    fn push(&mut self, bead: &Bead) -> usize {
        match self.tubes.get_mut(&self.target_tube) {
            Some(tube) => {
                tube.push(bead.clone());
                tube.len() - 1
            },
            None => {
                println!("{:?}", "error");
                usize::max_value()
            },
        }
    }
}
