use std::{convert::TryInto, ops::RangeBounds};
use std::convert::TryFrom;

use imui_glium::UiFrame;

pub fn range_select<N, R, L>(ui: &mut UiFrame<'_>, key: u8, range: R, step: isize, value: &mut N, label: L)
where
    N: Into<isize> + TryFrom<isize> + Copy,
    R: RangeBounds<isize>,
    L: FnOnce(&N) -> String,
{
    let cur_val: isize = (*value).into();

    ui.hbox(key, |ui| {
        if ui.button(0, "<").with_width(32.0).clicked() {
            let prec = cur_val - step;
            if range.contains(&prec) {
                *value = prec.try_into().unwrap_or(*value);
            }
        }

        ui.known_size(1, 120.0, 32.0, |ui| {
            ui.text(1, label(value))
                .center_x()
                .center_y();
        });

        if ui.button(2, ">").with_width(32.0).clicked() {
            let succ = cur_val + step;
            if range.contains(&succ) {
                *value = succ.try_into().unwrap_or(*value);
            }
        }
    });
}