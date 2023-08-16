// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           17
// Async Callback (empty):               1
// Total number of exported functions:  19

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    multi_transfer_esdt
    (
        batchTransferEsdtToken
        getAndClearFirstRefundBatch
        setWrappingContractAddress
        setBridgeProxyContractAddress
        getFailedTxFromBridgeProxy
        getWrappingContractAddress
        getBridgeProxyContractAddress
        setMaxTxBatchSize
        setMaxTxBatchBlockDuration
        getCurrentTxBatch
        getFirstBatchAnyStatus
        getBatch
        getBatchStatus
        getFirstBatchId
        getLastBatchId
        setMaxBridgedAmount
        getMaxBridgedAmount
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
