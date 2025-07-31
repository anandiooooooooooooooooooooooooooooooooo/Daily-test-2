use crate::app::{handle_add, handle_list};
use crate::infrastructure::fs_storage::FileTaskRepo;

pub fn run_cli() {
    let args: Vec<String> = std::env::args().collect();
    let mut repo = FileTaskRepo::new("tasks.json".to_string());
    repo.load_from_file().unwrap();

    match args.get(1).map(String::as_str) {
        Some("add") => {
            let title = args.get(2).cloned().unwrap_or_else(|| "Untitled".to_string());
            let desc = args.get(3).cloned();
            handle_add(&mut repo, title, desc);
        }
        Some("list") => handle_list(&repo),
        _ => println!("Available commands: add <title> [desc], list"),
    }
}// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [jwleHZsGkjVm]
// @1751239674 [cZ6gQsSQBD8p]
// @1751241407 [Fh5B4gBZ8Cnr]
// @1751241894 [ojfI1A0hIN6M]
// @1751253636 [6VWFfRnWdDV3]
// @1751340332 [DKJLbYGJ0bfq]
// @1751598844 [4u6giRkePkWR]
// @1752031075 [qtqbp75CW5JM]
// @1752117454 [dJ9Ine9E5c7Z]
// @1752204184 [gNwwkqGDwERD]
// @1752722589 [3NdOqfHTObg9]
// @1752809081 [KdHZXX1n7NoB]
// @1752895106 [xCO6pWDJVBM3]
// @1753069183 [ERC1L3ryl6br]
// @1753154678 [qoW6RfLZ0ky1]
// @1753241182 [UwzYxrkTgVKT]
// @1753327479 [GMbvnXzLMIZL]
// @1753413881 [BXT19us1e1oa]
// @1753500095 [12hPWmqJYbfD]
// @1753641161 [8hX0rEHORrOA]
// @1753650888 [HFWNOlI2sTKQ]
// @1753663908 [85L9htpniNFX]
// @1753684652 [6XIXgwhKSSFB]
// @1753694335 [6HszAAIi1oZr]
// @1753716179 [R3jjwPG1hiJz]
// @1753727830 [ujfX3PbK32Ly]
// @1753750373 [o1fPPznFw1Ci]
// @1753761987 [iszejasYI2pc]
// @1753770962 [KSr3glMwZyKR]
// @1753780620 [qjTnQl18MNuE]
// @1753802641 [t3AVZAk1qWqv]
// @1753823749 [AVR8fJTAseGQ]
// @1753836470 [Bi0gtYKppjkj]
// @1753848215 [0WdUdgrXbgRT]
// @1753857430 [C9jl8TbjIlR1]
// @1753867112 [mRGOjMM136BZ]
// @1753880219 [vxniGc3SZav9]
// @1753888947 [YQqMQ6g9GXHP]
// @1753900586 [bFLbgvyluqbh]
// @1753910154 [Ot6pttBYVGU7]
// @1753922854 [3i7wbvSxVUZH]
// @1753934557 [i5jQkKmF36Q2]
// @1753943757 [27cW91XKt5Ho]
// @1753953468 [xKbHuO3eLZxh]
