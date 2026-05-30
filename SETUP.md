# Project Setup Instructions

## Quick Start (5 minutes)

### 1. Clone/Download Project

You already have the project structure. Navigate to the main folder:

```bash
cd "/home/bonface/Documents/Stellar Bootcamp/smart contract-frontend integration"
```

### 2. Install Dependencies

#### Frontend
```bash
cd Frontend
npm install
```

#### Smart Contract (requires Rust)
```bash
cd "No-Loss Auction"
cargo build --target wasm32-unknown-unknown --release 2>&1 | tail -20
```

### 3. Run Frontend Locally

```bash
cd Frontend
npm run dev
```

Open http://localhost:5173 in your browser.

## Project Structure

### Smart Contract (`No-Loss Auction/`)
- **Cargo.toml** - Rust project dependencies
- **src/lib.rs** - Main contract implementation (253 lines)
- **README.md** - Contract documentation

### Frontend (`Frontend/`)
- **src/components/** - React components
  - AuctionForm.tsx - Create new auctions
  - AuctionList.tsx - Display auctions
  - BidForm.tsx - Place bids
- **src/services/** - Contract interaction
  - auctionService.ts - Mock contract calls
- **src/App.tsx** - Main application
- **package.json** - NPM dependencies
- **README.md** - Frontend documentation

### Configuration
- **README.md** - Main project documentation
- **DEPLOYMENT_GUIDE.md** - Step-by-step deployment
- **.env.example** - Environment variables template

## What's Included

### ✅ Smart Contract Features
- [x] Create auctions with deadlines
- [x] Place bids using SEP-41 tokens
- [x] Track highest bidder
- [x] Automatic bid refunds
- [x] Finalize auctions
- [x] Cancel auctions (no bids only)

### ✅ Frontend Features
- [x] Modern gradient UI
- [x] Auction creation form
- [x] Auction list with filtering
- [x] Bid placement interface
- [x] Real-time updates
- [x] Responsive design

### ✅ Development Files
- [x] TypeScript configuration
- [x] ESLint configuration
- [x] Vite build configuration
- [x] Environment templates
- [x] Git ignore files
- [x] Comprehensive documentation

## Next Steps

### For Local Testing
```bash
# Terminal 1: Run Frontend
cd Frontend
npm install
npm run dev

# Terminal 2: View in browser
# Open http://localhost:5173
```

### For Testnet Deployment
Follow DEPLOYMENT_GUIDE.md:
1. Install Soroban CLI
2. Configure testnet
3. Build contract
4. Deploy contract
5. Update environment variables
6. Deploy frontend

### To Customize

#### Modify Contract
Edit: `No-Loss Auction/src/lib.rs`
- Add new auction types
- Modify bid logic
- Add new contract functions

#### Modify Frontend
Edit: `Frontend/src/` components
- Change UI styling
- Add new features
- Modify forms

## File Statistics

| Component | Files | Lines of Code |
|-----------|-------|---------------|
| Contract | 2 | 253 |
| Frontend TSX | 5 | ~300 |
| Frontend CSS | 5 | ~200 |
| Frontend Config | 4 | ~50 |
| Configuration | 3 | ~100 |
| **Total** | **19** | **~900** |

## Key Files to Review

1. **Smart Contract Logic**
   - `No-Loss Auction/src/lib.rs` - Core auction implementation

2. **Frontend Components**
   - `Frontend/src/App.tsx` - Main app structure
   - `Frontend/src/services/auctionService.ts` - Contract integration

3. **Documentation**
   - `README.md` - Project overview
   - `DEPLOYMENT_GUIDE.md` - Step-by-step deployment

## Common Commands

```bash
# Build smart contract
cd "No-Loss Auction"
cargo build --target wasm32-unknown-unknown --release

# Run frontend development server
cd Frontend
npm run dev

# Build frontend for production
cd Frontend
npm run build

# Deploy to testnet
./deploy.sh

# Check Soroban version
soroban --version

# View testnet explorer
# https://stellar.expert/explorer/testnet/contracts
```

## Environment Setup

### Option 1: Docker (Easiest)
```bash
# If using Docker, most tools are pre-installed
docker run -it --rm -v $(pwd):/workspace node:18
npm install
npm run dev
```

### Option 2: Local Installation

#### macOS
```bash
# Install Homebrew if not installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install tools
brew install rust node

# Install Soroban
cargo install --locked soroban-cli
```

#### Ubuntu/Debian
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node (Ubuntu)
curl -sL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Soroban
cargo install --locked soroban-cli
```

#### Windows
```bash
# Install Rust from https://rustup.rs/
# Install Node from https://nodejs.org/

# Install Soroban
cargo install --locked soroban-cli
```

## Support Resources

### Documentation
- [Stellar Developers](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/)
- [React Documentation](https://react.dev/)

### Community
- [Stellar Discord](https://discord.gg/stellardev)
- [GitHub Discussions](https://github.com/stellar/soroban-examples)

### Testnet Tools
- [Soroban Testnet RPC](https://soroban-testnet.stellar.org)
- [Stellar Expert Explorer](https://stellar.expert/)
- [Friendbot (Get Testnet XLM)](https://developers.stellar.org/docs/reference/testnet)

## Troubleshooting

### Frontend won't start
```bash
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Contract won't build
```bash
rustup update
rustup target add wasm32-unknown-unknown
cd "No-Loss Auction"
cargo clean
cargo build --target wasm32-unknown-unknown --release
```

### Port 5173 already in use
```bash
# Use different port
npm run dev -- --port 3000
```

---

**You're all set! Start with `npm run dev` in the Frontend folder.**
