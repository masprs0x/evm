use utils::u256::U256;
use utils::gas::Gas;
use utils::address::Address;
use utils::hash::H256;

pub trait Block {
    fn account_code(&self, address: Address) -> Option<&[u8]>;
    fn coinbase(&self) -> Address;
    fn balance(&self, address: Address) -> Option<U256>;
    fn timestamp(&self) -> U256;
    fn number(&self) -> U256;
    fn difficulty(&self) -> U256;
    fn gas_limit(&self) -> Gas;
    fn create_account(&mut self, code: &[u8]) -> Option<Address>;
    fn account_storage(&self, address: Address) -> &[U256];
    fn set_account_storage(&mut self, address: Address, storage: &[U256]);
    fn log(&mut self, address: Address, data: &[u8], topics: &[U256]);
}

pub struct FakeBlock;

impl Block for FakeBlock {
    fn account_code(&self, address: Address) -> Option<&[u8]> {
        None
    }

    fn create_account(&mut self, code: &[u8]) -> Option<Address> {
        None
    }

    fn coinbase(&self) -> Address {
        Address::default()
    }

    fn balance(&self, address: Address) -> Option<U256> {
        None
    }

    fn timestamp(&self) -> U256 {
        U256::zero()
    }

    fn number(&self) -> U256 {
        U256::zero()
    }

    fn difficulty(&self) -> U256 {
        U256::zero()
    }

    fn gas_limit(&self) -> Gas {
        Gas::zero()
    }

    fn account_storage(&self, address: Address) -> &[U256] {
        unimplemented!()
    }

    fn set_account_storage(&mut self, address: Address, storage: &[U256]) {
        unimplemented!()
    }

    fn log(&mut self, address: Address, data: &[u8], topics: &[U256]) {
        unimplemented!()
    }
}

impl Default for FakeBlock {
    fn default() -> FakeBlock {
        FakeBlock
    }
}

pub trait Blockchain {
    fn blockhash(&self, n: U256) -> H256;
}

pub struct FakeBlockchain;

impl Blockchain for FakeBlockchain {
    fn blockhash(&self, n: U256) -> H256 {
        H256::default()
    }
}

impl Default for FakeBlockchain {
    fn default() -> FakeBlockchain {
        FakeBlockchain
    }
}