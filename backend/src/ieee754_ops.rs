use rust_decimal::prelude::*;
use std::ops::MulAssign;

pub struct FloatLayout {
    sign: u8,
    exponent: u8,
    mantissa: u8,
    exponent_bias: u32,
}

impl FloatLayout {
    const fn get_size(&self) -> usize {
        self.get_end_bit() + 1
    }

    const fn get_sign_size(&self) -> usize {
        self.sign as usize
    }

    const fn get_exponent_size(&self) -> usize {
        self.exponent as usize
    }

    const fn get_mantissa_size(&self) -> usize {
        self.mantissa as usize
    }

    const fn get_start_bit(&self) -> usize {
        0usize
    }

    const fn get_end_bit(&self) -> usize {
        self.sign as usize + self.exponent as usize + self.mantissa as usize - 1
    }

    const fn get_start_char(&self) -> usize {
        self.get_start_bit()
    }

    const fn get_end_char(&self) -> usize {
        self.get_end_bit()
    }

    const fn get_sign_start_char(&self) -> Option<usize> {
        if self.sign > 0 {
            Some(self.get_start_char())
        } else {
            None
        }
    }

    const fn get_sign_end_char(&self) -> Option<usize> {
        if self.sign > 0 {
            Some((self.sign - 1) as usize)
        } else {
            None
        }
    }

    const fn get_exponent_start_char(&self) -> usize {
        self.sign as usize
    }

    const fn get_exponent_end_char(&self) -> usize {
        self.get_mantissa_start_char() - 1
    }

    const fn get_mantissa_start_char(&self) -> usize {
        (self.sign + self.exponent) as usize
    }

    const fn get_mantissa_end_char(&self) -> usize {
        self.get_end_char()
    }

    const fn get_sign_start_bit(&self) -> Option<usize> {
        if self.sign > 0 {
            Some((self.mantissa + self.exponent) as usize)
        } else {
            None
        }
    }

    const fn get_sign_end_bit(&self) -> Option<usize> {
        if self.sign > 0 {
            Some(self.get_end_bit())
        } else {
            None
        }
    }

    const fn get_exponent_start_bit(&self) -> usize {
        self.mantissa as usize
    }

    const fn get_exponent_end_bit(&self) -> usize {
        (self.mantissa + self.exponent - 1) as usize
    }

    const fn get_mantissa_start_bit(&self) -> usize {
        self.get_start_bit()
    }

    const fn get_mantissa_end_bit(&self) -> usize {
        (self.mantissa - 1) as usize
    }

    const fn is_unsigned(&self) -> bool {
        self.sign == 0
    }
}

pub const FLOAT16_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 5,
    mantissa: 10,
    exponent_bias: 15,
};

pub const FLOAT32_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 8,
    mantissa: 23,
    exponent_bias: 127,
};

pub const FLOAT64_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 11,
    mantissa: 52,
    exponent_bias: 1023,
};

pub const FLOAT128_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 15,
    mantissa: 112,
    exponent_bias: 16383,
};

pub const FLOAT256_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 19,
    mantissa: 236,
    exponent_bias: 262143,
};

pub const FP8_E4M3_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 4,
    mantissa: 3,
    exponent_bias: 7,
};

pub const FP8_E5M2_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 5,
    mantissa: 2,
    exponent_bias: 15,
};

pub const BFLOAT16_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 8,
    mantissa: 7,
    exponent_bias: 127,
};

pub const TENSOR_FLOAT32_LAYOUT: FloatLayout = FloatLayout {
    sign: 1,
    exponent: 8,
    mantissa: 10,
    exponent_bias: 127,
};

