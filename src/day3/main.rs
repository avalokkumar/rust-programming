mod regex_parser;

use crate::regex_parser::match_regex;

fn main() {
   // Example 1: Simple match without groups
   let input_str = "The quick brown fox jumps over the lazy dog.";
   let regex_str = r#"brown\sfox"#;
   let result = match_regex(&regex_str, &input_str);
   println!("Example 1: {:?}", result); // Output: Some(["brown fox"])

   // Example 2: Match with groups
   let input_str = "John Smith was born on 1990-05-20.";
   let regex_str = r#"(\w+)\s(\w+)\swas born on (\d{4}-\d{2}-\d{2})"#;
   let result = match_regex(&regex_str, &input_str);
   println!("Example 2: {:?}", result); // Output: Some(["John", "Smith", "1990-05-20"])

   // Example 3: Match with optional groups
   let input_str = "John Smith was born on 1990-05-20.";
   let regex_str = r#"(\w+)\s(\w+)(?:\swas born on (\d{4}-\d{2}-\d{2}))?"#;
   let result = match_regex(&regex_str, &input_str);
   println!("Example 3: {:?}", result); // Output: Some(["John", "Smith", "1990-05-20"])

   // Example 4: Non-matching regex
   let input_str = "This is a test.";
   let regex_str = r#"foo"#;
   let result = match_regex(&regex_str, &input_str);
   println!("Example 4: {:?}", result); // Output: None

   // Example 5: Match with repeated groups
   let input_str = "The quick brown fox jumps over the lazy dog.";
   let regex_str = r#"(\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+)\."#;
   let result = match_regex(&regex_str, &input_str);
   println!("Example 5: {:?}", result); // Output: Some(["The", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"])


   // Example 6: Match with nested groups
   let input_str = "My phone number is (123) 456-7890.";
   let regex_str = r#"My phone number is \((\d{3})\) (\d{3})-(\d{4})\."#;
   let result = match_regex(&regex_str, &input_str);
   println!("Example 6: {:?}", result); // Output: Some(["123", "456", "7890"])
}