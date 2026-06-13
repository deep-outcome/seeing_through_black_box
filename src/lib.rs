pub fn gcd_naive(mut l_hand: u64, mut r_hand: u64) -> u64 {
    if l_hand == 0 {
        return r_hand;
    }

    if r_hand == 0 {
        return l_hand;
    }

    let lhand_even = l_hand & 1 == 0;
    if lhand_even {
        l_hand >>= 1;
    }

    let rhand_even = r_hand & 1 == 0;
    if rhand_even {
        r_hand >>= 1;
    }

    return if lhand_even || rhand_even {
        let gcd = gcd_naive(l_hand, r_hand);
        if lhand_even & rhand_even {
            gcd << 1
        } else {
            gcd
        }
    } else {
        if l_hand < r_hand {
            let swap = l_hand;
            l_hand = r_hand;
            r_hand = swap;
        }

        l_hand -= r_hand;
        l_hand >>= 1;

        gcd_naive(r_hand, l_hand)
    };
}

pub fn gcd_naive_2(mut l_hand: u64, mut r_hand: u64) -> u64 {
    if l_hand == 0 {
        return r_hand;
    }

    if r_hand == 0 {
        return l_hand;
    }

    if l_hand & 1 == 0 {
        l_hand >>= 1;

        return if r_hand & 1 == 0 {
            r_hand >>= 1;
            gcd_naive_2(l_hand, r_hand) << 1
        } else {
            gcd_naive_2(l_hand, r_hand)
        };
    }

    if r_hand & 1 == 0 {
        r_hand >>= 1;

        return gcd_naive_2(l_hand, r_hand);
    }

    return if l_hand > r_hand {
        l_hand -= r_hand;
        l_hand >>= 1;
        gcd_naive_2(l_hand, r_hand)
    } else {
        r_hand -= l_hand;
        r_hand >>= 1;
        gcd_naive_2(l_hand, r_hand)
    };
}
