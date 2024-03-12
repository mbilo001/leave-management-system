#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell, str::FromStr};

// Simplified the type alias for Memory and IdCell for consistency
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct LeaveRequest {
    id: u64,
    employee_id: u64,
    start_date: u64, // Consider changing these to a more appropriate type, e.g., `chrono::NaiveDate`
    end_date: u64,
    reason: String,
    status: LeaveStatus,
}

#[derive(Debug, PartialEq, candid::CandidType, Deserialize, Serialize, Clone)]
enum LeaveStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Employee {
    id: u64,
    name: String,
    department: String,
    position: String,
    remaining_leave_days: u32,
}

impl Storable for LeaveRequest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            eprintln!("Serialization error: {}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            eprintln!("Deserialization error: {}", e);
            Default::default() // Implement or derive Default for a sensible default
        })
    }
}

impl BoundedStorable for LeaveRequest {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Employee {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap_or_else(|e| {
            eprintln!("Serialization error: {}", e);
            vec![]
        }))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap_or_else(|e| {
            eprintln!("Deserialization error: {}", e);
            Default::default() // Implement or derive Default for a sensible default
        })
    }
}

impl BoundedStorable for Employee {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static LEAVE_REQUEST_STORAGE: RefCell<StableBTreeMap<u64, LeaveRequest, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static EMPLOYEE_STORAGE: RefCell<StableBTreeMap<u64, Employee, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
    SerializationError { msg: String },
    DeserializationError { msg: String },
}
// Query to get a leave request by its ID
#[ic_cdk::query]
fn get_leave_request(request_id: u64) -> Result<LeaveRequest, Error> {
    match _get_leave_request(&request_id) {
        Some(request) => Ok(request),
        None => Err(Error::NotFound {
            msg: format!("leave request with id={} not found", request_id),
        }),
    }
}

// Update to submit a leave request
#[ic_cdk::update]
fn submit_leave_request(employee_id: u64, start_date: u64, end_date: u64, reason: String) -> Result<LeaveRequest, Error> {
    // Validate input data and check remaining leave days

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let request = LeaveRequest {
        id,
        employee_id,
        start_date,
        end_date,
        reason,
        status:  LeaveStatus::Pending,
    };

    LEAVE_REQUEST_STORAGE.with(|service| service.borrow_mut().insert(id, request.clone()));
    Ok(request)
}

// Update to update a leave request
#[ic_cdk::update]
fn update_leave_request(request_id: u64, start_date: u64, end_date: u64, reason: String) -> Result<LeaveRequest, Error> {
    // Validate input data and check remaining leave days

    match _get_leave_request(&request_id) {
        Some(mut request) => {
            request.start_date = start_date;
            request.end_date = end_date;
            request.reason = reason;
            LEAVE_REQUEST_STORAGE.with(|service| service.borrow_mut().insert(request_id, request.clone()));
            Ok(request)
        },
        None => Err(Error::NotFound {
            msg: format!("leave request with id={} not found", request_id),
        }),
    }
}

// Update to delete a leave request
#[ic_cdk::update]
fn delete_leave_request(request_id: u64) -> Result<(), Error> {
    match LEAVE_REQUEST_STORAGE.with(|service| service.borrow_mut().remove(&request_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("leave request with id={} not found", request_id),
        }),
    }
}

// Query to list all leave requests
#[ic_cdk::query]
fn list_leave_requests() -> Vec<LeaveRequest> {
    LEAVE_REQUEST_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, request)| request.clone())
            .collect()
    })
}

// Helper function to get a leave request by its ID
fn _get_leave_request(request_id: &u64) -> Option<LeaveRequest> {
    LEAVE_REQUEST_STORAGE.with(|service| service.borrow().get(request_id))
}

// Query to get an employee by their ID
#[ic_cdk::query]
fn get_employee(employee_id: u64) -> Result<Employee, Error> {
    match _get_employee(&employee_id) {
        Some(employee) => Ok(employee),
        None => Err(Error::NotFound {
            msg: format!("employee with id={} not found", employee_id),
        }),
    }
}

