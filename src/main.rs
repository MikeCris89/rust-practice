fn main() {
    println!("Fieldwatch: invasive species review");
    println!("Data source: mock detections");

    let species: &str = "Japanese knotweed";
    let confidence: u8 = 92;
    let confirmed: bool = false;

    println!("Species: {species}");
    println!("Confidence: {confidence}%");
    println!("Confirmed: {confirmed}");
}
