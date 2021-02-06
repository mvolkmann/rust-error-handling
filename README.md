# rust-error-handling

This demonstrates several ways of handling errors
in functions that return multiple error types.

Try changing the `file_path` variable in `src/main.rs`
to a non-existent file to trigger a `std::error::Error` kind of error.

Try deleting a required character in `dogs.json`
to trigger a `serde_json::error::Error` kind of error.
