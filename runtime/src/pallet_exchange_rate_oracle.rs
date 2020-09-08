use crate::pallet_timestamp::Timestamp;
use crate::pallet_timestamp::TimestampEventsDecoder;
use core::marker::PhantomData;
pub use module_vault_registry::Vault;
use parity_scale_codec::Encode;
use std::fmt::Debug;
use substrate_subxt_proc_macro::{module, Store};

#[module]
pub trait ExchangeRateOracle: Timestamp {}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ExchangeRateStore<T: ExchangeRateOracle> {
    #[store(returns = u128)]
    pub _runtime: PhantomData<T>,
}

/// Last exchange rate time
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct LastExchangeRateTimeStore<T: ExchangeRateOracle> {
    #[store(returns = T::Moment)]
    pub _runtime: PhantomData<T>,
}