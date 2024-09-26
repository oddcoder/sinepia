use std::fmt::Display;

use salsa::Update;

use super::Prop;
use crate::token::{Assuming, Comma, Ergo, Semi};

#[derive(Update, Clone, Debug)]
pub struct HoareTriplet<'db, T: Update> {
    pub assume_token: Assuming,
    pub precondition: Prop<'db>,
    pub comma_token: Comma,
    pub inner: T,
    pub ergo_token: Ergo,
    pub post_condition: Prop<'db>,
    pub semi_token: Semi,
}

impl<'db, T> Display for HoareTriplet<'db, T>
where
    T: Update + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HoareTriplet{{\nprecondition: {},\ninner: {},\npostcondition: {}}}",
            self.precondition, self.inner, self.post_condition
        )
    }
}
