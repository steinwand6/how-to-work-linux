use linux_personality;

fn main() {
    linux_personality::personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();
    loop {}
}
