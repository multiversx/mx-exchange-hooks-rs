#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod amm;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod fee;
pub mod liquidity_pool;
pub mod locking_wrapper;
pub mod pair_actions;
pub mod pair_hooks;
pub mod read_pair_storage;
pub mod safe_price;
pub mod safe_price_view;

use crate::errors::*;

use common_structs::Percent;
use contexts::base::*;
use pair_actions::common_result_types::{
    AddLiquidityResultType, RemoveLiquidityResultType, SwapTokensFixedInputResultType,
    SwapTokensFixedOutputResultType,
};
use pausable::State;
use permissions_module::Permissions;

#[multiversx_sc::contract]
pub trait Pair<ContractReader>:
    amm::AmmModule
    + fee::FeeModule
    + liquidity_pool::LiquidityPoolModule
    + config::ConfigModule
    + token_send::TokenSendModule
    + events::EventsModule
    + read_pair_storage::ReadPairStorageModule
    + safe_price::SafePriceModule
    + safe_price_view::SafePriceViewModule
    + contexts::output_builder::OutputBuilderModule
    + locking_wrapper::LockingWrapperModule
    + permissions_module::PermissionsModule
    + pausable::PausableModule
    + pair_actions::initial_liq::InitialLiquidityModule
    + pair_actions::add_liq::AddLiquidityModule
    + pair_actions::remove_liq::RemoveLiquidityModule
    + pair_actions::swap::SwapModule
    + pair_actions::views::ViewsModule
    + pair_actions::common_methods::CommonMethodsModule
    + pair_hooks::change_hooks::ChangeHooksModule
    + pair_hooks::call_hook::CallHookModule
    + banned_addresses::BannedAddressModule
    + utils::UtilsModule
{
    #[init]
    fn init(
        &self,
        first_token_id: TokenIdentifier,
        second_token_id: TokenIdentifier,
        router_address: ManagedAddress,
        router_owner_address: ManagedAddress,
        total_fee_percent: Percent,
        special_fee_percent: Percent,
        initial_liquidity_adder: ManagedAddress,
        admins: MultiValueEncoded<ManagedAddress>,
    ) {
        require!(first_token_id.is_valid_esdt_identifier(), ERROR_NOT_AN_ESDT);
        require!(
            second_token_id.is_valid_esdt_identifier(),
            ERROR_NOT_AN_ESDT
        );
        require!(first_token_id != second_token_id, ERROR_SAME_TOKENS);

        self.set_fee_percents(total_fee_percent, special_fee_percent);
        self.state().set(State::Inactive);

        self.router_address().set(&router_address);
        self.first_token_id().set(first_token_id);
        self.second_token_id().set(second_token_id);

        let initial_liquidity_adder_opt = if !initial_liquidity_adder.is_zero() {
            Some(initial_liquidity_adder)
        } else {
            None
        };
        self.initial_liquidity_adder()
            .set(&initial_liquidity_adder_opt);

        self.add_permissions(router_address, Permissions::OWNER | Permissions::PAUSE);
        self.add_permissions(
            router_owner_address,
            Permissions::OWNER | Permissions::PAUSE,
        );
        self.add_permissions_for_all(admins, Permissions::ADMIN);

        let sc_address = self.blockchain().get_sc_address();
        self.banned_addresses().add(&sc_address);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(setLpTokenIdentifier)]
    fn set_lp_token_identifier(&self, token_identifier: TokenIdentifier) {
        self.require_caller_has_owner_permissions();

        require!(
            self.lp_token_identifier().is_empty(),
            ERROR_LP_TOKEN_NOT_ISSUED
        );
        require!(
            token_identifier != self.first_token_id().get()
                && token_identifier != self.second_token_id().get(),
            ERROR_LP_TOKEN_SAME_AS_POOL_TOKENS
        );
        require!(
            token_identifier.is_valid_esdt_identifier(),
            ERROR_NOT_AN_ESDT
        );

        self.lp_token_identifier().set(token_identifier);
    }
}
