#[cfg(feature = "liquid")]
use elements::address as elements_address;

use crate::chain::{script, Network, Script, TxIn, TxOut};
use script::Instruction::PushBytes;

pub struct InnerScripts {
    pub redeem_script: Option<Script>,
    pub witness_script: Option<Script>,
}

pub trait ScriptToAsm: std::fmt::Debug {
    fn to_asm(&self) -> String {
        let asm = format!("{:?}", self);
        (&asm[7..asm.len() - 1]).to_string()
    }
}
impl ScriptToAsm for bitcoin::Script {}
#[cfg(feature = "liquid")]
impl ScriptToAsm for elements::Script {}

pub trait ScriptToAddr {
    fn to_address_str(&self, network: Network) -> Option<String>;
}
#[cfg(not(feature = "liquid"))]
impl ScriptToAddr for bitcoin::Script {
    fn to_address_str(&self, network: Network) -> Option<String> {
        bitcoin::Address::from_script(self, network.into()).map(|s| s.to_string())
    }
}
#[cfg(feature = "liquid")]
impl ScriptToAddr for elements::Script {
    fn to_address_str(&self, network: Network) -> Option<String> {
        elements_address::Address::from_script(self, None, network.address_params())
            .map(|a| a.to_string())
    }
}

// Returns the witnessScript in the case of p2wsh, or the redeemScript in the case of p2sh.
pub fn get_innerscripts(txin: &TxIn, prevout: &TxOut) -> InnerScripts {
    // Wrapped redeemScript for P2SH spends
    let redeem_script = if prevout.script_pubkey.is_p2sh() {
        if let Some(Ok(PushBytes(redeemscript))) = txin.script_sig.instructions().last() {
            Some(Script::from(redeemscript.to_vec()))
        } else {
            None
        }
    } else {
        None
    };

    // Wrapped witnessScript for P2WSH or P2SH-P2WSH spends
    let witness_script = if prevout.script_pubkey.is_v0_p2wsh()
        || redeem_script.as_ref().map_or(false, |s| s.is_v0_p2wsh())
    {
        let witness = &txin.witness;
        #[cfg(feature = "liquid")]
        let witness = &witness.script_witness;
        witness.iter().last().cloned().map(Script::from)
    } else {
        None
    };

    InnerScripts {
        redeem_script,
        witness_script,
    }
}
