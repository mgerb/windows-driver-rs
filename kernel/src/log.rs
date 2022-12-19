use alloc::{format, string::ToString};
use log::{LevelFilter, Metadata, Record, SetLoggerError};

extern "cdecl" {
    pub fn DbgPrintEx(component_id: u32, level: u32, fmt: *const u8, ...) -> i32;
}

static LOGGER: KernelLogger = KernelLogger;

pub struct KernelLogger;

impl KernelLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!(
                "{:<5} [{}] {}\n\0",
                record.level().to_string(),
                record.target(),
                record.args()
            );
            unsafe { DbgPrintEx(0, 0, message.as_ptr() as _) };
        }
    }

    fn flush(&self) {}
}
