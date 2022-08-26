// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle {
	Car,
	Truck,
}

#[derive(Debug, Hash, PartialOrd, PartialEq)]
enum Status {
	Available,
	Unavailable,
	Maintenance,
	Rented,
}

#[derive(Debug)]
struct Rental {
	vehicle: Vehicle,
	vin: String,
	status: Status,
}

type Rentals = Rc<RefCell<Vec<Rental>>>;

#[derive(Debug)]
struct Corporate {
	rentals: Rentals,
}

impl Corporate {
	fn unlist_vehicle(&self, idx: usize) {
		let mut rentals = self.rentals.borrow_mut();
		if let Some(vehicle) = rentals.get_mut(idx) {
			vehicle.status = Status::Unavailable;
		}
	}
}

#[derive(Debug)]
struct StoreFront {
	rentals: Rentals,
}

impl StoreFront {
	fn rent_vehicle(&self, idx: usize) {
		let mut rentals = self.rentals.borrow_mut();
		if let Some(vehicle) = rentals.get_mut(idx) {
			vehicle.status = Status::Rented;
		}
	}
}

fn main() {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn update_status() {
		let vehicles = vec![
			Rental {
				vehicle: Vehicle::Car,
				vin: String::from("123"),
				status: Status::Available,
			},
			Rental {
				vehicle: Vehicle::Truck,
				vin: String::from("abc"),
				status: Status::Available,
			},
		];
		let rentals = Rc::new(RefCell::new(vehicles));

		let storefront = StoreFront {
			rentals: Rc::clone(&rentals),
		};
		let corporate = Corporate {
			rentals: Rc::clone(&rentals),
		};

		storefront.rent_vehicle(1);
		corporate.unlist_vehicle(0);

		dbg!(rentals.borrow());

		let mut rental_status = rentals.borrow_mut();
		if let Some(car) = rental_status.get_mut(0) {
			assert_eq!(car.status, Status::Unavailable)
		}
		if let Some(truck) = rental_status.get_mut(1) {
			assert_eq!(truck.status, Status::Rented)
		}
	}
}
