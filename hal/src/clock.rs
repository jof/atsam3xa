//! Configuring the system clock sources.
//! You will typically need to create an instance of `SystemClocks`
//! before you can set up most of the peripherals on the atsam3x target.
//! The other types in this module are used to enforce at compile time
//! that the peripherals have been correctly configured.
use crate::target_device;
use crate::time::{Hertz, MegaHertz};
use target_device::generic::Variant;
use target_device::pmc::ckgr_mor::MOSCRCF_A::*;
use target_device::pmc::pmc_mckr::{CSS_A::*, PRES_A::*};
use target_device::supc::sr::OSCSEL_A::*;
use target_device::{PMC, SUPC};

/// Valid frequency settings for the Fast RC oscillator
pub type FastRCFreq = target_device::pmc::ckgr_mor::MOSCRCF_A;
/// Valid clock sources for the system master clock
pub type ClockSource = target_device::pmc::pmc_mckr::CSS_A;
/// Valid prescaler values for the system master clock
pub type ClockPrescaler = target_device::pmc::pmc_mckr::PRES_A;

/// Oscillator sources that can be used by the slow clock.
///
/// The Low Power RC oscillator starts up faster, but is less accurate, so it's
/// the default clock for the system.  If more accurate timing is required,
/// switch to the Low Power Crystal oscillator.  Once the LP Crystal osc. has
/// been enabled, it is not possible to switch back.
pub enum SlowClockSource {
    /// Slow clock RC oscillator, runs at 32000Hz and less accurate
    LowPowerRC,
    /// Slow clock external crystal oscillator, runs at 32768Hz and
    /// more accurate
    LowPowerXtal32Khz,
}

/// Oscillator sources that can be used by the main clock.
///
/// The FastRc clock is configurable across three frequencies
/// - 4 MHz (uncalibrated, system power-on default)
/// - 8 MHz (calibrated)
/// - 12 MHz (calibrated)
///
/// The FastRc clock starts up quickly, but generally has lower accuracy than
/// whatever external crystal or ceramic oscillator has been connected, in
/// spite of the calibration on the 8MHz and 12MHz frequencies.
///
/// The Main Clock Crystal frequency is determined by the board designer, but
/// 12MHz is a common value.
pub enum MainClockSource {
    /// Internal, RC oscillator
    FastRc(FastRCFreq),
    /// External Crystal or Ceramic oscillator
    MainXtal,
}

/// Divider to apply to the master clock when using either PLLA or UPLL as
/// the source.
pub enum PllDiv {
    /// No divider
    One = 0,
    /// Run the master clock at half the frequency of the source PLL
    Two = 1,
}

/// Configuration options for setting up the PLLA clock source.  The output
/// frequency is the source clock frequency * (mula + 1)/diva.  The clock is
/// disabled when mula = 0.
pub struct PllAClockConfig {
    /// Clock multiplier minus one
    pub mula: u16,
    /// Clock divider
    pub diva: u8,
    /// how many slow clock ticks are required for PLLA to settle
    pub count: u8,
}

/// Configuration options for setting up the UPLL clock source.  To use USB,
/// the main clock must be configured to use the external crystal oscillator,
/// and it must be a 12MHz crystal.  The output frequency (TODO: unverified)
/// is the source clock frequency * 40, although clocks using this as an input
/// may apply other dividers to it.
pub struct UPllClockConfig {
    /// how many slow clock ticks are required for UPLL to settle
    pub count: u8,
}

impl From<u8> for UPllClockConfig {
    fn from(count: u8) -> Self {
        Self { count }
    }
}

