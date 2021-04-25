mod dummy;
mod win32;
mod win64;

use crate::{
    game::string::RCStr,
    gml::{self, Function, Value},
};
use cfg_if::cfg_if;
use encoding_rs::Encoding;
use serde::{Deserialize, Serialize};
use shared::dll;

pub use shared::dll::{CallConv, ValueType as DLLValueType};

cfg_if! {
    if
    #[cfg(all(target_os = "windows", target_arch = "x86"))] {
        use win32 as platform;
    } else if #[cfg(target_os = "windows")] {
        use win64 as platform;
    } else {
        use dummy as platform;
    }
}

pub enum Call {
    DummyNull(dll::ValueType),
    DummyOne,
    DllCall(platform::ExternalImpl),
    Emulated(Function),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DefineInfo {
    pub dll_name: RCStr,
    pub fn_name: RCStr,
    pub call_conv: CallConv,
    pub res_type: dll::ValueType,
    pub arg_types: Vec<dll::ValueType>,
}

pub struct External {
    call: Call,
    pub info: DefineInfo,
}

/*
Spec required for ExternalImpl {
    /// Create a new ExternalImpl with the given DefineInfo.
    /// The given encoding specifies the encoding of the strings in `info`.
    pub fn new(info: &DefineInfo, encoding: &'static Encoding) -> Result<Self, String>;
    /// Calls the ExternalImpl.
    /// Make sure the args iterator matches the function *before* calling it, as it will not be checked.
    // This takes an iterator in order to reduce the number of times the args list has to be copied.
    // Setting it up this way makes dealing with traits more annoying, so I just didn't.
    pub fn call<I: Iterator<Item = gml::Value>>(&self, args: I) -> Result<gml::Value, String>;
}
 */

impl External {
    pub fn new(info: DefineInfo, disable_sound: bool, encoding: &'static Encoding) -> Result<Self, String> {
        if info.arg_types.len() > 4 && info.arg_types.contains(&dll::ValueType::Str) {
            return Err("DLL functions with more than 4 arguments cannot have string arguments".into())
        }
        if info.arg_types.len() >= 16 {
            return Err("DLL functions can have at most 16 arguments".into())
        }
        let mut dll_name_lower = std::path::Path::new(info.dll_name.decode(encoding).as_ref())
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        dll_name_lower.make_ascii_lowercase();
        let call = match dll_name_lower.as_str() {
            "gmfmodsimple.dll" if disable_sound && info.fn_name.as_ref() == b"FMODSoundAdd" => Call::DummyOne,
            "gmfmodsimple.dll" | "ssound.dll" | "supersound.dll" | "sxms-3.dll" if disable_sound => {
                Call::DummyNull(info.res_type)
            },
            _ => Call::DllCall(platform::ExternalImpl::new(&info, encoding)?),
        };
        Ok(Self { call, info })
    }

    pub fn get_function(&self) -> Option<Function> {
        match self.call {
            Call::Emulated(f) => Some(f),
            _ => None,
        }
    }

    pub fn call(&self, args: &[Value]) -> gml::Result<Value> {
        if args.len() != self.info.arg_types.len() {
            eprintln!(
                "Warning: call to external function {} from {} with an invalid argument count was ignored",
                self.info.fn_name, self.info.dll_name
            );
            Ok(Default::default())
        } else {
            let args = args.iter().zip(&self.info.arg_types).map(|(v, t)| match t {
                dll::ValueType::Real => f64::from(v.clone()).into(),
                dll::ValueType::Str => RCStr::from(v.clone()).into(),
            });
            self.call.call(args)
        }
    }
}

impl Call {
    fn call(&self, args: impl Iterator<Item = Value>) -> gml::Result<Value> {
        match self {
            Call::DummyNull(res_type) => match res_type {
                dll::ValueType::Real => Ok(0.into()),
                dll::ValueType::Str => Ok("".into()),
            },
            Call::DummyOne => Ok(1.into()),
            Call::DllCall(call) => {
                call.call(args).map_err(|e| gml::Error::FunctionError("external_call".into(), e.into()))
            },
            _ => unimplemented!(),
        }
    }
}
