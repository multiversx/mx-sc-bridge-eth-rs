// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    bridged_tokens_wrapper
    (
        addWrappedToken
        updateWrappedToken
        removeWrappedToken
        whitelistToken
        updateWhitelistedToken
        blacklistToken
        depositLiquidity
        wrapTokens
        unwrapToken
        getUniversalBridgedTokenIds
        getTokenLiquidity
        getChainSpecificToUniversalMapping
        getchainSpecificTokenIds
        pause
        unpause
        isPaused
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
