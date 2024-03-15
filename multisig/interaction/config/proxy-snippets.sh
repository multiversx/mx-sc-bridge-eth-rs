deployBridgeProxy() {
    CHECK_VARIABLES PROXY_WASM MULTI_TRANSFER

    mxpy --verbose contract deploy --bytecode=${PROXY_WASM} --recall-nonce --pem=${ALICE} \
    --gas-limit=200000000 \
    --arguments ${MULTI_TRANSFER} \
    --send --outfile="deploy-proxy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(mxpy data parse --file="./deploy-proxy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(mxpy data parse --file="./deploy-proxy-testnet.interaction.json" --expression="data['contractAddress']")

    # mxpy data store --key=address-testnet-proxy --value=${ADDRESS}
    # mxpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Proxy contract address: ${ADDRESS}"
    update-config BRIDGE_PROXY ${ADDRESS}
}
