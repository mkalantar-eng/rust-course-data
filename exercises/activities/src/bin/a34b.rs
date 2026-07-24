// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement using rust a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * use generics for each state

use std::marker::PhantomData;

// ---------- States ----------

#[derive(Debug)]
struct CheckIn;

#[derive(Debug)]
struct OnLoading;

#[derive(Debug)]
struct Offloading;

#[derive(Debug)]
struct AwaitingPickup;

#[derive(Debug)]
struct EndCustody;

// ---------- Luggage ----------

#[derive(Debug)]
struct Luggage<State> {
    tracking_id: String,

    // We don't store a value of type `State`.
    // PhantomData tells the compiler that this struct is logically associated
    // with the generic state, making `Luggage<CheckIn>`,
    // `Luggage<OnLoading>`, etc. distinct compile-time types without
    // consuming any memory at runtime.
    _state: PhantomData<State>,
}

// ---------- Initial state ----------

impl Luggage<CheckIn> {
    fn check_in(tracking_id: impl Into<String>) -> Self {
        Self {
            tracking_id: tracking_id.into(),
            _state: PhantomData,
        }
    }

    fn start_loading(self) -> Luggage<OnLoading> {
        println!(
            "Luggage {} is being loaded onto the plane.",
            self.tracking_id
        );

        self.transition()
    }
}

// ---------- OnLoading state ----------

impl Luggage<OnLoading> {
    fn finish_loading_and_fly(self) -> Luggage<Offloading> {
        println!(
            "Luggage {} arrived and is being offloaded.",
            self.tracking_id
        );

        self.transition()
    }
}

// ---------- Offloading state ----------

impl Luggage<Offloading> {
    fn send_to_pickup_area(self) -> Luggage<AwaitingPickup> {
        println!(
            "Luggage {} is waiting in the pickup area.",
            self.tracking_id
        );

        self.transition()
    }
}

// ---------- AwaitingPickup state ----------

impl Luggage<AwaitingPickup> {
    fn pickup(self) -> Luggage<EndCustody> {
        println!(
            "Luggage {} was picked up by the passenger.",
            self.tracking_id
        );

        self.transition()
    }
}

// ---------- Final state ----------

impl Luggage<EndCustody> {
    fn is_complete(&self) -> bool {
        true
    }
}

// ---------- Shared functionality ----------

impl<State> Luggage<State> {
    fn tracking_id(&self) -> &str {
        &self.tracking_id
    }

    fn transition<NextState>(self) -> Luggage<NextState> {
        Luggage {
            tracking_id: self.tracking_id,
            _state: PhantomData,
        }
    }
}

// ---------- Example ----------

fn main() {
    let luggage = Luggage::<CheckIn>::check_in("BAG-2026-0001");

    println!("Tracking ID: {}", luggage.tracking_id());

    let luggage = luggage.start_loading();
    let luggage = luggage.finish_loading_and_fly();
    let luggage = luggage.send_to_pickup_area();
    let luggage = luggage.pickup();

    println!(
        "Tracking completed for {}: {}",
        luggage.tracking_id(),
        luggage.is_complete()
    );
}
