# Testing Guide

This project includes comprehensive automated tests to ensure all commands work as expected.

## Setup

1. **Copy the environment template:**
   ```bash
   cp .env.example .env
   ```

2. **Configure your test values in `.env`:**
   - `TEST_APP_ID`: Steam App ID for a game you want to test with
   - `TEST_ITEM_ID`: A workshop item ID for testing (used for both basic testing and subscription tests)
   - `TEST_COLLECTION_ID`: A workshop collection ID for testing

## Running Tests

### Run ALL Tests (One Command)
```bash
cargo test -- --include-ignored
```
**This single command runs everything** - safe tests, Steam-dependent tests, and subscription tests.

⚠️ **WARNING**: This includes tests that will subscribe/unsubscribe from workshop items!

### Run Only Safe Tests (No Steam Modifications)
```bash
cargo test
```
This runs only tests that don't modify your Steam subscriptions.

### Run Only Steam Modification Tests
```bash
cargo test -- --ignored
```
This runs only the tests that modify Steam subscriptions.

## Test Behavior

- **Subscription Test Intelligence**: The `test_subscribe_and_unsubscribe` test is smart - it checks your current subscription state and restores it after testing
- **Graceful Failure**: Tests expect commands to either succeed or fail gracefully with error messages
- **JSON Validation**: All tests verify that commands return valid JSON
- **Steam Availability**: Tests automatically detect if Steam is available and skip Steam-dependent tests if needed

## What Gets Tested

- **CLI Help Commands**: Verify help output is correct
- **Workshop Item Operations**: Fetch item info, check downloads, browse collections
- **Search Operations**: Test all search sorting methods and tag discovery
- **Cache Operations**: Test cache clearing
- **Subscription Operations**: Subscribe/unsubscribe while preserving your original state
- **JSON Output**: All commands verified to return valid JSON
