// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           72
// Async Callback:                       1
// Total number of exported functions:  75

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    farm_staking_nft
    (
        init => init
        upgrade => upgrade
        mergeFarmTokens => merge_farm_tokens_endpoint
        calculateRewardsForGivenPosition => calculate_rewards_for_given_position
        startProduceRewards => start_produce_rewards_endpoint
        getAccumulatedRewards => accumulated_rewards
        getRewardCapacity => reward_capacity
        getAnnualPercentageRewards => max_annual_percentage_rewards
        getMinUnbondEpochs => min_unbond_epochs
        getRewardNonce => reward_nonce
        getRewardPerShare => reward_per_share
        getRewardReserve => reward_reserve
        allowExternalClaimBoostedRewards => allow_external_claim_boosted_rewards
        getAllowExternalClaimRewards => get_allow_external_claim_rewards
        getFarmingTokenId => farming_token_id
        getRewardTokenId => reward_token_id
        getPerBlockRewardAmount => per_block_reward_amount
        getLastRewardBlockNonce => last_reward_block_nonce
        getDivisionSafetyConstant => division_safety_constant
        getUserTotalFarmPosition => user_total_farm_position
        getFarmPositionMigrationNonce => farm_position_migration_nonce
        registerFarmToken => register_farm_token
        getFarmTokenId => farm_token
        getFarmTokenSupply => farm_token_supply
        addToPauseWhitelist => add_to_pause_whitelist
        removeFromPauseWhitelist => remove_from_pause_whitelist
        pause => pause
        resume => resume
        getState => state
        addAdmin => add_admin_endpoint
        removeAdmin => remove_admin_endpoint
        updateOwnerOrAdmin => update_owner_or_admin_endpoint
        getPermissions => permissions
        setBurnRoleForAddress => set_burn_role_for_address
        stakeFarm => stake_farm_endpoint
        claimRewards => claim_rewards
        compoundRewards => compound_rewards
        unstakeFarm => unstake_farm
        unbondFarm => unbond_farm
        claimBoostedRewards => claim_boosted_rewards
        setBoostedYieldsRewardsPercentage => set_boosted_yields_rewards_percentage
        collectUndistributedBoostedRewards => collect_undistributed_boosted_rewards
        getBoostedYieldsRewardsPercentage => boosted_yields_rewards_percentage
        getAccumulatedRewardsForWeek => accumulated_rewards_for_week
        getFarmSupplyForWeek => farm_supply_for_week
        getRemainingBoostedRewardsToDistribute => remaining_boosted_rewards_to_distribute
        getUndistributedBoostedRewards => undistributed_boosted_rewards
        setBoostedYieldsFactors => set_boosted_yields_factors
        getBoostedYieldsFactors => get_boosted_yields_factors
        getCurrentWeek => get_current_week
        getFirstWeekStartEpoch => first_week_start_epoch
        getLastActiveWeekForUser => get_last_active_week_for_user_view
        getUserEnergyForWeek => get_user_energy_for_week_view
        getLastGlobalUpdateWeek => last_global_update_week
        getTotalRewardsForWeek => total_rewards_for_week
        getTotalEnergyForWeek => total_energy_for_week
        getTotalLockedTokensForWeek => total_locked_tokens_for_week
        updateEnergyForUser => update_energy_for_user
        getCurrentClaimProgress => current_claim_progress
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
        addBannedAddress => add_banned_address
        removeBannedAddress => remove_banned_address
        addHook => add_hook
        removeHook => remove_hook
        registerUnbondToken => register_unbond_token
        getUnbondTokenId => unbond_token
        topUpRewards => top_up_rewards
        topUpAndSetRewardNonce => top_up_and_set_reward_nonce
        withdrawRewards => withdraw_rewards
        endProduceRewards => end_produce_rewards
        setPerBlockRewardAmount => set_per_block_rewards
        setMaxApr => set_max_apr
        setMinUnbondEpochs => set_min_unbond_epochs_endpoint
    )
}

multiversx_sc_wasm_adapter::async_callback! { farm_staking_nft }
