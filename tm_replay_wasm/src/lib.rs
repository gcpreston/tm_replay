use tm_replay::{HumanPort, ReplayFlags, construct_tm_replay_from_slp};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
pub fn construct_tm_replay_from_slp_in_js(
    bytes: Vec<u8>,
    human_lowport: bool, // true to control the low port, false to control the high port
    frame: usize,
    duration: usize,
    name: &str,
    flags: ReplayFlags,
) -> Result<Vec<u8>, JsValue> {
    let human = if human_lowport {
        HumanPort::HumanLowPort
    } else {
        HumanPort::HumanHighPort
    };

    let game = slp_parser::parse_file(bytes.as_slice())
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    construct_tm_replay_from_slp(&game, human, frame, duration, name, flags)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}
