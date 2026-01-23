use xkpasswd::prelude::*;

// Integration tests to verify our proptest implementations work
// These tests validate the core functionality that our property-based tests exercise

#[test]
fn test_guess_time_entropy_consistency() {
    let time1 = GuessTime::for_entropy(50);
    let time2 = GuessTime::for_entropy(50);
    assert_eq!(
        time1, time2,
        "GuessTime should be consistent for same entropy"
    );
}

#[test]
fn test_guess_time_entropy_thresholds() {
    // Test threshold values that trigger special cases
    let time_44 = GuessTime::for_entropy(45);
    assert_eq!(
        time_44.years, 1001,
        "Entropy 45-54 should trigger 1001 years"
    );

    let time_54 = GuessTime::for_entropy(55);
    assert_eq!(
        time_54.years, 1_000_001,
        "Entropy 55-64 should trigger 1M+ years"
    );

    let time_64 = GuessTime::for_entropy(65);
    assert_eq!(
        time_64.years, 1_000_000_001,
        "Entropy 65+ should trigger 1B+ years"
    );
}

#[test]
fn test_higher_entropy_longer_time() {
    let time1 = GuessTime::for_entropy(20);
    let time2 = GuessTime::for_entropy(30);

    // Convert to total days for comparison
    let days1 = time1.years * 365 + time1.months as usize * 30 + time1.days as usize;
    let days2 = time2.years * 365 + time2.months as usize * 30 + time2.days as usize;

    assert!(
        days2 >= days1,
        "Higher entropy should produce longer or equal guess time"
    );
}

#[test]
fn test_guess_time_display_formatting() {
    // Test zero values
    let time = GuessTime {
        years: 0,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "less than a day");

    // Test large values
    let time = GuessTime {
        years: 1_000_000_001,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "more than a billion years");

    let time = GuessTime {
        years: 1_000_001,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "more than a million years");

    let time = GuessTime {
        years: 1_001,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "more than a thousand years");

    // Test normal values with all components
    let time = GuessTime {
        years: 5,
        months: 3,
        days: 10,
    };
    let display = format!("{}", time);
    assert!(display.contains("5 years"));
    assert!(display.contains("3 months"));
    assert!(display.contains("10 days"));

    // Test single components
    let time = GuessTime {
        years: 2,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "2 years");

    let time = GuessTime {
        years: 0,
        months: 5,
        days: 0,
    };
    assert_eq!(format!("{}", time), "5 months");

    let time = GuessTime {
        years: 0,
        months: 0,
        days: 15,
    };
    assert_eq!(format!("{}", time), "15 days");
}

#[test]
fn test_entropy_display_formatting() {
    // Equal min/max
    let entropy = Entropy {
        blind_min: 50,
        blind_max: 50,
        seen: 60,
        guess_time: GuessTime::for_entropy(60),
    };

    let display = format!("{}", entropy);
    assert!(display.contains("50 bits blind"));
    assert!(!display.contains("between"));

    // Different min/max
    let entropy = Entropy {
        blind_min: 40,
        blind_max: 60,
        seen: 70,
        guess_time: GuessTime::for_entropy(70),
    };

    let display = format!("{}", entropy);
    assert!(display.contains("between 40 & 60 bits"));
    assert!(display.contains("70 bits with full knowledge"));

    // Check required information is present
    assert!(display.contains(&format!("{} guesses/sec", GuessTime::GUESSES_PER_SEC)));
    assert!(display.contains("takes computers"));
    assert!(display.contains("to break"));
}

// Note: Dictionary loading and password generation with custom dictionaries
// are tested in the unit tests since those functions/fields are private