// 256 pows of 1/2
// Yes, I know that f64 is not enough for such precision to handle all of 256 numbers properly.
// I'll just wait for some good minimalistic library for that (to replace rust_decimal crate as well).
// Or let's wait for f128/f256 in Rust.
//
// All in all I don't REALLY need all that precision to be honest :)
const HALF_POWS: [f64; 255] = [
    0.5,
    0.25,
    0.125,
    0.0625,
    0.03125,
    0.015625,
    0.0078125,
    0.00390625,
    0.001953125,
    0.0009765625,
    0.00048828125,
    0.000244140625,
    0.0001220703125,
    6.103515625e-05,
    3.0517578125e-05,
    1.52587890625e-05,
    7.62939453125e-06,
    3.814697265625e-06,
    1.9073486328125e-06,
    9.5367431640625e-07,
    4.76837158203125e-07,
    2.384185791015625e-07,
    1.1920928955078125e-07,
    5.960464477539063e-08,
    2.9802322387695312e-08,
    1.4901161193847656e-08,
    7.450580596923828e-09,
    3.725290298461914e-09,
    1.862645149230957e-09,
    9.313225746154785e-10,
    4.656612873077393e-10,
    2.3283064365386963e-10,
    1.1641532182693481e-10,
    5.820766091346741e-11,
    2.9103830456733704e-11,
    1.4551915228366852e-11,
    7.275957614183426e-12,
    3.637978807091713e-12,
    1.8189894035458565e-12,
    9.094947017729282e-13,
    4.547473508864641e-13,
    2.2737367544323206e-13,
    1.1368683772161603e-13,
    5.684341886080802e-14,
    2.842170943040401e-14,
    1.4210854715202004e-14,
    7.105427357601002e-15,
    3.552713678800501e-15,
    1.7763568394002505e-15,
    8.881784197001252e-16,
    4.440892098500626e-16,
    2.220446049250313e-16,
    1.1102230246251565e-16,
    5.551115123125783e-17,
    2.7755575615628914e-17,
    1.3877787807814457e-17,
    6.938893903907228e-18,
    3.469446951953614e-18,
    1.734723475976807e-18,
    8.673617379884035e-19,
    4.336808689942018e-19,
    2.168404344971009e-19,
    1.0842021724855044e-19,
    5.421010862427522e-20,
    2.710505431213761e-20,
    1.3552527156068805e-20,
    6.776263578034403e-21,
    3.3881317890172014e-21,
    1.6940658945086007e-21,
    8.470329472543003e-22,
    4.235164736271502e-22,
    2.117582368135751e-22,
    1.0587911840678754e-22,
    5.293955920339377e-23,
    2.6469779601696886e-23,
    1.3234889800848443e-23,
    6.617444900424222e-24,
    3.308722450212111e-24,
    1.6543612251060553e-24,
    8.271806125530277e-25,
    4.1359030627651384e-25,
    2.0679515313825692e-25,
    1.0339757656912846e-25,
    5.169878828456423e-26,
    2.5849394142282115e-26,
    1.2924697071141057e-26,
    6.462348535570529e-27,
    3.2311742677852644e-27,
    1.6155871338926322e-27,
    8.077935669463161e-28,
    4.0389678347315804e-28,
    2.0194839173657902e-28,
    1.0097419586828951e-28,
    5.048709793414476e-29,
    2.524354896707238e-29,
    1.262177448353619e-29,
    6.310887241768095e-30,
    3.1554436208840472e-30,
    1.5777218104420236e-30,
    7.888609052210118e-31,
    3.944304526105059e-31,
    1.9721522630525295e-31,
    9.860761315262648e-32,
    4.930380657631324e-32,
    2.465190328815662e-32,
    1.232595164407831e-32,
    6.162975822039155e-33,
    3.0814879110195774e-33,
    1.5407439555097887e-33,
    7.703719777548943e-34,
    3.851859888774472e-34,
    1.925929944387236e-34,
    9.62964972193618e-35,
    4.81482486096809e-35,
    2.407412430484045e-35,
    1.2037062152420224e-35,
    6.018531076210112e-36,
    3.009265538105056e-36,
    1.504632769052528e-36,
    7.52316384526264e-37,
    3.76158192263132e-37,
    1.88079096131566e-37,
    9.4039548065783e-38,
    4.70197740328915e-38,
    2.350988701644575e-38,
    1.1754943508222875e-38,
    5.877471754111438e-39,
    2.938735877055719e-39,
    1.4693679385278594e-39,
    7.346839692639297e-40,
    3.6734198463196485e-40,
    1.8367099231598242e-40,
    9.183549615799121e-41,
    4.591774807899561e-41,
    2.2958874039497803e-41,
    1.1479437019748901e-41,
    5.739718509874451e-42,
    2.8698592549372254e-42,
    1.4349296274686127e-42,
    7.174648137343064e-43,
    3.587324068671532e-43,
    1.793662034335766e-43,
    8.96831017167883e-44,
    4.484155085839415e-44,
    2.2420775429197073e-44,
    1.1210387714598537e-44,
    5.605193857299268e-45,
    2.802596928649634e-45,
    1.401298464324817e-45,
    7.006492321624085e-46,
    3.503246160812043e-46,
    1.7516230804060213e-46,
    8.758115402030107e-47,
    4.3790577010150533e-47,
    2.1895288505075267e-47,
    1.0947644252537633e-47,
    5.473822126268817e-48,
    2.7369110631344083e-48,
    1.3684555315672042e-48,
    6.842277657836021e-49,
    3.4211388289180104e-49,
    1.7105694144590052e-49,
    8.552847072295026e-50,
    4.276423536147513e-50,
    2.1382117680737565e-50,
    1.0691058840368783e-50,
    5.345529420184391e-51,
    2.6727647100921956e-51,
    1.3363823550460978e-51,
    6.681911775230489e-52,
    3.3409558876152446e-52,
    1.6704779438076223e-52,
    8.352389719038111e-53,
    4.176194859519056e-53,
    2.088097429759528e-53,
    1.044048714879764e-53,
    5.22024357439882e-54,
    2.61012178719941e-54,
    1.305060893599705e-54,
    6.525304467998525e-55,
    3.2626522339992623e-55,
    1.6313261169996311e-55,
    8.156630584998156e-56,
    4.078315292499078e-56,
    2.039157646249539e-56,
    1.0195788231247695e-56,
    5.0978941156238473e-57,
    2.5489470578119236e-57,
    1.2744735289059618e-57,
    6.372367644529809e-58,
    3.1861838222649046e-58,
    1.5930919111324523e-58,
    7.965459555662261e-59,
    3.982729777831131e-59,
    1.9913648889155653e-59,
    9.956824444577827e-60,
    4.9784122222889134e-60,
    2.4892061111444567e-60,
    1.2446030555722283e-60,
    6.223015277861142e-61,
    3.111507638930571e-61,
    1.5557538194652854e-61,
    7.778769097326427e-62,
    3.8893845486632136e-62,
    1.9446922743316068e-62,
    9.723461371658034e-63,
    4.861730685829017e-63,
    2.4308653429145085e-63,
    1.2154326714572542e-63,
    6.077163357286271e-64,
    3.0385816786431356e-64,
    1.5192908393215678e-64,
    7.596454196607839e-65,
    3.7982270983039195e-65,
    1.8991135491519597e-65,
    9.495567745759799e-66,
    4.7477838728798994e-66,
    2.3738919364399497e-66,
    1.1869459682199748e-66,
    5.934729841099874e-67,
    2.967364920549937e-67,
    1.4836824602749686e-67,
    7.418412301374843e-68,
    3.7092061506874214e-68,
    1.8546030753437107e-68,
    9.273015376718553e-69,
    4.636507688359277e-69,
    2.3182538441796384e-69,
    1.1591269220898192e-69,
    5.795634610449096e-70,
    2.897817305224548e-70,
    1.448908652612274e-70,
    7.24454326306137e-71,
    3.622271631530685e-71,
    1.8111358157653425e-71,
    9.055679078826712e-72,
    4.527839539413356e-72,
    2.263919769706678e-72,
    1.131959884853339e-72,
    5.659799424266695e-73,
    2.8298997121333476e-73,
    1.4149498560666738e-73,
    7.074749280333369e-74,
    3.5373746401666845e-74,
    1.7686873200833423e-74,
    8.843436600416711e-75,
    4.421718300208356e-75,
    2.210859150104178e-75,
    1.105429575052089e-75,
    5.527147875260445e-76,
    2.7635739376302223e-76,
    1.3817869688151111e-76,
    6.908934844075556e-77,
    3.454467422037778e-77,
    1.727233711018889e-77,
];

