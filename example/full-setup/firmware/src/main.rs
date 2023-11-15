#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use crate::{
    comms::comms_task,
    usb::{configure_usb, usb_task, UsbResources},
};
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    rcc::{
        AHBPrescaler, APBPrescaler, Clock48MhzSrc, ClockSrc, Pll, PllM, PllN, PllQ, PllR, PllSource,
    },
    time::Hertz,
    Config,
};

use {defmt_rtt as _, panic_probe as _};

mod comms;
mod usb;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");

    let config = {
        let mut config = Config::default();
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE(Hertz(8_000_000)),
            prediv_m: PllM::DIV2,
            mul_n: PllN::MUL72,
            div_p: None,
            div_q: Some(PllQ::DIV6),
            div_r: Some(PllR::DIV2),
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.mux = ClockSrc::PLL;
        config.rcc.clock_48mhz_src = Some(Clock48MhzSrc::PllQ);
        config
    };

    let p = embassy_stm32::init(config);

    let usb_r = UsbResources {
        periph: p.USB,
        dp: p.PA12,
        dm: p.PA11,
    };
    let (d, c) = configure_usb(usb_r);

    spawner.must_spawn(usb_task(d));
    spawner.must_spawn(comms_task(c));
}
