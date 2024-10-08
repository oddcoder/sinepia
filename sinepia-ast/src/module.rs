use crate::{functions::ItemFn, logic::HoareTriplet, AstVisitor, Receiver};
use salsa::{Database, Update};
use std::fmt::Display;

#[derive(Update, Clone, Debug)]
pub enum ModuleItem<'db> {
    Fn(ItemFn<'db>),
    HoareFn(HoareTriplet<'db, ItemFn<'db>>),
}
impl<'db> Display for ModuleItem<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleItem::Fn(inner) => write!(f, "ModuleItem({inner})"),
            ModuleItem::HoareFn(inner) => write!(f, "ModuleItem({inner})"),
        }
    }
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for ModuleItem<'db> {
    fn accept(&self, visitor: &mut Vis, db: &'db DB) {
        let cont = visitor.visit_module_item_pre(db, self);
        match (cont.is_continue(), self) {
            (true, ModuleItem::Fn(item)) => item.accept(visitor, db),
            (true, ModuleItem::HoareFn(item)) => item.accept(visitor, db),
            (false, _) => (),
        };
        visitor.visit_module_item_post(db, self)
    }
}

#[salsa::tracked]
pub struct Module<'db> {
    pub items: Vec<ModuleItem<'db>>,
}

impl<'db> Display for Module<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        salsa::with_attached_database(|db| {
            writeln!(f, "Module[")?;
            let items: Vec<_> = self.items(db).iter().map(|i| format!("{i}")).collect();
            write!(f, "{}]", items.join("\n"))
        })
        .unwrap_or_else(|| panic!("Allowed to only run inside attach()"))
    }
}

impl<'db, DB: Database, Vis: AstVisitor<'db, DB>> Receiver<'db, DB, Vis> for Module<'db> {
    fn accept(&self, visitor: &mut Vis, db: &'db DB) {
        let cont = visitor.visit_module_pre(db, *self);
        if cont.is_continue() {
            let items = self.items(db);
            for item in items {
                item.accept(visitor, db);
            }
        }
        visitor.visit_module_post(db, *self)
    }
}
