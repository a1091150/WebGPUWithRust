use hello::run;

fn main() {
    pollster::block_on(run());
}