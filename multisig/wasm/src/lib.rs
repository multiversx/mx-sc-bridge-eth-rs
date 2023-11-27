// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           63
// Async Callback (empty):               1
// Total number of exported functions:  65

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    multisig
    (
        init => init
        distributeFeesFromChildContracts => distribute_fees_from_child_contracts
        stake => stake
        unstake => unstake
        proposeEsdtSafeSetCurrentTransactionBatchStatus => propose_esdt_safe_set_current_transaction_batch_status
        proposeMultiTransferEsdtBatch => propose_multi_transfer_esdt_batch
        moveRefundBatchToSafe => move_refund_batch_to_safe
        performAction => perform_action_endpoint
        sign => sign
        upgradeChildContractFromSource => upgrade_child_contract_from_source
        addBoardMember => add_board_member_endpoint
        removeUser => remove_user
        slashBoardMember => slash_board_member
        changeQuorum => change_quorum
        addMapping => add_mapping
        clearMapping => clear_mapping
        pauseEsdtSafe => pause_esdt_safe
        unpauseEsdtSafe => unpause_esdt_safe
        changeFeeEstimatorContractAddress => change_fee_estimator_contract_address
        changeElrondToEthGasLimit => change_elrond_to_eth_gas_limit
        changeDefaultPricePerGasUnit => change_default_price_per_gas_unit
        changeTokenTicker => change_token_ticker
        esdtSafeAddTokenToWhitelist => esdt_safe_add_token_to_whitelist
        setMultiTransferOnEsdtSafe => set_multi_transfer_on_esdt_safe
        setEsdtSafeOnMultiTransfer => set_esdt_safe_on_multi_transfer
        esdtSafeRemoveTokenFromWhitelist => esdt_safe_remove_token_from_whitelist
        esdtSafeSetMaxTxBatchSize => esdt_safe_set_max_tx_batch_size
        esdtSafeSetMaxTxBatchBlockDuration => esdt_safe_set_max_tx_batch_block_duration
        esdtSafeSetMaxBridgedAmountForToken => esdt_safe_set_max_bridged_amount_for_token
        multiTransferEsdtSetMaxBridgedAmountForToken => multi_transfer_esdt_set_max_bridged_amount_for_token
        multiTransferEsdtSetMaxRefundTxBatchSize => multi_transfer_esdt_set_max_refund_tx_batch_size
        multiTransferEsdtSetMaxRefundTxBatchBlockDuration => multi_transfer_esdt_set_max_refund_tx_batch_block_duration
        multiTransferEsdtSetWrappingContractAddress => multi_transfer_esdt_set_wrapping_contract_address
        getQuorum => quorum
        getNumBoardMembers => num_board_members
        getRequiredStakeAmount => required_stake_amount
        getAmountStaked => amount_staked
        getSlashAmount => slash_amount
        getSlashedTokensAmount => slashed_tokens_amount
        getLastExecutedEthBatchId => last_executed_eth_batch_id
        getLastExecutedEthTxId => last_executed_eth_tx_id
        getErc20AddressForTokenId => erc20_address_for_token_id
        getTokenIdForErc20Address => token_id_for_erc20_address
        getEsdtSafeAddress => esdt_safe_address
        getMultiTransferEsdtAddress => multi_transfer_esdt_address
        getCurrentTxBatch => get_current_tx_batch
        getCurrentRefundBatch => get_current_refund_batch
        wasActionExecuted => was_action_executed
        wasTransferActionProposed => was_transfer_action_proposed
        getActionIdForTransferBatch => get_action_id_for_transfer_batch
        wasSetCurrentTransactionBatchStatusActionProposed => was_set_current_transaction_batch_status_action_proposed
        getActionIdForSetCurrentTransactionBatchStatus => get_action_id_for_set_current_transaction_batch_status
        signed => signed
        userRole => user_role
        getAllBoardMembers => get_all_board_members
        getAllStakedRelayers => get_all_staked_relayers
        getActionSignerCount => get_action_signer_count
        getActionValidSignerCount => get_action_valid_signer_count
        quorumReached => quorum_reached
        getActionLastIndex => get_action_last_index
        getActionData => get_action_data
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
