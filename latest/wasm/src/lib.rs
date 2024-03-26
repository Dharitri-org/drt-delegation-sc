// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           79
// Async Callback:                       1
// Total number of exported functions:  81

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    delegation_latest
    (
        init => init
        version => version
        getNumNodes => num_nodes
        getNodeId => get_node_id
        getNodeSignature => get_node_signature_endpoint
        getNodeState => get_node_state_endpoint
        getAllNodeStates => get_all_node_states
        getNodeBlockNonceOfUnstake => get_node_bl_nonce_of_unstake_endpoint
        addNodes => add_nodes
        removeNodes => remove_nodes
        getUserId => get_user_id
        getUserAddress => get_user_address
        getNumUsers => get_num_users
        updateUserAddress => update_user_address
        userIdsWithoutAddress => user_ids_without_address
        fundById => fund_by_id
        totalStake => get_total_stake
        getUserStake => get_user_total_stake_endpoint
        getUserWithdrawOnlyStake => get_user_withdraw_only_stake
        getUserWaitingStake => get_user_waiting_stake
        getUserActiveStake => get_user_active_stake
        getUserUnstakedStake => get_user_unstaked_stake
        getUserDeferredPaymentStake => get_user_deferred_payment_stake
        getTotalWithdrawOnlyStake => get_total_withdraw_only_stake
        getTotalWaitingStake => get_total_waiting_stake
        getTotalActiveStake => get_total_active_stake
        getTotalUnstakedStake => get_total_unstaked_stake
        getTotalDeferredPaymentStake => get_total_deferred_payment_stake
        getUserStakeByType => get_user_stake_by_type_endpoint
        getTotalStakeByType => get_total_stake_by_type_endpoint
        getAllUserStakeByType => get_all_user_stake_by_type
        getUserDeferredPaymentList => get_user_deferred_payment_list
        getFullWaitingList => get_full_waiting_list
        getFullActiveList => get_full_active_list
        stakeNodes => stake_nodes
        unStakeNodes => unstake_nodes_endpoint
        unStakeNodesAndTokens => unstake_nodes_and_tokens_endpoint
        forceNodeUnBondPeriod => force_node_unbond_period
        unBondNodes => unbond_specific_nodes_endpoint
        unBondAllPossibleNodes => unbond_all_possible_nodes
        claimUnusedFunds => claim_unused_funds
        unJailNodes => unjail_nodes
        unStakeTokens => unstake_tokens
        unBondTokens => unbond_tokens
        getAuctionContractAddress => get_auction_contract_address
        getServiceFee => get_service_fee
        getTotalDelegationCap => get_total_delegation_cap
        isBootstrapMode => is_bootstrap_mode
        getOwnerMinStakeShare => get_owner_min_stake_share
        getNumBlocksBeforeUnBond => get_n_blocks_before_unbond
        setNumBlocksBeforeUnBond => set_n_blocks_before_unbond_endpoint
        getMinimumStake => get_minimum_stake
        setMinimumStake => set_minimum_stake_endpoint
        getGlobalOperationCheckpoint => global_op_checkpoint
        isGlobalOperationInProgress => is_global_op_in_progress
        getTotalCumulatedRewards => get_total_cumulated_rewards
        getClaimableRewards => get_claimable_rewards
        getTotalUnclaimedRewards => get_total_unclaimed_rewards
        getTotalUnProtected => total_unprotected
        validateOwnerStakeShare => validate_owner_stake_share
        validateDelegationCapInvariant => validate_delegation_cap_invariant
        continueGlobalOperation => continue_global_operation_endpoint
        modifyTotalDelegationCap => modify_total_delegation_cap
        setServiceFee => set_service_fee_endpoint
        claimRewards => claim_rewards
        stake => stake_endpoint
        unStake => unstake_endpoint
        getUnStakeable => get_unstakeable
        unBond => unbond_user
        getUnBondable => get_unbondable
        dustCleanupCheckpoint => dust_cleanup_checkpoint
        countDustItemsWaitingList => count_dust_items_waiting_list
        countDustItemsActive => count_dust_items_active
        dustCleanupWaitingList => dust_cleanup_waiting_list
        dustCleanupActive => dust_cleanup_active
        dnsRegister => dns_register
        setFeatureFlag => set_feature_flag_endpoint
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

dharitri_sc_wasm_adapter::async_callback! { delegation_latest }
