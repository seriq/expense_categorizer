use crate::model::Booking;
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum CategorizedBooking {
    Warmmiete(Booking),
}