/// Identifier used for enabling/disabling the clock to that peripheral, as
/// well as for controlling the peripher interrupt in the NVIC. Peripherals
/// 0-8, and 10 are always clocked.
pub enum PeripheralID {
    /// ID  0, Supply controller, NVIC Interrupt, No PMC clock control
    Id00Supc = 0,
    /// ID  1, Reset controller, NVIC Interrupt, No PMC clock control
    Id01Rstc,
    /// ID  2, Real-time clock, NVIC Interrupt, No PMC clock control
    Id02Rtc,
    /// ID  3, Real-time timer, NVIC Interrupt, No PMC clock control
    Id03Rtt,
    /// ID  4, Watchdog Timer, NVIC Interrupt, No PMC clock control
    Id04Wdg,
    /// ID  5, Power management controller, NVIC Interrupt, No PMC clock control
    Id05Pmc,
    /// ID  6, Enh. emb. flash cnt 1, NVIC Interrupt, No PMC clock control
    Id06Eefc0,
    /// ID  7, Enh. emb. flash cnt 2, NVIC Interrupt, No PMC clock control
    Id07Eefc1,
    /// ID  8, Univ. Async. Rx Tx, NVIC Interrupt, PMC clock controllable
    Id08Uart,
    /// ID  9, Static mem cnt, SDRAM cnt, NVIC Interrupt, PMC clock controllable
    Id09SmcSdramc,
    /// ID 10, SDRAM cnt, NVIC Interrupt, PMC clock controllable
    Id10Sdramc,
    /// ID 11, Parallel I/O controller A, NVIC Interrupt, PMC clock controllable
    Id11PioA,
    /// ID 12, Parallel I/O controller B, NVIC Interrupt, PMC clock controllable
    Id12PioB,
    /// ID 13, Parallel I/O controller C, NVIC Interrupt, PMC clock controllable
    Id13PioC,
    /// ID 14, Parallel I/O controller D, NVIC Interrupt, PMC clock controllable
    Id14PioD,
    /// ID 15, Parallel I/O controller E, NVIC Interrupt, PMC clock controllable
    Id15PioE,
    /// ID 16, Parallel I/O controller F, NVIC Interrupt, PMC clock controllable
    Id16PioF,
    /// ID 17, Sync UART 0, NVIC Interrupt, PMC clock controllable
    Id17Usart0,
    /// ID 18, Sync UART 1, NVIC Interrupt, PMC clock controllable
    Id18Usart1,
    /// ID 19, Sync UART 2, NVIC Interrupt, PMC clock controllable
    Id19Usart2,
    /// ID 20, Sync UART 3, NVIC Interrupt, PMC clock controllable
    Id20Usart3,
    /// ID 21, High speed media card intf., NVIC Interrupt, PMC clock controllable
    Id21Hsmci,
    /// ID 22, Two-wire intf. 0, NVIC Interrupt, PMC clock controllable
    Id22Twi0,
    /// ID 23, Two-wire intf. 1, NVIC Interrupt, PMC clock controllable
    Id23Twi1,
    /// ID 24, Serial peripheral intf. 0, NVIC Interrupt, PMC clock controllable
    Id24Spi0,
    /// ID 25, Serial peripheral intf. 1, NVIC Interrupt, PMC clock controllable
    Id25Spi1,
    /// ID 26, Sync. serial cnt, NVIC Interrupt, PMC clock controllable
    Id26Ssc,
    /// ID 27, Timer counter ch 0, NVIC Interrupt, PMC clock controllable
    Id27Tc0,
    /// ID 28, Timer counter ch 1, NVIC Interrupt, PMC clock controllable
    Id28Tc1,
    /// ID 29, Timer counter ch 2, NVIC Interrupt, PMC clock controllable
    Id29Tc2,
    /// ID 30, Timer counter ch 3, NVIC Interrupt, PMC clock controllable
    Id30Tc3,
    /// ID 31, Timer counter ch 4, NVIC Interrupt, PMC clock controllable
    Id31Tc4,
    /// ID 32, Timer counter ch 5, NVIC Interrupt, PMC clock controllable
    Id32Tc5,
    /// ID 33, Timer counter ch 6, NVIC Interrupt, PMC clock controllable
    Id33Tc6,
    /// ID 34, Timer counter ch 7, NVIC Interrupt, PMC clock controllable
    Id34Tc7,
    /// ID 35, Timer counter ch 8, NVIC Interrupt, PMC clock controllable
    Id35Tc8,
    /// ID 36, Pulse width mod. cnt, NVIC Interrupt, PMC clock controllable
    Id36Pwm,
    /// ID 37, ADC controller, NVIC Interrupt, PMC clock controllable
    Id37Adc,
    /// ID 38, DAC controller, NVIC Interrupt, PMC clock controllable
    Id38Dacc,
    /// ID 39, DMA controller, NVIC Interrupt, PMC clock controllable
    Id39Dmac,
    /// ID 40, USB OTG High Speed, NVIC Interrupt, PMC clock controllable
    Id40Uotghs,
    /// ID 41, True random number gen., NVIC Interrupt, PMC clock controllable
    Id41Trng,
    /// ID 42, Ethernet MAC, NVIC Interrupt, PMC clock controllable
    Id42Emac,
    /// ID 43, CAN controller 0, NVIC Interrupt, PMC clock controllable
    Id43Can0,
    /// ID 44, CAN controller 1, NVIC Interrupt, PMC clock controllable
    Id44Can1,
}

