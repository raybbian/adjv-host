mod pipewire;

use gst::prelude::*;
use gstreamer as gst;
use pipewire::get_capture_node_id;
use std::error::Error;

pub async fn build_network_pipeline() -> Result<gst::Pipeline, Box<dyn Error>> {
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
