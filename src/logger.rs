enum MsgType {
    error,
    warn,
    info,
    debug
}
struct LogMsg {
    r#type: MsgType,
    msg: String,
}
pub struct Logger {
    listeners: Vec<Box<dyn Fn(LogMsg) -> ()>>,
}

impl Logger {
    fn fire_event(&self, r#type: MsgType, msg: &str) -> () {
        let msg = LogMsg {
            r#type,
            msg: String::from(msg)
        };
    }

    fn add_listener(&mut self, listener: impl Fn(LogMsg) -> ()) {
        self.listeners.push(listener);

        self.listeners.iter().find(|&&x| {
            let y = *x;
            y == *listener;
            true
        });

        return (|| -> () {
            let next = self.listeners
                .iter()
                .filter(|x| {
                    true
                }).map(|x| {
                    *x
                }).collect::<Vec<_>>();
        })()

    }

    pub fn error(&self, msg: &str) -> () {
        self.fire_event(MsgType::error, msg)
    }
    pub fn warn(&self, msg: &str) -> () {
        self.fire_event(MsgType::warn, msg)
    }
    pub fn info(&self, msg: &str) -> (){
        self.fire_event(MsgType::info, msg)
    }
    pub fn debug(&self, msg: &str) -> () {
        self.fire_event(MsgType::debug, msg)
    }
}