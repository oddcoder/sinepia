use salsa::{Database, Storage};

#[salsa::db]
#[derive(Default)]
pub struct TestDb {
    storage: Storage<Self>,
}

#[salsa::db]
impl Database for TestDb {
    fn salsa_event(&self, _: &dyn Fn() -> salsa::Event) {}
}
