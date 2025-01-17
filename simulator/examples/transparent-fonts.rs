use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egrectangle, icoord, text_6x8};
use embedded_graphics_simulator::{DisplayBuilder, DisplayTheme};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new().theme(DisplayTheme::OledBlue).build();

    display.draw(
        egcircle!(
            (20, 20),
            20 as u32,
            stroke = Some(1u8.into()),
            fill = Some(1u8.into())
        )
        .into_iter()
        .chain(egrectangle!((20, 20), (100, 80), fill = Some(1u8.into()))),
    );

    display.draw(text_6x8!("Hello world! - no background").translate(icoord!(15, 15)));

    display.draw(
        text_6x8!(
            "Hello world! - filled background",
            stroke = Some(1u8.into()),
            fill = Some(0u8.into())
        )
        .translate(icoord!(15, 30)),
    );

    display.draw(
        text_6x8!(
            "Hello world! - inverse background",
            stroke = Some(0u8.into()),
            fill = Some(1u8.into())
        )
        .translate(icoord!(15, 45)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
