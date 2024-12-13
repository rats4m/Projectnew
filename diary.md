# Project Diary

---

### 9th–12th December 2024

**9th December:**  
- Reviewed project context, files, and codebase to ensure alignment with interim goals.  
- Enhanced `anomaly_detection.rs` with robust logging and additional edge case handling.  
- Drafted and tested `anomaly_detection_test.rs` to validate detection logic.  

**10th December:**  
- Designed and implemented `real_time.rs` for rendering real-time anomaly visualizations.  
- Integrated `plotters` crate for graphing anomalies dynamically.  
- Verified rendering functionality with mock data and real-world datasets.  

**11th December:**  
- Developed `logger.rs` for centralized logging, ensuring pipeline observability.  
- Added `error.rs` to standardize error reporting across modules.  
- Integrated all modules in `main.rs`, connecting data ingestion, ML, and visualization.  
- Conducted preliminary end-to-end tests to identify bottlenecks.  

**12th December:**  
- Finalized and validated integration tests in `integration_test.rs`.  
- Updated `README.md` with detailed instructions for running the interim pipeline.  
- Logged all milestones in `diary.md` for traceability.  
- Prepared codebase for demo video creation showcasing functionality.  

---

## 5th December 2024

### Progress Update: ML Engine Integration

- **Implemented Features:**
  - Created `anomaly_detection.rs` to identify suspicious records.
  - Developed logic to filter anomalies based on the "event_type" field.
  - Wrote comprehensive unit tests in `anomaly_detection_test.rs` for the module.
  - Added integration tests in `integration_tests.rs` to validate seamless data flow from preprocessing to anomaly detection.

- **Testing:**
  - All tests passed successfully:
    - Verified preprocessing pipeline functions (`remove_duplicates`, `normalize_fields`, `filter_irrelevant_data`) work as expected.
    - Confirmed accurate anomaly detection for test scenarios.

- **Result:**
  - The preprocessing and ML engine modules are fully integrated and functional.


---

## 4th December 2024

### Progress Update: Preprocessing Module

- **Features Added:**
  - `remove_duplicates` function to eliminate duplicate records.
  - `normalize_fields` function to standardize data like timestamps and IP addresses.
  - `filter_irrelevant_data` function to keep only important fields.

- **Testing:**
  - All functions were tested in `preprocessor_test.rs`:
    - Duplicates were removed correctly.
    - Fields were normalized as expected.
    - Filtering worked for different test cases.

- **Result:**
  - All tests passed, and the preprocessing module is ready for integration with the ML engine.

---

## 3rd December 2024

### Progress Update

- **Refactored Code:**
  - Updated `file_loader.rs` to align with code preferences, including:
    - Enhanced error handling for better readability and debugging.
    - Improved variable naming for clarity and maintainability.
  - Refactored `file_loader_test.rs` to:
    - Extract user-defined variables (e.g., file paths, test content) to the top of the file for better organization.
    - Added the `use PROJECT::data_ingestion::file_loader::{load_csv, load_json};` import for proper function access.
    - Simplified and clarified test case structure.

- **Validation and Review:**
  - Ensured that the updated code aligns with modular design principles and preferences for clarity and maintainability.
  - Confirmed no additional changes were made to unrelated parts of the `repo_contents.json`.

*Note: The lack of updates in recent months was due to unforeseen circumstances. Progress has now resumed with full focus and commitment.*

---

## 3rd November 2024

### Current Position

- **Code Breakdown**:

	- **`data_ingestion.rs`**: This file sets up a basic TCP listener using the Tokio library to handle real-time data ingestion.

		- **Library Imports**: Uses `tokio::net::TcpListener` for asynchronous networking and `std::io` for input/output handling.

		- **Data Handling**: Imports `deserialize_event` and `Event` from `data_serialization.rs`.

		- **Function `ingest_realtime_data`**:
			- Binds the listener to IP `127.0.0.1` on port `8080`.
			- Runs a loop to accept incoming connections.
			- For each connection, reads and processes data from the socket (currently using a placeholder `dummy_data` string).
			- Deserializes the `dummy_data` into an `Event` and prints it.

		- This function is asynchronous, making it suitable for real-time data handling.

	- **`data_serialization.rs`**: Defines the data structure for an `Event` and includes serialization/deserialization methods.

		- **Struct `Event`**:
			- Fields: `timestamp`, `source`, `destination`, `event_type`, and an optional `data` field, which align with typical cybersecurity event logs.

		- **Serialization and Deserialization**:
			- `serialize_event`: Converts an `Event` into a JSON string.
			- `deserialize_event`: Converts a JSON string back into an `Event`.

		- **Testing Module**:
			- Contains unit tests for `serialize_event` and `deserialize_event` functions to ensure data integrity during serialization and deserialization.

	- **Summary**: Both files are in early stages but structured well for basic real-time data ingestion and deserialization of cybersecurity event data.


### Next Steps

- **Refine and Test Data Serialization**:
	- Run `serialize_event` and `deserialize_event` functions with various data inputs to test robustness.
	- Test edge cases (e.g., missing fields, unusual characters in the `data` field) to ensure error handling is solid.

- **Parameterize the IP and Port in `data_ingestion.rs`**:
	- Update the `ingest_realtime_data` function to accept IP and port as parameters for easier modification.
	- Set up a configuration file for handling multiple addresses.

- **Implement Basic Logging**:
	- Add logging for each step in `ingest_realtime_data` (e.g., connection established, data received, deserialization result).
	- Logging is useful for debugging and for tracking real-time events during testing.

- **Update Diary and GitLab**:
	- Document progress in the project diary and update GitLab to reflect changes.


### Progress Update

- Began refining testing but have not yet moved on to the other steps.

---

## 1st November 2024

- Created an event structure for representing cybersecurity data.
- Implemented serialization and deserialization using Serde.
- Set up real-time data ingestion using Tokio and a TCP listener (using dummy data).

### Next Steps:
- Implement offline ingestion for batch data processing.
- Transition from dummy data to real-time data in ingestion.

---

## 22nd October 2024

- Decided not to use Python due to its lack of desired complexity for this project.
- Made changes to the project scope and tools:
  - Use Rust for data collection and processing.
  - Use JavaScript + WebGL for visualization frameworks.
  - Employ Rust for predictive learning.

---

## 18th October 2024

- Cloned the project repository and ensured it follows the correct structure according to project guidelines.
- Created a test branch in the repository to practice techniques before integrating them into the main project.
- Began initial testing by creating a script for synthetic data simulation.

### Next Steps:
- Continue research on data visualization techniques specific to cybersecurity.
- Focus on datasets for network traffic and host-based activities.

---

## 10th October 2024

- Completed the project plan.
- Identified risks and proposed mitigations in the project plan.

---

## 8th October 2024

- Completed a significant portion of the project plan.
- Defined the project scope.
- Established a timeline for key deliverables and personal goals.

---

## 4th October 2024

- Held the initial meeting with the supervisor.
- Discussed potential challenges with real-time data prediction.
- Highlighted the importance of defining the project scope before diving into development.
