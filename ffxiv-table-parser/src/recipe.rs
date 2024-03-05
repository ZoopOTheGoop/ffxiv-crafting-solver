use crate::raw_records::{RawItem, RawRecipe};

#[doc(inline)]
pub use crate::raw_records::RecipeLevel;

pub struct Recipe {
    rlvl_data: RecipeLevel,

    /* From Items table */
    item_name: String,
    ilvl: u32,
    is_collectable: bool,
}
