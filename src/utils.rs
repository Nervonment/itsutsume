pub fn eq5<T: PartialEq>(v1: &T, v2: &T, v3: &T, v4: &T, v5: &T) -> bool {
    v1 == v2 && v2 == v3 && v3 == v4 && v4 == v5
}
