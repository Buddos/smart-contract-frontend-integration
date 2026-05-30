#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    token, Address, Env, String, Symbol,
};

#[derive(Clone)]
#[contracttype]
pub struct Auction {
    pub id: u64,
    pub creator: Address,
    pub item_name: String,
    pub token: Address,
    pub start_time: u64,
    pub deadline: u64,
    pub highest_bid: i128,
    pub highest_bidder: Address,
    pub is_finalized: bool,
    pub is_cancelled: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    AuctionCounter,
    Auction(u64),
    Refund(u64, Address),
}

#[contract]
pub struct NoLossAuctionContract;

#[contractimpl]
impl NoLossAuctionContract {
    pub fn create_auction(
        env: Env,
        creator: Address,
        item_name: String,
        token: Address,
        deadline: u64,
    ) -> u64 {
        creator.require_auth();

        let current_time = env.ledger().timestamp();

        if deadline <= current_time {
            panic!("Deadline must be in the future");
        }

        let auction_counter: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::AuctionCounter)
            .unwrap_or(0);

        let auction_id = auction_counter + 1;

        let auction = Auction {
            id: auction_id,
            creator: creator.clone(),
            item_name,
            token,
            start_time: current_time,
            deadline,
            highest_bid: 0,
            highest_bidder: creator,
            is_finalized: false,
            is_cancelled: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Auction(auction_id), &auction);

        env.storage()
            .persistent()
            .set(&DataKey::AuctionCounter, &auction_id);

        auction_id
    }

    pub fn place_bid(
        env: Env,
        auction_id: u64,
        bidder: Address,
        amount: i128,
    ) {
        bidder.require_auth();

        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        if auction.is_finalized {
            panic!("Auction already finalized");
        }

        if auction.is_cancelled {
            panic!("Auction cancelled");
        }

        if env.ledger().timestamp() > auction.deadline {
            panic!("Auction deadline passed");
        }

        if amount <= auction.highest_bid {
            panic!("Bid must exceed current highest bid");
        }

        let token_client = token::Client::new(&env, &auction.token);

        token_client.transfer(
            &bidder,
            &env.current_contract_address(),
            &amount,
        );

        if auction.highest_bid > 0
            && auction.highest_bidder != auction.creator
        {
            env.storage().persistent().set(
                &DataKey::Refund(
                    auction_id,
                    auction.highest_bidder.clone(),
                ),
                &auction.highest_bid,
            );
        }

        let updated_auction = Auction {
            highest_bid: amount,
            highest_bidder: bidder.clone(),
            ..auction
        };

        env.storage()
            .persistent()
            .set(&DataKey::Auction(auction_id), &updated_auction);

        env.events().publish(
            (Symbol::new(&env, "bid_placed"),),
            (auction_id, bidder, amount),
        );
    }

    pub fn finalize_auction(
        env: Env,
        auction_id: u64,
    ) {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        if auction.is_finalized {
            panic!("Already finalized");
        }

        if env.ledger().timestamp() <= auction.deadline {
            panic!("Auction still active");
        }

        let updated_auction = Auction {
            is_finalized: true,
            ..auction
        };

        env.storage()
            .persistent()
            .set(&DataKey::Auction(auction_id), &updated_auction);

        env.events().publish(
            (Symbol::new(&env, "auction_finalized"),),
            (
                auction_id,
                updated_auction.highest_bidder.clone(),
                updated_auction.highest_bid,
            ),
        );
    }

    pub fn cancel_auction(
        env: Env,
        auction_id: u64,
    ) {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        auction.creator.require_auth();

        if auction.highest_bid > 0 {
            panic!("Cannot cancel auction with bids");
        }

        if auction.is_finalized {
            panic!("Cannot cancel finalized auction");
        }

        let updated_auction = Auction {
            is_cancelled: true,
            ..auction
        };

        env.storage()
            .persistent()
            .set(&DataKey::Auction(auction_id), &updated_auction);

        env.events().publish(
            (Symbol::new(&env, "auction_cancelled"),),
            auction_id,
        );
    }

    pub fn get_auction(
        env: Env,
        auction_id: u64,
    ) -> Auction {
        env.storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found")
    }

    pub fn get_highest_bidder(
        env: Env,
        auction_id: u64,
    ) -> Address {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        auction.highest_bidder
    }

    pub fn get_highest_bid(
        env: Env,
        auction_id: u64,
    ) -> i128 {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        auction.highest_bid
    }

    pub fn is_auction_finalized(
        env: Env,
        auction_id: u64,
    ) -> bool {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        auction.is_finalized
    }

    pub fn is_auction_cancelled(
        env: Env,
        auction_id: u64,
    ) -> bool {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        auction.is_cancelled
    }

    pub fn claim_refund(
        env: Env,
        auction_id: u64,
        bidder: Address,
    ) {
        bidder.require_auth();

        let amount: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Refund(
                auction_id,
                bidder.clone(),
            ))
            .expect("No refund available");

        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .expect("Auction not found");

        let token_client =
            token::Client::new(&env, &auction.token);

        token_client.transfer(
            &env.current_contract_address(),
            &bidder,
            &amount,
        );

        env.storage()
            .persistent()
            .remove(&DataKey::Refund(
                auction_id,
                bidder,
            ));
    }
}