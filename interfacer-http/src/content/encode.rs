use encoding::all::*;
use encoding::EncodingRef;
use std::collections::HashMap;

thread_local! {
    static ENCODER_MAP: HashMap<&'static str, EncodingRef> = encodings()
        .into_iter()
        .map(|encoder| {
            if let Some(name) = encoder.whatwg_name() {
                (name, *encoder)
            } else {
                (encoder.name(), *encoder)
            }
        })
        .collect();
}

pub fn find_encoder(encoding: &str) -> Option<EncodingRef> {
    ENCODER_MAP.with(|map| map.get(encoding).map(|encoder| *encoder))
}
