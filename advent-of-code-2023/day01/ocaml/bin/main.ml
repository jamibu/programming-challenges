let file = "../puzzleInput.txt"

let is_digit c =
    match c with '0' .. '9' -> true | _ -> false

let find_number line =
        let first_digit = line |> String.to_seq |> List.of_seq |> List.find (is_digit) in
        let last_digit = line |> String.to_seq |> List.of_seq |> List.rev |> List.find (is_digit) in
        let number = Printf.sprintf "%c%c" first_digit last_digit in
        let number = int_of_string number in
        number

let rec part1 channel acc =
    match input_line channel with
    | line -> 
        let acc = acc + find_number(line) in
        part1 channel acc

    | exception End_of_file -> acc

let () =
    let ic = open_in file in
    let part1_sum = part1 ic 0 in
    Printf.printf "Part 1: %d\n" part1_sum;

(* let rec part2 channel acc = *)
(*     match input_line channel with *)
(*     | line ->  *)
(*         let line = replace_words replacements line in *)
(*         let acc = acc + find_number(line) in *)
(*         part2 channel acc *)
(**)
(*     | exception End_of_file -> acc *)

(* let replacements = [ *)
(*     "zero", "z0e"; *)
(*     "one", "o1e"; *)
(*     "two", "t2o"; *)
(*     "three", "t3e"; *)
(*     "four", "f4r"; *)
(*     "five", "f5e"; *)
(*     "six", "s6x"; *)
(*     "seven", "s7n"; *)
(*     "eight", "e8t"; *)
(*     "nine", "n9e"; *)
(* ] *)
(**)
(* let rec replace_words replacements line = *)
(*      match replacements with *)
(*         | [] -> line *)
(*         | x :: xs -> String.replace x[0 x.1; replace_words replacements line  *)
(**)
