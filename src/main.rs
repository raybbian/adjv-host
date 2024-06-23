mod linux;

use gst::prelude::*;
use gstreamer as gst;
use linux::build_network_pipeline;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    gst::init()?;

    let pipeline = build_network_pipeline().await?;

    // Start playing the pipeline
    pipeline
        .set_state(gst::State::Playing)
        .expect("Could not start the pipeline");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => break,
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error from {}: {} ({:?})",
                    msg.src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| "None".into()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Clean up
    pipeline.set_state(gst::State::Null)?;

    Ok(())
}