/// `SystemClocks` encapsulates the PMC and SUPC clock hardware.
/// It provides a type safe way to configure the system clocks.
/// Initializing the `SystemClocks` instance configures the system to run at
/// 84MHz by configuring Main clock to run at 12MHz, then setting PLLA to run
/// at 14x the Main clock, and then setting Master Clock to divide PLLA by 2.
pub struct SystemClocks {
    /// Power Management Controller
    pub pmc: PMC,
    /// Power Supply Controller
    pub supc: SUPC,
}

impl core::ops::Deref for SystemClocks {
    type Target = PMC;

    fn deref(&self) -> &Self::Target {
        &self.pmc
    }
}

impl core::ops::DerefMut for SystemClocks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pmc
    }
}

impl SystemClocks {
    /// Select the specified slow clock oscillator, and clock the system to run
    /// at that frequency.
    pub fn with_slow_clk(pmc: PMC, supc: SUPC, use_external_crystal: bool) -> Self {
        let mut clk = Self {
            pmc,
            supc,
        };
        if use_external_crystal {
            clk.enable_slow_clock_xtal();
        }
        clk.set_master_clock_source_and_prescaler(ClockSource::SLOW_CLK, None, false);
        clk
    }

    /// Set the main clock source, and clock the system to run at that frequency.
    pub fn with_main_clk(pmc: PMC, supc: SUPC, source: MainClockSource) -> Self {
        let mut clk = Self {
            pmc,
            supc,
        };
        clk.set_main_clock_source(source);
        clk.set_master_clock_source_and_prescaler(ClockSource::MAIN_CLK, None, false);
        clk
    }

    /// Configure the main clock to run off the main oscillator crystal, then
    /// configure PLLA to run at 14x that, set the system to run at half the
    /// frequency of PLLA, and if the usb feature is enabled, enable UPLL as well.
    pub fn with_plla_clk(pmc: PMC, supc: SUPC) -> Self {
        let mut clk = Self::with_main_clk(pmc, supc, MainClockSource::MainXtal);

        clk.configure_plla(PllAClockConfig {
            mula: 14 - 1,
            diva: 1,
            count: 0x3f,
        });
        clk.set_master_clock_source_and_prescaler(ClockSource::PLLA_CLK, None, true);

        #[cfg(feature = "usb")]
        clk.enable_upll(3);

        clk
    }

    /// Set the main clock source, and clock the system to run at that frequency.
    /// TODO: Either we have a bug in how we configure PLLA, or we have a bug in
    /// how to derive SysTick intervals based on PLLA configuration, either way
    /// until that's fixed, we'll default to using main clk.
    pub fn new(pmc: PMC, supc: SUPC) -> Self {
        Self::with_main_clk(pmc, supc, MainClockSource::MainXtal)
    }

    /// Return the frequency that the main clock is operating at
    pub fn get_slow_clock_rate(&self) -> Hertz {
        match self.supc.sr.read().oscsel().variant() {
            RC => Hertz(32000),
            CRYST => Hertz(32768),
        }
    }

    /// Return the frequency that the main clock is operating at
    pub fn get_main_clock_rate(&self) -> Hertz {
        if self.ckgr_mor.read().moscsel().bits() {
            MegaHertz(12).into()
        } else {
            match self.ckgr_mor.read().moscrcf().variant() {
                Variant::Val(_4_MHZ) => MegaHertz(4).into(),
                Variant::Val(_8_MHZ) => MegaHertz(8).into(),
                Variant::Val(_12_MHZ) => MegaHertz(12).into(),
                Variant::Res(_) => unreachable!(),
            }
        }
    }

