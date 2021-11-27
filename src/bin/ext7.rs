#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position, // derive should be used in Position enum too
    work_hour: i32,
}
fn print_emp(emp: Employee) {
    println!("{:?}", emp);
}
fn main() {
    let me = Employee {
        position: Position::Manager,
        work_hour: 40,
    };

    // this statement can be removed by derive macro i.e applied to Enum and Structs

    // match me.position {
    //     Position::Manager => println!("manager"),
    //     Position::Supervisor => println!("supervisor"),
    //     Position::Worker => println!("worker"),
    // }

    println!("{:?}", me.position);
    println!("{:?}", me);

    print_emp(me);
    print_emp(me);
}
