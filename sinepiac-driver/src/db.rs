use salsa::{db, Database, Storage};

#[db]
#[derive(Default)]
pub struct CompilerDb {
    storage: Storage<Self>,
}

#[db]
impl Database for CompilerDb {
    fn salsa_event(&self, _: &dyn Fn() -> salsa::Event) {}
}