// Update to register a new employee
#[ic_cdk::update]
fn register_employee(name: String, department: String, position: String, remaining_leave_days: u32) -> Result<Employee, Error> {
    // Validate input data

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let employee = Employee {
        id,
        name,
        department,
        position,
        remaining_leave_days,
    };

    EMPLOYEE_STORAGE.with(|service| service.borrow_mut().insert(id, employee.clone()));
    Ok(employee)
}

// Update to update an employee's information
#[ic_cdk::update]
fn update_employee(employee_id: u64, name: String, department: String, position: String, remaining_leave_days: u32) -> Result<Employee, Error> {
    // Validate input data

    match _get_employee(&employee_id) {
        Some(mut employee) => {
            employee.name = name;
            employee.department = department;
            employee.position = position;
            employee.remaining_leave_days = remaining_leave_days;
            EMPLOYEE_STORAGE.with(|service| service.borrow_mut().insert(employee_id, employee.clone()));
            Ok(employee)
        },
        None => Err(Error::NotFound {
            msg: format!("employee with id={} not found", employee_id),
        }),
    }
}

// Update to delete an employee
#[ic_cdk::update]
fn delete_employee(employee_id: u64) -> Result<(), Error> {
    match EMPLOYEE_STORAGE.with(|service| service.borrow_mut().remove(&employee_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("employee with id={} not found", employee_id),
        }),
    }
}

// Query to list all employees
#[ic_cdk::query]
fn list_employees() -> Vec<Employee> {
    EMPLOYEE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, employee)| employee.clone())
            .collect()
    })
}

// Helper function to get an employee by their ID
fn _get_employee(employee_id: &u64) -> Option<Employee> {
    EMPLOYEE_STORAGE.with(|service| service.borrow().get(employee_id))
}

// Update to approve a leave request
#[ic_cdk::update]
fn approve_leave_request(request_id: u64) -> Result<(), Error> {
    match _get_leave_request(&request_id) {
        Some(mut request) => {
            request.status = LeaveStatus::Approved;
            LEAVE_REQUEST_STORAGE.with(|service| service.borrow_mut().insert(request_id, request.clone()));
            Ok(())
        },
        None => Err(Error::NotFound {
            msg: format!("leave request with id={} not found", request_id),
        }),
    }
}

// Update to reject a leave request
#[ic_cdk::update]
fn reject_leave_request(request_id: u64) -> Result<(), Error> {
    match _get_leave_request(&request_id) {
        Some(mut request) => {
            request.status = LeaveStatus::Rejected;
            LEAVE_REQUEST_STORAGE.with(|service| service.borrow_mut().insert(request_id, request.clone()));
            Ok(())
        },
        None => Err(Error::NotFound {
            msg: format!("leave request with id={} not found", request_id),
        }),
    }
}

// Query to calculate the leave balance of an employee
#[ic_cdk::query]
fn calculate_leave_balance(employee_id: u64) -> Result<u32, Error> {
    match _get_employee(&employee_id) {
        Some(employee) => Ok(employee.remaining_leave_days),
        None => Err(Error::NotFound {
            msg: format!("employee with id={} not found", employee_id),
        }),
    }
}

// Query to get leave requests by employee ID
#[ic_cdk::query]
fn get_leave_requests_by_employee_id(employee_id: u64) -> Vec<LeaveRequest> {
    LEAVE_REQUEST_STORAGE
        .with(|service| {
            service
                .borrow()
                .iter()
                .filter(|(_, request)| request.employee_id == employee_id)
                .map(|(_, request)| request.clone())
                .collect()
        })
}

// Query to get pending leave requests
#[ic_cdk::query]
fn get_pending_leave_requests() -> Vec<LeaveRequest> {
    LEAVE_REQUEST_STORAGE
        .with(|service| {
            service
                .borrow()
                .iter()
                .filter(|(_, request)| request.status == LeaveStatus::Pending)
                .map(|(_, request)| request.clone())
                .collect()
        })
}

// Export the Candid interface
ic_cdk::export_candid!();
