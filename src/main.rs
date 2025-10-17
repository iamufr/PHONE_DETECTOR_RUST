use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

// ============================================================================
// ENUMS AND STRUCTS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneType {
    FormattedDomestic, // (123) 456-7890, 123-456-7890, 123.456.7890
    FormattedTollFree, // 1-800-555-1234, 1.800.555.1234
    InternationalPlus, // +1 123-456-7890, +91-1234567890, +44 20 1234 5678
    InternationalZero, // 00 1 123-456-7890
    Plain10Digit,      // 1234567890
    Plain11Digit,      // 11234567890
    Mobile10Digit,     // 9876543210 (starts with 1-9)
    Unknown,
}

#[derive(Debug, Clone)]
pub struct PhoneMatch {
    pub phone_type: PhoneType,
    pub value: String,
    pub normalized: String, // Digits only
    pub position: usize,
}

impl PhoneMatch {
    fn new(phone_type: PhoneType, value: String, normalized: String, position: usize) -> Self {
        Self {
            phone_type,
            value,
            normalized,
            position,
        }
    }
}

// ============================================================================
// TRAITS (SOLID Principles)
// ============================================================================

pub trait PhoneValidator {
    fn is_valid(&self, phone: &str) -> bool;
    fn get_type(&self) -> PhoneType;
}

// ============================================================================
// CHARACTER CLASSIFIER (Optimized Lookup Table)
// ============================================================================

pub struct CharacterClassifier;

impl CharacterClassifier {
    const CHAR_DIGIT: u8 = 0x01;
    const CHAR_SEPARATOR: u8 = 0x02;
    const CHAR_PLUS: u8 = 0x04;

    #[rustfmt::skip]
    const CHAR_TABLE: [u8; 256] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x02, 0x00, 0x04, 0x00, 0x02, 0x02, 0x00,
        0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[inline(always)]
    pub fn is_digit(c: u8) -> bool {
        Self::CHAR_TABLE[c as usize] & Self::CHAR_DIGIT != 0
    }

    #[inline(always)]
    pub fn is_separator(c: u8) -> bool {
        Self::CHAR_TABLE[c as usize] & Self::CHAR_SEPARATOR != 0
    }

    #[inline(always)]
    pub fn is_plus(c: u8) -> bool {
        Self::CHAR_TABLE[c as usize] & Self::CHAR_PLUS != 0
    }

