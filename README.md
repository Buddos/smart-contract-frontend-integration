# No-Loss Auction Protocol - Stellar Soroban

A complete decentralized auction system built on Stellar using Soroban smart contracts with an integrated React frontend.

## Project Overview

This project implements a **no-loss auction protocol** where participants can bid on items using SEP-41 tokens.

### Key Features

**Smart Contract:**
- Create auctions with custom deadlines
- Place bids using SEP-41 tokens
- Track highest bidder automatically
- Auction finalization after deadline
- Cancel auctions with no bids

**Frontend:**
- Modern, responsive UI
- Real-time auction tracking
- Bid placement with validation
- Auction creation interface

## Quick Start

### 1. Smart Contract Setup

```bash
cd "No-Loss Auction"
cargo build --target wasm32-unknown-unknown --release
```

### 2. Frontend Setup

```bash
cd Frontend
npm install
npm run dev
```

## Deployment

### Deploy to Testnet

```bash
chmod +x deploy.sh
./deploy.sh
```

This will:
1. Configure Soroban CLI for testnet
2. Build the contract
3. Deploy to Stellar testnet
4. Create frontend environment configuration

### Manual Deployment

```bash
cd "No-Loss Auction"
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/no_loss_auction.wasm \
  --source-account my_key \
  --network testnet
```

## Project Structure

```
smart contract-frontend integration/
├── No-Loss Auction/           # Soroban Smart Contract
│   ├── Cargo.toml
│   ├── src/lib.rs
│   └── README.md
├── Frontend/                  # React Frontend
│   ├── src/
│   ├── package.json
│   ├── README.md
│   └── .env.example
└── README.md
```

## Contract Functions

- `create_auction(creator, item_name, token, deadline)` → auction_id
- `place_bid(auction_id, bidder, amount)`
- `finalize_auction(auction_id)`
- `cancel_auction(auction_id)`
- `get_auction(auction_id)` → Auction
- `get_highest_bidder(auction_id)` → Address
- `get_highest_bid(auction_id)` → i128

## Testnet Contract Deployment

**Contract ID:** [Add ID after deployment]

Update this README with your contract ID after successful deployment!

## Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Docs](https://soroban.stellar.org/)
- [React Docs](https://react.dev/)

## License

MIT

---

**Built with ❤️ using Stellar Soroban**