    /// Return the frequency that the main clock is operating at, based
    /// on the slow clock rate.
    pub fn get_main_clock_rate_calibrated(&self) -> Hertz {
        // Wait until mainf has been calibrated since the last change of the
        // main clock
        while !self.ckgr_mcfr.read().mainfrdy().bits() {}

        // mainf is how many times the main clock ticks during the count of 16
        // slow clock cycles
        let mainf = self.ckgr_mcfr.read().mainf().bits() as u32;
        let mainf_freq = (mainf * self.get_slow_clock_rate().0) / 16;
        Hertz(mainf_freq)
    }

    /// Return the frequency that the plla clock is operating at
    pub fn get_plla_clock_rate(&self) -> Hertz {
        // plla clock = mainck * (mula + 1)/diva
        let mut tmp_clk = self.get_main_clock_rate();
        tmp_clk.0 *= (self.ckgr_pllar.read().mula().bits() + 1) as u32;
        tmp_clk.0 /= self.ckgr_pllar.read().diva().bits() as u32;
        tmp_clk
    }

    /// Return the frequency that the upll clock is operating at
    pub fn get_upll_clock_rate(&self) -> Hertz {
        // upll clock = mainck * 40
        // but it's only valid if mainck == 12MHz
        let mut tmp_clk = self.get_main_clock_rate();
        tmp_clk.0 *= 40;
        tmp_clk
    }

    /// Return the frequency that the master clock is operating at
    pub fn get_syscore(&mut self) -> Hertz {
        /* Determine clock frequency according to clock register values */
        let mut clk_unscaled: Hertz = match self.pmc_mckr.read().css().variant() {
            SLOW_CLK => self.get_slow_clock_rate(),
            MAIN_CLK => self.get_main_clock_rate(),
            PLLA_CLK => self.get_plla_clock_rate(),
            UPLL_CLK => self.get_upll_clock_rate(),
        };
        // Apply pll-specific divider if set
        if self.pmc_mckr.read().css().variant() == PLLA_CLK {
            clk_unscaled.0 /= 1 << (self.pmc_mckr.read().plladiv2().bits() as u8);
        }
        if self.pmc_mckr.read().css().variant() == PLLA_CLK {
            clk_unscaled.0 /= 1 << (self.pmc_mckr.read().uplldiv2().bits() as u8);
        }
        // Apply prescaler
        match self.pmc_mckr.read().pres().variant() {
            CLK_3 => Hertz(clk_unscaled.0 / 3),
            x => Hertz(clk_unscaled.0 >> (x as u8)),
        }
    }

    /// Slow clock is always enabled, but is sourced from a low-accuracy RC
    /// oscillator.  This enables the more accurate crystal oscillator and
    /// switch to use that as the slow clock source.  Once the crystal
    /// oscillator has been enabled, the RC oscillator is disabled and cannot
    /// be re-enabled.
    pub fn enable_slow_clock_xtal(&mut self) {
        self.supc
            .cr
            .write_with_zero(|w| w.key().passwd().xtalsel().set_bit());
    }

    /// Disabling the main clock is usually only done to enter low power/idle
    /// states.  It may only be re-enabled by an interrupt or rebooting.
    pub fn disable_main_clock(&mut self) {
        self.ckgr_mor.modify(|_, w| {
            w.key()
                .passwd()
                .moscrcen()
                .clear_bit()
                .moscxten()
                .clear_bit()
        });
    }