fn all_chars_are(s: &str, c: char) -> bool {
    s.chars().all(|x| x == c)
}

fn add_leading_zeroes(s: &str, target_size: usize) -> String {
    "0".repeat(target_size - s.len()) + s
}

pub enum SpecialValue {
    SmallestPositiveSubnormalNumber,
    LargestSubnormalNumber,
    SmallestPositiveNormalNumber,
    LargestNormalNumber,
    LargestNumberLessThanOne,
    One,
    SmallestNumberLargerThanOne,
}

pub fn make_binary_zero(layout: FloatLayout, is_positive: bool) -> String {
    if is_positive || layout.is_unsigned() {
        "0".repeat(layout.get_size())
    } else {
        add_leading_zeroes("1", layout.get_sign_size())
            + &"0".repeat(layout.get_size() - layout.get_sign_size())
    }
}

pub fn make_binary_infinity(layout: FloatLayout, is_positive: bool) -> String {
    add_leading_zeroes(if is_positive { "0" } else { "1" }, layout.get_sign_size())
        + &"1".repeat(layout.get_exponent_size())
        + &"0".repeat(layout.get_mantissa_size())
}

pub fn make_binary_nan(layout: FloatLayout, is_signaling: bool, payload: u32) -> String {
    // TODO: use payload
    add_leading_zeroes("0", layout.get_sign_size())
        + &"1".repeat(layout.get_exponent_size())
        + if is_signaling { "0" } else { "1" }
        + &"0".repeat(layout.get_mantissa_size() - 2)
        + "1"
}

