# 🧩 SmartContracts Repository

Welcome to the **SmartContracts** repository 🎉
This repo contains a collection of example **Solidity** and **Rust (Internet Computer)** smart contracts that can be used for learning, experimentation, or as a starting point for your own blockchain projects.

---

## 🚀 Available Contracts

### Solidity Contracts

#### 1. Counter Contract

A simple contract to demonstrate the basics of Solidity.
**Features:**

* Initialize a counter with value `0`
* Increment the counter with `increment()`
* Retrieve the counter value with `getCounter()`
* Retrieve the contract description with `getDescription()`

#### 2. Proposal Contract

A contract that manages proposals with voting.
**Features:**

* Define a `Proposal` struct with `title` and `description`
* Create proposals with `createProposal()`
* Vote on proposals using `vote()`
* Retrieve proposal details using `getProposal()`
* Check proposal state (`Succeeded`, `Failed`, `Pending`) with `getProposalState()`

### Rust / Internet Computer (IC) Contracts

#### 1. Auction / Marketplace Contract

A Rust smart contract running on the Internet Computer for managing item auctions.
**Features:**

* List items for auction (`list_item`)
* Place bids (`bid`)
* Update item details (`update_item`)
* Stop auction and determine the highest bidder (`stop_item`)
* Query individual items or all items (`get_item`, `list_items`)
* Retrieve statistics: most expensive item (`most_expensive_item`) and most bid item (`most_bid_item`)

**Note:** Uses IC’s `StableBTreeMap` for storing bids and `RefCell` for in-memory mutable state.

---

## 🛠 How to Use

### Solidity

1. Clone the repository:

```bash
git clone https://github.com/Keenaanash0/SmartContracts.git
```

2. Navigate to the contract folder:

```bash
cd SmartContracts/Solidity/counter-contract
```

3. Open the contract in [Remix IDE](https://remix.ethereum.org/), then compile and deploy.

### Rust / Internet Computer

1. Navigate to the Rust contract folder:

```bash
cd SmartContracts/Rust
```

2. Build and deploy using [DFINITY SDK](https://internetcomputer.org/docs/current/developer-docs/quickstart/):

```bash
dfx deploy
```

3. Interact with the contract using:

```bash
dfx canister call <canister_name> <function_name> '(<args>)'
```

---

## 📌 Roadmap

* Add more Solidity contracts (ERC20 Token, NFT, Voting, etc.)
* Add more Rust contracts for IC (NFT marketplace, token sale, governance, etc.)
* Provide detailed usage instructions and automated tests (Hardhat / Foundry / IC test framework)

---

## 🤝 Contributing

Contributions are welcome!
Feel free to add new smart contracts, improve documentation, or enhance code quality by opening a **Pull Request**.

---

## 📜 License

This repository is licensed under the **MIT License**.
You are free to use, modify, and distribute the code as you wish.

---

✨ Hands-on examples make learning blockchain development fun. Stay tuned for more contracts coming soon!
