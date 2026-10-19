fn main() {
    println!("Fieldwatch: invasive species review");
    println!("Data source: mock detections");

    let detection = Detections {
        species: String::from("Japanese knotweed"),
        confidence: 92,
        confirmed: false,
    };

    let is_high = is_high_confidence(detection.confidence);

    println!("Species: {}", detection.species);
    println!("Confidence: {}%", detection.confidence);
    println!("High Confidence: {}", is_high);
    println!("Confirmed: {}", detection.confirmed);
}

fn is_high_confidence(confidence: u8) -> bool {
    confidence >= 80
}

struct Detections {
    species: String,
    confidence: u8,
    confirmed: bool,
}
