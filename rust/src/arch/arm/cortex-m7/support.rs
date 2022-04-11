use cortex_m::register::control;

#[derive(Debug)]
pub enum CPUStatus {
    Pmode(SP),
    Umode(SP),
}

#[derive(Debug)]
pub enum SP {
    MSP,
    PSP,
}

pub fn cpu_status() -> CPUStatus {

    let sp = match control::read().spsel() {
        control::Spsel::Msp => SP::MSP,
        control::Spsel::Psp => SP::PSP,
    };

    match control::read().npriv() {
        control::Npriv::Unprivileged => CPUStatus::Umode(sp),
        control::Npriv::Privileged => CPUStatus::Pmode(sp),
    }
}
