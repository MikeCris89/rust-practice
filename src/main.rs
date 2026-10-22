fn main() {
    println!("Fieldwatch: invasive species review");
    println!("Data source: mock detections");

    let detection = Detections {
        species: String::from("Japanese knotweed"),
        confidence: 92,
        status: ReviewStatus::Pending,
    };

    let is_high = is_high_confidence(detection.confidence);

    let status_label = match detection.status {
        ReviewStatus::Pending => "Pending",
        ReviewStatus::Confirmed => "Confirmed",
        ReviewStatus::Dismissed => "Dismissed",
    };

    println!("Species: {}", detection.species);
    println!("Confidence: {}%", detection.confidence);
    println!("High Confidence: {}", is_high);
    println!("Review Status: {}", status_label);

    let detections = vec![
        detection,
        Detections {
            species: String::from("Giant hogweed"),
            confidence: 80,
            status: ReviewStatus::Pending,
        },
        Detections {
            species: String::from("Himalayan balsam"),
            confidence: 55,
            status: ReviewStatus::Pending,
        },
    ];

    println!("Length: {}", detections.len());
}

fn is_high_confidence(confidence: u8) -> bool {
    confidence >= 80
}

struct Detections {
    species: String,
    confidence: u8,
    status: ReviewStatus,
}

enum ReviewStatus {
    Pending,
    Confirmed,
    Dismissed,
}
