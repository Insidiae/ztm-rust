// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)

struct LuggageId(usize);

// * Optionally use generics for each state
struct Luggage<State> {
	id: LuggageId,
	state: State,
}

struct CheckIn;
struct OnLoading;
struct OffLoading;
struct AwaitingPickup;
struct EndCustody;

impl<State> Luggage<State> {
	//? Not borrowing the self ref lets us deprecate the previous state
	fn transition<NextState>(self, state: NextState) -> Luggage<NextState> {
		Luggage { id: self.id, state }
	}
}

impl Luggage<CheckIn> {
	fn new(id: LuggageId) -> Self {
		Self { id, state: CheckIn }
	}

	fn on_load(self) -> Luggage<OnLoading> {
		self.transition(OnLoading)
	}
}

impl Luggage<OnLoading> {
	fn off_load(self) -> Luggage<OffLoading> {
		self.transition(OffLoading)
	}
}

impl Luggage<OffLoading> {
	fn carousel(self) -> Luggage<AwaitingPickup> {
		self.transition(AwaitingPickup)
	}
}

impl Luggage<AwaitingPickup> {
	fn pickup(self) -> Luggage<EndCustody> {
		self.transition(EndCustody)
	}
}

fn main() {
	let luggage = Luggage::new(LuggageId(69));
	let luggage = luggage.on_load().off_load().carousel();
	luggage.pickup();
}
