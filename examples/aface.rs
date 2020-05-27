use rand::SeedableRng;

fn main() -> std::io::Result<()> {
    let mut rng = rand_xorshift::XorShiftRng::from_entropy();
    let mut file = std::fs::File::create("face.svg")?;
    svg_face::generate_face(&mut file, &mut rng)
}
