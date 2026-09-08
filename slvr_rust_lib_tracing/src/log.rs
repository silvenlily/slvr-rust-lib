use std::collections::HashMap;
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

pub use tracing::Level;

#[derive(Clone)]
pub struct LoggingConfig {
    pub default_level: Level,
    pub filters: HashMap<String, Level>,
    pub crate_level: Level,
}

impl LoggingConfig {
    pub fn new(default_level: Level) -> Self {
        Self {
            default_level,
            filters: HashMap::new(),
            crate_level: default_level,
        }
    }

    #[allow(unused_variables)]
    pub fn new_dev(default_level: Level, dev_default_level: Level) -> Self {
        #[cfg(debug_assertions)]
        let lvl = dev_default_level;

        #[cfg(not(debug_assertions))]
        let lvl = default_level;

        Self::new(lvl)
    }

    #[inline]
    pub fn crate_level(mut self, level: Level) -> Self {
        self.crate_level = level;
        self
    }

    #[allow(unused_variables)]
    #[inline]
    pub fn crate_level_dev(self, level: Level, dev_level: Level) -> Self {
        #[cfg(debug_assertions)]
        return self.crate_level(dev_level);
        #[cfg(not(debug_assertions))]
        return self.crate_level(level);
    }

    pub fn filter(mut self, filter: String, level: Level) -> Self {
        self.filters.insert(filter, level);
        self
    }

    #[allow(unused_variables)]
    #[inline]
    pub fn dev_filter(self, filter: String, level: Level, dev_level: Level) -> Self {
        #[cfg(debug_assertions)]
        return self.filter(filter, dev_level);
        #[cfg(not(debug_assertions))]
        self.filter(filter, level)
    }

    #[inline]
    pub fn dev_only_filter(self, filter: String, dev_level: Level) -> Self {
        #[cfg(debug_assertions)]
        self.filter(filter, dev_level)
    }

    pub fn init_logging(self) {
        let mut filter = Vec::new();
        filter.push(self.default_level.as_str().to_string());

        for (f, l) in self.filters {
            filter.push(Self::lvl_string(f, l));
        }

        filter.push(Self::lvl_string(env!("CARGO_CRATE_NAME").to_string(),self.crate_level));

        let env_filter = filter.join(",");

        let subscriber = FmtSubscriber::builder()
            .with_env_filter(env_filter)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        LogTracer::init().expect("could not initialize log compat layer")
    }

    #[inline]
    fn lvl_string(c: String, level: Level) -> String {
        format!("{}={}", c, level.as_str())
    }
}
