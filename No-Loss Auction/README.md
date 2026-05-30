# No-Loss Auction Smart Contract

A decentralized no-loss auction system built on Stellar using Soroban.

## Features

- **Create Auctions**: Auction creators can create new auctions with custom deadlines
- **Place Bids**: Users can place bids using SEP-41 tokens
- **Track Highest Bidder**: Automatic tracking of the current highest bidder
- **Automatic Refunds**: Previous highest bidders can claim their refunds
- **Finalize Auctions**: Close auctions after the deadline
- **Cancel Auctions**: Cancel auctions only if no bids exist

## Building the Contract

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [Soroban CLI](https://soroban.stellar.org/docs/learn/storing-data)

### Build

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Build the contract
cargo build --target wasm32-unknown-unknown --release
```

## Contract Functions

### create_auction
Creates a new auction.

**Parameters:**
- `creator: Address` - The auction creator
- `item_name: String` - Name of the item being auctioned
- `token: Address` - SEP-41 token address for bidding
- `deadline: u64` - Unix timestamp of auction deadline

**Returns:** `u64` - The auction ID

### place_bid
Places a bid on an auction.

**Parameters:**
- `auction_id: u64` - The auction ID
- `bidder: Address` - The bidder's address
- `amount: i128` - Bid amount

### finalize_auction
Finalizes an auction after the deadline has passed.

**Parameters:**
- `auction_id: u64` - The auction ID

### cancel_auction
Cancels an auction (only if no bids exist).

**Parameters:**
- `auction_id: u64` - The auction ID

### Query Functions

- `get_auction(auction_id)` - Returns auction details
- `get_highest_bidder(auction_id)` - Returns the highest bidder
- `get_highest_bid(auction_id)` - Returns the highest bid amount
- `is_auction_finalized(auction_id)` - Checks if auction is finalized
- `is_auction_cancelled(auction_id)` - Checks if auction is cancelled

## Testnet Deployment

**Contract ID:** [To be added after deployment]

See the main README for deployment instructions.
