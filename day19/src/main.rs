fn main() {
    // ore -> clay -> obsidian -> geode
    let blueprints: Vec<_> = std::io::stdin()
        .lines()
        .map(|l| {
            let line = l.unwrap();
            // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.
            let parts: Vec<_> = line.split(" ").collect();
            // costs
            [
                [parts[6].parse::<isize>().unwrap(), 0, 0, 0],
                [parts[12].parse::<isize>().unwrap(), 0, 0, 0],
                [
                    parts[18].parse::<isize>().unwrap(),
                    parts[21].parse::<isize>().unwrap(),
                    0,
                    0,
                ],
                [
                    parts[27].parse::<isize>().unwrap(),
                    0,
                    parts[30].parse::<isize>().unwrap(),
                    0,
                ],
            ]
        })
        .collect();

    let mut now = std::time::SystemTime::now();
    let mut quality = 0;
    for (i, blueprint) in blueprints.into_iter().enumerate() {
        let score = evaluate_blueprint(&blueprint);
        println!("{i} {score} time: {}", now.elapsed().unwrap().as_secs());
        now = std::time::SystemTime::now();
        quality += score * (i + 1);
    }
    println!("quality: {quality}",);
}

// How about a DP approach?
// what is the best number of geodes at time T?
// what is best number of robots geode-robots at time T?
// wait, but if you know that at time T you can have N geodes and M geode robots, then that does
// not mean that at time T+1 you can have N+M geodes, because to get to N geodes you built fewer
// than M geode robots.
//
// How can I prune states better?

const MAX_TIME: usize = 32;
// 375 was too low
fn evaluate_blueprint(blueprint: &[[isize; 4]; 4]) -> usize {
    // use std::collections::VecDeque;
    const GEODES: usize = 3;
    struct State {
        time: usize,
        materials: [isize; 4], //ore, clay, obsidian, geodes
        robots: [isize; 4],    // ore, clay, obsidian, geodes
    }
    let mut best = 0;
    let mut best_at_least = [0; MAX_TIME + 1];
    let mut best_count = [0; MAX_TIME + 1];
    fn dfs(
        State {
            time,
            materials,
            robots,
        }: State,
        best: &mut isize,
        best_at_least: &mut [isize; MAX_TIME + 1],
        best_count: &mut [isize; MAX_TIME + 1],
        blueprint: &[[isize; 4]; 4],
    ) -> isize {
        if time > MAX_TIME {
            return 0;
        }
        let remaining_time = (MAX_TIME - time) as isize;
        // let will_produce_at_least = materials[GEODES] + robots[GEODES] * remaining_time;
        let will_produce_at_most = materials[GEODES]
            + (robots[GEODES] + robots[GEODES] + remaining_time - 1) * remaining_time / 2;
        if will_produce_at_most <= best_at_least[time] {
            return 0;
        }
        *best = (*best).max(materials[GEODES]);

        // TODO: Is this sound heuristic?
        // if materials[GEODES] < best_count[time] {
        //     return 0;
        // }
        // best_count[time] = materials[GEODES];

        if time == MAX_TIME {
            best_at_least[time] = best_at_least[time].max(materials[GEODES]);
            return materials[GEODES];
        }

        // println!("{time} {materials:?} {robots:?}");
        let mut ans = 0;

        for i in (0..4).rev() {
            if let Some(materials) = sub(materials, blueprint[i]) {
                let mut new_robots = robots;
                new_robots[i] += 1;
                let x = dfs(
                    State {
                        time: time + 1,
                        materials: add(materials, robots),
                        robots: new_robots,
                    },
                    best,
                    best_at_least,
                    best_count,
                    blueprint,
                );
                ans = ans.max(x);
                // // TODO: Is this sound optimization?
                // if i == 3 {
                //     break; // no reason to try to build inferior robots
                //            // is that true?
                // }
                // // break;
            }
        }

        // no robots
        let x = dfs(
            State {
                time: time + 1,
                materials: add(materials, robots),
                robots,
            },
            best,
            best_at_least,
            best_count,
            blueprint,
        );
        ans = ans.max(x);

        best_at_least[time] = best_at_least[time].max(ans);
        return ans;
    }

    dfs(
        State {
            time: 0,
            materials: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
        },
        &mut best,
        &mut best_at_least,
        &mut best_count,
        &blueprint,
    );
    best as usize
}

fn add(x: [isize; 4], y: [isize; 4]) -> [isize; 4] {
    let mut ans = [0; 4];
    for (i, (x, y)) in x.iter().zip(y).enumerate() {
        ans[i] = x + y;
    }
    ans
}
fn sub(x: [isize; 4], y: [isize; 4]) -> Option<[isize; 4]> {
    let mut ans = [0; 4];
    for (i, (x, y)) in x.iter().zip(y).enumerate() {
        ans[i] = x - y;
        if ans[i] < 0 {
            return None;
        }
    }
    Some(ans)
}
