#[macro_use]
extern crate neon;

extern crate rodio;

use std::io::BufReader;
use neon::prelude::*;
use std::path::Path;

fn play(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let music_library = "/Users/miika.henttonen/repos/musa-rust/examples";
    let song = cx.argument::<JsString>(0)?.value();
    let path_to_song = Path::new(&music_library).join(song);

    let file = std::fs::File::open(path_to_song).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("play", play)
});
