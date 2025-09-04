use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_stable_structures::{Storable, StableBTreeMap, DefaultMemoryImpl};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static ITEMS: RefCell<HashMap<u64, Item>> = RefCell::new(HashMap::new());
    static ITEM_COUNT: RefCell<u64> = RefCell::new(0);
    static BIDS: RefCell<HashMap<u64, StableBTreeMap<u64, u64, DefaultMemoryImpl>>> = RefCell::new(HashMap::new());
}

#[derive(Clone, CandidType, Deserialize)]
struct Item {
    id: u64,
    owner: String,
    name: String,
    description: String,
    active: bool,
    new_owner: Option<String>,
}

impl Storable for u64 {
    fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
    fn from_bytes(bytes: &[u8]) -> Self { u64::from_le_bytes(bytes.try_into().unwrap()) }
}

#[ic_cdk_macros::update]
fn list_item(owner: String, name: String, description: String) -> u64 {
    let id = ITEM_COUNT.with(|c| {
        let mut count = c.borrow_mut();
        *count += 1;
        *count
    });
    let item = Item { id, owner: owner.clone(), name, description, active: true, new_owner: None };
    ITEMS.with(|i| i.borrow_mut().insert(id, item));
    BIDS.with(|b| b.borrow_mut().insert(id, StableBTreeMap::init(DefaultMemoryImpl::default())));
    id
}

#[ic_cdk_macros::update]
fn bid(item_id: u64, bidder: String, amount: u64) -> bool {
    let mut success = false;
    ITEMS.with(|i| {
        if let Some(item) = i.borrow().get(&item_id) {
            if item.active {
                BIDS.with(|b| {
                    let mut bids = b.borrow_mut();
                    if let Some(map) = bids.get_mut(&item_id) {
                        map.insert(bidder.parse().unwrap_or(0), amount);
                        success = true;
                    }
                });
            }
        }
    });
    success
}

#[ic_cdk_macros::update]
fn update_item(item_id: u64, owner: String, name: Option<String>, description: Option<String>) -> bool {
    let mut success = false;
    ITEMS.with(|i| {
        let mut items = i.borrow_mut();
        if let Some(item) = items.get_mut(&item_id) {
            if item.owner == owner && item.active {
                if let Some(n) = name { item.name = n; }
                if let Some(d) = description { item.description = d; }
                success = true;
            }
        }
    });
    success
}

#[ic_cdk_macros::update]
fn stop_item(item_id: u64, owner: String) -> bool {
    let mut success = false;
    ITEMS.with(|i| {
        let mut items = i.borrow_mut();
        if let Some(item) = items.get_mut(&item_id) {
            if item.owner == owner && item.active {
                item.active = false;
                let mut highest_bid = 0;
                let mut highest_bidder = None;
                BIDS.with(|b| {
                    if let Some(map) = b.borrow().get(&item_id) {
                        for (bidder, bid_amount) in map.iter() {
                            if *bid_amount > highest_bid {
                                highest_bid = *bid_amount;
                                highest_bidder = Some(bidder.to_string());
                            }
                        }
                    }
                });
                item.new_owner = highest_bidder;
                success = true;
            }
        }
    });
    success
}

#[ic_cdk_macros::query]
fn get_item(item_id: u64) -> Option<Item> {
    ITEMS.with(|i| i.borrow().get(&item_id).cloned())
}

#[ic_cdk_macros::query]
fn list_items() -> Vec<Item> {
    ITEMS.with(|i| i.borrow().values().cloned().collect())
}

#[ic_cdk_macros::query]
fn items_count() -> u64 {
    ITEM_COUNT.with(|c| *c.borrow())
}

#[ic_cdk_macros::query]
fn most_expensive_item() -> Option<Item> {
    let mut max_amount = 0;
    let mut max_item = None;
    ITEMS.with(|i| {
        BIDS.with(|b| {
            for (id, item) in i.borrow().iter() {
                if let Some(map) = b.borrow().get(id) {
                    for amount in map.values() {
                        if *amount > max_amount {
                            max_amount = *amount;
                            max_item = Some(item.clone());
                        }
                    }
                }
            }
        });
    });
    max_item
}

#[ic_cdk_macros::query]
fn most_bid_item() -> Option<Item> {
    let mut max_count = 0;
    let mut max_item = None;
    ITEMS.with(|i| {
        BIDS.with(|b| {
            for (id, item) in i.borrow().iter() {
                if let Some(map) = b.borrow().get(id) {
                    if map.len() > max_count {
                        max_count = map.len();
                        max_item = Some(item.clone());
                    }
                }
            }
        });
    });
    max_item
}
