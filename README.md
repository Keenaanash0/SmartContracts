# 🧩 SmartContracts Repository

Welcome to the **SmartContracts** repository 🎉  
This repo contains a collection of example **Solidity** and **Rust (Internet Computer)** smart contracts that can be used for learning, experimentation, or as a starting point for your own blockchain projects.  

---

## 🚀 Available Contracts

### **Solidity Contracts**

#### 1. Counter Contract
A simple contract to demonstrate the basics of Solidity.  
**Features:**
- Initialize a counter with value `0`  
- Increment the counter with `increment()`  
- Retrieve the counter value with `getCounter()`  
- Retrieve the contract description with `getDescription()`  

#### 2. Proposal Contract
A contract that manages proposals with voting.  
**Features:**
- Define a `Proposal` struct with `title` and `description`  
- Create proposals with `createProposal()`  
- Vote on proposals using `vote()`  
- Retrieve proposal details using `getProposal()`  
- Check proposal state (`Succeeded`, `Failed`, `Pending`) with `getProposalState()`  

---

### **Rust / Internet Computer (IC) Contracts**

#### 1. Auction / Marketplace Contract
A Rust smart contract running on the Internet Computer for managing item auctions.  
**Features:**
- List items for auction (`list_item`)  
- Place bids (`bid`)  
- Update item details (`update_item`)  
- Stop auction and determine the highest bidder (`stop_item`)  
- Query individual items or all items (`get_item`, `list_items`)  
- Retrieve statistics: most expensive item (`most_expensive_item`) and most bid item (`most_bid_item`)  

**Note:** Uses IC’s `StableBTreeMap` for storing bids and `RefCell` for in-memory mutable state.  

---

## 🛠 How to Use

### **Solidity**
1. Clone the repository:
   ```bash
   git clone https://github.com/username/SmartContracts.git
