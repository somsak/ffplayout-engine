use std::{path::Path, sync::{Arc, Mutex},};

use simplelog::*;

use crate::utils::{sec_to_time, GlobalConfig, MediaProbe, Playlist};

pub async fn validate_playlist(playlist: Playlist, is_terminated: Arc<Mutex<bool>>, config: GlobalConfig) {
    let date = playlist.date;
    let mut length = config.playlist.length_sec.unwrap();
    let mut begin = config.playlist.start_sec.unwrap();

    length += begin;

    debug!("validate playlist from: <yellow>{date}</>");

    for item in playlist.program.iter() {
        if *is_terminated.lock().unwrap() {
            return
        }

        if Path::new(&item.source).is_file() {
            let probe = MediaProbe::new(item.source.clone());

            if probe.format.is_none() {
                error!(
                    "No Metadata from file <b><magenta>{}</></b> at <yellow>{}</>",
                    sec_to_time(begin),
                    item.source
                );
            }
        } else {
            error!(
                "File on position <yellow>{}</> not exists: <b><magenta>{}</></b>",
                sec_to_time(begin),
                item.source
            );
        }

        begin += item.out - item.seek;
    }

    if length > begin + 1.0 {
        error!(
            "Playlist from <yellow>{date}</> not long enough, <yellow>{}</> needed!",
            sec_to_time(length - begin),
        );
    }
}
