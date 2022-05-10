use dbus::{blocking::{Connection, stdintf::org_freedesktop_dbus::Properties}, arg};
use std::time::Duration;

use crate::BarModule;

// TODO
// IMPORTANT NOTE: this module is currently very experimental.
// One issue I've found is that the constant dbus reconnections and get requests
// make it absolutely flood dbus-monitor. A possible solution would involve
// some kind of a cache system to at least keep the connection open instead
// of reopening it every n seconds (or maybe its just fine?).

const ICON: &str = "";
const ICON_COLORED: &str = "^c#1ed760^^d^";

pub struct Spotify {
    pub playing_format: &'static str,
    pub idle_string: &'static str,
    pub crop_threshold: usize,
    pub update_interval: u32,
}

struct SpotifyMeta {
    artist: String,
    album: String,
    title: String,
}

impl Spotify {
    fn get_metadata(&self) -> SpotifyMeta {
	let dbus_conn = match Connection::new_session() {
	    Ok(dbus_conn) => dbus_conn,
	    Err(e) => panic!("dbus error - {}", e),
	};
	let dbus_proxy = dbus_conn.with_proxy(
	    "org.mpris.MediaPlayer2.spotify",
	    "/org/mpris/MediaPlayer2",
	    Duration::from_millis(5000));
	let metadata: arg::PropMap = match dbus_proxy.get(
	    "org.mpris.MediaPlayer2.Player",
	    "Metadata") 
	{
	    Ok(meta) => meta,
	    Err(_) => return SpotifyMeta { artist: "..".into(), album: "..".into(), title: "..".into() }
	};

	let artist: Option<&Vec<String>> = arg::prop_cast(&metadata, "xesam:artist");
	let album: Option<&String> = arg::prop_cast(&metadata, "xesam:album");
	let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");

	let process_meta = |item: Option<&String>| -> String {
	    match item {
		Some(item) => item.to_owned(),
		None => "..".to_owned(),
	    }
	};

	SpotifyMeta {
	    artist: process_meta(Some(&artist.unwrap()[0])),
	    album: process_meta(album),
	    title: process_meta(title),
	}
    }
    fn spotify_eval_keywords(&self, keywords: Vec<&str>, meta: SpotifyMeta) -> Vec<String> {
	
	let evaled_keywords: Vec<String> = keywords.into_iter()
	    .map(|keyword| {
		match keyword {
		    "icon" => String::from(ICON),
		    "icon_colored" => String::from(ICON_COLORED),
		    "artist" => meta.artist.clone(),
		    "album" => meta.album.clone(),
		    "title" => meta.title.clone(),
		    _ => keyword.to_string()
		}
	    }).collect();

	evaled_keywords
    }
}

impl BarModule for Spotify {
    fn parse_format(&self, format: String) -> String {
	let keywords: Vec<&str> = format.split(&['{', '}']).collect();
	let has_colored_logo = keywords.contains(&"icon_colored");
	let meta = self.get_metadata();
	if meta.title == ".." {
	    return String::from(self.idle_string);
	}
	let keywords = self.spotify_eval_keywords(keywords, meta);

	let mut ret = String::new();
	keywords.into_iter()
	    .for_each(|keyword| ret.push_str(keyword.as_str()));

	if ret.len() > self.crop_threshold {
	    if has_colored_logo {
		ret = String::from(&ret[..self.crop_threshold + ICON_COLORED.len() - 2]) + "..";
	    } else {
		ret = String::from(&ret[..self.crop_threshold]);
	    }
	}

	ret
    }
    
    fn get_value(&self) -> String {
        self.parse_format(self.playing_format.to_string())
    }
    
    fn get_timer(&self) -> u32 {
	self.update_interval
    }
}
