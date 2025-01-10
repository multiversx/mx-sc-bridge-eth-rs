// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    mock_multisig
    (
        init => init
        upgrade => upgrade
        getEsdtSafeAddress => esdt_safe_address
        getMultiTransferEsdtAddress => multi_transfer_esdt_address
        getProxyAddress => proxy_address
        getBridgedTokensWrapperAddress => bridged_tokens_wrapper_address
        getFeeEstimatorAddress => fee_estimator_address
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
