use std::{env, process::exit, fs::File};

use ac_ffmpeg::{format::{io::IO, demuxer::{Demuxer, DemuxerWithStreamInfo}}, Error, codec::{video::{VideoDecoder, frame::VideoFrame}, Decoder}};
use framebuffer::{Framebuffer, KdMode};

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    args
}

fn open_file_demuxer_with_stream_info(path: &str) -> Result<DemuxerWithStreamInfo<File>, Error> {
    let input = File::open(path)
        .map_err(|error| Error::new(format!("Can't open especified file {}, error: {}", path, error)))?;

    let io = IO::from_seekable_read_stream(input);

    Demuxer::builder()
        .build(io)?
        .find_stream_info(None)
        .map_err(|(_, err)| err)
}

fn start(input: &str) -> Result<(), Error> {
    let mut demuxer = open_file_demuxer_with_stream_info(input)?;

    let (stream_index, (stream, _)) = demuxer
        .streams()
        .iter()
        .map(|stream| (stream, stream.codec_parameters()))
        .enumerate()
        .find(|(_, (_, params))| params.is_video_codec())
        .ok_or_else(|| Error::new("No Video Stream"))?;

    let mut decoder = VideoDecoder::from_stream(stream)?.build()?;

    while let Some(packet) = demuxer.take()? {
        if packet.stream_index() != stream_index {
            continue;
        }

        decoder.push(packet)?;

        while let Some(frame) = decoder.take()? {
            write_frame_to_framebuffer(frame)?;
        }
    }

    decoder.flush()?;

    while let Some(frame) = decoder.take()? {
        write_frame_to_framebuffer(frame)?;
    }

    Ok(())
}

fn write_frame_to_framebuffer(frame: VideoFrame) -> Result<(), Error> {
    // TODO Write Frame to Framebuffer
    todo!("Make it write the frame to the framebuffer")
}

fn main() {
    ctrlc::set_handler(move || {
        Framebuffer::set_kd_mode(KdMode::Text).unwrap();
        exit(1);
    }).expect("Failed to set termination handler");

    let args = get_args();
    let file_arg = args.get(0).expect("Requires file path");
}
