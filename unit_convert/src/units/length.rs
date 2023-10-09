use crate::impl_units;

impl_units! {
    Length => {
        Meter => [
            <| |m| *m,
            |> |m| *m,
            aliases = ["m", "metre"],
            metric = true
        ],
        Inch => [
            <| |i| i * 0.0254,
            |> |m| m / 0.0254,
            aliases = ["in"]
        ],
        Thou => [
            <| |th| th * 0.0000254,
            |> |m| m / 0.0000254,
            aliases = ["mil"]
        ],
        Foot => [
            <| |f| f * 0.3048,
            |> |m| m / 0.3048,
            aliases = ["ft", "foot"]
        ],
        Yard => [
            <| |y| y * 0.9144,
            |> |m| m / 0.9144,
            aliases = ["yd"]
        ],
        Statute_Mile => [
            <| |mi| mi * 1609.344,
            |> |m| m / 1609.344,
            aliases = ["mi", "statute mile", "mile"]
        ],
        League => [
            <| |l| l * 4828.0417,
            |> |m| m / 4828.0417
        ],
        Astronmical_Unit => [
            <| |au| au * 1.495978707e11,
            |> |m| m / 1.495978707e11,
            aliases = ["au", "astronomical unit"]
        ],
        Siriometer => [
            <| |sir| sir * 1.495978707e17,
            |> |m| m / 1.495978707e17,
            aliases = ["sir"]
        ],
        Light_Year => [
            <| |ly| ly * 9460730472580800,
            |> |m| m / 9460730472580800,
            aliases = ["ly", "light-year"]
        ],
        Parsec => [
            <| |pc| pc * 3.0856776e16,
            |> |m| m / 3.0856776e16,
            aliases = ["pc"]
        ],
        Potrzebie => [
            <| |p| p * 0.0022633484517438173216473,
            |> |m| m / 0.0022633484517438173216473
        ],
        Furlong => [
            <| |fur| fur * 201.168,
            |> |m| m / 201.168,
            aliases = ["fur"]
        ],
        Planck_Length => [
            <| |lp| lp * 1.616255e-35,
            |> |m| m / 1.616255e-35,
            aliases = ["planck length"]
        ],
        Rod => [
            <| |rod| rod * 5.0292,
            |> |m| m / 5.0292
        ],
        Nautical_Mile => [
            <| |nmi| nmi * 1852,
            |> |m| m / 1852,
            aliases = ["nmi", "nautical mile"]
        ],
        Hammer_Unit => [
            <| |qu| qu * 0.01905,
            |> |m| m / 0.01905,
            aliases = ["qu", "hammer unit"]
        ],
        Rack_Unit => [
            <| |U| U * 0.04445,
            |> |m| m / 0.04445,
            aliases = ["U", "rack unit"]
        ],
        Hand => [
            <| |hh| hh * 0.1016,
            |> |m| m / 0.1016,
            aliases = ["hh", "hand"]
        ],
        Light_Second => [
            <| |ls| ls * 299792458,
            |> |m| m / 299792458,
            aliases = ["ls", "light-second"]
        ],
        Earth_Radius => [
            <| |RE| RE * 6.3781e6,
            |> |m| m / 6.3781e6,
            aliases = ["earth radius"]
        ],
        Lunar_Distance => [
            <| |LD| LD * 3.84399e8,
            |> |m| m / 3.84399e8,
            aliases = ["LD", "lunar distance"]
        ],
        Smoot => [
            <| |smoot| smoot * 1.7,
            |> |m| m / 1.7
        ],
        Megalithic_Yard => [
            <| |megalithic_yard| megalithic_yard * 0.83,
            |> |m| m / 0.83,
            aliases = ["megalithic yard"]
        ],
        Fingerbreadth => [
            <| |fingerbreadth| fingerbreadth * 0.01905,
            |> |m| m / 0.01905
        ],
        Double_Decker_Bus => [
            <| |ddb| ddb * 18.75,
            |> |m| m / 18.75,
            aliases = ["double-decker bus"]
        ],
        Barleycorn => [
            <| |barley| barley * 0.00846667,
            |> |m| m / 0.00846667
        ],
        Mickey => [
            <| |mickey| mickey * 1.27e-4,
            |> |m| m / 1.27e-4
        ],
        Nail => [
            <| |nail| nail * 0.05715,
            |> |m| m / 0.05715
        ],
        Altuve => [
            <| |altuve| altuve * 1.68,
            |> |m| m / 1.68
        ],
        Metric_Inch => [
            <| |metrinch| metrinch * 0.025,
            |> |m| m / 0.025,
            aliases = ["metric inch"]
        ],
        Metic_Foot => [
            <| |metrifoot| metrifoot * 0.3,
            |> |m| m / 0.3,
            aliases = ["metric foot"]
        ],
        Metric_Chain => [
            <| |metrichain| metrichain * 20,
            |> |m| m / 20,
            aliases = ["metric chain"]
        ],
        Metric_Lieue => [
            <| |metrilieue| metrilieue * 4000,
            |> |m| m / 4000,
            aliases = ["metric lieue"]
        ],
        Scandinavian_Mile => [
            <| |scanmile| scanmile * 10000,
            |> |m| m / 10000,
            aliases = ["scandinavian mile"]
        ],
        Fermi => [
            <| |fm| fm * 1e-15,
            |> |m| m / 1e-15,
            aliases = ["fm"]
        ],
        Pica => [
            <| |pica| pica * 0.0042333,
            |> |m| m / 0.0042333
        ]
    }
}
