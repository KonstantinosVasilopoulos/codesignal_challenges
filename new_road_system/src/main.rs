fn solution(road_register: Vec<Vec<bool>>) -> bool {
    // Generate two vectors storing the number of roads per city
    let mut city_roads: Vec<i32> = vec![];
    for _ in 0..road_register.len() {
        city_roads.push(0);
    }

    // Iterate over the road register and count the incoming and outcoming connections
    for (i, row) in road_register.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column {
                city_roads[i] += 1;
                city_roads[j] -= 1;
            }
        }
    }

    // Assert that each city has the exact number of incoming and outcoming roads
    // Every city's vector entry ought to be 0
    for roads in city_roads.iter() {
        if *roads != 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut road_register = vec![
        vec![false, true,  false, false],
        vec![false, false, true,  false],
        vec![true,  false, false, true ],
        vec![false, false, true,  false],
    ];
    assert!(solution(road_register));

    road_register = vec![
        vec![false, true,  false, false, false, false, false],
        vec![true,  false, false, false, false, false, false],
        vec![false, false, false, true,  false, false, false],
        vec![false, false, true,  false, false, false, false],
        vec![false, false, false, false, false, false, true ],
        vec![false, false, false, false, true,  false, false],
        vec![false, false, false, false, false, true,  false],
    ];
    assert!(solution(road_register));
}
