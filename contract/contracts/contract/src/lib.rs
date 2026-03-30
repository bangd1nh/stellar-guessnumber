#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Symbol
};

#[contract]
pub struct GuessNumberGame;

#[contractimpl]
impl GuessNumberGame {

    // Admin khởi tạo game
    pub fn init_game(env: Env, admin: Address, secret_number: i32) {
        admin.require_auth();

        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
        env.storage().instance().set(&symbol_short!("SECRET"), &secret_number);
    }

    // Người chơi đoán số
    pub fn guess(
        env: Env,
        player: Address,
        number: i32,
        token: Address,
        reward: i128,
    ) -> Symbol {

        player.require_auth();

        let secret: i32 = env
            .storage()
            .instance()
            .get(&symbol_short!("SECRET"))
            .unwrap();

        let diff = number - secret;

        // 🎯 Đoán đúng
        if diff == 0 {
            let token_client = soroban_sdk::token::Client::new(&env, &token);
            let contract_address = env.current_contract_address();

            token_client.transfer(&contract_address, &player, &reward);

            return symbol_short!("correct");
        }

        // 🔥 Gần đúng (±5)
        if diff.abs() <= 5 {
            if number < secret {
                return symbol_short!("close_hi"); // hơi thấp
            } else {
                return symbol_short!("close_lo"); // hơi cao
            }
        }

        // 🔼 / 🔽 xa hơn
        if number < secret {
            symbol_short!("higher")
        } else {
            symbol_short!("lower")
        }
    }
}

stellar contract invoke \
  --id CCQYYXY2ZMPTA2JOTQIO3ALA6TABRGLVI6QJK5QIF4BGRVUR6NA4PVXQ \
  --source student \
  --network testnet \
  -- \
  guess \
  --player student \
  --number 50

  stellar contract invoke \
  --id CCQYYXY2ZMPTA2JOTQIO3ALA6TABRGLVI6QJK5QIF4BGRVUR6NA4PVXQ \
  --source student \
  --network testnet \
  -- \
  init_game \
  --admin student \
  --secret_number 73