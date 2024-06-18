fn main() {
    let mut x: String = String::from("hello");  // ROW

    let x_newown: String = x; // RO_

    // ERR: no W access to `x` 
    // > Write access is revoked when ownership is transfer
    // x += "hello"; // R__

    // ERR: no O access to `x`
    // > Referencing is not possible without ownership access
    // let x_prevown = &x;

    // ERR: no W access to `x_newown`
    // > Write access is path dependent, not variable/memory dependent
    // x_newown += "bye";

    let mut x: String = x_newown; // ROW

    let x_ref: &String = &x; // R__
    println!("x_ref can be read {x_ref}");
    
    // ERR: no W access to &String
    // > W access is not given when reference is created by default
    // *x_ref += "x_ref can be modified";

    let x_ref_mut: &mut String = &mut x; // R_W, Reference Ownership is transfer from `x_ref` to
                                         // `x_ref_mut`
    *x_ref_mut += "x_ref_mut can be modified";
    println!("x_ref_mut can be read, {x_ref_mut}");
    *x_ref_mut += "mutability returned";

    // ERR: no RO access to x anymore
    // > RO access is transfer when new reference created
    // println!("x_ref is also modified {x_ref}");
}