    /// Select the oscillator source to use for the main clock
    pub fn set_main_clock_source(&mut self, source: MainClockSource) {
        // TODO: if master clock source is PLLA or UPLL, and the current
        // MainClockSource is FastRC, a special procedure is required:
        //   - save the current master clock source selection
        //   - switch master clock to main
        //   - make the change to the main clock
        //   - wait for the change to settle
        //   - save the pll state
        //   - disable the pll
        //   - restore the pll state
        //     (the count value cannot be recovered from current state, though)
        //   - wait for the pll to settle
        //   - restore the master clock source selection

        // Crystal oscillator startup time
        // startup cycles = 8 * startup_time / SLCK
        let startup_time: u8 = 8;

        // To ensure a smooth transition in case other clocks are running off
        // main clock, we enable both clocks, wait out the startup time,
        // switch to the desired clock, then disable the unused clock
        self.ckgr_mor.modify(|_, w| unsafe {
            w.key()
                .passwd()
                .moscrcen()
                .set_bit()
                .moscxten()
                .set_bit()
                .moscxtst()
                .bits(startup_time)
        });
        // Wait until RC startup time runs out
        while !self.pmc_sr.read().moscrcs().bits() {}
        // Wait until Xtal startup time runs out
        while !self.pmc_sr.read().moscxts().bits() {}

        match source {
            MainClockSource::FastRc(f) => {
                // Set RC osc frequency
                self.ckgr_mor
                    .modify(|_, w| w.key().passwd().moscrcf().variant(f));
                // Let RC osc stabilize at new frequency
                while !self.pmc_sr.read().moscrcs().bits() {}

                // Switch main clock to RC osc
                self.ckgr_mor
                    .modify(|_, w| w.key().passwd().moscsel().clear_bit());
                // Wait until oscillator selection reports ready
                // 0 = done, 1 = in progress
                while !self.pmc_sr.read().moscsels().bits() {}

                // Disable unused xtal oscillator
                self.ckgr_mor
                    .modify(|_, w| w.key().passwd().moscxten().clear_bit());
            }
            MainClockSource::MainXtal => {
                self.ckgr_mor
                    .modify(|_, w| w.key().passwd().moscsel().set_bit());
                // Wait until oscillator selection reports ready
                // 0 = done, 1 = in progress
                while !self.pmc_sr.read().moscsels().bits() {}

                // Disable unused RC oscillator
                self.ckgr_mor
                    .modify(|_, w| w.key().passwd().moscrcen().clear_bit());
            }
        }
    }

    /// Disable PLLA by setting the clock multiplier to zero
    // Setting the multiplier to 0 disables it
    pub fn disable_plla(&mut self) {
        self.configure_plla(PllAClockConfig {
            mula: 0,
            diva: 1,
            count: 0,
        });
    }

    /// PLLA is always "enabled" but defaults to a multiplier of zero,
    /// effectively disabling it.  The resulting clock speed is the
    /// main clock * (mula + 1)/diva.
    pub fn configure_plla(&mut self, config: PllAClockConfig) {
        self.ckgr_pllar.write(|w| unsafe {
            w.one()
                .set_bit()
                .mula()
                .bits(config.mula)
                .diva()
                .bits(config.diva)
                .pllacount()
                .bits(config.count)
        });

        // Wait until pll is locked
        // 0 = not locked, 1 = locked
        while !self.pmc_sr.read().locka().bits() {}
    }

    /// Enable the UTMI PLL, primarily used for clocking USB.
    pub fn enable_upll(&mut self, config: UPllClockConfig) {
        self.ckgr_uckr
            .write(|w| unsafe { w.upllen().set_bit().upllcount().bits(config.count) });

        // Wait until pll is locked
        // 0 = not locked, 1 = locked
        while !self.pmc_sr.read().locku().bits() {}
    }

    /// Disable the UTMI PLL, disabling the USB bus and any clocks configured
    /// to use it as a source.
    pub fn disable_upll(&mut self) {
        self.ckgr_uckr.write(|w| w.upllen().clear_bit());
    }

