pub struct Solution;

#[derive(Clone, Copy)]
#[repr(usize)]
enum Garbage {
    Glass,
    Metal,
    Paper,
}

#[derive(Default)]
struct GarbagePile([u8; 3]);
impl GarbagePile {
    fn remove_piece(&mut self, garbage: Garbage) -> bool {
        if self.0[garbage as usize] != 0 {
            self.0[garbage as usize] -= 1;
            true
        } else {
            false
        }
    }
}
impl From<String> for GarbagePile {
    fn from(garbage: String) -> Self {
        let mut pile = Self::default();

        for piece_of_garbage in garbage.bytes() {
            match piece_of_garbage {
                b'G' => {
                    pile.0[Garbage::Glass as usize] += 1;
                }
                b'M' => {
                    pile.0[Garbage::Metal as usize] += 1;
                }
                _ => {
                    pile.0[Garbage::Paper as usize] += 1;
                }
            }
        }

        pile
    }
}

struct GarbageCollection(Vec<GarbagePile>);
impl GarbageCollection {
    fn collect_garbage(&mut self, garbage: Garbage, route: &[i32]) -> i32 {
        let mut time = 0;
        // Route time is calculated separately, as if all further stops after the last stop don't
        // have garbage of the specified type, we skip that part of the route.
        let mut route_time = 0;
        for (next_route_stop, garbage_pile) in self.0.iter_mut().enumerate() {
            while garbage_pile.remove_piece(garbage) {
                // If we pick up trash, add the route time to that point and reset it to zero.
                if route_time != 0 {
                    time += route_time;
                    route_time = 0;
                }

                // Every time we pick up trash, one minute passes.
                time += 1;
            }

            // Keep track of time spent on route. If no further trash is picked up, this never gets
            // added to time.
            if let Some(time_to_next_stop) = route.get(next_route_stop) {
                route_time += time_to_next_stop;
            }
        }

        time
    }
}
impl From<Vec<String>> for GarbageCollection {
    fn from(household_trashes: Vec<String>) -> Self {
        let mut garbage_collection = Self(Vec::with_capacity(household_trashes.len()));

        for household_trash in household_trashes {
            garbage_collection.0.push(household_trash.into());
        }

        garbage_collection
    }
}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut garbage_collection: GarbageCollection = garbage.into();
        let mut time = 0;

        time += garbage_collection.collect_garbage(Garbage::Glass, &travel);
        time += garbage_collection.collect_garbage(Garbage::Metal, &travel);
        time += garbage_collection.collect_garbage(Garbage::Paper, &travel);

        time
    }
}
