open Core
let file = "../puzzleInput.txt"

let is_digit c =
    match c with '0' .. '9' -> true | _ -> false

let make_number first last = match (first, last) with
    | (Some x, Some y) -> int_of_string(Printf.sprintf "%c%c" x y)
    | _ -> 0

let find_number line =
    let chars = String.to_list line in
    let first_digit = List.find chars ~f:is_digit in
    let last_digit = List.find (List.rev chars) ~f:is_digit  in
    make_number first_digit last_digit


let replace_words line = 
    let line = Str.global_replace (Str.regexp "zero") "z0o" (line) in
    let line = Str.global_replace (Str.regexp "one") "o1e" (line) in
    let line = Str.global_replace (Str.regexp "two") "t2o" (line) in
    let line = Str.global_replace (Str.regexp "three") "t3e" (line) in
    let line = Str.global_replace (Str.regexp "four") "f4r" (line) in
    let line = Str.global_replace (Str.regexp "five") "f5e" (line) in
    let line = Str.global_replace (Str.regexp "six") "s6x" (line) in
    let line = Str.global_replace (Str.regexp "seven") "s7n" (line) in
    let line = Str.global_replace (Str.regexp "eight") "e8t" (line) in
    let line = Str.global_replace (Str.regexp "nine") "n9e" (line) in
    line

let rec solve channel part1 part2 =
    match In_channel.input_line channel with
    | Some line -> 
        let part1 = part1 + find_number(line) in
        let part2 = part2 + find_number(replace_words(line)) in
        solve channel part1 part2
    | None -> (part1, part2)

let () =
    let ic = In_channel.create file in
    let (part1, part2) = solve ic 0 0 in
    Printf.printf "Part 1: %d\n" part1;
    Printf.printf "Part 2: %d\n" part2;

