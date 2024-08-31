// Topic: Delegating functionality
//
// Summary:
//   The below program is used as part of an inventory management system that tracks the total
//   quantity of available items. The business would like a notification to be sent when the
//   inventory of an item gets low.
//
// Requirements:
// - Create a proxy structure named `InventoryAlerter` around `BasicInventory` that
//   prints a message whenever the quantity of an item reaches or falls below a threshold
//   - The threshold should be specified per-item
//   - The message should be `Low quantity of {item}: {amount}`
//   - Implement a method named `set_alert_threshold` on the `InventoryAlerter` to set
//     the alert threshold per item
// - Update the main function to use the `InventoryAlerter`
// - When implemented correctly, you should get 2 alerts:
//     low quantity of apple: 50
//     low quantity of cilantro: 55

use std::collections::HashMap;

/// Manages the quantity of an inventory.
pub trait InventoryManager {
    /// Change the quantity of an item. If the item does not exist, it will be added.
    fn update_quantity<I: Into<String>>(&mut self, item: I, amount: i32);

    /// Returns the total quantity of an item, if the item was found.
    fn get_quantity<I: AsRef<str>>(&self, item: I) -> Option<i32>;
}

/// An in-memory inventory manager backed by a hashmap.
#[derive(Debug, Default)]
struct BasicInventory {
    inventory: HashMap<String, i32>,
}

impl InventoryManager for BasicInventory {
    fn update_quantity<I: Into<String>>(&mut self, item: I, amount: i32) {
        let item = item.into();
        let entry = self.inventory.entry(item).or_default();
        *entry += amount;
    }

    fn get_quantity<I: AsRef<str>>(&self, item: I) -> Option<i32> {
        self.inventory.get(item.as_ref()).copied()
    }
}

/***********************************************************************************************
* Do not edit this function. It should work with the structure that you create without having to
* make any modifications to the function.
***********************************************************************************************/
fn change_quantity<M, I>(manager: &mut M, item: I, amount: i32)
where
    M: InventoryManager,
    I: Into<String>,
{
    manager.update_quantity(item, amount);
}

fn main() {
    /******************************************************
     * Change the below line to create your proxy structure
     ******************************************************/
    let mut inventory = BasicInventory::default();

    /***********************************************************************************************
     * Do not change anything else in this function. When implemented correctly, you should get 2
     * alerts printed to the terminal.
     **********************************************************************************************/

    // should have an alert
    inventory.update_quantity("apple", 50);

    // no alert
    inventory.update_quantity("tomato", 120);
    // no alert
    inventory.update_quantity("cilantro", 60);

    // change the threshold for cilantro
    inventory.set_alert_threshold("cilantro", 55);

    // should have an alert
    change_quantity(&mut inventory, "cilantro", -6);
}
