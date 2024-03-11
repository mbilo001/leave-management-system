## Leave Management System Documentation

### Overview
The Leave Management System is a web-based application designed to facilitate leave request management for employees. It provides functionalities for submitting, updating, and managing leave requests, as well as managing employee information. The system aims to streamline the leave reqeust process and provide a centralized platform for employees and administrators.

The application is built using Rust programming language with the Internet Computer (IC) Canister SDK, ensuring secure and decentralized management of leave requests and employee information. It leverages stable data structures for efficient storage and retrieval of data, providing a reliable platform for organizations to manage leave-related activities.

### Table of Contents
1. [Dependencies](#dependencies)
2. [Data Structures](#data-structures)
3. [Functions](#functions)
4. [Usage](#usage)
5. [Setting Up the Project](#setup)

### Dependencies <a name="dependencies"></a>
- `serde`: Serialization and deserialization library for Rust.
- `candid`: Library for Candid serialization and deserialization.
- `ic_stable_structures`: Library providing stable data structures for the Internet Computer.
- `std`: Standard library for Rust.

### Data Structures <a name="data-structures"></a>
#### Structs
1. `LeaveRequest`: Represents a leave request with fields such as ID, employee ID, start date, end date, reason, and status.
2. `Employee`: Represents an employee with fields including ID, name, department, position, and remaining leave days.

#### Enums
1. `LeaveStatus`: Represents the possible statuses for a leave request including Pending, Approved, and Rejected.

### Functions <a name="functions"></a>
The Leave Management System provides various functions for managing leave requests and employee information. Some key functions include:
- `submit_leave_request`: Submit a new leave request.
- `update_leave_request`: Update an existing leave request.
- `delete_leave_request`: Delete a leave request.
- `list_leave_requests`: List all leave requests.
- `register_employee`: Register a new employee.
- `update_employee`: Update an employee's information.
- `delete_employee`: Delete an employee.
- `list_employees`: List all employees.
- `approve_leave_request`: Approve a leave request.
- `reject_leave_request`: Reject a leave request.
- `calculate_leave_balance`: Calculate the leave balance of an employee.
- `get_leave_requests_by_employee_id`: Get leave requests by employee ID.
- `get_pending_leave_requests`: Get pending leave requests.

### Usage <a name="usage"></a>
The Leave Management System offers a user-friendly interface for employees and administrators to interact with the system. Employees can submit leave requests, view their leave balance, and track the status of their requests. Administrators can manage leave requests, approve or reject requests, and maintain employee information.

To use the application, users can navigate through the interface, perform desired actions, and interact with the system seamlessly. Proper error handling is implemented to handle cases such as invalid input or missing data.

### Setting Up the Project <a name="setup"></a>
To set up and start working on the Leave Management System project, follow these steps:

1. **Install Rust and Dependencies**
   - Ensure you have Rust installed, version 1.64 or higher. You can install it using the following commands:
     ```bash
     $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     $ source "$HOME/.cargo/env"
     ```
   - Install the `wasm32-unknown-unknown` target:
     ```bash
     $ rustup target add wasm32-unknown-unknown
     ```
   - Install `candid-extractor`:
     ```bash
     $ cargo install candid-extractor
     ```

2. **Install DFINITY SDK (`dfx`)**
   - Install `dfx` using the following commands:
     ```bash
     $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
     $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
     $ source ~/.bashrc
     $ dfx start --background
     ```

3. **Update Dependencies**
   - Update the `dependencies` block in `/src/{canister_name}/Cargo.toml` with the required dependencies.

4. **Autogenerate DID**
   - Add the provided script to the root directory of the project.
   - Update line 16 with the name of your canister.
   - Run the script each time you modify/add/remove exported functions of the canister.

5. **Running the Project Locally**
   - Start the replica, running in the background:
     ```bash
     $ dfx start --background
     ```
   - Deploy your canisters to the replica and generate your Candid interface:
     ```bash
     $ npm run gen-deploy
     ```