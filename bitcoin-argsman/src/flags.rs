// ---------------- [ File: bitcoin-argsman/src/flags.rs ]
crate::ix!();

bitflags!{
    pub struct ArgsManagerFlags: u32 {

        /*
          | Boolean options can accept negation
          | syntax
          | 
          | -noOPTION or -noOPTION=1
          |
          */
        const ALLOW_BOOL   = 0x01;
        const ALLOW_INT    = 0x02;
        const ALLOW_STRING = 0x04;
        const ALLOW_ANY    = 
            Self::ALLOW_BOOL.bits 
            | Self::ALLOW_INT.bits 
            | Self::ALLOW_STRING.bits;

        const DEBUG_ONLY   = 0x100;

        /*
          | Some options would cause cross-contamination
          | if values for mainnet were used while
          | running on regtest/testnet (or vice-versa).
          | 
          | Setting them as NETWORK_ONLY ensures
          | that sharing a config file between mainnet
          | and regtest/testnet won't cause problems
          | due to these parameters by accident.
          |
          */
        const NETWORK_ONLY = 0x200;

        /*
          | This argument's value is sensitive
          | (such as a password).
          |
          */
        const SENSITIVE = 0x400;
        const COMMAND   = 0x800;
    }
}
