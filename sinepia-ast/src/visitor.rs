use crate::{
    functions::{ItemFn, ReturnType, Signature}, literals::Ident, logic::HoareTriplet, types::Type, Module, ModuleItem
};
use salsa::Database;
/// All methods named `*_pre` will run before we visit the children of the node in question.
/// All methods named `*_post` will run after we visit the children of the node in question.
pub trait AstVisitor<'db, DB: Database> {
    fn visit_module_pre(&mut self, _: &'db DB, _: Module<'db>) -> VisitingStatus {
        VisitingStatus::Continue
    }
    fn visit_module_post(&mut self, _: &'db DB, _: Module<'db>) {}
    fn visit_module_item_pre(&mut self, _: &'db DB, _: &ModuleItem<'db>) -> VisitingStatus {
        VisitingStatus::Continue
    }
    fn visit_module_item_post(&mut self, _: &'db DB, _: &ModuleItem<'db>) {}
    fn visit_fn_pre(&mut self, _: &'db DB, _: ItemFn<'db>) -> VisitingStatus {
        VisitingStatus::Continue
    }
    fn visit_fn_post(&mut self, _: &'db DB, _: ItemFn<'db>) {}
    fn visit_hoare_fn_pre(
        &mut self,
        _: &'db DB,
        _: &HoareTriplet<'db, ItemFn<'db>>,
    ) -> VisitingStatus {
        VisitingStatus::Continue
    }
    fn visit_hoare_fn_post(&mut self, _: &'db DB, _: &HoareTriplet<'db, ItemFn<'db>>) {}
    fn visit_sig_pre(&mut self, _: &'db DB, _: Signature<'db>) -> VisitingStatus {
        VisitingStatus::Continue
    }
    fn visit_sig_post(&mut self, _: &'db DB, _: Signature<'db>) {}
    fn visit_return_type_pre(&mut self, _: &'db DB, _: &ReturnType<'db>) -> VisitingStatus {
        VisitingStatus::Continue
    }
    fn visit_return_type_post(&mut self, _: &'db DB, _: &ReturnType<'db>) {}
    fn visit_type_pre(&mut self, _: &'db DB, _: &Type<'db>) -> VisitingStatus {
        VisitingStatus::Continue
    }
    fn visit_type_post(&mut self, _: &'db DB, _: &Type<'db>) {}
    fn visit_ident(&mut self, _: &'db DB, _: Ident<'db>) {}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VisitingStatus {
    Continue,
    Break,
}

impl VisitingStatus {
    pub fn is_continue(self) -> bool {
        self == VisitingStatus::Continue
    }
}

pub trait Receiver<'db, DB: Database, Vis: AstVisitor<'db, DB>> {
    fn accept(&self, vis: &mut Vis, db: &'db DB);
}
