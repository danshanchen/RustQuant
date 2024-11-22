use time::macros::date;
use time::Date;
use RustQuant::data::*;
use RustQuant::time::*;

fn main() {
    let _cal = AustraliaCalendar;
    let _curve = Curve::<Date>::new_from_slice(&DATES, &RATES);

    let mut discount_curve = DiscountCurve::<Date, AustraliaCalendar>::new(&DATES, &RATES);

    let new_dates = [
        date!(2025 - 01 - 01),
        date!(2026 - 01 - 01),
        date!(2027 - 01 - 01),
        date!(2028 - 01 - 01),
        date!(2029 - 01 - 01),
        date!(2030 - 01 - 01),
        date!(2033 - 01 - 01),
        date!(2036 - 01 - 01),
        date!(2040 - 01 - 01),
        date!(2044 - 01 - 01),
        date!(2046 - 01 - 01),
        date!(2048 - 01 - 01),
        date!(2050 - 01 - 01),
        date!(2053 - 01 - 01),
    ];

    discount_curve.get_rates(&new_dates);

    discount_curve.curve.plot();
}

const DATES: [Date; 33] = [
    date!(2024 - 11 - 03),
    date!(2025 - 02 - 02),
    date!(2025 - 05 - 04),
    date!(2025 - 08 - 04),
    date!(2026 - 08 - 04),
    date!(2027 - 08 - 04),
    date!(2028 - 08 - 03),
    date!(2029 - 08 - 03),
    date!(2030 - 08 - 03),
    date!(2031 - 08 - 03),
    date!(2032 - 08 - 02),
    date!(2033 - 08 - 02),
    date!(2034 - 08 - 02),
    date!(2035 - 08 - 02),
    date!(2036 - 08 - 01),
    date!(2037 - 08 - 01),
    date!(2038 - 08 - 01),
    date!(2039 - 08 - 01),
    date!(2040 - 07 - 31),
    date!(2041 - 07 - 31),
    date!(2042 - 07 - 31),
    date!(2043 - 07 - 31),
    date!(2044 - 07 - 30),
    date!(2045 - 07 - 30),
    date!(2046 - 07 - 30),
    date!(2047 - 07 - 30),
    date!(2048 - 07 - 29),
    date!(2049 - 07 - 29),
    date!(2050 - 07 - 29),
    date!(2051 - 07 - 29),
    date!(2052 - 07 - 28),
    date!(2053 - 07 - 28),
    date!(2054 - 07 - 28),
];

const RATES: [f64; 33] = [
    0.991534731388138,
    0.9838359286590855,
    0.9767467474737808,
    0.9701361466138612,
    0.9465501358547515,
    0.924575634732985,
    0.9021409746007933,
    0.8785156561998614,
    0.8536376778773106,
    0.8277468732256259,
    0.8011892601385907,
    0.7743181525840983,
    0.747449451062916,
    0.72084513263844,
    0.6947118803630943,
    0.6692062636539615,
    0.6444420340874304,
    0.6204978743800441,
    0.5974244062002886,
    0.5752498332158714,
    0.5539856251246074,
    0.5336298794380864,
    0.5141705804517491,
    0.4955886653745916,
    0.4778592623853122,
    0.4609539419402714,
    0.4448417313346598,
    0.4294898156268353,
    0.41486459877519694,
    0.4009324097988462,
    0.3876596480762123,
    0.3750132501134792,
    0.3629609595319168,
];
