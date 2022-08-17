use esp_idf_sys::c_types::c_int;

#[repr(C)]
struct Pair {
    a: c_int,
    b: c_int,
}

extern "C" {
    fn abi_test_5(p1: c_int, p2: c_int, p3: c_int, p4: c_int, p5: Pair);
    fn abi_test_6(p1: c_int, p2: c_int, p3: c_int, p4: c_int, p5: c_int, p6: Pair);
}

fn main() {
    unsafe {
        abi_test_5(0, 0, 0, 0, Pair { a: 1, b: 2 });
        abi_test_6(0, 0, 0, 0, 0, Pair { a: 1, b: 2 });
    }
}
