// Copyright 2025 Shingo OKAWA. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Prints a message.
fn print(message: impl AsRef<str> + std::fmt::Display) {
    println!("{}", message);
}

/// An awesome feature.
fn do_something() {
    println!("Do something");
}

/// Entry point.
fn main() {
    print("Hello, cliff!");
    do_something();
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_something() {
        assert!(true);
    }
}
