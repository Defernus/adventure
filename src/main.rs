use adventure::run;

fn main() {
    pollster::block_on(run());
}
