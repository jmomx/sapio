use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(remote = "bitcoin::Network")]
pub enum NetworkDef {
    /// Classic Bitcoin
    Bitcoin,
    /// Bitcoin's testnet
    Testnet,
    /// Bitcoin's signet
    Signet,
    /// Bitcoin's regtest
    Regtest,
}

/// # Arguments For Creating this Contract
/// Provide this information to create an instance of a contract
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateArgs<S> {
    /// # The Main Contract Arguments
    pub arguments: S,
    #[serde(with = "NetworkDef")]
    /// # The Network the contract should be created for.
    pub network: bitcoin::Network,
    #[serde(with = "bitcoin::util::amount::serde::as_btc")]
    #[schemars(with = "f64")]
    /// # The Amount of Funds Available to the Contract as Bitcoin.
    pub amount: bitcoin::util::amount::Amount,
}
