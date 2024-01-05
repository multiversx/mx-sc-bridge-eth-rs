// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           20
// Async Callback (empty):               1
// Total number of exported functions:  22

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    multi_transfer_esdt
    (
        init => init
        upgrade => upgrade
        batchTransferEsdtToken => batch_transfer_esdt_token
        getAndClearFirstRefundBatch => get_and_clear_first_refund_batch
        setWrappingContractAddress => set_wrapping_contract_address
        setBridgeProxyContractAddress => set_bridge_proxy_contract_address
        setEsdtSafeContractAddress => set_esdt_safe_contract_address
        getFailedTxFromBridgeProxy => get_failed_tx_from_bridge_proxy
        getWrappingContractAddress => wrapping_contract_address
        getBridgeProxyContractAddress => bridge_proxy_contract_address
        getEsdtSafeContractAddress => esdt_safe_contract_address
        setMaxTxBatchSize => set_max_tx_batch_size
        setMaxTxBatchBlockDuration => set_max_tx_batch_block_duration
        getCurrentTxBatch => get_current_tx_batch
        getFirstBatchAnyStatus => get_first_batch_any_status
        getBatch => get_batch
        getBatchStatus => get_batch_status
        getFirstBatchId => first_batch_id
        getLastBatchId => last_batch_id
        setMaxBridgedAmount => set_max_bridged_amount
        getMaxBridgedAmount => max_bridged_amount
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
