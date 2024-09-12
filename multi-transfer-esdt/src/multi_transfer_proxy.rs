// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct MultiTransferEsdtProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for MultiTransferEsdtProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = MultiTransferEsdtProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        MultiTransferEsdtProxyMethods { wrapped_tx: tx }
    }
}

pub struct MultiTransferEsdtProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> MultiTransferEsdtProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> MultiTransferEsdtProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> MultiTransferEsdtProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn batch_transfer_esdt_token<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, transaction::EthTransaction<Env::Api>>>,
    >(
        self,
        batch_id: Arg0,
        transfers: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("batchTransferEsdtToken")
            .argument(&batch_id)
            .argument(&transfers)
            .original_result()
    }

    pub fn move_refund_batch_to_safe(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("moveRefundBatchToSafe")
            .original_result()
    }

    pub fn set_wrapping_contract_address<
        Arg0: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_new_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setWrappingContractAddress")
            .argument(&opt_new_address)
            .original_result()
    }

    pub fn set_bridge_proxy_contract_address<
        Arg0: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_new_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setBridgeProxyContractAddress")
            .argument(&opt_new_address)
            .original_result()
    }

    pub fn set_esdt_safe_contract_address<
        Arg0: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_new_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setEsdtSafeContractAddress")
            .argument(&opt_new_address)
            .original_result()
    }

    pub fn wrapping_contract_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getWrappingContractAddress")
            .original_result()
    }

    pub fn bridge_proxy_contract_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBridgeProxyContractAddress")
            .original_result()
    }

    pub fn esdt_safe_contract_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getEsdtSafeContractAddress")
            .original_result()
    }

    pub fn set_max_tx_batch_size<
        Arg0: ProxyArg<usize>,
    >(
        self,
        new_max_tx_batch_size: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setMaxTxBatchSize")
            .argument(&new_max_tx_batch_size)
            .original_result()
    }

    pub fn set_max_tx_batch_block_duration<
        Arg0: ProxyArg<u64>,
    >(
        self,
        new_max_tx_batch_block_duration: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setMaxTxBatchBlockDuration")
            .argument(&new_max_tx_batch_block_duration)
            .original_result()
    }

    pub fn get_current_tx_batch(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue2<u64, MultiValueEncoded<Env::Api, MultiValue6<u64, u64, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>>>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCurrentTxBatch")
            .original_result()
    }

    pub fn get_first_batch_any_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue2<u64, MultiValueEncoded<Env::Api, MultiValue6<u64, u64, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>>>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFirstBatchAnyStatus")
            .original_result()
    }

    pub fn get_batch<
        Arg0: ProxyArg<u64>,
    >(
        self,
        batch_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue2<u64, MultiValueEncoded<Env::Api, MultiValue6<u64, u64, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>>>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBatch")
            .argument(&batch_id)
            .original_result()
    }

    pub fn get_batch_status<
        Arg0: ProxyArg<u64>,
    >(
        self,
        batch_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, tx_batch_module::batch_status::BatchStatus<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBatchStatus")
            .argument(&batch_id)
            .original_result()
    }

    pub fn first_batch_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFirstBatchId")
            .original_result()
    }

    pub fn last_batch_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLastBatchId")
            .original_result()
    }

    pub fn set_max_bridged_amount<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        max_amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setMaxBridgedAmount")
            .argument(&token_id)
            .argument(&max_amount)
            .original_result()
    }

    pub fn max_bridged_amount<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMaxBridgedAmount")
            .argument(&token_id)
            .original_result()
    }
}
