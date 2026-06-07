macro_rules! quantity {
    ($qty:ident, $unit:ident, base = $base:literal, {
        $($variant:ident => ($factor:expr, $sym:literal, $stem:ident)),* $(,)?
    }) => {
        #[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $qty(f64);

        impl $qty {
            fn canonical_bits(self) -> u64 {
                if self.0.is_nan() {
                    f64::NAN.to_bits()
                } else if self.0 == 0.0 {
                    0
                } else {
                    self.0.to_bits()
                }
            }
        }

        impl PartialEq for $qty {
            fn eq(&self, other: &Self) -> bool {
                self.canonical_bits() == other.canonical_bits()
            }
        }
        impl Eq for $qty {}
        impl std::hash::Hash for $qty {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.canonical_bits().hash(state);
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $unit { $($variant),* }

        impl $unit {
            pub const fn factor(self) -> f64 {
                match self { $(Self::$variant => $factor),* }
            }
            pub const fn symbol(self) -> &'static str {
                match self { $(Self::$variant => $sym),* }
            }
        }

        impl std::fmt::Display for $unit {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.symbol())
            }
        }

        pastey::paste! {
            impl $qty {
                pub const fn new(value: f64, unit: $unit) -> Self { Self(value * unit.factor()) }
                pub const fn get(self, unit: $unit) -> f64 { self.0 / unit.factor() }

                $(
                    #[doc = concat!("Construct from a value in ", $sym, ".")]
                    pub const fn [<from_ $stem>](value: f64) -> Self { Self::new(value, $unit::$variant) }

                    #[doc = concat!("Get the value in ", $sym, ".")]
                    pub const fn $stem(self) -> f64 { self.get($unit::$variant) }
                )*
            }
        }

        impl std::ops::Add for $qty { type Output = $qty; fn add(self, r: $qty) -> $qty { $qty(self.0 + r.0) } }
        impl std::ops::AddAssign for $qty { fn add_assign(&mut self, r: $qty) { self.0 += r.0; } }
        impl std::ops::Sub for $qty { type Output = $qty; fn sub(self, r: $qty) -> $qty { $qty(self.0 - r.0) } }
        impl std::ops::SubAssign for $qty { fn sub_assign(&mut self, r: $qty) { self.0 -= r.0; } }
        impl std::ops::Mul<f64> for $qty { type Output = $qty; fn mul(self, s: f64) -> $qty { $qty(self.0 * s) } }
        impl std::ops::Div<f64> for $qty { type Output = $qty; fn div(self, s: f64) -> $qty { $qty(self.0 / s) } }
        impl std::ops::Div for $qty { type Output = f64; fn div(self, r: $qty) -> f64 { self.0 / r.0 } }
        impl std::ops::Neg for $qty { type Output = $qty; fn neg(self) -> $qty { $qty(-self.0) } }

        impl std::fmt::Display for $qty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.0, $base)
            }
        }
    };
}

macro_rules! quant_mul_div {
    ($a:ident * $b:ident => $c:ident) => {
        impl std::ops::Mul<$b> for $a {
            type Output = $c;
            fn mul(self, r: $b) -> $c {
                $c(self.0 * r.0)
            }
        }

        impl std::ops::Mul<$a> for $b {
            type Output = $c;
            fn mul(self, r: $a) -> $c {
                $c(self.0 * r.0)
            }
        }

        impl std::ops::Div<$a> for $c {
            type Output = $b;
            fn div(self, r: $a) -> $b {
                $b(self.0 / r.0)
            }
        }

        impl std::ops::Div<$b> for $c {
            type Output = $a;
            fn div(self, r: $b) -> $a {
                $a(self.0 / r.0)
            }
        }
    };
}

quantity!(Energy, EnergyUnit, base = "J", {
    Joule        => (1.0,         "J",    joules),
    Kilojoule    => (1_000.0,     "kJ",   kilojoules),
    WattHour     => (3_600.0,     "Wh",   watt_hours),
    KilowattHour => (3_600_000.0, "kWh",  kilowatt_hours),
    Calorie      => (4.184,       "cal",  calories),
    Kilocalorie  => (4_184.0,     "kcal", kilocalories),
});

quantity!(Power, PowerUnit, base = "W", {
    Watt     => (1.0,  "W",  watts),
    Kilowatt => (1_e3, "kW", kilowatts),
    Megawatt => (1_e6, "MW", megawatts),
    Gigawatt => (1_e9, "GW", gigawatts),
});

quantity!(Time, TimeUnit, base = "s", {
    Second => (1.0,      "s",   seconds),
    Minute => (60.0,     "min", minutes),
    Hour   => (3_600.0,  "h",   hours),
    Day    => (86_400.0, "d",   days),
});

quant_mul_div!(Power * Time => Energy);
