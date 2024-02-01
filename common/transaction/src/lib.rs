#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use eth_address::EthAddress;
use multiversx_sc::codec::{EncodeErrorHandler, NestedDecodeInput, TopEncodeOutput};
// use multiversx_sc::codec::{DecodeErrorHandler, EncodeErrorHandler, NestedDecodeInput, TopDecodeInput, TopEncodeOutput};

pub mod transaction_status;

// revert protection
pub const MIN_BLOCKS_FOR_FINALITY: u64 = 10;
pub const TX_MULTIRESULT_NR_FIELDS: usize = 6;

pub type TxNonce = u64;
pub type BlockNonce = u64;
pub type SenderAddressRaw<M> = ManagedBuffer<M>;
pub type ReceiverAddressRaw<M> = ManagedBuffer<M>;
pub type TxAsMultiValue<M> = MultiValue6<
    BlockNonce,
    TxNonce,
    SenderAddressRaw<M>,
    ReceiverAddressRaw<M>,
    TokenIdentifier<M>,
    BigUint<M>,
>;
pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub type TxBatchSplitInFields<M> = MultiValue2<u64, MultiValueEncoded<M, TxAsMultiValue<M>>>;

#[derive(NestedEncode, NestedDecode, TypeAbi, Clone, ManagedVecItem)]
pub struct EthTransaction<M: ManagedTypeApi> {
    pub from: EthAddress<M>,
    pub to: ManagedAddress<M>,
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub tx_nonce: TxNonce,
    pub data: ManagedBuffer<M>,
    pub gas_limit: u64,
    pub args: ManagedVec<M, ManagedBuffer<M>>,
}

impl<M: ManagedTypeApi> TopEncode for EthTransaction<M> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        let mut nested_buffer = output.start_nested_encode();
        self.from.dep_encode_or_handle_err(&mut nested_buffer, h)?;
        self.to.dep_encode_or_handle_err(&mut nested_buffer, h)?;
        self.token_id
            .dep_encode_or_handle_err(&mut nested_buffer, h)?;
        self.amount
            .dep_encode_or_handle_err(&mut nested_buffer, h)?;
        self.tx_nonce
            .dep_encode_or_handle_err(&mut nested_buffer, h)?;
        self.data.dep_encode_or_handle_err(&mut nested_buffer, h)?;
        self.gas_limit
            .dep_encode_or_handle_err(&mut nested_buffer, h)?;
        for arg in &self.args {
            arg.dep_encode_or_handle_err(&mut nested_buffer, h)?;
        }
        output.finalize_nested_encode(nested_buffer);
        Result::Ok(())
    }
}

impl<M: ManagedTypeApi> TopDecode for EthTransaction<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let mut nested_buffer = input.into_nested_buffer();
        let from = EthAddress::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let to = ManagedAddress::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let token_id = TokenIdentifier::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let amount = BigUint::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let tx_nonce = TxNonce::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let data = ManagedBuffer::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let gas_limit = u64::dep_decode_or_handle_err(&mut nested_buffer, h)?;
        let mut args = ManagedVec::new();

        while !nested_buffer.is_depleted() {
            args.push(ManagedBuffer::dep_decode_or_handle_err(
                &mut nested_buffer,
                h,
            )?);
        }

        Result::Ok(EthTransaction {
            from,
            to,
            token_id,
            amount,
            tx_nonce,
            data,
            gas_limit,
            args,
        })
    }
}

// impl<M: ManagedTypeApi> codec::TopEncode for EthTransaction<M> {
//     fn top_encode_or_handle_err<O, H>(
//         &self,
//         output: O,
//         h: H,
//     ) -> core::result::Result<(), H::HandledErr>
//     where
//         O: codec::TopEncodeOutput,
//         H: codec::EncodeErrorHandler,
//     {
//         self {

//         }
//         match self.args. {
//             TokenMapperState::NotSet => codec::TopEncode::top_encode_or_handle_err(&"", output, h),
//             TokenMapperState::Pending => {
//                 codec::TopEncode::top_encode_or_handle_err(&"pending", output, h)
//             },
//             TokenMapperState::Token(token) => {
//                 codec::TopEncode::top_encode_or_handle_err(&token, output, h)
//             },
//         }
//     }
// }

