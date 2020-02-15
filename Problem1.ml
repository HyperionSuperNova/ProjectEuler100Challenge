(*
Multiples of 3 and 5
   
Problem 1
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*)

(* Solution 1.1 (iterative) *)

(*Approximative Complexit0 : O(n)*)


let solutionBrute =
    let rec brute itera =
        if itera = 1 then 0
        else if (itera mod 3 = 0 || itera mod 5 = 0) && itera != 0 then itera + (brute (itera-1)) 
        else brute (itera-1)
    in brute 999
    ;;

Printf.printf "%s" (string_of_int(solutionBrute));;