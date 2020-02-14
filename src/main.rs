use std::io;

fn main() {
    //************************************************************************************ */
    //      number of blocks of the chocolate bar which are cut into exactly two pieces of 
    //      equal area = gcd iff   (m/gcd) is odd   AND   (n/gcd) is odd
    //************************************************************************************ */

    let input: Vec<String> = read_line_vec_string();
    // let input: Vec<String> = vec![
    //     "15041290507349142600".to_string(),
    //     "2064515835498780600".to_string(),
    // ];// gives a gcd = nr of blocks = 8063664517800


    match (input[1]).parse::<u64>() {
        Ok(n) => {

            let mut m_is_reduced_flag = false;
            let m: u64 = if input[0].len() > 19 {
                //18,446,744,073,709,551,615
                m_is_reduced_flag = true;
                reduce_num(&input[0], n)
            } else {
                input[0].parse().unwrap()
            };

            let g = gcd(m, n);
            let mut is_nums_div_gcd_odd: bool = false;

            let is_gcd_odd = 1 & g == 1;

            // check if is_nums_div_gcd_odd
            if is_gcd_odd {
                let mut m = m;

                if m_is_reduced_flag {
                    let last_digt_m: char =
                        input[0].chars().skip(input[0].len() - 1).next().unwrap();
                    m = last_digt_m.to_digit(10).unwrap() as u64; // NOTICE: shadowing m
                }

                // iff (m/gcd) is odd    AND    (n/gcd) is odd ==> the num of blocks that "cutted in halv" = gcd; Otherwise zero.
                // when gcd is odd it's efficient to just check if both(m and n) are odd, becuase odd/odd=odd
                // (1 & any_odd_num) is equal to 1
                is_nums_div_gcd_odd = (1 & m == 1) && (1 & n == 1);
            } else {
                // if gcd is even, then niether ("Unreduced" m) nor n can be odd, because odd / (an even gcd) != int
                // so m and n are even, but (even / even = unknown):
                //      iff (m/gcd) is odd    AND    (n/gcd) is odd ==> the num of blocks that "cutted in halv" = gcd; Otherwise zero.
                let mut m = m ;

                if m_is_reduced_flag {
                    // we know that g,m and n are even, and we have to perform (m/g) and (n/g) to assert that both will result in odd numbers
                    m = long_num_as_str_division(&input[0], 4); // NOTICE: shadowing m
                }
                is_nums_div_gcd_odd = (1 & (m / g ) == 1) && (1 & (n / g) == 1);
            }

            if is_nums_div_gcd_odd {
                println!("{}", g);
            } else {
                println!("0");
            }
        }
        _ => {
            panic!(" (input [1]) overflows u64!")
        }
    }
}

fn reduce_num(m: &str, n: u64) -> u64 {
    let mut new_m: u64 = 0;
    for c in m.chars() {
        let digit = c.to_digit(10).unwrap();
        new_m = (((new_m % n) * (10 % n)) % n + digit as u64) % n
    }
    new_m
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

fn read_line_vec_string() -> Vec<String> {
    let stdin = io::stdin();
    let mut val = String::new();
    stdin.read_line(&mut val).expect("Failed to read line");
    let val: Vec<String> = val
        .trim()
        .split_whitespace()
        .map(|st| st.to_string())
        .collect();
    val
}

fn long_num_as_str_division(number: &str, divisor: u64) -> u64 {
    let divisor: u64 = divisor;
    let mut ans: u64 = 0;

    let mut it = number.chars().map(|c: char| c.to_digit(10).unwrap() as u64); // it of u64
    let mut temp = 0;

    // Find prefix of number that is
    // larger than divisor.
    while temp < divisor {
        temp = temp * 10 + it.next().unwrap();
    }
    ans = (ans * 10) + (temp / divisor);

    // Repeatedly divide divisor with temp. After
    // every division, update temp to include one
    // more digit.
    while let Some(digit) = it.next() {
        // Take next digit of number
        temp = (temp % divisor) * 10 + digit;

        // Store result in answer i.e. temp / divisor
        ans = (ans * 10) + (temp / divisor);
    }

    println!("{}", ans);
    return ans;
}

fn is_res_of_div_odd(number: &str, divisor: u64) -> bool {
    let mut it = number.chars().map(|c: char| c.to_digit(10).unwrap() as u64); // it of u64
    let mut temp = 0;

    // Find prefix of number that is
    // larger than divisor.
    while temp < divisor {
        temp = temp * 10 + it.next().unwrap() as u64;
    }

    // Repeatedly divide divisor with temp. After
    // every division, update temp to include one
    // more digit.
    while let Some(digit) = it.next() {
        // Take next digit of number
        temp = (temp % divisor) * 10 + digit;
    }

    println!("{}", 1 & temp == 1);
    return 1 & temp == 1;
}

