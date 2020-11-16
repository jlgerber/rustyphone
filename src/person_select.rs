

use crate::Selectable;

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
pub struct PersonSelect {
    pub id: bool,
    pub first: bool,
    pub last: bool,
    pub login: bool
}

impl Selectable for PersonSelect {

    fn select(&self) -> String {
        let mut fields = Vec::new();
        if self.id {
            fields.push("id");
        }
        if self.first {
            fields.push("first");
        }
        if self.last {
            fields.push("last");
        }
        if self.login {
            fields.push("login");
        }
       
        fields.join(",")
    }
}
impl PersonSelect {
    pub fn all() -> Self {
        Self {
            id: true,
            first: true,
            last: true,
            login: true,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn first(mut self, value: bool) -> Self {
        self.first = value;
        self
    }

    pub fn last(mut self, value: bool) -> Self {
        self.last = value;
        self
    }
    pub fn login(mut self, value: bool) -> Self {
        self.login = value;
        self
    }
    

}

impl Default for PersonSelect {
    fn default() -> Self {
        Self {
            id: true,
            first: false,
            last: false,
            login: false,
        }
    }
}