    /// Select which clock source the master clock should use, along with some
    /// options for dividing the source clock.
    pub fn set_master_clock_source_and_prescaler(
        &mut self,
        source: ClockSource,
        prescaler: Option<ClockPrescaler>,
        pll_div2: bool,
    ) {
        // For PLLs, prescaler should be applied before changing the clock source
        if source == ClockSource::PLLA_CLK || source == ClockSource::UPLL_CLK {
            if let Some(prescaler) = prescaler {
                self.pmc_mckr.modify(|_, w| w.pres().variant(prescaler));

                // Wait for the prescaler to latch
                // 0 = not ready, 1 = ready
                while !self.pmc_sr.read().mckrdy().bits() {}
            }
        }
        // For switching to PLL, we have to prime it by first setting main clock and the pll
        // divider before we switch to the PLL
        if source == ClockSource::PLLA_CLK {
            self.pmc_mckr
                .modify(|_, w| w.css().main_clk().plladiv2().bit(pll_div2));
            while !self.pmc_sr.read().mckrdy().bits() {}
        }
        if source == ClockSource::UPLL_CLK {
            self.pmc_mckr
                .modify(|_, w| w.css().main_clk().uplldiv2().bit(pll_div2));
            while !self.pmc_sr.read().mckrdy().bits() {}
        }

        // Switch to the desired clock
        match source {
            ClockSource::SLOW_CLK => self.pmc_mckr.modify(|_, w| w.css().slow_clk()),
            ClockSource::MAIN_CLK => self.pmc_mckr.modify(|_, w| w.css().main_clk()),
            ClockSource::PLLA_CLK => self
                .pmc_mckr
                .modify(|_, w| w.css().plla_clk().plladiv2().bit(pll_div2)),
            ClockSource::UPLL_CLK => self
                .pmc_mckr
                .modify(|_, w| w.css().upll_clk().uplldiv2().bit(pll_div2)),
        }

        // Wait until master clock reports ready
        // 0 = not ready, 1 = ready
        while !self.pmc_sr.read().mckrdy().bits() {}

        // For slow and main clocks, prescaler should be applied afterchanging
        // the clock source
        if source == ClockSource::SLOW_CLK || source == ClockSource::MAIN_CLK {
            if let Some(prescaler) = prescaler {
                self.pmc_mckr.modify(|_, w| w.pres().variant(prescaler));

                // Wait for the prescaler to latch
                // 0 = not ready, 1 = ready
                while !self.pmc_sr.read().mckrdy().bits() {}
            }
        }
    }

