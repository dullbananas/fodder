use fodder::fodder;


fodder!("examples/worker/elm.json");


// force recompile
const _: &'static str = include_str!("/proc/uptime");


fn main() {
}
