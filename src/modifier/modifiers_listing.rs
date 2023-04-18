mod modifier_check;
mod modifier_list;

lazy_static::lazy_static! {
    /// Hashmap containing the name of the modifier as key and the Modifier (Global, Offensive, Defensive, Weapon) and the Boolean or the Function to know if the Modifier can be applied
    ///
    /// THE 2 FIRST ELEMENTS MUST BE DELETED
    /// They are only here as an example for the boolean and the function
    static ref HASHMAP: std::collections::HashMap<modifier_list::ModifierList, (std::sync::Arc<crate::modifier::Modifier>, modifier_check::ModifierCheck)> = {
        let mut m: std::collections::HashMap<modifier_list::ModifierList, (std::sync::Arc<crate::modifier::Modifier>, modifier_check::ModifierCheck)> = std::collections::HashMap::new();
        m.insert(modifier_list::ModifierList::MockBool, (std::sync::Arc::new(crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0)), modifier_check::ModifierCheck::AlwaysTrue(true)));
        m.insert(modifier_list::ModifierList::MockFunction, (std::sync::Arc::new(crate::modifier::Modifier::new_offensive(0, 0, 0, 0, 0)), modifier_check::ModifierCheck::Function(modifier_check::check_mockfunction)));
        m
    };
}