    /// Enable the clock for the specified peripheral.  Some peripherals'
    /// clocks are not under PMC control - passing the ID for these clocks
    /// will silently do nothing.
    pub fn enable_peripheral_clock(&mut self, pid: PeripheralID) {
        match pid {
            PeripheralID::Id00Supc => (),  // Clock not under PMC control
            PeripheralID::Id01Rstc => (),  // Clock not under PMC control
            PeripheralID::Id02Rtc => (),   // Clock not under PMC control
            PeripheralID::Id03Rtt => (),   // Clock not under PMC control
            PeripheralID::Id04Wdg => (),   // Clock not under PMC control
            PeripheralID::Id05Pmc => (),   // Clock not under PMC control
            PeripheralID::Id06Eefc0 => (), // Clock not under PMC control
            PeripheralID::Id07Eefc1 => (), // Clock not under PMC control
            PeripheralID::Id08Uart => self.pmc_pcer0.write_with_zero(|w| w.pid8().set_bit()),
            PeripheralID::Id09SmcSdramc => self.pmc_pcer0.write_with_zero(|w| w.pid9().set_bit()),
            PeripheralID::Id10Sdramc => self.pmc_pcer0.write_with_zero(|w| w.pid10().set_bit()),
            PeripheralID::Id11PioA => self.pmc_pcer0.write_with_zero(|w| w.pid11().set_bit()),
            PeripheralID::Id12PioB => self.pmc_pcer0.write_with_zero(|w| w.pid12().set_bit()),
            PeripheralID::Id13PioC => self.pmc_pcer0.write_with_zero(|w| w.pid13().set_bit()),
            PeripheralID::Id14PioD => self.pmc_pcer0.write_with_zero(|w| w.pid14().set_bit()),
            PeripheralID::Id15PioE => self.pmc_pcer0.write_with_zero(|w| w.pid15().set_bit()),
            PeripheralID::Id16PioF => self.pmc_pcer0.write_with_zero(|w| w.pid16().set_bit()),
            PeripheralID::Id17Usart0 => self.pmc_pcer0.write_with_zero(|w| w.pid17().set_bit()),
            PeripheralID::Id18Usart1 => self.pmc_pcer0.write_with_zero(|w| w.pid18().set_bit()),
            PeripheralID::Id19Usart2 => self.pmc_pcer0.write_with_zero(|w| w.pid19().set_bit()),
            PeripheralID::Id20Usart3 => self.pmc_pcer0.write_with_zero(|w| w.pid20().set_bit()),
            PeripheralID::Id21Hsmci => self.pmc_pcer0.write_with_zero(|w| w.pid21().set_bit()),
            PeripheralID::Id22Twi0 => self.pmc_pcer0.write_with_zero(|w| w.pid22().set_bit()),
            PeripheralID::Id23Twi1 => self.pmc_pcer0.write_with_zero(|w| w.pid23().set_bit()),
            PeripheralID::Id24Spi0 => self.pmc_pcer0.write_with_zero(|w| w.pid24().set_bit()),
            PeripheralID::Id25Spi1 => self.pmc_pcer0.write_with_zero(|w| w.pid25().set_bit()),
            PeripheralID::Id26Ssc => self.pmc_pcer0.write_with_zero(|w| w.pid26().set_bit()),
            PeripheralID::Id27Tc0 => self.pmc_pcer0.write_with_zero(|w| w.pid27().set_bit()),
            PeripheralID::Id28Tc1 => self.pmc_pcer0.write_with_zero(|w| w.pid28().set_bit()),
            PeripheralID::Id29Tc2 => self.pmc_pcer0.write_with_zero(|w| w.pid29().set_bit()),
            PeripheralID::Id30Tc3 => self.pmc_pcer0.write_with_zero(|w| w.pid30().set_bit()),
            PeripheralID::Id31Tc4 => self.pmc_pcer0.write_with_zero(|w| w.pid31().set_bit()),
            PeripheralID::Id32Tc5 => self.pmc_pcer1.write_with_zero(|w| w.pid32().set_bit()),
            PeripheralID::Id33Tc6 => self.pmc_pcer1.write_with_zero(|w| w.pid33().set_bit()),
            PeripheralID::Id34Tc7 => self.pmc_pcer1.write_with_zero(|w| w.pid34().set_bit()),
            PeripheralID::Id35Tc8 => self.pmc_pcer1.write_with_zero(|w| w.pid35().set_bit()),
            PeripheralID::Id36Pwm => self.pmc_pcer1.write_with_zero(|w| w.pid36().set_bit()),
            PeripheralID::Id37Adc => self.pmc_pcer1.write_with_zero(|w| w.pid37().set_bit()),
            PeripheralID::Id38Dacc => self.pmc_pcer1.write_with_zero(|w| w.pid38().set_bit()),
            PeripheralID::Id39Dmac => self.pmc_pcer1.write_with_zero(|w| w.pid39().set_bit()),
            PeripheralID::Id40Uotghs => self.pmc_pcer1.write_with_zero(|w| w.pid40().set_bit()),
            PeripheralID::Id41Trng => self.pmc_pcer1.write_with_zero(|w| w.pid41().set_bit()),
            PeripheralID::Id42Emac => self.pmc_pcer1.write_with_zero(|w| w.pid42().set_bit()),
            PeripheralID::Id43Can0 => self.pmc_pcer1.write_with_zero(|w| w.pid43().set_bit()),
            PeripheralID::Id44Can1 => self.pmc_pcer1.write_with_zero(|w| w.pid44().set_bit()),
        }
    }

