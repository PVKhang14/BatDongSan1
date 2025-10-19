#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Map};

#[contract]
pub struct RealEstateContract;

#[contractimpl]
impl RealEstateContract {
    // 🏠 Tạo tài sản mới
    pub fn create_property(env: Env, id: String, owner: Address, location: String, price: i128) {
        // Lấy dữ liệu hoặc khởi tạo Map mới
        let mut store: Map<String, (Address, String, i128)> =
            env.storage()
                .persistent()
                .get(&"properties")
                .unwrap_or_else(|| Map::new(&env));

        store.set(id.clone(), (owner, location, price));

        env.storage().persistent().set(&"properties", &store);
    }

    // 🔍 Xem thông tin tài sản
    pub fn get_property(env: Env, id: String) -> Option<(Address, String, i128)> {
        let store: Map<String, (Address, String, i128)> =
            env.storage()
                .persistent()
                .get(&"properties")
                .unwrap_or_else(|| Map::new(&env));

        store.get(id)
    }

    // 🔄 Chuyển nhượng tài sản
    pub fn transfer_property(env: Env, id: String, new_owner: Address) {
        let mut store: Map<String, (Address, String, i128)> =
            env.storage()
                .persistent()
                .get(&"properties")
                .unwrap_or_else(|| Map::new(&env));

        if let Some((_, location, price)) = store.get(id.clone()) {
            store.set(id.clone(), (new_owner, location, price));
            env.storage().persistent().set(&"properties", &store);
        }
    }
}