pub fn make_binary_special(layout: FloatLayout, special_value: SpecialValue) -> String {
    match special_value {
        SpecialValue::SmallestPositiveSubnormalNumber => {
            add_leading_zeroes("0", layout.get_sign_size())
                + &"0".repeat(layout.get_exponent_size())
                + &add_leading_zeroes("1", layout.get_mantissa_size())
        }
        SpecialValue::LargestSubnormalNumber => {
            add_leading_zeroes("0", layout.get_sign_size())
                + &"0".repeat(layout.get_exponent_size())
                + &"1".repeat(layout.get_mantissa_size())
        }
        SpecialValue::SmallestPositiveNormalNumber => {
            add_leading_zeroes("0", layout.get_sign_size())
                + &add_leading_zeroes("1", layout.get_exponent_size())
                + &"0".repeat(layout.get_mantissa_size())
        }
        SpecialValue::LargestNormalNumber => {
            add_leading_zeroes("0", layout.get_sign_size())
                + &"1".repeat(layout.get_exponent_size() - 1)
                + "0"
                + &"1".repeat(layout.get_mantissa_size())
        }
        SpecialValue::LargestNumberLessThanOne => {
            add_leading_zeroes("0", layout.get_sign_size())
                + "0"
                + &"1".repeat(layout.get_exponent_size() - 2)
                + "0"
                + &"1".repeat(layout.get_mantissa_size())
        }
        SpecialValue::One => {
            add_leading_zeroes("0", layout.get_sign_size())
                + "0"
                + &"1".repeat(layout.get_exponent_size() - 1)
                + &"0".repeat(layout.get_mantissa_size())
        }
        SpecialValue::SmallestNumberLargerThanOne => {
            add_leading_zeroes("0", layout.get_sign_size())
                + "0"
                + &"1".repeat(layout.get_exponent_size() - 1)
                + &add_leading_zeroes("1", layout.get_mantissa_size())
        }
    }
}

pub fn decimal_to_binary(decimal: &str, layout: FloatLayout) -> String {
    let decimal = decimal.trim().to_lowercase();

    if decimal.contains("inf") {
        return make_binary_infinity(layout, !decimal.starts_with('-'));
    }

    if decimal.contains("nan") {
        return make_binary_nan(layout, false, 0);
    }

    let dec = Decimal::from_str(&decimal).unwrap();
    let positive = dec.is_sign_positive() && !decimal.starts_with('-');

    if dec.is_zero() {
        return make_binary_zero(layout, positive);
    }

    let dec = dec.abs();
    let int = dec.trunc();
    let mut fract = dec.fract();

    let int_bin = if int.is_zero() {
        String::new()
    } else {
        format!("{:b}", int.to_usize().unwrap())[1..].to_owned()
    };

    let mut negative_exponent: usize = 0;
    let mut fract_bin = String::new();

    if int.is_zero() {
        loop {
            if fract.is_zero() {
                break;
            }

            fract.mul_assign(Decimal::new(2, 0));
            negative_exponent += 1;

            let exit = !fract.trunc().is_zero();

            fract = fract.fract();

            if exit {
                break;
            }
        }
    }

    for _ in 0..(layout.mantissa as isize - int_bin.len() as isize) {
        if fract.is_zero() {
            break;
        }

        fract.mul_assign(Decimal::new(2, 0));
        fract_bin += if fract.trunc().is_zero() { "0" } else { "1" };
        fract = fract.fract();
    }

    let exponent = (int_bin.len() as isize - negative_exponent as isize
        + layout.exponent_bias as isize) as usize;
    let exponent_bin = format!("{:b}", exponent);
    let mut mantissa_bin = int_bin + &fract_bin;
    mantissa_bin.truncate(layout.get_mantissa_size());

    let mut binary = String::new();

    if layout.sign > 1 {
        binary.push_str(&"0".repeat(layout.sign as usize - 1));
    }
    if layout.sign > 0 {
        let sign = if positive { '0' } else { '1' };
        binary.push(sign);
    }

    if layout.exponent as usize > exponent_bin.len() {
        binary.push_str(&"0".repeat(layout.exponent as usize - exponent_bin.len()));
    }
    if layout.exponent > 0 {
        binary.push_str(&exponent_bin);
    }

    if layout.mantissa as usize > mantissa_bin.len() {
        binary.push_str(&"0".repeat(layout.mantissa as usize - mantissa_bin.len()));
    }
    if layout.mantissa > 0 {
        binary.push_str(&mantissa_bin);
    }

    // rounding routine
    if !fract.is_zero() {
        fract.mul_assign(Decimal::new(2, 0));
        let carry = !fract.trunc().is_zero();

        if carry {
            if all_chars_are(&binary[layout.get_mantissa_start_char()..], '1')
                && all_chars_are(
                    &binary[layout.get_exponent_start_char()..layout.get_exponent_end_char()],
                    '1',
                )
            {
                binary = make_binary_infinity(layout, true);
            } else {
                for i in (layout.get_exponent_start_char()..binary.len()).rev() {
                    let c = binary.chars().nth(i).unwrap();
                    if c == '0' {
                        binary.replace_range(i..i + 1, "1");
                        break;
                    } else {
                        binary.replace_range(i..i + 1, "0");
                    }
                }
            }
        }
    }

    binary
}