// impl<M: ManagedTypeApi> TopDecode for EthTransaction<M> {
//     fn top_decode_or_handle_err<I, H>(top_input: I, h: H) -> Result<Self, H::HandledErr>
//     where
//         I: codec::TopDecodeInput,
//         H: codec::DecodeErrorHandler,
//     {
//         let mut nested_buffer = top_input.into_nested_buffer();
//         let result = Self::dep_decode_or_handle_err(&mut nested_buffer, h)?;
//         if !codec::NestedDecodeInput::is_depleted(&nested_buffer) {
//             return Err(h.handle_error(codec::DecodeError::INPUT_TOO_LONG));
//         }
//         Ok(result)
//     }
// }

// impl<M: ManagedTypeApi> codec::TopDecode for EthTransaction<M> {
//     fn top_decode_or_handle_err<I, H>(input: I, h: H) -> core::result::Result<Self, H::HandledErr>
//     where
//         I: codec::TopDecodeInput,
//         H: codec::DecodeErrorHandler,
//     {
//         let decoded_input = ManagedBuffer::top_decode_or_handle_err(input, h)?;
//         if decoded_input.is_empty() {
//             Ok(TokenMapperState::NotSet)
//         } else if decoded_input == PENDING_ENCODING {
//             Ok(TokenMapperState::Pending)
//         } else {
//             let token_id = TokenIdentifier::from_esdt_bytes(decoded_input);
//             Ok(TokenMapperState::Token(token_id))
//         }
//     }
// }

// impl<T: NestedDecode, M: ManagedTypeApi> TopDecode for EthTransaction<M> {
//     fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
//     where
//         I: codec::TopDecodeInput,
//         H: codec::DecodeErrorHandler,
//     {
//         let mut result: EthTransaction<T, M> = EthTransaction::new();
//         let mut nested_arg = input.into_nested_buffer();
//         while !nested_arg.is_depleted() {
//             if let Err(capacity_error) =
//                 result.try_push(T::dep_decode_or_handle_err(&mut nested_arg, h)?)
//             {
//                 return Err(h.handle_error(DecodeError::from(capacity_error)));
//             }
//         }
//         if !nested_arg.is_depleted() {
//             if let Err(capacity_error) =
//                 result.try_push(T::dep_decode_or_handle_err(&mut nested_arg, h)?)
//             {
//                 return Err(h.handle_error(DecodeError::from(capacity_error)));
//             }
//         }
//         Ok(result)
//     }

//     fn top_decode<I>(input: I) -> Result<Self, DecodeError>
//     where
//         I: TopDecodeInput,
//     {
//         Self::top_decode_or_handle_err(input, codec::DefaultErrorHandler)
//     }
// }

pub type EthTxAsMultiValue<M> = MultiValue8<
    EthAddress<M>,
    ManagedAddress<M>,
    TokenIdentifier<M>,
    BigUint<M>,
    TxNonce,
    ManagedBuffer<M>,
    u64,
    ManagedVec<M, ManagedBuffer<M>>,
>;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct EthTransactionPayment<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
    pub eth_tx: EthTransaction<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct Transaction<M: ManagedTypeApi> {
    pub block_nonce: BlockNonce,
    pub nonce: TxNonce,
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
    pub token_identifier: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub is_refund_tx: bool,
}

impl<M: ManagedTypeApi> From<TxAsMultiValue<M>> for Transaction<M> {
    fn from(tx_as_multiresult: TxAsMultiValue<M>) -> Self {
        let (block_nonce, nonce, from, to, token_identifier, amount) =
            tx_as_multiresult.into_tuple();

        Transaction {
            block_nonce,
            nonce,
            from,
            to,
            token_identifier,
            amount,
            is_refund_tx: false,
        }
    }
}

impl<M: ManagedTypeApi> Transaction<M> {
    pub fn into_multiresult(self) -> TxAsMultiValue<M> {
        (
            self.block_nonce,
            self.nonce,
            self.from,
            self.to,
            self.token_identifier,
            self.amount,
        )
            .into()
    }
}