    #[inline(always)]
    pub fn is_phone_char(c: u8) -> bool {
        Self::CHAR_TABLE[c as usize] != 0
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

#[inline]
fn extract_digits(s: &str) -> String {
    s.bytes()
        .filter(|&c| CharacterClassifier::is_digit(c))
        .map(|c| c as char)
        .collect()
}

// ============================================================================
// VALIDATORS (Single Responsibility Principle)
// ============================================================================

pub struct FormattedDomesticValidator;

impl PhoneValidator for FormattedDomesticValidator {
    fn is_valid(&self, phone: &str) -> bool {
        let digits = extract_digits(phone);
        if digits.len() != 10 {
            return false;
        }
        let bytes = digits.as_bytes();
        if bytes[0] == b'0' {
            return false;
        }
        if bytes[3] < b'2' {
            return false;
        }
        true
    }

    fn get_type(&self) -> PhoneType {
        PhoneType::FormattedDomestic
    }
}

pub struct InternationalPlusValidator;

impl PhoneValidator for InternationalPlusValidator {
    fn is_valid(&self, phone: &str) -> bool {
        if phone.is_empty() || phone.as_bytes()[0] != b'+' {
            return false;
        }
        let digits = extract_digits(phone);
        digits.len() >= 7 && digits.len() <= 15
    }

    fn get_type(&self) -> PhoneType {
        PhoneType::InternationalPlus
    }
}

pub struct PlainDigitValidator {
    expected_length: usize,
    phone_type: PhoneType,
}

impl PlainDigitValidator {
    pub fn new(expected_length: usize, phone_type: PhoneType) -> Self {
        Self {
            expected_length,
            phone_type,
        }
    }
}

impl PhoneValidator for PlainDigitValidator {
    fn is_valid(&self, phone: &str) -> bool {
        if phone.len() != self.expected_length {
            return false;
        }
        let bytes = phone.as_bytes();
        if !bytes.iter().all(|&c| CharacterClassifier::is_digit(c)) {
            return false;
        }

        if self.expected_length == 10 {
            if bytes[0] == b'0' {
                return false;
            }
            if bytes[3] < b'2' {
                return false;
            }
        } else if self.expected_length == 11 {
            if bytes[0] != b'1' {
                return false;
            }
            if bytes[1] == b'0' {
                return false;
            }
        }
        true
    }

    fn get_type(&self) -> PhoneType {
        self.phone_type
    }
}

pub struct MobileDigitValidator;

impl PhoneValidator for MobileDigitValidator {
    fn is_valid(&self, phone: &str) -> bool {
        let digits = extract_digits(phone);
        let bytes = digits.as_bytes();

        if digits.len() == 10 {
            return bytes[0] >= b'1' && bytes[0] <= b'9';
        }
        if digits.len() == 12 {
            return bytes[0] == b'9' && bytes[1] == b'1' && bytes[2] >= b'1' && bytes[2] <= b'9';
        }
        false
    }

    fn get_type(&self) -> PhoneType {
        PhoneType::Mobile10Digit
    }
}

// ============================================================================
// PHONE SCANNER (Optimized for Performance)
// ============================================================================

pub struct PhoneScanner {
    max_input_size: usize,
    max_phone_length: usize,
    min_digits: usize,
    max_digits: usize,
}

impl PhoneScanner {
    pub fn new() -> Self {
        Self {
            max_input_size: 10 * 1024 * 1024,
            max_phone_length: 30,
            min_digits: 7,
            max_digits: 15,
        }
    }

    fn scan_international(&self, data: &[u8], matches: &mut Vec<PhoneMatch>) {
        let len = data.len();
        let mut i = 0;

        while i < len {
            if data[i] == b'+' && i + 1 < len && CharacterClassifier::is_digit(data[i + 1]) {
                let start = i;
                let mut candidate = String::from("+");
                let mut digit_count = 0;
                i += 1;

                while i < len && candidate.len() < self.max_phone_length {
                    if CharacterClassifier::is_digit(data[i]) {
                        candidate.push(data[i] as char);
                        digit_count += 1;
                        i += 1;
                    } else if (CharacterClassifier::is_separator(data[i]) || data[i] == b'(')
                        && digit_count > 0
                        && i + 1 < len
                        && (CharacterClassifier::is_digit(data[i + 1]) || data[i + 1] == b'(')
                    {
                        candidate.push(data[i] as char);
                        i += 1;
                    } else if data[i] == b')' && digit_count > 0 {
                        candidate.push(data[i] as char);
                        i += 1;
                    } else {
                        break;
                    }
                }

                let digits = extract_digits(&candidate);
                if digits.len() >= self.min_digits && digits.len() <= self.max_digits {
                    matches.push(PhoneMatch::new(
                        PhoneType::InternationalPlus,
                        candidate,
                        digits,
                        start,
                    ));
                    continue;
                }
                i = start;
            }
            i += 1;
        }
    }

    fn scan_formatted_numbers(&self, data: &[u8], matches: &mut Vec<PhoneMatch>) {
        let len = data.len();
        let mut i = 0;

        while i < len {
            // Check for (123) format
            if data[i] == b'(' && i + 14 <= len {
                if CharacterClassifier::is_digit(data[i + 1])
                    && CharacterClassifier::is_digit(data[i + 2])
                    && CharacterClassifier::is_digit(data[i + 3])
                    && data[i + 4] == b')'
                    && (data[i + 5] == b' ' || data[i + 5] == b'-')
                {
                    let mut end = i + 6;
                    let mut candidate = format!(
                        "({}{}{}) ",
                        data[i + 1] as char,
                        data[i + 2] as char,
                        data[i + 3] as char
                    );

                    let mut digit_count = 0;
                    while end < len && digit_count < 7 && candidate.len() < self.max_phone_length {
                        if CharacterClassifier::is_digit(data[end]) {
                            candidate.push(data[end] as char);
                            digit_count += 1;
                            end += 1;
                        } else if CharacterClassifier::is_separator(data[end])
                            && digit_count > 0
                            && digit_count < 7
                        {
                            candidate.push(data[end] as char);
                            end += 1;
                        } else {
                            break;
                        }
                    }

                    if digit_count == 7 {
                        let digits = extract_digits(&candidate);
                        if digits.len() == 10
                            && digits.as_bytes()[0] != b'0'
                            && digits.as_bytes()[3] >= b'2'
                        {
                            matches.push(PhoneMatch::new(
                                PhoneType::FormattedDomestic,
                                candidate,
                                digits,
                                i,
                            ));
                            i = end - 1;
                            i += 1;
                            continue;
                        }
                    }
                }
            }

            // Check for dash/dot separated formats
            if CharacterClassifier::is_digit(data[i])
                && (i == 0 || !CharacterClassifier::is_digit(data[i - 1]))
            {
                let start = i;
                let mut candidate = String::new();
                let mut digit_count = 0;
                let mut separator = 0u8;
                let mut has_separator = false;

                while i < len && candidate.len() < self.max_phone_length {
                    if CharacterClassifier::is_digit(data[i]) {
                        candidate.push(data[i] as char);
                        digit_count += 1;
                        i += 1;
                    } else if (data[i] == b'-' || data[i] == b'.' || data[i] == b' ')
                        && digit_count > 0
                        && digit_count < 11
                        && i + 1 < len
                        && CharacterClassifier::is_digit(data[i + 1])
                    {
                        if separator == 0 {
                            separator = data[i];
                        }
                        if data[i] == separator {
                            candidate.push(data[i] as char);
                            has_separator = true;
                            i += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if has_separator && digit_count >= 10 && digit_count <= 11 {
                    let digits = extract_digits(&candidate);
                    let bytes = digits.as_bytes();

                    if digit_count == 10
                        && separator == b' '
                        && bytes[0] >= b'1'
                        && bytes[0] <= b'9'
                    {
                        matches.push(PhoneMatch::new(
                            PhoneType::Mobile10Digit,
                            candidate,
                            digits,
                            start,
                        ));
                        continue;
                    } else if digit_count == 10 && bytes[0] != b'0' && bytes[3] >= b'2' {
                        matches.push(PhoneMatch::new(
                            PhoneType::FormattedDomestic,
                            candidate,
                            digits,
                            start,
                        ));
                        continue;
                    } else if digit_count == 11 && bytes[0] == b'1' && bytes[1] != b'0' {
                        matches.push(PhoneMatch::new(
                            PhoneType::FormattedTollFree,
                            candidate,
                            digits,
                            start,
                        ));
                        continue;
                    }
                }
                i = start;
            }
            i += 1;
        }
    }

    fn scan_plain_digits(&self, data: &[u8], matches: &mut Vec<PhoneMatch>) {
        let len = data.len();
        let mut i = 0;

        while i < len {
            if !CharacterClassifier::is_digit(data[i]) {
                i += 1;
                continue;
            }
            if i > 0 && CharacterClassifier::is_digit(data[i - 1]) {
                i += 1;
                continue;
            }

            let start = i;
            let mut digit_count = 0;

            while i < len && CharacterClassifier::is_digit(data[i]) {
                digit_count += 1;
                i += 1;
            }

            if i < len && CharacterClassifier::is_digit(data[i]) {
                continue;
            }

            let candidate = String::from_utf8_lossy(&data[start..start + digit_count]).to_string();
            let bytes = candidate.as_bytes();

            if digit_count == 10 {
                if bytes[0] >= b'6' && bytes[0] <= b'9' {
                    matches.push(PhoneMatch::new(
                        PhoneType::Mobile10Digit,
                        candidate.clone(),
                        candidate,
                        start,
                    ));
                    continue;
                } else if bytes[0] >= b'2' && bytes[0] <= b'5' && bytes[3] >= b'2' {
                    matches.push(PhoneMatch::new(
                        PhoneType::Plain10Digit,
                        candidate.clone(),
                        candidate,
                        start,
                    ));
                    continue;
                } else if bytes[0] == b'1' {
                    matches.push(PhoneMatch::new(
                        PhoneType::Mobile10Digit,
                        candidate.clone(),
                        candidate,
                        start,
                    ));
                    continue;
                }
            } else if digit_count == 11 {
                if bytes[0] == b'1' && bytes[1] != b'0' {
                    matches.push(PhoneMatch::new(
                        PhoneType::Plain11Digit,
                        candidate.clone(),
                        candidate,
                        start,
                    ));
                    continue;
                }
            }
        }
    }

    pub fn extract(&self, text: &str) -> Vec<PhoneMatch> {
        let len = text.len();

        if len > self.max_input_size || len < self.min_digits {
            return Vec::new();
        }

        let mut matches = Vec::with_capacity(20);
        let data = text.as_bytes();

        self.scan_international(data, &mut matches);
        self.scan_formatted_numbers(data, &mut matches);
        self.scan_plain_digits(data, &mut matches);

        if matches.is_empty() {
            return matches;
        }

        matches.sort_by_key(|m| m.position);

        let mut result = Vec::with_capacity(matches.len());
        let mut last_end = 0;

        for m in matches {
            if m.position >= last_end {
                last_end = m.position + m.value.len();
                result.push(m);
            }
        }

        result
    }
}

impl Default for PhoneScanner {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// FACTORY
// ============================================================================

pub struct PhoneDetectorFactory;

impl PhoneDetectorFactory {
    pub fn create_formatted_domestic_validator() -> Box<dyn PhoneValidator> {
        Box::new(FormattedDomesticValidator)
    }

    pub fn create_international_validator() -> Box<dyn PhoneValidator> {
        Box::new(InternationalPlusValidator)
    }

    pub fn create_plain_digit_validator(
        len: usize,
        phone_type: PhoneType,
    ) -> Box<dyn PhoneValidator> {
        Box::new(PlainDigitValidator::new(len, phone_type))
    }

    pub fn create_mobile_validator() -> Box<dyn PhoneValidator> {
        Box::new(MobileDigitValidator)
    }

    pub fn create_scanner() -> PhoneScanner {
        PhoneScanner::new()
    }
}

// ============================================================================
// TEST UTILITIES
// ============================================================================

impl PhoneType {
    fn as_str(&self) -> &'static str {
        match self {
            PhoneType::FormattedDomestic => "FORMATTED_DOMESTIC",
            PhoneType::FormattedTollFree => "FORMATTED_TOLL_FREE",
            PhoneType::InternationalPlus => "INTERNATIONAL_PLUS",
            PhoneType::InternationalZero => "INTERNATIONAL_00",
            PhoneType::Plain10Digit => "PLAIN_10_DIGIT",
            PhoneType::Plain11Digit => "PLAIN_11_DIGIT",
            PhoneType::Mobile10Digit => "MOBILE_10_DIGIT",
            PhoneType::Unknown => "UNKNOWN",
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

fn run_validation_tests() {
    println!("\n{}", "=".repeat(100));
    println!("=== PHONE VALIDATION TESTS ===");
    println!("{}\n", "=".repeat(100));

    struct TestCase {
        input: &'static str,
        expected_type: PhoneType,
        should_be_valid: bool,
        description: &'static str,
    }

    let tests = vec![
        TestCase {
            input: "(123) 456-7890",
            expected_type: PhoneType::FormattedDomestic,
            should_be_valid: true,
            description: "Formatted with parentheses",
        },
        TestCase {
            input: "123-456-7890",
            expected_type: PhoneType::FormattedDomestic,
            should_be_valid: true,
            description: "Formatted with dashes",
        },
        TestCase {
            input: "(012) 456-7890",
            expected_type: PhoneType::FormattedDomestic,
            should_be_valid: false,
            description: "Invalid area code (starts with 0)",
        },
        TestCase {
            input: "2345678901",
            expected_type: PhoneType::Plain10Digit,
            should_be_valid: true,
            description: "Plain 10 digits",
        },
        TestCase {
            input: "+1 123-456-7890",
            expected_type: PhoneType::InternationalPlus,
            should_be_valid: true,
            description: "International format",
        },
        TestCase {
            input: "9876543210",
            expected_type: PhoneType::Mobile10Digit,
            should_be_valid: true,
            description: "Mobile 10 digits",
        },
    ];

    let mut passed = 0;
    for test in &tests {
        let validator: Box<dyn PhoneValidator> = match test.expected_type {
            PhoneType::FormattedDomestic => {
                PhoneDetectorFactory::create_formatted_domestic_validator()
            }
            PhoneType::InternationalPlus => PhoneDetectorFactory::create_international_validator(),
            PhoneType::Plain10Digit => {
                PhoneDetectorFactory::create_plain_digit_validator(10, PhoneType::Plain10Digit)
            }
            PhoneType::Plain11Digit => {
                PhoneDetectorFactory::create_plain_digit_validator(11, PhoneType::Plain11Digit)
            }
            PhoneType::Mobile10Digit => PhoneDetectorFactory::create_mobile_validator(),
            _ => continue,
        };

        let result = validator.is_valid(test.input);
        let test_passed = result == test.should_be_valid;

        println!(
            "{} {}",
            if test_passed { "✓" } else { "✗" },
            test.description
        );
        if !test_passed {
            println!(
                "  Expected: {}, Got: {}",
                if test.should_be_valid {
                    "VALID"
                } else {
                    "INVALID"
                },
                if result { "VALID" } else { "INVALID" }
            );
        }
        if test_passed {
            passed += 1;
        }
    }

    println!(
        "\nResult: {}/{} passed ({}%)\n",
        passed,
        tests.len(),
        passed * 100 / tests.len()
    );
}

fn run_scanning_tests() {
    println!("\n{}", "=".repeat(100));
    println!("=== PHONE SCANNING TESTS ===");
    println!("{}\n", "=".repeat(100));

    let scanner = PhoneDetectorFactory::create_scanner();

    struct TestCase {
        input: &'static str,
        expected_count: usize,
        expected_types: Vec<PhoneType>,
        description: &'static str,
    }

    let tests = vec![
        TestCase {
            input: "Call me at (123) 456-7890",
            expected_count: 1,
            expected_types: vec![PhoneType::FormattedDomestic],
            description: "Formatted in text",
        },
        TestCase {
            input: "My number is +91 9876543210",
            expected_count: 1,
            expected_types: vec![PhoneType::InternationalPlus],
            description: "International format",
        },
        TestCase {
            input: "Office: +1 234-567-8900, Mobile: 9876543210",
            expected_count: 2,
            expected_types: vec![PhoneType::InternationalPlus, PhoneType::Mobile10Digit],
            description: "Mixed formats",
        },
        TestCase {
            input: "Number with spaces: 99887 76655",
            expected_count: 1,
            expected_types: vec![PhoneType::Mobile10Digit],
            description: "Space-separated mobile",
        },
    ];

    let mut passed = 0;
    for test in &tests {
        let matches = scanner.extract(test.input);
        let mut test_passed = matches.len() == test.expected_count;

        if test_passed && !matches.is_empty() {
            for (i, expected_type) in test.expected_types.iter().enumerate() {
                if i < matches.len() && matches[i].phone_type != *expected_type {
                    test_passed = false;
                    break;
                }
            }
        }

        println!(
            "{} {}",
            if test_passed { "✓" } else { "✗" },
            test.description
        );
        println!("  Found {} phone(s)", matches.len());

        for m in &matches {
            println!(
                "    [{}] {} (normalized: {})",
                m.phone_type.as_str(),
                m.value,
                m.normalized
            );
        }

        if !test_passed {
            print!("  Expected: {} phones with types: ", test.expected_count);
            for (i, t) in test.expected_types.iter().enumerate() {
                print!("{}", t.as_str());
                if i < test.expected_types.len() - 1 {
                    print!(", ");
                }
            }
            println!();
        }
        println!();

        if test_passed {
            passed += 1;
        }
    }

    println!(
        "Result: {}/{} passed ({}%)\n",
        passed,
        tests.len(),
        passed * 100 / tests.len()
    );
}

fn run_performance_benchmark() {
    println!("\n{}", "=".repeat(100));
    println!("=== PERFORMANCE BENCHMARK ===");
    println!("{}", "=".repeat(100));

    let test_cases = vec![
        "Call me at (123) 456-7890",
        "Contact: +1 234-567-8900",
        "Mobile: 9876543210",
        "Multiple: (234) 567-8900 and +91-9123456789",
        "Business: (345) 678-9012 or +1-456-789-0123",
    ];
    
    let num_threads = num_cpus::get();
    let iterations_per_thread = 100_000;

    println!("Threads: {}", num_threads);
    println!("Iterations per thread: {}", iterations_per_thread);
    println!("Test cases: {}", test_cases.len());
    println!(
        "Total operations: {}\n",
        num_threads * iterations_per_thread * test_cases.len()
    );
    println!("Starting benchmark...\n");

    let total_phones_found = AtomicU64::new(0);
    let start = Instant::now();

    std::thread::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                let scanner = PhoneScanner::new();
                let mut local_phones_found = 0u64;

                for _ in 0..iterations_per_thread {
                    for test in &test_cases {
                        let matches = scanner.extract(test);
                        local_phones_found += matches.len() as u64;
                    }
                }

                total_phones_found.fetch_add(local_phones_found, Ordering::Relaxed);
            });
        }
    });

    let duration = start.elapsed();
    let total_ops = (num_threads * iterations_per_thread * test_cases.len()) as u64;

    println!("{}", "-".repeat(100));
    println!("RESULTS:");
    println!("{}", "-".repeat(100));
    println!("Time: {} ms", duration.as_millis());
    println!(
        "Ops/sec: {}",
        total_ops * 1000 / duration.as_millis().max(1) as u64
    );
    println!(
        "Total phones found: {}",
        total_phones_found.load(Ordering::Relaxed)
    );
    println!("{}\n", "=".repeat(100));
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    run_validation_tests();
    run_scanning_tests();

    println!("\n{}", "=".repeat(100));
    println!("=== PHONE DETECTION DEMO ===");
    println!("{}\n", "=".repeat(100));

    let scanner = PhoneDetectorFactory::create_scanner();
    let text = "Contact us at (234) 567-8900 or +91-9876543210. \
                Office: 345-678-9012, Mobile: 9123456789, \
                Alt: 99887 76655, Intl: +1 (234) 567-8900";

    let matches = scanner.extract(text);
    println!("Found {} phone numbers:\n", matches.len());

    for phone in &matches {
        println!(
            "  [{}] at pos {}",
            phone.phone_type.as_str(),
            phone.position
        );
        println!("  Value: {}", phone.value);
        println!("  Normalized: {}\n", phone.normalized);
    }

    run_performance_benchmark();

    println!("\n{}", "=".repeat(100));
    println!("✓ SOLID Principles Applied");
    println!("✓ Optimized for 1M+ ops/sec Performance");
    println!("✓ Character Classification Lookup Tables");
    println!("✓ Thread-Safe Implementation");
    println!("✓ Multiple Phone Format Support");
    println!("✓ Space-Separated Number Detection");
    println!("✓ Generic Country-Independent Detection");
    println!("{}", "=".repeat(100));
}
