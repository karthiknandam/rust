use lib::{
    activity_32::main_a33,
    activity_error::main_error,
    activity_lifetimes::activiti_lifetimes,
    err_accessing::err_acc,
    lifetimes::{lifetimes, test_2},
    new_features::main_new_feature,
};

fn main() {
    lifetimes();
    test_2();
    activiti_lifetimes();
    main_a33();
    err_acc();
    main_error();
    main_new_feature();
}
