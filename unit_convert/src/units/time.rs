use crate::impl_units;

impl_units! {
    Time => {
        Second => [
            <| |s| *s,
            |> |s| *s,
            aliases = ["s", "sec"],
            metric = true
        ],
        Minute => [
            <| |m| m * 60.0,
            |> |s| s / 60.0,
            aliases = ["min"]
        ],
        Hour => [
            <| |h| h * 3600.0,
            |> |s| s / 3600.0,
            aliases = ["h", "hr"]
        ],
        Week => [
            <| |w| w * 604800.0,
            |> |s| s / 604800.0,
            aliases = ["wk"]
        ],
        /// Apparent interval between two successive returns of the Sun to the same meridian
        Sol => [
            <| |s| s * 88740.244,
            |> |s| s / 88740.244
        ],
        Day => [
            <| |D| D * 86400,
            |> |s| s / 86400,
            aliases = ["D"]
        ],
        Julian_Year => [
            <| |jy| jy * 31557600,
            |> |s| s / 31557600,
            aliases = ["julian year"]
        ],
        Fortnight => [
            <| |ftn| ftn * 1209600,
            |> |s| s / 1209600,
            aliases = ["ftn"]
        ],
        Planck_Time => [
            <| |tp| tp * 5.391247e-44,
            |> |s| s / 5.391247e-44,
            aliases = ["planck time"]
        ],
        Atom => [
            <| |atom| atom * 0.15957,
            |> |s| s / 0.15957
        ],
        Martian_Vernal_Equinox_Year => [
            <| |mvey| mvey * 59264867.1384,
            |> |s| s / 59264867.1384,
            aliases = ["martian vernal equinox year"]
        ],
        Ghurry => [
            <| |ghurry| ghurry * 1440,
            |> |s| s / 1440
        ],
        Lustre => [
            <| |lustre| lustre * 157788000,
            |> |s| s / 157788000
        ],
        Nundine => [
            <| |nundine| nundine * 777600,
            |> |s| s / 777600
        ],
        Punct => [
            <| |punct| punct * 900,
            |> |s| s / 900
        ],
        Quadrant => [
            <| |quadrant| quadrant * 21600,
            |> |s| s / 21600
        ],
        ///This one needs an accent
        Quinzieme => [
            <| |quinzieme| quinzieme * 1296000,
            |> |s| s / 1296000
        ],
        Jubilee => [
            <| |jubilee| jubilee * 1577880000,
            |> |s| s / 1577880000
        ],
        Sidereal_Day => [
            <| |sday| sday * 86164.0891217,
            |> |s| s / 86164.0891217,
            aliases = ["sidereal day"]
        ],
        Shake => [
            <| |shake| shake * 1e-8,
            |> |s| s / 1e-8
        ],
        Jiffy => [
            <| |jiffy| jiffy * 3.33564e-11,
            |> |s| s / 3.33564e-11
        ],
        Galactic_Year => [
            <| |gay| gay * 19440000e6,
            |> |s| s / 19440000e6,
            aliases = ["galactic year"]
        ],
        Kermit => [
            <| |kermit| kermit * 864,
            |> |s| s / 864
        ],
        Third => [
            <| |third| third / 60,
            |> |s| s * 60
        ],
        Fourth => [
            <| |fourth| fourth / 3600,
            |> |s| s * 3600
        ]
    }
}
