// range (0-9)
r: 48+til 10;

// D: ("one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine");

fn: {
// FIXME: part 2
/
  0N 0 0N 0N 0N 0N 0N 0N 4
  0N 4 7 0N 0N 0N 0N 0 0N
  3 0N 7 0N 0N 0N 0N 0N 0N
  3 1 0N 7 0N 0N 0N 0N 0N
  0N 0N 0N 0N 0N 0N 11 0N 1
  1 0N 0N 0N 0N 0N 0N 3 0N
  0N 0N 0N 0N 0N 6 0N 0N 0N
\
  // p: {[x;y] first ss[x;y] }[x] each D;

  d: ({[c] n: "i"$c; r ? n} x) except 10;
  (first d * 10) + last d

// NOTE
/
  v: {[c]
    // treat a char as an integer
    n: "i"$c;

    // get an index (position) if it is included in a range of 0-9 (otherwise 9+1)
    r ? n

    } x;

  // filter 10 (the ones are not found in a range of 0-9)
  d: v except 10;

  // this supports conversion of a list of one element like (,7)
  (first d * 10) + last d
\
  }

main: {
  // input.txt
  // fileh: `$"./data/input.txt";
  // input: read0 fileh;
  // fn each input

  // example in the part 1
  show sum fn each ("1abc2"; "pqr3stu8vwx"; "a1b2c3d4e5f"; "treb7uchet");

  // example in the part 2
  // show sum fn each ("two1nine"; "eightwothree"; "abcone2threexyz"; "xtwone3four"; "4nineeinghtseven2"; "zoneight234"; "7pqrstsixteen");
  }

result: main ();
show result;