    /// Disable the clock for the specified peripheral.  Some peripherals'
    /// clocks are not under PMC control - passing the ID for these clocks
    /// will silently do nothing.
    pub fn disable_peripheral_clock(&mut self, pid: PeripheralID) {
        match pid {
            PeripheralID::Id00Supc => (),  // Clock not under PMC control
            PeripheralID::Id01Rstc => (),  // Clock not under PMC control
            PeripheralID::Id02Rtc => (),   // Clock not under PMC control
            PeripheralID::Id03Rtt => (),   // Clock not under PMC control
            PeripheralID::Id04Wdg => (),   // Clock not under PMC control
            PeripheralID::Id05Pmc => (),   // Clock not under PMC control
            PeripheralID::Id06Eefc0 => (), // Clock not under PMC control
            PeripheralID::Id07Eefc1 => (), // Clock not under PMC control
            PeripheralID::Id08Uart => self.pmc_pcdr0.write_with_zero(|w| w.pid8().set_bit()),
            PeripheralID::Id09SmcSdramc => self.pmc_pcdr0.write_with_zero(|w| w.pid9().set_bit()),
            PeripheralID::Id10Sdramc => self.pmc_pcdr0.write_with_zero(|w| w.pid10().set_bit()),
            PeripheralID::Id11PioA => self.pmc_pcdr0.write_with_zero(|w| w.pid11().set_bit()),
            PeripheralID::Id12PioB => self.pmc_pcdr0.write_with_zero(|w| w.pid12().set_bit()),
            PeripheralID::Id13PioC => self.pmc_pcdr0.write_with_zero(|w| w.pid13().set_bit()),
            PeripheralID::Id14PioD => self.pmc_pcdr0.write_with_zero(|w| w.pid14().set_bit()),
            PeripheralID::Id15PioE => self.pmc_pcdr0.write_with_zero(|w| w.pid15().set_bit()),
            PeripheralID::Id16PioF => self.pmc_pcdr0.write_with_zero(|w| w.pid16().set_bit()),
            PeripheralID::Id17Usart0 => self.pmc_pcdr0.write_with_zero(|w| w.pid17().set_bit()),
            PeripheralID::Id18Usart1 => self.pmc_pcdr0.write_with_zero(|w| w.pid18().set_bit()),
            PeripheralID::Id19Usart2 => self.pmc_pcdr0.write_with_zero(|w| w.pid19().set_bit()),
            PeripheralID::Id20Usart3 => self.pmc_pcdr0.write_with_zero(|w| w.pid20().set_bit()),
            PeripheralID::Id21Hsmci => self.pmc_pcdr0.write_with_zero(|w| w.pid21().set_bit()),
            PeripheralID::Id22Twi0 => self.pmc_pcdr0.write_with_zero(|w| w.pid22().set_bit()),
            PeripheralID::Id23Twi1 => self.pmc_pcdr0.write_with_zero(|w| w.pid23().set_bit()),
            PeripheralID::Id24Spi0 => self.pmc_pcdr0.write_with_zero(|w| w.pid24().set_bit()),
            PeripheralID::Id25Spi1 => self.pmc_pcdr0.write_with_zero(|w| w.pid25().set_bit()),
            PeripheralID::Id26Ssc => self.pmc_pcdr0.write_with_zero(|w| w.pid26().set_bit()),
            PeripheralID::Id27Tc0 => self.pmc_pcdr0.write_with_zero(|w| w.pid27().set_bit()),
            PeripheralID::Id28Tc1 => self.pmc_pcdr0.write_with_zero(|w| w.pid28().set_bit()),
            PeripheralID::Id29Tc2 => self.pmc_pcdr0.write_with_zero(|w| w.pid29().set_bit()),
            PeripheralID::Id30Tc3 => self.pmc_pcdr0.write_with_zero(|w| w.pid30().set_bit()),
            PeripheralID::Id31Tc4 => self.pmc_pcdr0.write_with_zero(|w| w.pid31().set_bit()),
            PeripheralID::Id32Tc5 => self.pmc_pcdr1.write_with_zero(|w| w.pid32().set_bit()),
            PeripheralID::Id33Tc6 => self.pmc_pcdr1.write_with_zero(|w| w.pid33().set_bit()),
            PeripheralID::Id34Tc7 => self.pmc_pcdr1.write_with_zero(|w| w.pid34().set_bit()),
            PeripheralID::Id35Tc8 => self.pmc_pcdr1.write_with_zero(|w| w.pid35().set_bit()),
            PeripheralID::Id36Pwm => self.pmc_pcdr1.write_with_zero(|w| w.pid36().set_bit()),
            PeripheralID::Id37Adc => self.pmc_pcdr1.write_with_zero(|w| w.pid37().set_bit()),
            PeripheralID::Id38Dacc => self.pmc_pcdr1.write_with_zero(|w| w.pid38().set_bit()),
            PeripheralID::Id39Dmac => self.pmc_pcdr1.write_with_zero(|w| w.pid39().set_bit()),
            PeripheralID::Id40Uotghs => self.pmc_pcdr1.write_with_zero(|w| w.pid40().set_bit()),
            PeripheralID::Id41Trng => self.pmc_pcdr1.write_with_zero(|w| w.pid41().set_bit()),
            PeripheralID::Id42Emac => self.pmc_pcdr1.write_with_zero(|w| w.pid42().set_bit()),
            PeripheralID::Id43Can0 => self.pmc_pcdr1.write_with_zero(|w| w.pid43().set_bit()),
            PeripheralID::Id44Can1 => self.pmc_pcdr1.write_with_zero(|w| w.pid44().set_bit()),
        }
    }
}
