use super::*;

pub fn ebml_header(webm : bool) -> Element {
    use super::database::Class::*;
    el(EBML, vec![
        el_uns(EBMLVersion,     1),
        el_uns(EBMLReadVersion, 1),
        el_uns(EBMLMaxIDLength,  4),
        el_uns(EBMLMaxSizeLength, 8),
        el_txt(DocType, String::from(if webm { "webm" } else {"matroska"} )),
        el_uns(DocTypeVersion, 2),
        el_uns(DocTypeReadVersion, 2),
    ])
}

pub fn cluster_with_one_simple_block(cluster_timecode: u64, block_content: Vec<u8>) -> Element {
    use super::database::Class::*;
    el(Cluster, vec![
        el_uns(Timecode, cluster_timecode),
        el_bin(SimpleBlock, block_content),
    ])
}

pub fn cluster_with_block_duration(cluster_timecode: u64, duration: u64, block_content: Vec<u8>) -> Element {
    use super::database::Class::*;
    el(Cluster, vec![
        el_uns(Timecode, cluster_timecode),
        el(BlockGroup, vec![
            el_uns(BlockDuration, duration),
            el_bin(Block, block_content),
        ]),
    ])
}

pub fn video_track_entry(
                        track_number:u64,
                        track_uid:u64,
                        codec_id:String,
                        codec_private:Option<Vec<u8>>,
                        default_duration:Option<u64>,
                        width:u64,
                        height:u64,
                        display_width:u64,
                        display_height:u64) -> Element {
    use super::database::Class::*;
    el(TrackEntry, vec![
        el_uns(TrackNumber, track_number),
        el_uns(TrackUID,    track_uid),
        el_uns(TrackType,   1),
        el_txt(CodecID,     codec_id),
        match codec_private { Some(x) => el_bin(CodecPrivate, x), None => el_bin(Void, vec![]) },
        match default_duration { Some(x) => el_uns(DefaultDuration, x), None=> el_bin(Void, vec![]) },
        el(Video, vec![
            el_uns(PixelWidth,     width),
            el_uns(PixelHeight,    height),
            el_uns(DisplayWidth,   display_width),
            el_uns(DisplayHeight,  display_height),
        ]),
    ])
}


pub fn audio_track_entry(
                        track_number:u64,
                        track_uid:u64,
                        codec_id:String,
                        codec_private:Option<Vec<u8>>,
                        default_duration:Option<u64>,
                        sampling_frequency:u64,
                        channels:u64) -> Element {
    use super::database::Class::*;
    el(TrackEntry, vec![
        el_uns(TrackNumber, track_number),
        el_uns(TrackUID,    track_uid),
        el_uns(TrackType,   2),
        el_txt(CodecID,     codec_id),
        match codec_private { Some(x) => el_bin(CodecPrivate, x), None => el_bin(Void, vec![]) },
        match default_duration { Some(x) => el_uns(DefaultDuration, x), None=> el_bin(Void, vec![]) },
        el(Audio, vec![
            el_uns(SamplingFrequency,     sampling_frequency),
            el_uns(Channels,              channels),
        ]),
    ])
}

pub fn segment_info(
                    timecode_scale : u64,
                    muxing_app : String,
                    writing_app : String,
                    title : Option<String>,
                    duration: Option<f64>,
                    date: Option<i64>,
                    segment_uid: Option<Vec<u8>>) -> Element {
    use super::database::Class::*;
    el(Info, vec![
        el_uns(TimecodeScale, timecode_scale),
        el_txt(MuxingApp, muxing_app),
        el_txt(WritingApp, writing_app),
        match title { Some(x) => el_txt(Title, x), None => el_bin(Void, vec![]) },
        match segment_uid { Some(x) => el_bin(SegmentUID, x), None => el_bin(Void, vec![]) },
        match duration { Some(x) => el_flo(Duration, x), None => el_bin(Void, vec![]) },
        match date { Some(x) => el_date(DateUTC, x), None => el_bin(Void, vec![]) },
    ]) 
}

pub fn matroska_file( segment_info: Element, tracks: Vec<Element>, clusters: Vec<Element> ) -> Vec<u8>
{
    use super::database::Class::*;

    let mut v = vec![];

    v.append(&mut super::generator::generate(&ebml_header(false)));
    v.append(&mut super::generator::generate(
        &mut el(Segment, {let mut x =  vec![segment_info, el(Tracks, tracks)]; x.extend(clusters); x})
        ));
    v
}
