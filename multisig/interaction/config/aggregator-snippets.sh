# 1. deployAggregator
# 2. Call submitAggregatorBatch to set gas price for eth

deployAggregator() {
    CHECK_VARIABLES AGGREGATOR_WASM CHAIN_SPECIFIC_TOKEN ALICE_ADDRESS

    erdpy --verbose contract deploy --bytecode=${AGGREGATOR_WASM} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 --arguments str:${CHAIN_SPECIFIC_TOKEN} ${ALICE_ADDRESS} 1 2 0 \
    --send --outfile=deploy-price-agregator-testnet.interaction.json --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="./deploy-price-agregator-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="./deploy-price-agregator-testnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-testnet-safe --value=${ADDRESS}
    erdpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Price agregator: ${ADDRESS}"
}

submitAggregatorBatch() {
    CHECK_VARIABLES AGGREGATOR CHAIN_SPECIFIC_TOKEN MIN_AMOUNT NR_DECIMALS
    
    MIN=$(echo "$MIN_AMOUNT*10^$NR_DECIMALS" | bc)
    erdpy --verbose contract call ${AGGREGATOR} --recall-nonce --pem=${ALICE} \
    --gas-limit=15000000 --function="submitBatch" \
    --arguments str:GWEI str:${CHAIN_SPECIFIC_TOKEN} ${MIN} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} || return
}