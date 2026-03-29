#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short,
    Address, BytesN, Env, Map, Symbol, Vec,
};
use soroban_sdk::token;

const ADMIN: Symbol = symbol_short!("ADMIN");
const BILLS: Symbol = symbol_short!("BILLS");

#[contracttype]
#[derive(Clone)]
pub struct Bill {
    pub token: Address,
    pub payee: Address,
    pub total: i128,
    pub shares: Map<Address, i128>, // member -> owed
    pub paid: Map<Address, i128>,   // member -> paid
    pub settled: bool,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InvalidInput = 3,
    BillNotFound = 4,
    AlreadySettled = 5,
    NotMember = 6,
    AmountTooLarge = 7,
    NotFullyFunded = 8,
}

#[contract]
pub struct UniSplit;

#[contractimpl]
impl UniSplit {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);

        let bills: Map<BytesN<32>, Bill> = Map::new(&env);
        env.storage().instance().set(&BILLS, &bills);
        Ok(())
    }

    pub fn create_bill(
        env: Env,
        bill_id: BytesN<32>,
        creator: Address,
        token_addr: Address,
        payee: Address,
        members: Vec<Address>,
        amounts: Vec<i128>,
    ) -> Result<(), Error> {
        if !env.storage().instance().has(&ADMIN) {
            return Err(Error::NotInitialized);
        }
        creator.require_auth();

        if members.len() == 0 || members.len() != amounts.len() {
            return Err(Error::InvalidInput);
        }

        let mut shares: Map<Address, i128> = Map::new(&env);
        let mut total: i128 = 0;

        for i in 0..members.len() {
            let m = members.get(i).ok_or(Error::InvalidInput)?;
            let a = amounts.get(i).ok_or(Error::InvalidInput)?;
            if a <= 0 {
                return Err(Error::InvalidInput);
            }
            shares.set(m, a);
            total = total.checked_add(a).ok_or(Error::AmountTooLarge)?;
        }

        let mut bills: Map<BytesN<32>, Bill> =
            env.storage().instance().get(&BILLS).unwrap_or(Map::new(&env));

        if bills.contains_key(bill_id.clone()) {
            return Err(Error::InvalidInput);
        }

        bills.set(
            bill_id,
            Bill {
                token: token_addr,
                payee,
                total,
                shares,
                paid: Map::new(&env),
                settled: false,
            },
        );

        env.storage().instance().set(&BILLS, &bills);
        Ok(())
    }

    // Member pays; if bill is fully funded after this payment -> auto settle to payee.
    pub fn pay_and_try_settle(
        env: Env,
        bill_id: BytesN<32>,
        from: Address,
        amount: i128,
    ) -> Result<(), Error> {
        from.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidInput);
        }

        let mut bills: Map<BytesN<32>, Bill> =
            env.storage().instance().get(&BILLS).unwrap_or(Map::new(&env));
        let mut bill = bills.get(bill_id.clone()).ok_or(Error::BillNotFound)?;

        if bill.settled {
            return Err(Error::AlreadySettled);
        }

        let owed = bill.shares.get(from.clone()).ok_or(Error::NotMember)?;
        let already = bill.paid.get(from.clone()).unwrap_or(0);

        let new_paid = already.checked_add(amount).ok_or(Error::AmountTooLarge)?;
        if new_paid > owed {
            return Err(Error::InvalidInput);
        }

        // transfer token from payer -> contract
        let t = token::Client::new(&env, &bill.token);
        t.transfer(&from, &env.current_contract_address(), &amount);

        bill.paid.set(from, new_paid);

        // check fully funded
        let mut funded: i128 = 0;
        for (_addr, p) in bill.paid.iter() {
            funded = funded.checked_add(p).ok_or(Error::AmountTooLarge)?;
        }

        if funded == bill.total {
            // settle
            t.transfer(
                &env.current_contract_address(),
                &bill.payee,
                &bill.total,
            );
            bill.settled = true;
        } else if funded > bill.total {
            // should never happen, but keep safe
            return Err(Error::AmountTooLarge);
        }

        bills.set(bill_id, bill);
        env.storage().instance().set(&BILLS, &bills);
        Ok(())
    }
}


stellar contract invoke \
  --id CCSOIKLVOR4G3AG2SJUL7FWRSTHHA5MSC2CMDOY2ICMBT7LJPHW7BXZC \
  --source-account student \
  --network testnet \
  -- init \
  --admin student 