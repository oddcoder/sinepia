use crate::{functions::ItemFn, logic::HoareTriplet};
use salsa::Update;
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
