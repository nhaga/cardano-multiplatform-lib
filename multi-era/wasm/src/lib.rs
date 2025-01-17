#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
pub mod allegra;
pub mod alonzo;
pub mod byron;
pub mod mary;
pub mod shelley;
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::{
    allegra::{
        AllegraAuxiliaryData, AllegraBlock, AllegraTransactionBody, AllegraTransactionWitnessSet,
    },
    alonzo::{
        AlonzoAuxiliaryData, AlonzoBlock, AlonzoTransactionBody, AlonzoTransactionOutput,
        AlonzoTransactionWitnessSet,
    },
    byron::block::ByronBlock,
    mary::{MaryBlock, MaryTransactionBody},
    shelley::{
        MultisigScript, ShelleyBlock, ShelleyTransactionBody, ShelleyTransactionOutput,
        ShelleyTransactionWitnessSet,
    },
};
use cml_chain_wasm::{
    block::Block, certs::StakeCredential, transaction::ShelleyTxOut, Coin, StakeCredentialList,
    TransactionIndex,
};
use cml_core_wasm::{
    impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_list, impl_wasm_map,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

impl_wasm_list!(
    cml_multi_era::allegra::AllegraTransactionBody,
    AllegraTransactionBody,
    AllegraTransactionBodyList
);

impl_wasm_list!(
    cml_multi_era::allegra::AllegraTransactionWitnessSet,
    AllegraTransactionWitnessSet,
    AllegraTransactionWitnessSetList
);

impl_wasm_list!(
    cml_multi_era::alonzo::AlonzoTransactionBody,
    AlonzoTransactionBody,
    AlonzoTransactionBodyList
);

impl_wasm_list!(
    cml_multi_era::alonzo::AlonzoTransactionWitnessSet,
    AlonzoTransactionWitnessSet,
    AlonzoTransactionWitnessSetList
);

impl_wasm_list!(
    cml_multi_era::alonzo::AlonzoTransactionOutput,
    AlonzoTransactionOutput,
    AlonzoTransactionOutputList
);

impl_wasm_map!(
    cml_chain::certs::StakeCredential,
    cml_chain::assets::Coin,
    StakeCredential,
    Coin,
    StakeCredentialList,
    MapStakeCredentialToCoin,
    false,
    true,
    false,
    true
);

impl_wasm_map!(
    cml_chain::TransactionIndex,
    cml_multi_era::allegra::AllegraAuxiliaryData,
    TransactionIndex,
    AllegraAuxiliaryData,
    Vec<TransactionIndex>,
    MapTransactionIndexToAllegraAuxiliaryData,
    true,
    false,
    true,
    false
);

impl_wasm_map!(
    cml_chain::TransactionIndex,
    cml_multi_era::alonzo::AlonzoAuxiliaryData,
    TransactionIndex,
    AlonzoAuxiliaryData,
    Vec<TransactionIndex>,
    MapTransactionIndexToAlonzoAuxiliaryData,
    true,
    false,
    true,
    false
);

impl_wasm_list!(
    cml_multi_era::mary::MaryTransactionBody,
    MaryTransactionBody,
    MaryTransactionBodyList
);

impl_wasm_list!(
    cml_multi_era::shelley::MultisigScript,
    MultisigScript,
    MultisigScriptList
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiEraBlock(cml_multi_era::MultiEraBlock);

impl_wasm_cbor_json_api!(MultiEraBlock);

impl_wasm_conversions!(cml_multi_era::MultiEraBlock, MultiEraBlock);

#[wasm_bindgen]
impl MultiEraBlock {
    pub fn new_byron(byron: &ByronBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_byron(
            byron.clone().into(),
        ))
    }

    pub fn new_shelley(shelley: &ShelleyBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_allegra(allegra: &AllegraBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_allegra(
            allegra.clone().into(),
        ))
    }

    pub fn new_mary(mary: &MaryBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_mary(mary.clone().into()))
    }

    pub fn new_alonzo(alonzo: &AlonzoBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_alonzo(
            alonzo.clone().into(),
        ))
    }

    pub fn new_babbage(babbage: &Block) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_babbage(
            babbage.clone().into(),
        ))
    }

    pub fn kind(&self) -> MultiEraBlockKind {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Byron(_) => MultiEraBlockKind::Byron,
            cml_multi_era::MultiEraBlock::Shelley(_) => MultiEraBlockKind::Shelley,
            cml_multi_era::MultiEraBlock::Allegra(_) => MultiEraBlockKind::Allegra,
            cml_multi_era::MultiEraBlock::Mary(_) => MultiEraBlockKind::Mary,
            cml_multi_era::MultiEraBlock::Alonzo(_) => MultiEraBlockKind::Alonzo,
            cml_multi_era::MultiEraBlock::Babbage(_) => MultiEraBlockKind::Babbage,
        }
    }

    pub fn as_byron(&self) -> Option<ByronBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Byron(byron) => Some(byron.clone().into()),
            _ => None,
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Shelley(shelley) => Some(shelley.clone().into()),
            _ => None,
        }
    }

    pub fn as_allegra(&self) -> Option<AllegraBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Allegra(allegra) => Some(allegra.clone().into()),
            _ => None,
        }
    }

    pub fn as_mary(&self) -> Option<MaryBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Mary(mary) => Some(mary.clone().into()),
            _ => None,
        }
    }

    pub fn as_alonzo(&self) -> Option<AlonzoBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Alonzo(alonzo) => Some(alonzo.clone().into()),
            _ => None,
        }
    }

    pub fn as_babbage(&self) -> Option<Block> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Babbage(babbage) => Some(babbage.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum MultiEraBlockKind {
    Byron,
    Shelley,
    Allegra,
    Mary,
    Alonzo,
    Babbage,
}

impl_wasm_list!(
    cml_multi_era::shelley::ShelleyTransactionBody,
    ShelleyTransactionBody,
    ShelleyTransactionBodyList
);

impl_wasm_list!(
    cml_multi_era::shelley::ShelleyTransactionOutput,
    ShelleyTransactionOutput,
    ShelleyTransactionOutputList
);

impl_wasm_list!(
    cml_multi_era::shelley::ShelleyTransactionWitnessSet,
    ShelleyTransactionWitnessSet,
    ShelleyTransactionWitnessSetList
);

impl_wasm_list!(
    cml_chain::transaction::ShelleyTxOut,
    ShelleyTxOut,
    ShelleyTxOutList
);