pub fn binary_to_decimal(binary: &str, layout: FloatLayout, precision: u8) -> String {
    let b = if binary.len() < layout.get_size() {
        add_leading_zeroes(binary, layout.get_size())
    } else if binary.len() > layout.get_size() {
        let remove_range = 0..(binary.len() - layout.get_size());
        assert!(all_chars_are(&binary[remove_range.clone()], '0'));

        let mut b = binary.to_owned();
        b.replace_range(remove_range, "");
        b
    } else {
        binary.to_owned()
    };

    let sign_binary = if layout.get_sign_size() > 0 {
        &b[0..layout.get_sign_size()]
    } else {
        ""
    };
    let exponent_binary = &b[layout.get_exponent_start_char()..layout.get_exponent_end_char() + 1];
    let mantissa_binary = &b[layout.get_mantissa_start_char()..layout.get_mantissa_end_char() + 1];

    let sign: i8 = if sign_binary.ends_with('1') { -1 } else { 1 };
    let exponent = i32::from_str_radix(exponent_binary, 2).unwrap() - layout.exponent_bias as i32;
    let mut mantissa = 1f64;

    for (i, bit) in mantissa_binary.chars().enumerate() {
        if bit == '1' {
            mantissa += HALF_POWS[i];
        }
    }

    // println!("{}", sign);
    // println!("{}", exponent);
    // println!("{}", mantissa);

    let res: f64 = sign as f64 * 2f64.powi(exponent) * mantissa;
    // res.to_string()
    let dec = Decimal::from_f64(res).unwrap();
    dec.round_dp(precision as u32).normalize().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_layouts() {
        assert_eq!(FLOAT16_LAYOUT.get_size(), 16);
        assert_eq!(FLOAT16_LAYOUT.get_start_bit(), 0);
        assert_eq!(FLOAT16_LAYOUT.get_end_bit(), 15);
        assert_eq!(FLOAT16_LAYOUT.get_start_char(), 0);
        assert_eq!(FLOAT16_LAYOUT.get_end_char(), 15);
        assert_eq!(FLOAT16_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(FLOAT16_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(FLOAT16_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(FLOAT16_LAYOUT.get_exponent_end_char(), 5);
        assert_eq!(FLOAT16_LAYOUT.get_mantissa_start_char(), 6);
        assert_eq!(FLOAT16_LAYOUT.get_mantissa_end_char(), 15);
        assert_eq!(FLOAT16_LAYOUT.get_sign_start_bit(), Some(15));
        assert_eq!(FLOAT16_LAYOUT.get_sign_end_bit(), Some(15));
        assert_eq!(FLOAT16_LAYOUT.get_exponent_start_bit(), 10);
        assert_eq!(FLOAT16_LAYOUT.get_exponent_end_bit(), 14);
        assert_eq!(FLOAT16_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(FLOAT16_LAYOUT.get_mantissa_end_bit(), 9);

        assert_eq!(FLOAT32_LAYOUT.get_size(), 32);
        assert_eq!(FLOAT32_LAYOUT.get_start_bit(), 0);
        assert_eq!(FLOAT32_LAYOUT.get_end_bit(), 31);
        assert_eq!(FLOAT32_LAYOUT.get_start_char(), 0);
        assert_eq!(FLOAT32_LAYOUT.get_end_char(), 31);
        assert_eq!(FLOAT32_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(FLOAT32_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(FLOAT32_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(FLOAT32_LAYOUT.get_exponent_end_char(), 8);
        assert_eq!(FLOAT32_LAYOUT.get_mantissa_start_char(), 9);
        assert_eq!(FLOAT32_LAYOUT.get_mantissa_end_char(), 31);
        assert_eq!(FLOAT32_LAYOUT.get_sign_start_bit(), Some(31));
        assert_eq!(FLOAT32_LAYOUT.get_sign_end_bit(), Some(31));
        assert_eq!(FLOAT32_LAYOUT.get_exponent_start_bit(), 23);
        assert_eq!(FLOAT32_LAYOUT.get_exponent_end_bit(), 30);
        assert_eq!(FLOAT32_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(FLOAT32_LAYOUT.get_mantissa_end_bit(), 22);

        assert_eq!(FLOAT64_LAYOUT.get_size(), 64);
        assert_eq!(FLOAT64_LAYOUT.get_start_bit(), 0);
        assert_eq!(FLOAT64_LAYOUT.get_end_bit(), 63);
        assert_eq!(FLOAT64_LAYOUT.get_start_char(), 0);
        assert_eq!(FLOAT64_LAYOUT.get_end_char(), 63);
        assert_eq!(FLOAT64_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(FLOAT64_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(FLOAT64_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(FLOAT64_LAYOUT.get_exponent_end_char(), 11);
        assert_eq!(FLOAT64_LAYOUT.get_mantissa_start_char(), 12);
        assert_eq!(FLOAT64_LAYOUT.get_mantissa_end_char(), 63);
        assert_eq!(FLOAT64_LAYOUT.get_sign_start_bit(), Some(63));
        assert_eq!(FLOAT64_LAYOUT.get_sign_end_bit(), Some(63));
        assert_eq!(FLOAT64_LAYOUT.get_exponent_start_bit(), 52);
        assert_eq!(FLOAT64_LAYOUT.get_exponent_end_bit(), 62);
        assert_eq!(FLOAT64_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(FLOAT64_LAYOUT.get_mantissa_end_bit(), 51);

        assert_eq!(FLOAT128_LAYOUT.get_size(), 128);
        assert_eq!(FLOAT128_LAYOUT.get_start_bit(), 0);
        assert_eq!(FLOAT128_LAYOUT.get_end_bit(), 127);
        assert_eq!(FLOAT128_LAYOUT.get_start_char(), 0);
        assert_eq!(FLOAT128_LAYOUT.get_end_char(), 127);
        assert_eq!(FLOAT128_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(FLOAT128_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(FLOAT128_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(FLOAT128_LAYOUT.get_exponent_end_char(), 15);
        assert_eq!(FLOAT128_LAYOUT.get_mantissa_start_char(), 16);
        assert_eq!(FLOAT128_LAYOUT.get_mantissa_end_char(), 127);
        assert_eq!(FLOAT128_LAYOUT.get_sign_start_bit(), Some(127));
        assert_eq!(FLOAT128_LAYOUT.get_sign_end_bit(), Some(127));
        assert_eq!(FLOAT128_LAYOUT.get_exponent_start_bit(), 112);
        assert_eq!(FLOAT128_LAYOUT.get_exponent_end_bit(), 126);
        assert_eq!(FLOAT128_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(FLOAT128_LAYOUT.get_mantissa_end_bit(), 111);

        assert_eq!(FLOAT256_LAYOUT.get_size(), 256);
        assert_eq!(FLOAT256_LAYOUT.get_start_bit(), 0);
        assert_eq!(FLOAT256_LAYOUT.get_end_bit(), 255);
        assert_eq!(FLOAT256_LAYOUT.get_start_char(), 0);
        assert_eq!(FLOAT256_LAYOUT.get_end_char(), 255);
        assert_eq!(FLOAT256_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(FLOAT256_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(FLOAT256_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(FLOAT256_LAYOUT.get_exponent_end_char(), 19);
        assert_eq!(FLOAT256_LAYOUT.get_mantissa_start_char(), 20);
        assert_eq!(FLOAT256_LAYOUT.get_mantissa_end_char(), 255);
        assert_eq!(FLOAT256_LAYOUT.get_sign_start_bit(), Some(255));
        assert_eq!(FLOAT256_LAYOUT.get_sign_end_bit(), Some(255));
        assert_eq!(FLOAT256_LAYOUT.get_exponent_start_bit(), 236);
        assert_eq!(FLOAT256_LAYOUT.get_exponent_end_bit(), 254);
        assert_eq!(FLOAT256_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(FLOAT256_LAYOUT.get_mantissa_end_bit(), 235);

        assert_eq!(FP8_E4M3_LAYOUT.get_size(), 8);
        assert_eq!(FP8_E4M3_LAYOUT.get_start_bit(), 0);
        assert_eq!(FP8_E4M3_LAYOUT.get_end_bit(), 7);
        assert_eq!(FP8_E4M3_LAYOUT.get_start_char(), 0);
        assert_eq!(FP8_E4M3_LAYOUT.get_end_char(), 7);
        assert_eq!(FP8_E4M3_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(FP8_E4M3_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(FP8_E4M3_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(FP8_E4M3_LAYOUT.get_exponent_end_char(), 4);
        assert_eq!(FP8_E4M3_LAYOUT.get_mantissa_start_char(), 5);
        assert_eq!(FP8_E4M3_LAYOUT.get_mantissa_end_char(), 7);
        assert_eq!(FP8_E4M3_LAYOUT.get_sign_start_bit(), Some(7));
        assert_eq!(FP8_E4M3_LAYOUT.get_sign_end_bit(), Some(7));
        assert_eq!(FP8_E4M3_LAYOUT.get_exponent_start_bit(), 3);
        assert_eq!(FP8_E4M3_LAYOUT.get_exponent_end_bit(), 6);
        assert_eq!(FP8_E4M3_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(FP8_E4M3_LAYOUT.get_mantissa_end_bit(), 2);

        assert_eq!(FP8_E5M2_LAYOUT.get_size(), 8);
        assert_eq!(FP8_E5M2_LAYOUT.get_start_bit(), 0);
        assert_eq!(FP8_E5M2_LAYOUT.get_end_bit(), 7);
        assert_eq!(FP8_E5M2_LAYOUT.get_start_char(), 0);
        assert_eq!(FP8_E5M2_LAYOUT.get_end_char(), 7);
        assert_eq!(FP8_E5M2_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(FP8_E5M2_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(FP8_E5M2_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(FP8_E5M2_LAYOUT.get_exponent_end_char(), 5);
        assert_eq!(FP8_E5M2_LAYOUT.get_mantissa_start_char(), 6);
        assert_eq!(FP8_E5M2_LAYOUT.get_mantissa_end_char(), 7);
        assert_eq!(FP8_E5M2_LAYOUT.get_sign_start_bit(), Some(7));
        assert_eq!(FP8_E5M2_LAYOUT.get_sign_end_bit(), Some(7));
        assert_eq!(FP8_E5M2_LAYOUT.get_exponent_start_bit(), 2);
        assert_eq!(FP8_E5M2_LAYOUT.get_exponent_end_bit(), 6);
        assert_eq!(FP8_E5M2_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(FP8_E5M2_LAYOUT.get_mantissa_end_bit(), 1);

        assert_eq!(BFLOAT16_LAYOUT.get_size(), 16);
        assert_eq!(BFLOAT16_LAYOUT.get_start_bit(), 0);
        assert_eq!(BFLOAT16_LAYOUT.get_end_bit(), 15);
        assert_eq!(BFLOAT16_LAYOUT.get_start_char(), 0);
        assert_eq!(BFLOAT16_LAYOUT.get_end_char(), 15);
        assert_eq!(BFLOAT16_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(BFLOAT16_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(BFLOAT16_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(BFLOAT16_LAYOUT.get_exponent_end_char(), 8);
        assert_eq!(BFLOAT16_LAYOUT.get_mantissa_start_char(), 9);
        assert_eq!(BFLOAT16_LAYOUT.get_mantissa_end_char(), 15);
        assert_eq!(BFLOAT16_LAYOUT.get_sign_start_bit(), Some(15));
        assert_eq!(BFLOAT16_LAYOUT.get_sign_end_bit(), Some(15));
        assert_eq!(BFLOAT16_LAYOUT.get_exponent_start_bit(), 7);
        assert_eq!(BFLOAT16_LAYOUT.get_exponent_end_bit(), 14);
        assert_eq!(BFLOAT16_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(BFLOAT16_LAYOUT.get_mantissa_end_bit(), 6);

        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_size(), 19);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_start_bit(), 0);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_end_bit(), 18);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_start_char(), 0);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_end_char(), 18);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_sign_start_char(), Some(0));
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_sign_end_char(), Some(0));
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_exponent_start_char(), 1);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_exponent_end_char(), 8);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_mantissa_start_char(), 9);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_mantissa_end_char(), 18);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_sign_start_bit(), Some(18));
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_sign_end_bit(), Some(18));
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_exponent_start_bit(), 10);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_exponent_end_bit(), 17);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_mantissa_start_bit(), 0);
        assert_eq!(TENSOR_FLOAT32_LAYOUT.get_mantissa_end_bit(), 9);
    }

    #[test]
    fn test_decimal_to_binary32() {
        assert_eq!(
            decimal_to_binary("3.14", FLOAT32_LAYOUT),
            "01000000010010001111010111000011"
        );
        assert_eq!(
            decimal_to_binary("1.0", FLOAT32_LAYOUT),
            "00111111100000000000000000000000"
        );
        assert_eq!(
            decimal_to_binary("-1.0", FLOAT32_LAYOUT),
            "10111111100000000000000000000000"
        );
        assert_eq!(
            decimal_to_binary("0.0", FLOAT32_LAYOUT),
            "00000000000000000000000000000000"
        );
        assert_eq!(
            decimal_to_binary("+0.0", FLOAT32_LAYOUT),
            "00000000000000000000000000000000"
        );
        assert_eq!(
            decimal_to_binary("-0.0", FLOAT32_LAYOUT),
            "10000000000000000000000000000000"
        );
        assert_eq!(
            decimal_to_binary("inf", FLOAT32_LAYOUT),
            "01111111100000000000000000000000"
        );
        assert_eq!(
            decimal_to_binary("-iNfInItY", FLOAT32_LAYOUT),
            "11111111100000000000000000000000"
        );
        assert_eq!(
            decimal_to_binary("NaN", FLOAT32_LAYOUT),
            "01111111110000000000000000000001"
        );
        assert_eq!(
            decimal_to_binary("5959.59", FLOAT32_LAYOUT),
            "01000101101110100011110010111000"
        );
        assert_eq!(
            decimal_to_binary("-0.0001", FLOAT32_LAYOUT),
            "10111000110100011011011100010111"
        );
        assert_eq!(
            decimal_to_binary("0.1", FLOAT32_LAYOUT),
            "00111101110011001100110011001101"
        );
        assert_eq!(
            decimal_to_binary("0.3333333", FLOAT32_LAYOUT),
            "00111110101010101010101010101010"
        );
        assert_eq!(
            decimal_to_binary("0.33333333", FLOAT32_LAYOUT),
            "00111110101010101010101010101011"
        );
        assert_eq!(
            decimal_to_binary("1.000000119", FLOAT32_LAYOUT),
            "00111111100000000000000000000001"
        );
        assert_eq!(
            decimal_to_binary("16777215.0", FLOAT32_LAYOUT),
            "01001011011111111111111111111111"
        );
        assert_eq!(
            decimal_to_binary("16777216.0", FLOAT32_LAYOUT),
            "01001011100000000000000000000000"
        );
    }

    #[test]
    fn test_binary32_to_decimal() {
        assert_eq!(
            binary_to_decimal("00000000000000000000000000000000", FLOAT32_LAYOUT, 4),
            "0"
        );
        assert_eq!(
            binary_to_decimal(
                &make_binary_special(FLOAT32_LAYOUT, SpecialValue::One),
                FLOAT32_LAYOUT, 4
            ),
            "1"
        );
        assert_eq!(
            binary_to_decimal("01000000010010001111010111000011", FLOAT32_LAYOUT, 4),
            "3.14"
        );
        assert_eq!(
            binary_to_decimal("01000101101110100011110010111000", FLOAT32_LAYOUT, 2),
            "5959.59"
        );
        assert_eq!(
            binary_to_decimal("10111000110100011011011100010111", FLOAT32_LAYOUT, 4),
            "-0.0001"
        );
        assert_eq!(
            binary_to_decimal("00111101110011001100110011001101", FLOAT32_LAYOUT, 4),
            "0.1"
        );
        assert_eq!(
            binary_to_decimal("00111110101010101010101010101010", FLOAT32_LAYOUT, 4),
            "0.3333"
        );
        assert_eq!(
            binary_to_decimal("00111110101010101010101010101011", FLOAT32_LAYOUT, 4),
            "0.3333"
        );
        assert_eq!(
            binary_to_decimal("00111111100000000000000000000001", FLOAT32_LAYOUT, 4),
            "1"
        );
        assert_eq!(
            binary_to_decimal("01001011011111111111111111111111", FLOAT32_LAYOUT, 4),
            "16777215"
        );
        assert_eq!(
            binary_to_decimal("01001011100000000000000000000000", FLOAT32_LAYOUT, 4),
            "16777216"
        );
    }

    #[test]
    fn test_special_values() {
        assert_eq!(
            make_binary_zero(FLOAT32_LAYOUT, true),
            "00000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_zero(FLOAT32_LAYOUT, false),
            "10000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_infinity(FLOAT32_LAYOUT, true),
            "01111111100000000000000000000000"
        );
        assert_eq!(
            make_binary_infinity(FLOAT32_LAYOUT, false),
            "11111111100000000000000000000000"
        );
        assert_eq!(
            make_binary_nan(FLOAT32_LAYOUT, false, 0),
            "01111111110000000000000000000001"
        );
        assert_eq!(
            make_binary_special(
                FLOAT32_LAYOUT,
                SpecialValue::SmallestPositiveSubnormalNumber
            ),
            "00000000000000000000000000000001"
        );
        assert_eq!(
            make_binary_special(FLOAT32_LAYOUT, SpecialValue::LargestSubnormalNumber),
            "00000000011111111111111111111111"
        );
        assert_eq!(
            make_binary_special(FLOAT32_LAYOUT, SpecialValue::SmallestPositiveNormalNumber),
            "00000000100000000000000000000000"
        );
        assert_eq!(
            make_binary_special(FLOAT32_LAYOUT, SpecialValue::LargestNormalNumber),
            "01111111011111111111111111111111"
        );
        assert_eq!(
            make_binary_special(FLOAT32_LAYOUT, SpecialValue::LargestNumberLessThanOne),
            "00111111011111111111111111111111"
        );
        assert_eq!(
            make_binary_special(FLOAT32_LAYOUT, SpecialValue::One),
            "00111111100000000000000000000000"
        );
        assert_eq!(
            make_binary_special(FLOAT32_LAYOUT, SpecialValue::SmallestNumberLargerThanOne),
            "00111111100000000000000000000001"
        );

        assert_eq!(
            make_binary_zero(FLOAT64_LAYOUT, true),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_zero(FLOAT64_LAYOUT, false),
            "1000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_infinity(FLOAT64_LAYOUT, true),
            "0111111111110000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_infinity(FLOAT64_LAYOUT, false),
            "1111111111110000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_nan(FLOAT64_LAYOUT, false, 0),
            "0111111111111000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(
            make_binary_special(
                FLOAT64_LAYOUT,
                SpecialValue::SmallestPositiveSubnormalNumber
            ),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(
            make_binary_special(FLOAT64_LAYOUT, SpecialValue::LargestSubnormalNumber),
            "0000000000001111111111111111111111111111111111111111111111111111"
        );
        assert_eq!(
            make_binary_special(FLOAT64_LAYOUT, SpecialValue::SmallestPositiveNormalNumber),
            "0000000000010000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_special(FLOAT64_LAYOUT, SpecialValue::LargestNormalNumber),
            "0111111111101111111111111111111111111111111111111111111111111111"
        );
        assert_eq!(
            make_binary_special(FLOAT64_LAYOUT, SpecialValue::LargestNumberLessThanOne),
            "0011111111101111111111111111111111111111111111111111111111111111"
        );
        assert_eq!(
            make_binary_special(FLOAT64_LAYOUT, SpecialValue::One),
            "0011111111110000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            make_binary_special(FLOAT64_LAYOUT, SpecialValue::SmallestNumberLargerThanOne),
            "0011111111110000000000000000000000000000000000000000000000000001"
        );
    }
}
