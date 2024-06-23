mod pipewire;

use gstreamer as gst;
use gst::prelude::*;
use pipewire::get_capture_node_id;
use std::error::Error;

async fn build_network_pipeline() -> Result<gst::Pipeline, Box<dyn Error>> {
    let pipeline = gst::Pipeline::new();

    let node_id = get_capture_node_id().await?;
    let src = gst::ElementFactory::make("pipewiresrc")
        .property("path", &format!("{}", node_id))
        .build()?;

    let h264enc = gst::ElementFactory::make("nvh264enc")
        .property_from_str("preset", &"lossless-hp")
        .property("zerolatency", &true)
        .build()?;

    let h264pay = gst::ElementFactory::make("rtph264pay")
        .property("mtu", &65507u32)
        .property("config-interval", &-1)
        .build()?;

    let sink = gst::ElementFactory::make("udpsink")
        .property("host", &"169.254.159.54")
        .property("port", &8080)
        .property("sync", &false)
        .build()?;

    pipeline.add_many(&[&src, &h264enc, &h264pay, &sink])?;
    gst::Element::link_many(&[&src, &h264enc, &h264pay, &sink])?;

    Ok(pipeline)
}

async fn build_self_pipeline() -> Result<gst::Pipeline, Box<dyn Error>> {
    let pipeline = gst::Pipeline::new();

    let node_id = get_capture_node_id().await?;
    let src = gst::ElementFactory::make("pipewiresrc")
        .property("path", &format!("{}", node_id))
        .build()?;

    let sink = gst::ElementFactory::make("fpsdisplaysink")
        .build()?;

    pipeline.add_many(&[&src, &sink])?;
    gst::Element::link_many(&[&src, &sink])?;

    Ok(pipeline)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    gst::init()?;

    let pipeline = build_self_pipeline().await?;

    // Start playing the pipeline
    pipeline.set_state(gst::State::Playing)
        .expect("Could not start the pipeline");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => break,
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error from {}: {} ({:?})",
                    msg.src().map(|s| s.path_string()).unwrap_or_else(|| "None".into()),
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


