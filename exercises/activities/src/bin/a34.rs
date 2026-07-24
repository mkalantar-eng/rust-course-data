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
// Notes:
// * Optionally use generics for each state

#[derive(Debug)]
struct Luggage<T> {
    tracking_id: String,
    state: T,
}
#[derive(Debug)]
struct CheckIn;
impl Luggage<CheckIn> {
    fn new(tracking_id: String) -> Self {
        Self {
            tracking_id,
            state: CheckIn,
        }
    }
    fn load(self) -> Luggage<OnLoading> {
        Luggage::<OnLoading> {
            tracking_id: self.tracking_id,
            state: OnLoading,
        }
    }
}
#[derive(Debug)]
struct OnLoading;
impl Luggage<OnLoading> {
    fn offload(self) -> Luggage<Offloading> {
        Luggage::<Offloading> {
            tracking_id: self.tracking_id,
            state: Offloading,
        }
    }
}
#[derive(Debug)]
struct Offloading;
impl Luggage<Offloading> {
    fn await_pickup(self) -> Luggage<AwaitingPickup> {
        Luggage::<AwaitingPickup> {
            tracking_id: self.tracking_id,
            state: AwaitingPickup,
        }
    }
}
#[derive(Debug)]
struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    fn end_custody(self) -> Luggage<EndCustody> {
        Luggage::<EndCustody> {
            tracking_id: self.tracking_id,
            state: EndCustody,
        }
    }
}
#[derive(Debug)]
struct EndCustody;

fn main() {
    let luggage = Luggage::new("ftre44380k".to_string());
    let c = luggage.load().offload().await_pickup().end_custody();
    println!("{:?}", c);
}
