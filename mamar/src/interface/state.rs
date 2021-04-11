use std::{error::Error, io::Read};
use std::path::PathBuf;
use std::fs::File;

use pm64::bgm::*;

use crate::interface::form::range_select;

#[derive(Default, PartialEq, Clone)]
pub struct State {
    pub document: Option<Document>,
}

#[derive(Clone)]
pub struct Document {
    pub bgm: Bgm,
    pub path: PathBuf,

    selected_segment_idx: u8,
    selected_subsegment_idx: u8,
}

// Change of anything other than self.bgm should not be considered a History-changing action.
impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.bgm == other.bgm
    }
}

impl Document {
    /// Prompt an 'Open File' dialog to open a document. Must be run on the main thread.
    pub fn open() -> Result<Option<Self>, Box<dyn Error>> {
        let path = tinyfiledialogs::open_file_dialog("Open File", "", Some((&[
            "*.bgm",
            "*.mid",
            "*.midi",
            "*.bin",
        ], "BGM and MIDI files")));

        if let Some(path) = path {
            let path = PathBuf::from(path);
            let mut file = File::open(&path)?;

            let bgm;
            if pm64::bgm::midi::is_midi(&mut file)? {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                bgm = pm64::bgm::midi::to_bgm(&buf)?;
            } else {
                bgm = Bgm::decode(&mut file)?;
            }

            Ok(Some(Document {
                bgm,
                path,
                selected_segment_idx: 0,
                selected_subsegment_idx: 0,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn can_save(&self) -> bool {
        let ext = self.path.extension().unwrap_or_default().to_str().unwrap_or_default();

        match ext {
            "bgm" => true,
            "bin" => true,
            _ => false,
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        assert!(self.can_save()); // TODO: return Err

        let mut file = File::create(&self.path)?;
        self.bgm.encode(&mut file)?;

        Ok(())
    }

    /// Shows as 'Save As' dialog prompt then saves the document to a file. Must be run on the main thread.
    pub fn save_as(&mut self) -> Result<(), Box<dyn Error>> {
        let current_path = self.path.with_extension("bgm");

        let path = tinyfiledialogs::save_file_dialog_with_filter(
            "Save As",
            current_path.to_str().unwrap_or_default(),
            &["*.bgm"],
            "BGM",
        );

        if let Some(path) = path {
            let mut path = PathBuf::from(path);

            if path.extension().is_none() {
                path.set_extension("bgm");
            }

            std::mem::swap(&mut self.path, &mut path);
            let prev_path = path;

            if self.can_save() {
                self.save()
            } else {
                self.path = prev_path;
                // TODO: probably error
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub fn update(&mut self, ui: &mut imui_glium::UiFrame<'_>) {
        ui.vbox(0, |ui| {
            range_select(
                ui,
                0,
                0..self.bgm.segments.len() as isize,
                1,
                &mut self.selected_segment_idx,
                |v| format!("Segment {}", v + 1),
            );

            ui.pad(1, 5.0);

            if let Some(segment) = self.bgm.segments[self.selected_segment_idx as usize].as_mut() {
                let range = 0..segment.subsegments.len() as isize;

                if !range.contains(&(self.selected_subsegment_idx as isize)) {
                    self.selected_subsegment_idx = 0;
                }

                range_select(
                    ui,
                    2,
                    range,
                    1,
                    &mut self.selected_subsegment_idx,
                    |v| format!("Subsegment {}", v + 1),
                );

                ui.pad(3, 10.0);

                if let Some(subseg) = segment.subsegments.get_mut(self.selected_subsegment_idx as usize) {
                    ui.text(4, format!("Flags: {:08X}", subseg.flags()));

                    match subseg {
                        Subsegment::Unknown { data, .. } => {
                            ui.text(5, format!("Control data: {:02X}{:02X}{:02X}", data[0], data[1], data[2]));
                        }
                        Subsegment::Tracks { track_list, .. } => {
                            let track_list = &mut self.bgm.track_lists[*track_list];

                            ui.pad(6, 10.0);

                            ui.vbox(7, |ui| {
                                draw_track_list(ui, track_list);
                            });
                        }
                    }
                }
            } else {
                ui.text(99, "This segment has no data.");
            }
        });
    }
}

fn draw_track_list(ui: &mut imui_glium::UiFrame<'_>, track_list: &mut TrackList) {
    for (i, track) in track_list.tracks.iter_mut().enumerate() {
        ui.hbox(i as u8, |ui| {
            ui.button(0, format!("Track {}", i + 1));

            ui.pad(1, 8.0);

            ui.text(2, format!("Flags: {:04X}", track.flags)).center_y();

            if track.flags != 0 {
                ui.pad(3, 8.0);

                if ui.button(
                    4,
                    if track.is_drum() {
                        "Drum"
                    } else {
                        "Voice"
                    })
                    .clicked()
                {
                    track.set_drum(!track.is_drum());
                }
            }
        });
    }
}