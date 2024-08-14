// Topic: Delegating functionality
//
// Summary:
//   The below program is used as part of an inventory management system that tracks the total
//   quantity of available items. The business would like a notification to be sent when the
//   inventory of an item gets low.
//
// Requirements:
// - Create a proxy structure named `InventoryQuantityTracker` around `InMemoryInventory` that
//   prints a message whenever the quantity of an item reaches or falls below a threshold
//   - The threshold should be specified per-item
//   - The message should be `Low quantity of {item}: {amount}`
//   - Implement a method named `set_alert_threshold` on the `InventoryQuantityTracker` to set
//     the alert threshold per item
// - Update the main function to use the `InventoryQuantityTracker`
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

/// A proxy structure that adds alerting functionality to an inventory manager.
#[derive(Debug, Default)]
struct InventoryQuantityTracker<M> {
    thresholds: HashMap<String, i32>,
    manager: M,
}

impl<M> InventoryQuantityTracker<M>
where
    M: InventoryManager,
{
    fn new(manager: M) -> Self {
        Self {
            thresholds: HashMap::default(),
            manager,
        }
    }

    fn set_alert_threshold<I: Into<String>>(&mut self, item: I, threshold: i32) {
        let item = item.into();
        self.thresholds.insert(item, threshold);
    }
}

impl<M> InventoryManager for InventoryQuantityTracker<M>
where
    M: InventoryManager,
{
    fn update_quantity<I: Into<String>>(&mut self, item: I, amount: i32) {
        let item = item.into();
        // set default threshold to 50 if it hasn't been previously set
        self.thresholds.entry(item.clone()).or_insert_with(|| 50);

        // send alert if the quantity is below the threshold
        {
            let existing_quantity = self.manager.get_quantity(&item).unwrap_or_default();
            let threshold = self.thresholds[&item];
            let new_quantity = existing_quantity + amount;
            if new_quantity <= threshold {
                println!("low quantity of {item}: {new_quantity}");
            }
        }

        // delegate the call to the manager
        self.manager.update_quantity(item, amount);
    }

    fn get_quantity<I: AsRef<str>>(&self, item: I) -> Option<i32> {
        self.manager.get_quantity(item)
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
    let mut inventory = InventoryQuantityTracker::new(BasicInventory::default());
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
