use std::fmt::Debug;
use tracing::{debug, error, info, trace, warn};

pub trait TracingStructExt {
    fn ok_or_print(&self) -> Option<String>;

    fn ok_or_trace(&self) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                trace!("{}", s)
            }
        }
        self
    }

    fn ok_or_trace_msg(&self, msg: &str) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                trace!("{} {}", msg, s)
            }
        }
        self
    }

    fn ok_or_debug(&self) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                debug!("{}", s)
            }
        }
        self
    }

    fn ok_or_debug_msg(&self, msg: &str) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                debug!("{} {}", msg, s)
            }
        }
        self
    }

    fn ok_or_info(&self) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                info!("{}", s)
            }
        }
        self
    }

    fn ok_or_info_msg(&self, msg: &str) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                info!("{} {}", msg, s)
            }
        }
        self
    }

    fn ok_or_warn(&self) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                warn!("{}", s)
            }
        }
        self
    }

    fn ok_or_warn_msg(&self, msg: &str) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                warn!("{} {}", msg, s)
            }
        }
        self
    }

    fn ok_or_error(&self) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                error!("{}", s)
            }
        }
        self
    }

    fn ok_or_error_msg(&self, msg: &str) -> &Self {
        match self.ok_or_print() {
            None => {}
            Some(s) => {
                error!("{} {}", msg, s)
            }
        }
        self
    }
}

impl<OK, ERR: Debug> TracingStructExt for Result<OK, ERR> {
    fn ok_or_print(&self) -> Option<String> {
        match self {
            Ok(_) => None,
            Err(err) => Some(format!("{:?}", err)),
        }
    }
}
